#[derive(Debug)]
pub enum ReqwlessError {
    Error(reqwless::Error),
}

impl defmt::Format for ReqwlessError {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        match self {
            Self::Error(reqwless::Error::Dns) => defmt::write!(fmt, "{}", "Dns"),
            Self::Error(reqwless::Error::Network(err_kind)) => {
                defmt::write!(fmt, "Network({:?})", defmt::Debug2Format(err_kind))
            }
            Self::Error(reqwless::Error::Codec) => defmt::write!(fmt, "{}", "Codec"),
            Self::Error(reqwless::Error::InvalidUrl(err_kind)) => {
                defmt::write!(fmt, "InvalidUrl({:?})", defmt::Debug2Format(err_kind))
            }
            Self::Error(reqwless::Error::Tls(err_kind)) => {
                defmt::write!(fmt, "Tls({:?})", defmt::Debug2Format(err_kind))
            }
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
