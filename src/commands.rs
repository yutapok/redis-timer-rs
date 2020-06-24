use redis_mod::redis;
use redis_mod::redis::Command;

use redis_mod::error;
use redis_mod::RModError;

use crate::timer::Timer;


//TIMER.SET <key> <sheduleJob_str>
pub struct TimerSetCommand {}
impl Command for TimerSetCommand {
    fn name(&self) -> &'static str {
        "TIMER.SET"
    }

    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), RModError> {
        if args.len() < 4 {
            return Err(error!(
                "Usage: {} <method>",
                self.name()
            ));
        }
        let key = args[1];

        let sche = Timer::new(args[2], args[3]);
        let next_unix_t = match sche.find_next_schedule(){
            Some(v) => v,
            None => return Err(error!(&format!(
                "TimeSetError: Failed to get next schedule from input args, format: '{}', timezone: '{}'",
                args[2],
                args[3]
            )))
        };

        let val = format!("{}{}{}", next_unix_t.timestamp(), args[2], args[3]);
        r.open_key_writable(&key).write(&val)?;
        r.replicate_verbatim();
        Ok(r.reply_ok())
    }

    fn str_flags(&self) -> &'static str {
        "write fast deny-oom"
    }

}


//TIMER.GET <key> <debug flag>
pub struct TimerGetCommand {}
impl Command for TimerGetCommand {
    fn name(&self) -> &'static str {
        "TIMER.GET"
    }

    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), RModError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(error!(
                "Usage: {} <key> [<debug flag>]",
                self.name()
            ));
        }

        let key = args[1];
        let (debug_flag, arr_n): (bool, i64) = match args.len() {
            2 => (false, 1),
            3 => (true, 3),
            _ => (false, 1)
        };

        let raw_val_opt = r.open_key_writable(&key).read()?;
        let raw_val = match raw_val_opt {
            Some(v) => v,
            None => return Ok(r.reply_null())
        };

        let (store_unix_t, store_cron_s, timezone_str) = match parse_timeset_str(&raw_val) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let sche = Timer::new(store_cron_s, timezone_str);
        sche.update_left_t(store_unix_t as i32);

        let left_t = sche.left_unix_t.get() as i64;
        if left_t >= 1 {
            r.reply_array(arr_n)?;
            r.reply_integer(left_t)?;
            if debug_flag {
                r.reply_integer(store_unix_t as i64)?;
                r.reply_with_simple_string(&store_cron_s);
            }
            return Ok(())
        };

        let next_datetime = match sche.find_next_schedule(){
            Some(v) => v,
            None => return Err(error!(&format!(
                "TimeGETError: Failed to get next schedule from stored values, format: '{}', timezone: '{}'",
                args[2],
                args[3]
            )))
        };

        let val = format!("{}{}{:?}", next_datetime.timestamp(), store_cron_s, next_datetime.timezone());
        r.open_key_writable(&key).write(&val)?;

        r.reply_array(arr_n)?;
        r.reply_integer(left_t)?;
        if debug_flag {
            r.reply_integer(store_unix_t as i64)?;
            r.reply_with_simple_string(&store_cron_s);
        }
        r.replicate_verbatim();
        Ok(())
    }

    fn str_flags(&self) -> &'static str {
        "write fast deny-oom"
    }

}



pub fn parse_timeset_str(raw_str: &str) -> Result<(u32, &str, &str), RModError> {
    //Note:
    //  inside value fmt: <unixtime><crontab str><timezone string>

    let sp:Vec<&str> = raw_str.split('').collect();
    let unix_t_res = sp[0].parse::<u32>()
        .map_err(|_| error!("RModError: Failed to parse str as timeset format"));
    match unix_t_res {
        Ok(u) => Ok((u, sp[1], sp[2])),
        Err(e) => Err(e)
    }
}
