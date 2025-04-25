pub mod time_date;
pub use time_date::*;

pub mod date;
pub use date::*;

pub trait FormatTime {
    fn format_s(&self) -> String;
    fn format_m(&self) -> String {
        self.format_s()
    }
    fn format_l(&self) -> String;
}

use time::{Month, Weekday};

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
