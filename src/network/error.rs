#[derive(Debug)]
pub enum ReqwlessError {
    Error(reqwless::Error),
}

impl defmt::Format for ReqwlessError {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        match self {
            Self::Error(reqwless::Error::Network(_)) => defmt::write!(fmt, "{}", "Network"),
            Self::Error(_) => defmt::write!(fmt, "{}", "Other"),
        }
    }
}

impl From<reqwless::Error> for ReqwlessError {
    fn from(err: reqwless::Error) -> Self {
        Self::Error(err)
    }
}
