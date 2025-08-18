use std::{any::Any, fmt::Display};

use capnp::message::{Allocator, Builder};
use qunet::buffers::ByteReaderError;
use thiserror::Error;

#[repr(align(8))]
pub struct CapnpAlloc<const N: usize> {
    buf: [u8; N],
    called: bool,
}

unsafe impl<const N: usize> Allocator for CapnpAlloc<N> {
    #[inline]
    fn allocate_segment(&mut self, size_words: u32) -> (*mut u8, u32) {
        if self.called {
            panic!("CapnpAlloc::allocate_segment called multiple times");
        }

        let size = (size_words * 8) as usize;

        self.called = true;

        if size > N {
            panic!("Not enough space in CapnpAlloc");
        }

        (self.buf.as_mut_ptr(), (N / 8) as u32)
    }

    #[inline]
    unsafe fn deallocate_segment(&mut self, _ptr: *mut u8, _word_size: u32, _words_used: u32) {}
}

impl<const N: usize> CapnpAlloc<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            called: false,
        }
    }

    pub fn into_builder(self) -> Builder<Self> {
        Builder::new(self)
    }
}

impl<const N: usize> Default for CapnpAlloc<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CapnpBorrowAlloc<'a> {
    buf: &'a mut [u8],
    called: bool,
}

unsafe impl<'a> Allocator for CapnpBorrowAlloc<'a> {
    #[inline]
    fn allocate_segment(&mut self, size_words: u32) -> (*mut u8, u32) {
        if self.called {
            panic!("CapnpAlloc::allocate_segment called multiple times");
        }

        let size = (size_words * 8) as usize;

        self.called = true;

        if size > self.buf.len() {
            panic!("Not enough space in CapnpAlloc");
        }

        (self.buf.as_mut_ptr(), (self.buf.len() / 8) as u32)
    }

    #[inline]
    unsafe fn deallocate_segment(&mut self, _ptr: *mut u8, _word_size: u32, _words_used: u32) {}
}

impl<'a> CapnpBorrowAlloc<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        // zero the buffer
        buf.fill(0);

        // safety: buffer is zeroed
        unsafe { Self::new_assert_zeroed(buf) }
    }

    pub unsafe fn new_assert_zeroed(buf: &'a mut [u8]) -> Self {
        #[cfg(debug_assertions)]
        {
            if !buf.iter().all(|&x| x == 0) {
                panic!("CapnpBorrowAlloc buffer must be zeroed");
            }
        }

        Self { buf, called: false }
    }

    pub fn into_builder(self) -> Builder<Self> {
        Builder::new(self)
    }
}

// Encoding macros

#[derive(Debug, Error)]
pub enum DataDecodeError {
    #[error("capnp error: {0}")]
    Capnp(#[from] capnp::Error),
    #[error("invalid enum/union discriminant")]
    InvalidDiscriminant,
    #[error("invalid utf-8 string: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("username too long")]
    UsernameTooLong,
    #[error("no message handler for the incoming message type")]
    NoMessageHandler,
    #[error("message too long: {0} bytes")]
    MessageTooLong(usize),
    #[error("failed to decode message length: {0}")]
    InvalidBinary(#[from] ByteReaderError),
    #[error("supplied string was longer than permitted: {0} bytes (limit: {1})")]
    StringTooLong(usize, usize),
    #[error("nan/inf value in floating point data")]
    InvalidFloat,
    #[error("data is logically invalid, validation failed")]
    ValidationFailed,
}

#[macro_export]
macro_rules! decode_message_match {
    ($($schema:ident)::*, $srvr:expr, $data:expr, $unpacked_data:ident, {$($variant:ident($msg_var:ident) => {  $($t:tt)* }),* $(,)?}) => {{
        use $($schema::)*{self as schema};
        use $crate::encoding::MaybeIntoResult;

        let _res: Result<_, $crate::encoding::DataDecodeError> = try {
            let mut reader = qunet::buffers::ByteReader::new($data.as_bytes());
            let unpacked_len = reader.read_varuint()? as usize;

            if unpacked_len > 1024 * 1024 {
                Err($crate::encoding::DataDecodeError::MessageTooLong(unpacked_len))?;
            }

            // allocate a buffer for the unpacked message
            let mut $unpacked_data = $srvr.request_buffer(unpacked_len);

            let mut rembuf = reader.remaining_bytes();
            let reader = capnp::serialize_packed::read_message_no_alloc(
                &mut rembuf,
                unsafe { $unpacked_data.write_window(unpacked_len).unwrap() },
                capnp::message::ReaderOptions::new(),
            )?;

            $data.discard();

            let message = reader
                .get_root::<schema::message::Reader>()
                .map_err(|_| $crate::encoding::DataDecodeError::InvalidDiscriminant)?;

            match message.which().map_err(|_| $crate::encoding::DataDecodeError::InvalidDiscriminant)? {
                $(schema::message::Which::$variant(msg) => {
                    let $msg_var = msg._maybe_into_result()?;
                    $($t)*
                })*

                _ => Err($crate::encoding::DataDecodeError::NoMessageHandler)?,
            }
        };

        _res
    }};
}

#[derive(Debug)]
pub enum EncodeMessageError {
    Panic {
        payload: Box<dyn Any + Send + 'static>,
        file: &'static str,
        line: u32,
    },

    MessageTooLong,
}

impl std::error::Error for EncodeMessageError {}

impl Display for EncodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Panic {
                payload,
                file,
                line,
            } => {
                if let Some(err) = payload.downcast_ref::<String>() {
                    write!(f, "error: {err} ({file}:{line})")
                } else if let Some(err) = payload.downcast_ref::<&str>() {
                    write!(f, "error: {err} ({file}:{line})")
                } else {
                    write!(
                        f,
                        "unknown error type: {:?} ({}:{})",
                        (**payload).type_id(),
                        file,
                        line
                    )
                }
            }

            Self::MessageTooLong => {
                write!(f, "message too long, could not encode")
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! encode_with_builder {
    ($($schema:ident)::*, $srvr:expr, $estcap:expr, $builder:expr, $msg:ident => $code:expr) => {{
        use $($schema::)*{self as schema};

        let _res: Result<qunet::message::BufferKind, $crate::encoding::EncodeMessageError> = try {
            let server = $srvr;

            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut $msg = $builder.init_root::<schema::message::Builder>();
                $code
            }))
            .map_err(|e| $crate::encoding::EncodeMessageError::Panic {
                payload: e,
                file: file!(),
                line: line!(),
            })?;

            let ser_size = capnp::serialize::compute_serialized_size_in_words(&$builder) * 8;

            #[cfg(debug_assertions)]
            {
                if ser_size <= $estcap {
                    let wasted_bytes = $estcap - ser_size as usize;
                    let wasted_percent = (wasted_bytes as f64 / $estcap as f64) * 100.0;

                    tracing::trace!("Encoding used {}/{} bytes ({wasted_percent:.1}% wasted) ({}:{})", ser_size, $estcap, file!(), line!());
                } else {
                    tracing::warn!("Encoding used {}/{} bytes which is a bug ({}:{})", ser_size, $estcap, file!(), line!());
                }
            }

            // the constant here is added for the varuint length prefix (4) and for potential packing overhead (4)
            let mut buf = server.request_buffer(ser_size + 8);

            let mut tmp_len_buf = [0u8; 4];
            let mut len_buf = qunet::buffers::ByteWriter::new(&mut tmp_len_buf);
            len_buf.write_varuint(ser_size as u64).map_err(|_| $crate::encoding::EncodeMessageError::MessageTooLong)?;
            let len_written = len_buf.written();
            buf.append_bytes(len_written);

            // this must never fail at this point
            capnp::serialize_packed::write_message(&mut buf, &$builder).expect("capnp write failed");

            buf
        };

        _res
    }};
}

