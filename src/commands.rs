use redis_mod::redis;
use redis_mod::redis::Command;

use redis_mod::error;
use redis_mod::RModError;

use crate::schedule::Schedule;

pub struct TimerSetCommand {}
impl Command for TimerSetCommand {
    fn name(&self) -> &'static str {
        "TIMER.SET"
    }

    //TIMER.SET <key> <sheduleJob_str>
    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), RModError> {
        if args.len() < 3 {
            return Err(error!(
                "Usage: {} <method>",
                self.name()
            ));
        }
        let key = args[1];
        let sche = Schedule::new(args[2]);
        let next_unix_t = match sche.find_next_schedule_as_unix_t(){
            Some(v) => v,
            None => return Err(error!(&format!(
                "TimeSetError: Failed to get next schedule from '{}'",
                args[1]
            )))
        };

        let val = format!("{}{}", next_unix_t, args[2]);
        r.open_key_writable(&key).write(&val)?;
        Ok(r.reply_ok())
    }

    fn str_flags(&self) -> &'static str {
        "write fast deny-oom"
    }

}

pub struct TimerGetCommand {}
impl Command for TimerGetCommand {
    fn name(&self) -> &'static str {
        "TIMER.GET"
    }

    //TIMER.GET <key>
    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), RModError> {
        if args.len() < 2 {
            return Err(error!(
                "Usage: {} <key>",
                self.name()
            ));
        }
        let key = args[1];


        let raw_val_opt = r.open_key_writable(&key).read()?;
        let raw_val = match raw_val_opt {
            Some(v) => v,
            None => return Ok(r.reply_null())
        };

        let (store_unix_t, store_cron_s) = match parse_timeset_str(&raw_val) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let sche = Schedule::new(store_cron_s);
        sche.update_left_t(store_unix_t as i32);

        let left_t = sche.left_unix_t.get() as i64;
        if left_t >= 1 {
            return Ok(r.reply_integer(left_t)?)
        };

        let next_unix_t = match sche.find_next_schedule_as_unix_t(){
            Some(v) => v,
            None => 0
        };

        let val = format!("{}{}", next_unix_t, store_cron_s);
        r.open_key_writable(&key).write(&val)?;

        Ok(r.reply_integer(left_t)?)
    }

    fn str_flags(&self) -> &'static str {
        "write fast deny-oom"
    }

}



pub fn parse_timeset_str(raw_str: &str) -> Result<(u32, &str), RModError> {
    //Note:
    //  inside value fmt: <unixtime><crontab str>

    let sp:Vec<&str> = raw_str.split('').collect();
    let unix_t_res = sp[0].parse::<u32>()
        .map_err(|_| error!("RModError: Failed to parse str as timeset format"));
    match unix_t_res {
        Ok(u) => Ok((u,sp[1])),
        Err(e) => Err(e)
    }
}
