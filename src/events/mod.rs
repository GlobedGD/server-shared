// bug in bitpiece crate i think
#![allow(clippy::unused_unit)]

use std::{
    collections::{HashMap, HashSet},
    io::Write,
    sync::Arc,
};

use arc_swap::ArcSwap;
use bitpiece::{B2, bitpiece};
use heapless::CapacityError;
use qunet::buffers::{BinaryWriter, ByteReader, ByteReaderError, HeapByteWriter};
use thiserror::Error;

mod builtins;
use builtins::*;
use tracing::trace;

const MAX_EVENT_LENGTH: usize = 1024;
const MAX_EVENT_COUNT: usize = 128;

#[bitpiece(8)]
#[derive(Default)]
pub struct EventFlags {
    pub target_players: bool,
    pub no_data: bool,
    pub reliable: bool,
    pub urgent: bool,
    pub sent_by_player: bool,

    pub padding: B2,

    pub more_flags: bool,
}

#[derive(Default, Clone)]
pub struct EventOptions {
    pub reliable: bool,
    pub urgent: bool,
    pub target_players: Vec<i32>,
    pub sent_by_player: Option<i32>,
}

#[derive(Default)]
pub struct EventStringCache {
    cache: ArcSwap<HashSet<Arc<str>>>,
}
#[derive(Error, Debug)]
pub enum EventDictionaryBuildError {
    #[error("Failed to decode event dictionary: {0}")]
    Decode(#[from] ByteReaderError),
    #[error("Too many events ({0})")]
    TooManyEvents(u32),
    #[error("Unsupported builtins version ({0})")]
    UnsupportedBuiltinsVersion(u32),
    #[error("Total event count at the end does not match the count in the dictionary")]
    EventCountMismatch,
    #[error("A string was too long")]
    StringTooLong,
}

impl From<CapacityError> for EventDictionaryBuildError {
    fn from(_: CapacityError) -> Self {
        EventDictionaryBuildError::StringTooLong
    }
}

#[derive(Error, Debug)]
pub enum EventEncodingError {
    #[error("Event not found in dictionary")]
    UnknownEvent,
    #[error("Failed to write event data: {0}")]
    WriteError(#[from] std::io::Error),
    #[error("{0}")]
    Custom(String),
}

#[derive(Error, Debug)]
pub enum EventDecodingError {
    #[error("Failed to decode event data: {0}")]
    Decode(#[from] ByteReaderError),
    #[error("Unknown event: {0}")]
    UnknownEvent(u32),
    #[error("Data too long in event ({0} bytes)")]
    DataTooLong(usize),
    #[error("Too many events sent ({0})")]
    TooManyEvents(usize),
}

pub struct RawDecodedEvent<'a> {
    pub id: Arc<str>,
    pub data: &'a [u8],
    pub options: EventOptions,
}

#[derive(Clone)]
pub struct OwnedEvent {
    pub id: Arc<str>,
    pub data: Vec<u8>,
    pub options: EventOptions,
}

pub struct EventEncoder {
    mapping: Vec<Arc<str>>,
    inv_mapping: HashMap<Arc<str>, u32>,
}

impl EventEncoder {
    pub fn create_with_dictionary(
        data: &[u8],
        cache: &EventStringCache,
        game: bool,
    ) -> Result<Self, EventDictionaryBuildError> {
        let mut reader = ByteReader::new(data);

        let builtins_version = reader.read_u32()?;
        let total_events = reader.read_u32()?;
        if total_events > 1024 {
            return Err(EventDictionaryBuildError::TooManyEvents(total_events));
        }

        let mut mapping = Vec::with_capacity(total_events as usize);

        if game {
            build_game_builtins(builtins_version, &mut mapping, cache)?;
        } else {
            build_central_builtins(builtins_version, &mut mapping, cache)?;
        }

        while reader.remaining() > 0 {
            let mod_id = heapless::String::<256>::try_from(reader.read_string_u8()?)?;

            let count = reader.read_varuint()?;

            for _ in 0..count {
                let mut full_id = mod_id.clone();
                let event_id = reader.read_string_var()?;

                // make up a full id
                full_id.push('/')?;
                full_id.push_str(event_id)?;

                mapping.push(cache.get(&full_id));
            }
        }

        if mapping.len() != total_events as usize {
            return Err(EventDictionaryBuildError::EventCountMismatch);
        }

        // build inverse mapping for speed
        let inv_mapping = mapping
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i as u32))
            .collect();

        Ok(Self {
            mapping,
            inv_mapping,
        })
    }

    fn lookup(&self, event_id: u32) -> Option<&Arc<str>> {
        self.mapping.get(event_id as usize)
    }

    fn lookup_id(&self, event: &str) -> Option<u32> {
        self.inv_mapping.get(event).copied()
    }

    pub fn knows_event(&self, event: &str) -> bool {
        self.inv_mapping.contains_key(event)
    }

    pub fn encode_event(
        &self,
        id: &str,
        data: &[u8],
        options: &EventOptions,
        writer: &mut impl Write,
    ) -> Result<(), EventEncodingError> {
        let mut writer = BinaryWriter::new(writer);
        let event_id = self.lookup_id(id).ok_or(EventEncodingError::UnknownEvent)?;
        let total = self.mapping.len();

        if total < 256 {
            writer.write_u8(event_id as u8)?;
        } else if total < 65536 {
            writer.write_u16(event_id as u16)?;
        } else {
            writer.write_u32(event_id)?;
        }

        let mut flags = EventFlags::default();
        if !options.target_players.is_empty() {
            flags.set_target_players(true);
        }
        if data.is_empty() {
            flags.set_no_data(true);
        }
        if options.reliable {
            flags.set_reliable(true);
        }
        if options.urgent {
            flags.set_urgent(true);
        }
        if options.sent_by_player.is_some() {
            flags.set_sent_by_player(true);
        }

        writer.write_bits(flags)?;
        if !options.target_players.is_empty() {
            writer.write_varuint(options.target_players.len() as u64)?;
            for player in &options.target_players {
                writer.write_i32(*player)?;
            }
        }

        if let Some(player_id) = options.sent_by_player {
            writer.write_i32(player_id)?;
        }

        if !data.is_empty() {
            writer.write_varuint(data.len() as u64)?;
            writer.write_bytes(data)?;
        }

        Ok(())
    }

