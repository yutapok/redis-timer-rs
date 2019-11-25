extern crate crontab;
extern crate time;

use std::cell::Cell;
use time::now;

use crontab::Crontab;

pub struct Schedule<'a> {
    pub setting: &'a str,
    pub left_unix_t: Cell<u32>,
}

impl<'a> Schedule<'a> {
    pub fn new(_setting: &'a str) -> Self {
        Schedule {
            setting: _setting,
            left_unix_t: Cell::new(0)
        }
    }

    pub fn find_next_schedule_as_unix_t(&self) -> Option<u32> {
        let cron = match self.to_crontab(){
            Some(c) => c,
            None => return None
        };

        match cron.find_next_event(){
            Some(tm) => Some(tm.to_timespec().sec as u32),
            None => None
        }
    }

    pub fn update_left_t(&self, base_t: i32) -> () {
        // Note:
        //  When time is up, left is going to be zero.
        let now_t =  now().to_utc().to_timespec().sec as i32;
        let diff = match base_t - now_t {
            i if i > 0 => i as u32,
            _ => 0 as u32
        };
        self.left_unix_t.set(diff)
    }

    fn to_crontab(&self) -> Option<Crontab> {
        let cron_res = Crontab::parse(self.setting);
        if let Err(_) = cron_res {
            return None
        }
        Some(cron_res.unwrap())
    }

}

