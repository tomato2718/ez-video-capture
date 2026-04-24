#[derive(Debug)]
pub enum Error {
    FailedToOpenSource(String),
    NoVideoStream,
    ReadError,
    UnsupportedPlatform,
    FailedToOpenDecoder,
    NoHwConfig,
    FailedToOpenWriter,
    FailedToWriteFrame,
    FailedToWriteTrailer,
    ConnectionClosed,
}
