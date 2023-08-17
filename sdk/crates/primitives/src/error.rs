#[derive(Clone, Debug)]
pub enum Error {
    /// An error was emitted from the codec
    Codec(crate::CodecError),
}

impl From<crate::CodecError> for Error {
    fn from(err: crate::CodecError) -> Self {
        Error::Codec(err)
    }
}
