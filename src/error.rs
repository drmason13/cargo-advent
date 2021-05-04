use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Invalid year for Advent of Code")]
    InvalidYear,
    #[error("It is not currently Advent, please specify a year and/or day to choose an input")]
    OutsideAdvent,
    #[error("That day's input will not be available yet")]
    AheadOfAdvent,
    #[error("The year {0} is not between 2015 and 9999 inclusive and is therefore invalid")]
    YearOutsideValidRange(u32),
    #[error("The day {0} is not between 1 and 25 inclusive and is therefore invalid")]
    DayOutsideValidRange(u32),
    #[error("Cannot infer the day you want from only a year, please provide a day")]
    NeedDayGivenYear,
}
