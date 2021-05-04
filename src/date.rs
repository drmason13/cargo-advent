// use crate::error::no_assume_message;
use crate::error::AdventError;
use chrono::prelude::*;
use std::convert::TryInto;

pub struct Advent {
    year: Year,
    day: Day,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Year(u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Day(u32);

impl Day {
    fn new(n: u32) -> Result<Self, AdventError> {
        if n < 1 || 25 < n {
            Err(AdventError::DayOutsideValidRange(n))
        } else {
            Ok(Day(n))
        }
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl Year {
    fn new(n: u32) -> Result<Self, AdventError> {
        if n < 2015 || 9999 < n {
            Err(AdventError::YearOutsideValidRange(n))
        } else {
            Ok(Year(n))
        }
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl Advent {
    pub fn year(&self) -> Year {
        self.year.clone()
    }

    pub fn day(&self) -> Day {
        self.day.clone()
    }

    pub fn new(y: Option<u32>, d: Option<u32>) -> Result<Self, AdventError> {
        let now = Local::now();

        Ok(match (y, d) {
            (Some(y), Some(d)) => Advent::from_year_and_day(y, d, &now),
            (None, None) => Advent::now(&now),
            (Some(_y), None) => Err(AdventError::NeedDayGivenYear),
            (None, Some(d)) => Advent::from_day(d, &now),
        }?)
    }

    fn now(now: &DateTime<Local>) -> Result<Self, AdventError> {
        let day = Day::new(now.day())?;
        if now.month() != 12 {
            return Err(AdventError::OutsideAdvent);
        }
        let year = Year::new(
            now.year()
                .try_into()
                .map_err(|_| AdventError::InvalidYear)?,
        )?;

        Ok(Advent { day, year })
    }

    fn from_year_and_day(y: u32, d: u32, now: &DateTime<Local>) -> Result<Advent, AdventError> {
        let this_year = now.year().try_into().unwrap();
        let this_month = now.month();
        let today = now.day();

        // at this stage, year and day are within valid ranges but are not checked against the current date
        let year = Year::new(y)?;
        let day = Day::new(d)?;

        if y > this_year {
            // the year is in the future, that's a no go
            return Err(AdventError::AheadOfAdvent);
        }

        if y == this_year {
            // if the year is this year, it better be December or it ain't Advent
            if this_month == 12 {
                // even if it is December this year, the day better not be in the future
                if d > today {
                    return Err(AdventError::AheadOfAdvent);
                };
            } else {
                return Err(AdventError::AheadOfAdvent);
            }
        }

        Ok(Advent { year, day })
    }

    fn from_day(d: u32, now: &DateTime<Local>) -> Result<Self, AdventError> {
        let mut this_year = now.year().try_into().unwrap();
        let this_month = now.month();
        let today = now.day();

        // if it is December, assume they want to use this year
        if this_month == 12 {
            if d > today {
                return Err(AdventError::AheadOfAdvent);
            };
        } else {
            // assume they want to use last year
            this_year -= 1;
        }

        let day = Day::new(d)?;
        let year = Year::new(this_year)?;

        Ok(Advent { year, day })
    }
}
