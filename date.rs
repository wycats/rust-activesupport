use extra::time;
use extra::time::{Tm};
use TimeChange;

#[deriving(Clone)]
pub struct Date {
  year: int,
  month: uint,
  day: uint
}

// Magic variables from http://en.wikipedia.org/wiki/Julian_day#Gregorian_calendar_from_Julian_day_number

static Y: uint = 4716;
static J: uint = 1401;
static M: uint = 2;
static N: uint = 12;
static R: uint = 4;
static P: uint = 1461;
static V: uint = 3;
static U: uint = 5;
static S: uint = 153;
static W: uint = 2;
static B: uint = 274277;
static C: uint = 38;

impl Date {
  fn from_jdn(jdn: uint) -> Date {
    let f = jdn + J + (((4 * jdn + B)/146097) * 3)/4 - C;
    let e = R * f + V;
    let g = (e % P) / R;
    let h = U * g + W;
    let day = (h % S)/U + 1;
    let month = ((h/S + M) % N) + 1;
    let year = e/P - Y + (N + M - month)/N;

    Date{ day: day, month: month, year: year as int }
  }

  pub fn new(year: int, month: uint, day: uint) -> Date {
    Date{ day: day, month: month, year: year }
  }

  pub fn advance_days(&self, n: int) -> Date {
    let jdn = self.to_jdn() as int;
    Date::from_jdn((jdn + n) as uint)
  }

  pub fn advance_years(&self, n: int) -> Date {
    Date::new(self.year + n, self.month, self.day)
  }

  pub fn advance_months(&self, n: int) -> Date {
    if (n >= 0) {
      self.add_months(n as uint)
    } else {
      self.subtract_months(-n as uint)
    }
  }

  fn add_months(&self, n: uint) -> Date {
    let remaining_in_year = 12 - self.month;
    let after_this_year = n - remaining_in_year;

    if (after_this_year > 0) {
      let years = after_this_year / 12;
      let months = after_this_year % 12;

      Date::new(self.year + years as int, months, self.day)
    } else {
      Date::new(self.year, self.month + n, self.day)
    }
  }

  fn subtract_months(&self, n: uint) -> Date {
    if (n > self.month) {
      let years = n / 12;
      let remaining_in_year = n % 12;
      Date::new(self.year - years as int, 12 - remaining_in_year + self.month, self.day)
    } else {
      Date::new(self.year, self.month - n, self.day)
    }
  }

  pub fn to_jdn(&self) -> uint {
    let month = self.month;

    let a = (14 - self.month) / 12;    // 1 for Jan/Feb, 0 otherwise
    let y: uint = (self.year as uint) + 4800 - a;
    let m = self.month + (12 * a) - 3; // 0 for Mar, 11 for Feb

    let mut jdn: uint = self.day;
    jdn += ((153 * m) + 2) / 5;
    jdn += (365 * y);
    jdn += (y / 4);
    jdn -= (y / 100);
    jdn += (y / 400);
    jdn -= 32045;

    jdn as uint
  }
}

pub trait ToDate {
  fn to_date(&self) -> Date;
}

impl ToDate for Tm {
  fn to_date(&self) -> Date {
    Date{ day: self.tm_mday as uint, month: self.tm_mon as uint, year: self.tm_year as int }
  }
}