    pub fn encode_events(
        &self,
        events: &[OwnedEvent],
        writer: &mut impl Write,
    ) -> Result<(), EventEncodingError> {
        let mut writer = BinaryWriter::new(writer);
        writer.write_varuint(events.len() as u64)?;

        for event in events {
            self.encode_event(&event.id, &event.data, &event.options, &mut writer)?;
        }

        Ok(())
    }

    pub fn decode_event<'b>(
        &self,
        reader: &mut ByteReader<'b>,
    ) -> Result<RawDecodedEvent<'b>, EventDecodingError> {
        let event_id = if self.mapping.len() < 256 {
            reader.read_u8()? as u32
        } else if self.mapping.len() < 65536 {
            reader.read_u16()? as u32
        } else {
            reader.read_u32()?
        };

        let id = self
            .lookup(event_id)
            .ok_or(EventDecodingError::UnknownEvent(event_id))?
            .clone();

        let flags: EventFlags = reader.read_bits()?;

        let mut target_players = Vec::new();
        if flags.target_players() {
            let count = reader.read_varuint()?;
            for _ in 0..count {
                target_players.push(reader.read_i32()?);
            }
        }

        let sent_by_player = if flags.sent_by_player() {
            Some(reader.read_i32()?)
        } else {
            None
        };

        let data = if !flags.no_data() {
            let len = reader.read_varuint()? as usize;
            if len > MAX_EVENT_LENGTH {
                return Err(EventDecodingError::DataTooLong(len));
            }

            // grab the remainder (which is data bytes + rest of data), then after skipping we know we have 'len' bytes left
            let rem = reader.remaining_bytes();
            reader.skip_bytes(len)?;
            &rem[..len]
        } else {
            &[]
        };

        Ok(RawDecodedEvent {
            id,
            data,
            options: EventOptions {
                reliable: flags.reliable(),
                urgent: flags.urgent(),
                sent_by_player,
                target_players,
            },
        })
    }

    pub fn decode_events_owned(&self, data: &[u8]) -> Result<Vec<OwnedEvent>, EventDecodingError> {
        trace!("decoding event buf: {data:x?}");

        if data.is_empty() {
            return Ok(Vec::new());
        }

        let mut reader = ByteReader::new(data);
        let mut events = Vec::new();

        let count = reader.read_varuint()?;
        if count as usize > MAX_EVENT_COUNT {
            return Err(EventDecodingError::TooManyEvents(count as usize));
        }

        events.reserve(count as usize);

        for _ in 0..count {
            let event = self.decode_event(&mut reader)?;
            events.push(event.into());
        }

        Ok(events)
    }
}

impl EventStringCache {
    /// Unlike `default()`, this will pre-fill the cache with built-in events
    pub fn new() -> Self {
        let this = Self::default();

        // fill the cache with built-in events immediately
        let mut vec = Vec::new();
        build_central_builtins(CENTRAL_BUILTINS_MAX, &mut vec, &this).unwrap();
        build_game_builtins(GAME_BUILTINS_MAX, &mut vec, &this).unwrap();

        // disregard value of the vector, since the functions already call .get and will pre-fill the cache

        this
    }

    pub fn get(&self, value: &str) -> Arc<str> {
        let cache = self.cache.load();
        if let Some(cached) = cache.get(value) {
            return cached.clone();
        }

        let arc_str: Arc<str> = Arc::from(value);
        self.cache.rcu(|cache| {
            let mut new_cache = (**cache).clone();
            new_cache.insert(arc_str.clone());
            Arc::new(new_cache)
        });

        arc_str
    }
}

impl From<RawDecodedEvent<'_>> for OwnedEvent {
    fn from(value: RawDecodedEvent) -> Self {
        Self {
            id: value.id,
            data: value.data.to_vec(),
            options: value.options,
        }
    }
}

impl OwnedEvent {
    pub fn from_encodable<'a, T: EventEncode>(
        value: &'a T,
        options: EventOptions,
        cache: impl Into<Option<&'a EventStringCache>>,
    ) -> Self {
        let mut writer = HeapByteWriter::new();
        value.encode(&mut writer);

        let id = match cache.into() {
            Some(cache) => cache.get(T::id()),
            None => Arc::from(T::id()),
        };

        Self {
            id,
            data: writer.into_inner(),
            options,
        }
    }

    pub fn max_encoded_size(&self) -> usize {
        // 4 for event id, 1 for flags
        let mut count = 4 + 1;

        if !self.options.target_players.is_empty() {
            count += 4 + 4 * self.options.target_players.len();
        }

        if self.options.sent_by_player.is_some() {
            count += 4;
        }

        count += self.data.len();

        count
    }
}

/// an interface for encoding a custom struct into an OwnedEvent
pub trait EventEncode {
    fn size_bound(&self) -> Option<usize>;
    fn id() -> &'static str;

    fn encode(&self, writer: &mut HeapByteWriter);
}
