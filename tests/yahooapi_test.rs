use chrono::prelude::*;

#[path="../src/yahooapi.rs"]
mod yahooapi;

#[test]
fn test_parse() {
    assert_eq!(yahooapi::parse_yahoo_date("202101161655"),
               Ok(NaiveDate::from_ymd(2021, 1, 16).and_hms(16, 55, 0)));
}

