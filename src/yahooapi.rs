use chrono::prelude::*;

pub fn parse_yahoo_date(string: &str) -> chrono::ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M")
}

