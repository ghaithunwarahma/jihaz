extern crate time;

use chrono::Local;
use jihaz_primal::bits::max_bitwise_value_for_bits;
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use time::{Date as DateOg, Duration, Time};

use super::FormatTime;
use super::date::Date;

/// The max value these bits is 1_073_741_823, which is more than the maximum nanoseconds in a second (999_999_999 or 1_999_999_999 in a leap year)
const NANOSEC_BITS: usize = 30;
// /// The max value these bits is 63, which is more than the maximum seconds in a minute (60)
// const SEC_BITS: usize = 6;
/// The max value these bits is 63, which is more than the maximum minutes in an hour (60)
const MIN_BITS: usize = 6;
/// The max value these bits is 31, which is more than the maximum hours in a day (24)
const HOUR_BITS: usize = 5;
/// The max value these bits is 511, which is more than the maximum days in a year (365)
const ORD_BITS: usize = 9;
/// The max value these bits is 4095, which means we can store up to year 4095
const YEAR_BITS: usize = 12; 

/// This allows us to store time and date in the smallest storage space.
/// 
/// Storing the year, ordinal, hour, minute, nanosecond fractions in the rightmost 64 bits of a 64 bit unsigned integer; field 0.
/// 
/// And then storing the seconds in the 8 bits unsigned integer; field 1.
/// 
/// The breakdown of the first field:
/// 
/// The rightmost 31 bits [i.e. max=2147483647] that can contain nanosecondss (max is 999,999,999 nanoseconds or 1,999,999,999 nanoseconds in a leap year).
/// 
/// Followed by 6 bits [i.e. max=63] that contains minutes (max is 60 minutes).
/// 
/// Followed by 5 bits [i.e. max=31] that contains hours (max is 24 hours).
/// 
/// Followed by 9 bits [i.e. max=511] that contains the ordinal (max ordinal is 365 days).
/// 
/// Followed by the last 12 bits [i.e. max=4095] that contains the year (maximum is year 4095).
/// 
/// The second field is an 8 bits unsigned integer [i.e. max=156] that contains seconds (max is 60 second).

/// Therefore, TimeAndDate requires a 64 and 8 bits unsigned integers to store its data.
/// 
/// note:
/// time::DateOg has year as i32 and ordinal u16, but we only need year as u16.
/// time::Time has seconda, nanosecondss, padding but we only need hour and minute.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimeAndDate(u64, u8);

impl Default for TimeAndDate {
    fn default() -> Self {
        Self::now()
    }
}

impl TimeAndDate {
    pub const fn new(year: u16, ordinal: u16, hour: u8, minute: u8) -> Self {
        Self(
            (year as u64) << (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS) |
            (ordinal as u64) << (NANOSEC_BITS + MIN_BITS + HOUR_BITS) |
            (hour as u64) << (NANOSEC_BITS + MIN_BITS) |
            (minute as u64) << (NANOSEC_BITS),
            0
        )
    }
    
    pub const fn new_with_nanos(year: u16, ordinal: u16, hour: u8, minute: u8, second: u8, nanos: u32) -> Self {
        Self(
            (year as u64) << (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS) |
            (ordinal as u64) << (NANOSEC_BITS + MIN_BITS + HOUR_BITS) |
            (hour as u64) << (NANOSEC_BITS + MIN_BITS) |
            (minute as u64) << (NANOSEC_BITS) |
            nanos as u64,
            second
        )
    }
    
    pub const fn from_date(date: Date, hour: u8, minute: u8) -> Self {
        Self::new(date.year(), date.ordinal(), hour, minute)
    }

    /// Get the year.
    /// 
    /// The returned will be from 0, and up to the maximum value of 4095 years that can be stored in 12 bits.
    /// 
    /// The year is stored in 12 bits in the first 64 bits unsigned integer.
    /// 
    /// we cancel the remaining bits by applying bit-wise AND for only the bits of year
    pub const fn year(&self) -> u16 {
        let max_value = max_bitwise_value_for_bits(YEAR_BITS) as u64;
        ((self.0 >> (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS)) & max_value) as u16
    }

