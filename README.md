# redis-timer-rs
## Dependencies
- cron format parse
  - [zslayton/cron](https://github.com/zslayton/cron)

## Usage

### SET
``` TIMER.SET <key> <cron fmt str> <timezone str>[default=UTC]```
- if success, return `OK`
- if not success, return `Error`
- macro format is available
  - @yearly @monthly @daily @hourly
- string format at week is available
  - Mon,Sun

### GET
``` TIMER.GET <key> ```
- default
  - always time left (sec) as integer return
- on debug flg
  - value has never updated even when time is upon.
  - detail info as array return
    1. next unix time which is limit
    2. next datetime which is limit
    3. cron format which registered
    4. time left (sec)

### TIMER
``` TIMER <action>[SET, GET] <key> ...```
- level is higher above those command.
- TIMER SET is same as TIMER.SET
- TIMER GET is same as TIMER.GET

For example:
```
127.0.0.1:6379> TIMER.GET sample
1) (integer) 2 <--- left 2sec

127.0.0.1:6379> TIMER.GET sample debug
1) (integer) 1574668140     <--- timeup unixtime
2) 2019-11-25 16:49:00 UTC
3) 0/1 * * * *              <--- sheduled as every 1 minutes later
4) (integer) 2              <--- left 2sec
```


### Use Case:
```
127.0.0.1:6379> TIMER.SET sample_job '0/15 * * * *' 'Asia/Tokyo'  <-- (simple format) timeup per 15 minutes
OK
127.0.0.1:6379> TIMER.GET sample
1) (integer) 18
127.0.0.1:6379> TIMER.GET sample 1
1) (integer) 17
2) (integer) 1574668140
3) */1 * * * *
127.0.0.1:6379> TIMER.GET sample 1
1) (integer) 0   <--- timeup
2) (integer) 1574668140
3) */1 * * * *
127.0.0.1:6379> TIMER.GET sample 1
1) (integer) 37
2) (integer) 1574668200  <--- change to next time by time upon.
3) */1 * * * *
127.0.0.1:6379> TIMER.SET sample_job '* 0/15 * * * Fri *' 'Asia/Tokyo'  <-- (full format) timeup per 15 minutes on Fri
OK
127.0.0.1:6379> TIMER.SET sample_job '@hourly' 'Asia/Tokyo'  <-- (macro format) timeup per 1 hours
OK
```
