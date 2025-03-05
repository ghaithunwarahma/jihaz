extern crate time;

use time::{Date, Duration, Month, Time, Weekday};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use druid::Data;

/// Datetime
// Example [ year (2021), ordinal (366), hour (14), minute (39) ]. changed to
/// Example [ year (2021), ordinal (366), seconds (3611), second fraction (350). u16 max is 65535, seconds in day is 96400
/// time::Date has year as i32 and ordinal u16, but we only need year as u16.
/// time::Time has seconda, nanoseconds, padding but we only need hour and minute.
/// 
/// field 0 is year, 
/// field 1 is ordinal and seconds stored adjacently in u32.
/// field 2 fraction of a second in nanosecond (max is 999,999,999)
/// 
/// The ordinal and seconds field stores the seconds in the rightmost set of 16 bits,
/// and stores the ordinal in the leftmost set of 16 bits.
/// (If we only had time, then h:m:s as u8, u8 and u8 totaling 24 bits is smaller than time in seconds as u32,
/// but since we have ordinal, 265 days which needs u16, then u32 for all is better than u16, u8, u8 and u8 totaling 40 bits)
/// 
#[derive(Clone, Copy, Data, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DT(u16, u32, u16);

#[derive(Clone, Copy, Data, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Date2(u16, u16);

impl Date2 {
    pub fn new(year: u16, ordinal: u16) -> Self {
        Self(year, ordinal)
    }

    pub fn from_year_month_day(year: u16, month: u8, day: u8) -> Self {
        use std::convert::TryFrom;
        let month = time::Month::try_from(month).unwrap();
        let date = Date::from_calendar_date(year as i32, month, day).unwrap();
        Self(year, date.ordinal())
    }

    pub fn now() -> Self {
        let res = crate::common::make_day_month_full_year();
        use std::convert::TryFrom;
        let month = time::Month::try_from(res.1).unwrap();
        let date = Date::from_calendar_date(res.2 as i32, month, res.0).unwrap();
        Self(res.2, date.ordinal())
    }

    pub fn date(&self) -> Date {
        Date::from_ordinal_date(self.0 as i32, self.1).unwrap()
    }

    pub fn before(&self, other: &Self) -> bool {
        self.lt(&other)
    }

    pub fn year(&self) -> u16 {
        self.0
    }

    pub fn ordinal(&self) -> u16 {
        self.1
    }

    pub fn same_calendar_month(&self, other: &Date2) -> bool {
        self.0.same(&other.0) && self.date().month() == other.date().month()
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
        Date::from_calendar_date(year, next_month, day).unwrap().into()
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
        Date::from_calendar_date(year, prev_month, day).unwrap().into()
    }

    /// used for generating strings for printing purposes, 
    /// i.e. followed by using FormatTime methods on (u16, time::Month)
    pub fn month_year(&self) -> (Month, u16) {
        (self.date().month(), self.0)
    }

    pub fn days_in_month(&self) -> u8 {
        time::util::days_in_year_month(self.0 as i32, self.date().month())
    }

    pub fn month_dates(&self) -> Vec<(Date2, u8)> {
        let mut res = Vec::with_capacity(31);
        let first_month_date_oridal = Date::from_calendar_date(self.0 as i32, self.date().month(), 1).unwrap().ordinal();
        for i in 0..time::util::days_in_year_month(self.0 as i32, self.date().month()) {
            res.push((Date2::new(self.1, first_month_date_oridal + 1), i + 1));
        }
        res
    }
}

impl DT {
    pub fn new(year: u16, ordinal: u16, hour: u8, minute: u8) -> Self {
        Self(year, ordinal, hour, minute)
    }

    pub fn from_date2(date2: Date2, hour: u8, minute: u8) -> Self {
        Self(date2.year(), date2.ordinal(), hour, minute)
    }

    pub fn now() -> Self {
        let (hour, min) = crate::common::make_time();
        let res = crate::common::make_day_month_full_year();
        use std::convert::TryFrom;
        let month = time::Month::try_from(res.1).unwrap();
        let date = Date::from_calendar_date(res.2 as i32, month, res.0).unwrap();
        Self(res.2, date.ordinal(), hour, min)
    }

