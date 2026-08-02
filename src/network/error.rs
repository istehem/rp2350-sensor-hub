#[derive(Debug)]
pub enum SendMeasurementError {
    ReqwlessError(reqwless::Error),
    SerializationError,
}

impl defmt::Format for SendMeasurementError {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        match self {
            Self::ReqwlessError(reqwless::Error::Dns) => defmt::write!(fmt, "{}", "Dns"),
            Self::ReqwlessError(reqwless::Error::Network(err_kind)) => {
                defmt::write!(fmt, "Network({:?})", defmt::Debug2Format(err_kind))
            }
            Self::ReqwlessError(reqwless::Error::Codec) => defmt::write!(fmt, "{}", "Codec"),
            Self::ReqwlessError(reqwless::Error::InvalidUrl(err_kind)) => {
                defmt::write!(fmt, "InvalidUrl({:?})", defmt::Debug2Format(err_kind))
            }
            Self::ReqwlessError(reqwless::Error::Tls(err_kind)) => {
                defmt::write!(fmt, "Tls({:?})", defmt::Debug2Format(err_kind))
            }
            Self::ReqwlessError(reqwless::Error::BufferTooSmall) => {
                defmt::write!(fmt, "{}", "BufferTooSmall")
            }
            Self::ReqwlessError(reqwless::Error::AlreadySent) => {
                defmt::write!(fmt, "{}", "AlreadySent")
            }
            Self::ReqwlessError(reqwless::Error::IncorrectBodyWritten) => {
                defmt::write!(fmt, "{}", "IncorrectBodyWritten")
            }
            Self::ReqwlessError(reqwless::Error::ConnectionAborted) => {
                defmt::write!(fmt, "{}", "ConnectionAborted")
            }
            Self::SerializationError => {
                defmt::write!(fmt, "{}", "SerializationError")
            }
        }
    }
}

impl From<reqwless::Error> for SendMeasurementError {
    fn from(err: reqwless::Error) -> Self {
        Self::ReqwlessError(err)
    }
}
