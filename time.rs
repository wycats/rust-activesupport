use date::Date;
use extra::time::{Tm,now_utc,now};

#[deriving(Clone)]
pub struct Time {
  date: Date,
  hours: uint,
  minutes: uint,
  seconds: uint,
  nanos: uint
}

impl Time {
  pub fn from_tm(tm: Tm) -> Time {
    let year = tm.tm_year + 1900;
    let month = tm.tm_mon + 1;

    let date = Date{ year: year as int, month: month as uint, day: tm.tm_mday as uint };
    Time{
      date: date,
      hours: tm.tm_hour as uint,
      minutes: tm.tm_min as uint,
      seconds: tm.tm_sec as uint,
      nanos: tm.tm_nsec as uint
    }
  }

  pub fn new() -> Time {
    let date = Date{ year: 0, month: 0, day: 0 };
    Time{ date: date, hours: 0, minutes: 0, seconds: 0, nanos: 0 }
  }

  pub fn now() -> Time {
    Time::from_tm(now())
  }

  pub fn now_utc() -> Time {
    Time::from_tm(now_utc())
  }

  pub fn with_date(date: Date) -> Time {
    Time{ date: date, hours: 0, minutes: 0, seconds: 0, nanos: 0 }
  }

  pub fn year(mut self, year: int) -> Time {
    self.date.year = year;
    self
  }

  pub fn month(mut self, month: uint) -> Time {
    self.date.month = month;
    self
  }

  pub fn day(mut self, day: uint) -> Time {
    self.date.day = day;
    self
  }

  pub fn hour(mut self, hour: uint) -> Time {
    self.hours = hour;
    self
  }

  pub fn minute(mut self, minute: uint) -> Time {
    self.minutes = minute;
    self
  }

  pub fn second(mut self, second: uint) -> Time {
    self.seconds = second;
    self
  }

  pub fn nanos(mut self, nanos: uint) -> Time {
    self.nanos = nanos;
    self
  }
}
