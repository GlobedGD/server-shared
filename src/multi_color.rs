use base64::{DecodeSliceError, Engine, engine::general_purpose::STANDARD_NO_PAD as b64e};
use qunet::buffers::{ByteReader, ByteReaderError, ByteWriter};
use serde::{Deserialize, Serialize, de::Visitor};
use smallvec::SmallVec;
use std::fmt::Write;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn encode(&self, writer: &mut ByteWriter<'_>) {
        writer.write_u8(self.r);
        writer.write_u8(self.g);
        writer.write_u8(self.b);
    }

    pub fn to_hex_string(self, hash: bool) -> heapless::String<8> {
        let mut str = heapless::String::new();

        if hash {
            let _ = str.push('#');
        }

        write!(str, "{:02x}{:02x}{:02x}", self.r, self.g, self.b).unwrap();

        str
    }

    pub fn from_hex_string(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);

        if s.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
}

pub type ColorVec = SmallVec<[RGBColor; 8]>;

/// Represents up to 63 colors, with 3 variations: static, tinting, gradient.
/// Can be efficiently encoded to a byte writer and back.
#[derive(Clone, Debug)]
pub enum MultiColor {
    Static(RGBColor),
    Tinting(ColorVec),
    Gradient(ColorVec),
}

impl Default for MultiColor {
    fn default() -> Self {
        Self::Static(RGBColor::new(255, 255, 255))
    }
}

#[derive(Debug, Error)]
pub enum MultiColorDecodeError {
    #[error("Invalid data: {0}")]
    Decode(#[from] ByteReaderError),
    #[error("Invalid color type")]
    InvalidType,
    #[error("Invalid base64: {0}")]
    Base64(#[from] DecodeSliceError),
}

#[derive(Debug, Error)]
pub enum MultiColorParseError {
    #[error("Invalid hex color: '{0}'")]
    InvalidHex(String),
    #[error("Too many colors ({0})")]
    TooManyColors(usize),
}

impl MultiColor {
    pub fn decode(reader: &mut ByteReader<'_>) -> Result<Self, MultiColorDecodeError> {
        let header = reader.read_u8()?;
        let count = (header & 0b00111111) as usize;

        match header >> 6 {
            0b00 => Err(MultiColorDecodeError::InvalidType),
            0b01 => Ok(Self::Static(Self::read_rgb(reader)?)),
            0b10 => Ok(Self::Tinting(Self::read_multi(reader, count)?)),
            0b11 => Ok(Self::Gradient(Self::read_multi(reader, count)?)),
            _ => unreachable!(),
        }
    }

    pub fn decode_from_string(s: &str) -> Result<Self, MultiColorDecodeError> {
        let mut buf = [0u8; 256];
        let size = b64e.decode_slice(s, &mut buf)?;
        let mut reader = ByteReader::new(&buf[..size]);

        Self::decode(&mut reader)
    }

    pub fn parse_from_human_str(s: &str) -> Result<Self, MultiColorParseError> {
        if s.contains('>') {
            Ok(Self::Tinting(Self::parse_separated_color_str(s, '>')?))
        } else if s.contains('|') {
            Ok(Self::Gradient(Self::parse_separated_color_str(s, '|')?))
        } else {
            match RGBColor::from_hex_string(s) {
                Some(c) => Ok(Self::Static(c)),
                None => Err(MultiColorParseError::InvalidHex(s.to_owned())),
            }
        }
    }

    fn parse_separated_color_str(s: &str, sep: char) -> Result<ColorVec, MultiColorParseError> {
        let mut vec = ColorVec::new();

        for col in s.split(sep) {
            let col = col.trim();
            match RGBColor::from_hex_string(col) {
                Some(c) => vec.push(c),
                None => return Err(MultiColorParseError::InvalidHex(col.to_owned())),
            }
        }

        Ok(vec)
    }

    fn read_rgb(reader: &mut ByteReader<'_>) -> Result<RGBColor, MultiColorDecodeError> {
        let r = reader.read_u8()?;
        let g = reader.read_u8()?;
        let b = reader.read_u8()?;
        Ok(RGBColor { r, g, b })
    }

    fn read_multi(
        reader: &mut ByteReader<'_>,
        count: usize,
    ) -> Result<ColorVec, MultiColorDecodeError> {
        let mut out = SmallVec::new();
        out.reserve(count);

        for _ in 0..count {
            out.push(Self::read_rgb(reader)?);
        }

        Ok(out)
    }

    pub fn encode(&self, writer: &mut ByteWriter<'_>) {
        let ty: u8 = match self {
            Self::Static(_) => 0b01,
            Self::Tinting(_) => 0b10,
            Self::Gradient(_) => 0b11,
        };

        let mut header = ty << 6;

        match self {
            Self::Static(rgb) => {
                writer.write_u8(header);
                rgb.encode(writer);
            }

            Self::Gradient(colors) | Self::Tinting(colors) => {
                assert!(colors.len() < 64);

                header |= colors.len() as u8 & 0b00111111;

                writer.write_u8(header);

                for rgb in colors {
                    rgb.encode(writer);
                }
            }
        }
    }

    pub fn encode_to_string(&self) -> String {
        let mut buf = [0u8; 256];
        let mut writer = ByteWriter::new(&mut buf);

        self.encode(&mut writer);

        b64e.encode(writer.written())
    }

    pub fn encoded_len(&self) -> usize {
        let common = 1;

        match self {
            Self::Static(_) => common + 3,
            Self::Tinting(c) | Self::Gradient(c) => common + c.len() * 3,
        }
    }
}

struct MultiColorVisitor;

impl<'de> Visitor<'de> for MultiColorVisitor {
    type Value = MultiColor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a multi color string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        MultiColor::parse_from_human_str(v).map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for MultiColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MultiColorVisitor)
    }
}

impl Serialize for MultiColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Static(rgb) => serializer.serialize_str(&rgb.to_hex_string(true)),

            _ => panic!("cannot serialize non-Static multicolor"),
        }
    }
}