    pub fn date(&self) -> Date {
        Date::from_ordinal_date(self.0 as i32, self.1).unwrap()
    }

    pub fn time(&self) -> Time {
        Time::from_hms(self.2, self.3, 0).unwrap()
    }

    pub fn date2(&self) -> Date2 {
        Date2(self.0, self.1)
    }

    pub fn before(&self, other: &Self) -> bool {
        self.lt(&other)
    }

    pub fn year(&self) -> u16 {
        self.0
    }

    pub fn ordinal(&self) -> u16 {
        self.1
    }

    pub fn hour(&self) -> u8 {
        self.2
    }

    pub fn minute(&self) -> u8 {
        self.3
    }

    pub fn same_calendar_month(&self, other: &DT) -> bool {
        self.0.same(&other.0) && self.date().month() == other.date().month()
    }

    // pub fn time_to_string(&self, am_pm: bool) -> String {
    //     if am_pm {
    //         let mut hour = self.2;
    //         let mut am = true;
    //         if hour > 11 {
    //             if hour > 12 {
    //                 hour -= 12;
    //             }
    //             am = false;
    //         }
    //         format!("{}:{} {}", hour, self.3, if am { "am" } else { "pm" })
    //     } else {
    //         format!("{}:{}", self.2, self.3)
    //     }
    // }

    pub fn time_am_pm(&self) -> (u8, u8, &'static str) {
        let mut hour = self.2;
        let mut am = true;
        if hour > 11 {
            if hour > 12 {
                hour -= 12;
            }
            am = false;
        }
        (hour, self.3, if am { "am" } else { "pm" })
    }

    pub fn duration(&self, other: &Self) -> Duration {
        use time::util::days_in_year;
        assert!(self.before(other));
        let this_year = self.year();
        let other_year = other.year();
        let mut days = 0u16;
        for year in this_year..=other_year {
            days += if year == this_year {
                days_in_year(year as i32) - self.ordinal()
            } else if year == other_year {
                self.ordinal()
            } else {
                days_in_year(year as i32)
            };
        }
        let this_time = self.hour() as u16 * 3_600 + self.minute() as u16 * 60;
        let other_time = other.hour() as u16 * 3_600 + other.minute() as u16 * 60;
        Duration::new((days * 24 * 3_600 - this_time + other_time) as i64, 0)
    }

    /// in hours and minutes
    pub fn duration_hm(&self, other: &Self) -> (usize, usize) {
        let mut hours = (self.2 - other.2) as isize * 365 * 24 
            + (self.3 - other.3) as isize * 30 * 24 
            + (self.2 - other.2) as isize * 24 
            + (self.1 - other.1) as isize;
        let mut mins = hours * 60 + (self.3 - other.3) as isize;
        hours = mins / 60;
        mins = mins % 60;
        (hours as usize, mins as usize)
    }
}

impl From<Date> for Date2 {
    fn from(data: Date) -> Self {
        Self(data.year() as u16, data.ordinal())
    }
}

pub trait FormatTime {
    fn format_s(&self) -> String;
    fn format_m(&self) -> String {
        self.format_s()
    }
    fn format_l(&self) -> String;
}

impl FormatTime for DT {
    fn format_s(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date2().format_s(), t.0, t.1, t.2)
    }

    fn format_m(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date2().format_m(), t.0, t.1, t.2)
    }

    fn format_l(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date2().format_l(), t.0, t.1, t.2)
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

impl FormatTime for Date2 {
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

impl FormatTime for Month {
    fn format_s(&self) -> String {
        match self {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        }.to_string()
    }

    fn format_l(&self) -> String {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }.to_string()
    }
}

impl FormatTime for Weekday {
    fn format_s(&self) -> String {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }.to_string()
    }

    fn format_l(&self) -> String {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }.to_string()
    }
}

impl Ord for DT {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
            .then(self.1.cmp(&other.1))
            .then(self.2.cmp(&other.2))
            .then(self.3.cmp(&other.3))
    }
}

impl PartialOrd for DT {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
            .then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for Date2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// impl PartialEq for DT {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//             && self.1 == other.1
//             && self.2 == other.2
//             && self.3 == other.3
//     }
// }