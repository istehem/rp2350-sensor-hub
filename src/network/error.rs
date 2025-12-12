#[derive(Debug)]
pub enum ReqwlessError {
    Error(reqwless::Error),
}

impl defmt::Format for ReqwlessError {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        match self {
            Self::Error(reqwless::Error::Dns) => defmt::write!(fmt, "{}", "Dns"),
            Self::Error(reqwless::Error::Network(_)) => defmt::write!(fmt, "{}", "Network"),
            Self::Error(reqwless::Error::Codec) => defmt::write!(fmt, "{}", "Codec"),
            Self::Error(reqwless::Error::InvalidUrl(_)) => defmt::write!(fmt, "{}", "InvalidUrl"),
            Self::Error(reqwless::Error::Tls(_)) => defmt::write!(fmt, "{}", "Tls"),
            Self::Error(reqwless::Error::BufferTooSmall) => {
                defmt::write!(fmt, "{}", "BufferTooSmall")
            }
            Self::Error(reqwless::Error::AlreadySent) => defmt::write!(fmt, "{}", "AlreadySent"),
            Self::Error(reqwless::Error::IncorrectBodyWritten) => {
                defmt::write!(fmt, "{}", "IncorrectBodyWritten")
            }
            Self::Error(reqwless::Error::ConnectionAborted) => {
                defmt::write!(fmt, "{}", "ConnectionAborted")
            }
        }
    }
}

impl From<reqwless::Error> for ReqwlessError {
    fn from(err: reqwless::Error) -> Self {
        Self::Error(err)
    }
}