    /// Get the ordinal.
    /// 
    /// The returned will always be in the range `1..365`.
    /// 
    /// The ordinal is stored in 9 bits in the first 64 bits unsigned integer.
    /// 
    /// we cancel the remaining bits by applying bit-wise AND for only the bits of ordinal
    pub const fn ordinal(&self) -> u16 {
        let max_value = max_bitwise_value_for_bits(ORD_BITS) as u64;
        ((self.0 >> (NANOSEC_BITS + MIN_BITS + HOUR_BITS)) & max_value) as u16
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    /// 
    /// The hour is stored in 5 bits in the first 64 bits unsigned integer.
    /// 
    /// we cancel the remaining bits by applying bit-wise AND for only the bits of hour
    pub const fn hour(&self) -> u8 {
        let max_value = max_bitwise_value_for_bits(HOUR_BITS) as u64;
        ((self.0 >> (NANOSEC_BITS + MIN_BITS)) & max_value) as u8
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    /// 
    /// The minute is stored in 6 bits in the first 64 bits unsigned integer.
    /// 
    /// we cancel the remaining bits by applying bit-wise AND for only the bits of minute
    pub const fn minute(&self) -> u8 {
        let max_value = max_bitwise_value_for_bits(MIN_BITS) as u64;
        ((self.0 >> NANOSEC_BITS) & max_value) as u8
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// The second is stored in the second 8 bits unsigned integer.
    /// 
    /// we cancel the remaining bits by applying bit-wise AND for only the bits of second
    pub const fn second(&self) -> u8 {
        self.1
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_999_999_999` (more than 1_000_000_000` for the case of leap seconds).
    ///
    /// The nanoseconds are therefore stored in 31 bits to allow for values ranging between 0..1_999_999_999`.
    /// 
    /// We cancel the remaining bits by applying bit-wise AND for only the bits of nano seconds
    pub const fn nanosecond(&self) -> u32 {
        let max_value = max_bitwise_value_for_bits(NANOSEC_BITS) as u64;
        (self.0 & max_value) as u32
    }

    pub const fn date(&self) -> Date {
        Date::new(self.year(), self.ordinal())
    }

    pub fn date_og(&self) -> DateOg {
        DateOg::from_ordinal_date(self.year() as i32, self.ordinal()).unwrap()
    }

    pub fn time(&self) -> Time {
        Time::from_hms(self.hour(), self.minute(), self.second()).unwrap()
    }

    // pub const fn new_from_naive_time(year: u16, ordinal: u16, hour: u8, naive_time: NaiveTime) -> Self {
    //     TimeAndDate::new_with_nanos(year, ordinal, )
    // }

    pub fn now() -> Self {
        let local_date_time = Local::now();
        
        let naive_date = local_date_time.date().naive_local();
        let naive_time = local_date_time.time();
        
        use chrono::Datelike;
        let year = naive_date.year();
        debug_assert!(year >= 0i32);
        // note that the ordinal returned here starts with 1, for zero starting value, use ordinal0

        use chrono::Timelike;
        Self::new_with_nanos(
            year as u16, 
            naive_date.ordinal() as u16, 
            naive_time.hour() as u8,
            naive_time.minute() as u8,
            naive_time.second() as u8,
            naive_time.nanosecond(),
        )
    }
}

impl TimeAndDate {
    pub fn earlier_than(&self, other: &Self) -> bool {
        self.lt(&other)
    }

    pub fn same_calendar_month(&self, other: &TimeAndDate) -> bool {
        // use chrono::Datelike;
        // self.0.same(&other.0) && self.date_og().month() == other.date_og().month()
        self.0 == other.0 && self.date_og().month() == other.date_og().month()
    }

    pub fn time_am_pm(&self) -> (u8, u8, &'static str) {
        let mut hour = self.hour();
        let mut am = true;
        if hour > 11 {
            if hour > 12 {
                hour -= 12;
            }
            am = false;
        }
        (hour, self.minute(), if am { "am" } else { "pm" })
    }

    pub fn duration(&self, other: &Self) -> Duration {
        use time::util::days_in_year;
        assert!(self.earlier_than(other));
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

    // /// in hours and minutes
    // pub fn duration_hm(&self, other: &Self) -> (usize, usize) {
    //     let mut hours = (self.hour() - other.2) as isize * 365 * 24 
    //         + (self.minute() - other.3) as isize * 30 * 24 
    //         + (self.hour() - other.2) as isize * 24 
    //         + (self.1 - other.1) as isize;
    //     let mut mins = hours * 60 + (self.minute() - other.3) as isize;
    //     hours = mins / 60;
    //     mins = mins % 60;
    //     (hours as usize, mins as usize)
    // }

    pub fn one_more_nanosecond(self) -> Self {
        let mut time = Time::from_hms_nano(
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
        ).unwrap();
        time = time + Duration::new(0, 1);
        Self::new_with_nanos(
            self.year(), 
            self.ordinal(), 
            time.hour(), 
            time.minute(), 
            time.second(),
            time.nanosecond()
        )
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl FormatTime for TimeAndDate {
    fn format_s(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date().format_s(), t.0, t.1, t.2)
    }

    fn format_m(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date().format_m(), t.0, t.1, t.2)
    }

    fn format_l(&self) -> String {
        let t = self.time_am_pm();
        format!("{}, {}:{} {}", self.date().format_l(), t.0, t.1, t.2)
    }
}


impl Ord for TimeAndDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year().cmp(&other.year())
            .then(self.ordinal().cmp(&other.ordinal()))
            .then(self.hour().cmp(&other.hour()))
            .then(self.minute().cmp(&other.minute()))
            .then(self.second().cmp(&other.second()))
            .then(self.nanosecond().cmp(&other.nanosecond()))
    }
}


impl PartialOrd for TimeAndDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// impl PartialEq for TimeAndDate {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//             && self.1 == other.1
//             && self.hour() == other.2
//             && self.minute() == other.3
//     }
// }

impl From<(u64, u8)> for TimeAndDate {
    fn from(data: (u64, u8)) -> TimeAndDate {
        TimeAndDate(data.0, data.1)
    }
}


#[cfg(test)]
mod tests {
    use crate::{bits, time::FormatTime};
    use super::{TimeAndDate, HOUR_BITS, MIN_BITS, NANOSEC_BITS, ORD_BITS};


    #[test]
    fn bitwise() {
        let date = TimeAndDate::new_with_nanos(2000, 270, 8, 30, 20, 370);
        println!("2000: {:#b}", 2000);
        println!("2000 shifted: {:#b}", (2000 as u64) << (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS) as usize);
        println!("270: {:#b}", 270);
        println!("8: {:#b}", 8);
        println!("30: {:#b}", 30);
        println!("date: {:#b}", date.0);
        println!("date: {:#b}", date.0 >> (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS));
        println!("date: {:#b}", bits::MAX_U12 as u64);
        println!("date: {:#b}", (date.0 >> (NANOSEC_BITS + MIN_BITS + HOUR_BITS + ORD_BITS)) & bits::MAX_U12 as u64);
        println!("year: {}", date.year());
        println!("year: {:#}, ordinal: {:#}, hour: {:#}, minute: {:#}, second: {:?}, nanoseconds: {:?}", date.year(), date.ordinal(), date.hour(), date.minute(), date.second(), date.nanosecond());
        println!("year: {:#}, ordinal: {:#}, format_s: {:#}, format_m: {:#}, format_l: {:#}", date.year(), date.ordinal(), date.format_s(), date.format_m(), date.format_l());
        println!("which matches input details: year: 2000, ordinal: 270, hour: 8, minute: 30, second: 20, nanosecond: 370");
    
        assert!(date.year() == 2000);
        assert!(date.ordinal() == 270);
        assert!(date.hour() == 8);
        assert!(date.minute() == 30);
    }
}


// impl Add<Duration> for Time {
//     type Output = Self;

//     /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow.
//     ///
//     /// ```rust
//     /// # use time::{ext::NumericalDuration, macros::time};
//     /// assert_eq!(time!(12:00) + 2.hours(), time!(14:00));
//     /// assert_eq!(time!(0:00:01) + (-2).seconds(), time!(23:59:59));
//     /// ```
//     fn add(self, duration: Duration) -> Self::Output {
//         self.adjusting_add(duration).1
//     }
// }

// impl Add<StdDuration> for Time {
//     type Output = Self;

//     /// Add the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow.
//     ///
//     /// ```rust
//     /// # use time::{ext::NumericalStdDuration, macros::time};
//     /// assert_eq!(time!(12:00) + 2.std_hours(), time!(14:00));
//     /// assert_eq!(time!(23:59:59) + 2.std_seconds(), time!(0:00:01));
//     /// ```
//     fn add(self, duration: StdDuration) -> Self::Output {
//         self.adjusting_add_std(duration).1
//     }
// }

// impl_add_assign!(Time: Duration, StdDuration);

// impl Sub<Duration> for Time {
//     type Output = Self;

//     /// Subtract the sub-day time of the [`Duration`] from the `Time`. Wraps on overflow.
//     ///
//     /// ```rust
//     /// # use time::{ext::NumericalDuration, macros::time};
//     /// assert_eq!(time!(14:00) - 2.hours(), time!(12:00));
//     /// assert_eq!(time!(23:59:59) - (-2).seconds(), time!(0:00:01));
//     /// ```
//     fn sub(self, duration: Duration) -> Self::Output {
//         self.adjusting_sub(duration).1
//     }
// }

// impl Sub<StdDuration> for Time {
//     type Output = Self;

//     /// Subtract the sub-day time of the [`std::time::Duration`] from the `Time`. Wraps on overflow.
//     ///
//     /// ```rust
//     /// # use time::{ext::NumericalStdDuration, macros::time};
//     /// assert_eq!(time!(14:00) - 2.std_hours(), time!(12:00));
//     /// assert_eq!(time!(0:00:01) - 2.std_seconds(), time!(23:59:59));
//     /// ```
//     fn sub(self, duration: StdDuration) -> Self::Output {
//         self.adjusting_sub_std(duration).1
//     }
// }

// impl_sub_assign!(Time: Duration, StdDuration);

// impl Sub for Time {
//     type Output = Duration;

//     /// Subtract two `Time`s, returning the [`Duration`] between. This assumes both `Time`s are in
//     /// the same calendar day.
//     ///
//     /// ```rust
//     /// # use time::{ext::NumericalDuration, macros::time};
//     /// assert_eq!(time!(0:00) - time!(0:00), 0.seconds());
//     /// assert_eq!(time!(1:00) - time!(0:00), 1.hours());
//     /// assert_eq!(time!(0:00) - time!(1:00), (-1).hours());
//     /// assert_eq!(time!(0:00) - time!(23:00), (-23).hours());
//     /// ```
//     fn sub(self, rhs: Self) -> Self::Output {
//         let hour_diff = (self.hour as i8) - (rhs.hour as i8);
//         let minute_diff = (self.minute as i8) - (rhs.minute as i8);
//         let mut second_diff = (self.second as i8) - (rhs.second as i8);
//         let mut nanosecond_diff = (self.nanosecond as i32) - (rhs.nanosecond as i32);

//         cascade!(nanosecond_diff in 0..1_000_000_000 => second_diff);

//         Duration::new_unchecked(
//             hour_diff as i64 * 3_600 + minute_diff as i64 * 60 + second_diff as i64,
//             nanosecond_diff,
//         )
//     }
// }