/// Encodes a message into a buffer allocated by the qunet server, using the provided closure.
/// You are required to pass in the estimated maximum message size in bytes, if it proves to be too small,
/// a panic will occur and subsequently be caught and returned as an error.
#[macro_export]
macro_rules! encode_message_unsafe {
    ($($schema:ident)::*, $srvr:expr, $estcap:expr, $msg:ident => $code:expr) => {{
        let mut builder = $crate::encoding::CapnpAlloc::<$estcap>::new().into_builder();

        $crate::encode_with_builder!($($schema)::*, $srvr, $estcap, builder, $msg => $code)
    }};
}

/// Like `encode_message_unsafe!`, but uses heap buffers from server's bufferpool.
/// You are required to pass in the estimated maximum message size in bytes, if it proves to be too small,
/// a panic will occur and subsequently be caught and returned as an error.
#[macro_export]
macro_rules! encode_message_heap {
    ($($schema:ident)::*, $srvr:expr, $estcap:expr, $msg:ident => $code:expr) => {{
        let server = $srvr;

        // round up to a multiple of 8
        let estcap = ($estcap + 7) & !7;

        let mut buffer = server.request_buffer(estcap);

        // safety: we just allocated a buffer of size $estcap
        let wnd = unsafe { buffer.write_window(estcap).unwrap() };
        debug_assert!(wnd.len() >= estcap);

        let mut builder = $crate::encoding::CapnpBorrowAlloc::new(wnd).into_builder();

        $crate::encode_with_builder!($($schema)::*, server, estcap, builder, $msg => $code)
    }};
}

/// Resolves to either `encode_message_unsafe!` or `encode_message_heap!` depending on the size of the allocation.
/// Size must be a constant expression.
#[macro_export]
macro_rules! encode_message {
    ($($schema:ident)::*, $srvr:expr, $estcap:expr, $msg:ident => $code:expr) => {{
        if $estcap <= 2048 {
            $crate::encode_message_unsafe!($($schema)::*, $srvr, $estcap, $msg => $code)
        } else {
            $crate::encode_message_heap!($($schema)::*, $srvr, $estcap, $msg => $code)
        }
    }};
}

pub fn heapless_str_from_reader<'a, const N: usize>(
    reader: capnp::text::Reader<'a>,
) -> Result<heapless::String<N>, DataDecodeError> {
    let s = reader.to_str()?;
    heapless::String::try_from(s).map_err(|_| DataDecodeError::StringTooLong(s.len(), N))
}

// (very hacky) trait to allow us to use Void as a message type and invoke ? operator on it

pub trait MaybeIntoResult: Sized {
    type Output = Self;

    fn _maybe_into_result(self) -> Result<Self::Output, DataDecodeError>;
}

impl<T, E> MaybeIntoResult for Result<T, E>
where
    E: Into<DataDecodeError>,
{
    type Output = T;

    fn _maybe_into_result(self) -> Result<T, DataDecodeError> {
        self.map_err(Into::into)
    }
}

impl MaybeIntoResult for () {
    type Output = ();

    fn _maybe_into_result(self) -> Result<(), DataDecodeError> {
        Ok(())
    }
}
