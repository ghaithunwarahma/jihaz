extern crate time;

use time::{Date as DateOg, Month};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
// use druid::Data;

use super::FormatTime;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Date(u16, u16);

impl Date {
    pub const fn new(year: u16, ordinal: u16) -> Self {
        Self(year, ordinal)
    }

    pub fn from_year_month_day(year: u16, month: u8, day: u8) -> Self {
        let month = Month::try_from(month).unwrap();
        let date = DateOg::from_calendar_date(year as i32, month, day).unwrap();
        Self(year, date.ordinal())
    }

    pub fn now() -> Self {
        // let res = crate::common::make_day_month_full_year();
        // use std::convert::TryFrom;
        // let month = Month::try_from(res.1).unwrap();
        // let date = DateOg::from_calendar_date(res.2 as i32, month, res.0).unwrap();
        // Self(res.2, date.ordinal())

        let naive_date = chrono::Local::now().date().naive_local();
        use chrono::Datelike;
        let year = naive_date.year();
        debug_assert!(year >= 0i32);
        Self(year as u16, naive_date.ordinal() as u16)
    }

    pub fn date(&self) -> DateOg {
        DateOg::from_ordinal_date(self.0 as i32, self.1).unwrap()
    }

    pub fn before(&self, other: &Self) -> bool {
        self.lt(&other)
    }

    pub const fn year(&self) -> u16 {
        self.0
    }

    pub const fn ordinal(&self) -> u16 {
        self.1
    }

    pub fn same_calendar_month(&self, other: &Date) -> bool {
        // self.0.same(&other.0) && self.date().month() == other.date().month()
        self.0 == other.0 && self.date().month() == other.date().month()
    }

    pub fn one_month_forward(self) -> Self {
        use time::util::days_in_year_month;
        let date = self.date();
        let next_month = date.month().next();
        let next_month_days = days_in_year_month(date.year(), next_month);
        let mut year = date.year();
        if let Month::January = next_month {
            year += 1;
        }
        let day = date.day().min(next_month_days);
        DateOg::from_calendar_date(year, next_month, day).unwrap().into()
    }

    pub fn one_month_backward(self) -> Self {
        use time::util::days_in_year_month;
        let date = self.date();
        let prev_month = date.month().previous();
        let prev_month_days = days_in_year_month(date.year(), prev_month);
        let mut year = date.year();
        if let Month::December = prev_month {
            year -= 1;
        }
        let day = date.day().min(prev_month_days);
        DateOg::from_calendar_date(year, prev_month, day).unwrap().into()
    }

    /// used for generating strings for printing purposes, 
    /// i.e. followed by using FormatTime methods on (u16, time::Month)
    pub fn month_year(&self) -> (Month, u16) {
        (self.date().month(), self.0)
    }

    pub fn days_in_month(&self) -> u8 {
        time::util::days_in_year_month(self.0 as i32, self.date().month())
    }

    pub fn month_dates(&self) -> Vec<(Date, u8)> {
        let mut res = Vec::with_capacity(31);
        let first_month_date_oridal = DateOg::from_calendar_date(self.0 as i32, self.date().month(), 1).unwrap().ordinal();
        for i in 0..time::util::days_in_year_month(self.0 as i32, self.date().month()) {
            res.push((Date::new(self.1, first_month_date_oridal + 1), i + 1));
        }
        res
    }
}

impl From<DateOg> for Date {
    fn from(data: DateOg) -> Self {
        Self(data.year() as u16, data.ordinal())
    }
}


impl FormatTime for (Month, u16) {
    /// Sep, 2021
    fn format_s(&self) -> String {
        format!("{}, {}", self.0.format_s(), self.1)
    }

    /// September, 2021
    fn format_l(&self) -> String {
        format!("{}, {}", self.0.format_l(), self.1)
    }
}

impl FormatTime for Date {
    /// Mon, Sep 13, 2021
    fn format_s(&self) -> String {
        let date = self.date();
        format!(
            "{}, {} {}, {}", 
            date.weekday().format_s(), 
            date.month().format_s(), 
            date.day(), 
            date.year()
        )
    }

    /// Monday, Sep 13, 2021
    fn format_m(&self) -> String {
        let date = self.date();
        format!(
            "{}, {} {}, {}", 
            date.weekday().format_l(), 
            date.month().format_s(), 
            date.day(), 
            date.year()
        )
    }

    /// Monday, September 13, 2021
    fn format_l(&self) -> String {
        let date = self.date();
        format!(
            "{}, {} {}, {}", 
            date.weekday().format_l(), 
            date.month().format_l(), 
            date.day(), 
            date.year()
        )
    }
}


impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
            .then(self.1.cmp(&other.1))
    }
}


impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}