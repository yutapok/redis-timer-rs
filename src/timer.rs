extern crate cron;
extern crate time;
extern crate chrono;
extern crate chrono_tz;

use std::cell::Cell;
use std::str::FromStr;

use cron::Schedule;
use chrono::{Utc, DateTime, NaiveDateTime};
use chrono_tz::Tz;

pub struct Timer<'a> {
    pub setting: &'a str,
    pub left_unix_t: Cell<u32>,
    pub tz: &'a str
}


impl<'a> Timer<'a> {
    pub fn new(_setting: &'a str, _tz: &'a str) -> Self {
        Timer {
            setting: _setting,
            left_unix_t: Cell::new(0),
            tz: _tz
        }
    }

    pub fn find_next_schedule(&self) -> Option<DateTime<chrono_tz::Tz>>
    {
        let timezone = match self.tz.parse::<chrono_tz::Tz>(){
            Ok(tz) => tz,
            Err(_) => return None
        };

        let schedule = match self.to_crontab(){
            Some(c) => c,
            None => return None
        };

        schedule.upcoming(timezone).nth(0)
    }

    pub fn update_left_t(&self, base_t: i32) -> () {
        // Note:
        //  When time is up, left is going to be zero.

        let timezone = match self.tz.parse::<chrono_tz::Tz>(){
            Ok(tz) => tz,
            Err(_) => return
        };

        let now_t = self.now_timestamp(timezone);
        let diff = match base_t - now_t {
            i if i > 0 => i as u32,
            _ => 0 as u32
        };
        self.left_unix_t.set(diff)
    }

    fn to_crontab(&self) -> Option<Schedule> {
        let cron_res = Schedule::from_str(&self.correct_simplefmt());
        if let Err(_) = cron_res {
            return None
        }
        Some(cron_res.unwrap())
    }

    fn now_timestamp(&self, tz: Tz) -> i32 {
        Utc::now().with_timezone(&tz).timestamp() as i32
    }

    fn correct_simplefmt(&self) -> String {
        if self.is_macro_cronfmt(){
            return self.setting.to_string()
        }

        if self.is_simple_cronfmt() {
            return format!("* {} *", self.setting)
        }

        self.setting.to_string()
    }

    fn is_simple_cronfmt(&self) -> bool {
        self.setting
        .split(" ")
        .collect::<Vec<&str>>()
        .len() == 5
    }

    fn is_macro_cronfmt(&self) -> bool {
        self.setting
        .chars()
        .filter(|c| c.to_string() == "@")
        .collect::<Vec<char>>()
        .len() >= 1
    }

}


pub fn convert_to_datetime(timestamp: u32, timezone_str: &str) -> Option<DateTime<chrono_tz::Tz>> {
    let timezone = match timezone_str.parse::<chrono_tz::Tz>(){
        Ok(tz) => tz,
        Err(_) => return None
    };

    let datetime = DateTime::<Utc>::from_utc(
      NaiveDateTime::from_timestamp(timestamp as i64, 0),
      Utc
    );

    Some(datetime.with_timezone(&timezone))
}

