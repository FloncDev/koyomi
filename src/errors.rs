use scraper::error::SelectorErrorKind;

pub enum ParseError {
    SelectorError,
    ChronoParseError,
}

impl From<SelectorErrorKind<'_>> for ParseError {
    fn from(err: SelectorErrorKind) -> Self {
        tracing::error!("An error occured while parsing a selector. {:#?}", err);
        ParseError::SelectorError
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(err: chrono::ParseError) -> Self {
        tracing::error!("An error occured while parsing a date/time. {:#?}", err);
        ParseError::ChronoParseError
    }
}

impl From<chrono::ParseWeekdayError> for ParseError {
    fn from(err: chrono::ParseWeekdayError) -> Self {
        tracing::error!("An error occured while parsing a weekday. {:#?}", err);
        ParseError::ChronoParseError
    }
}
