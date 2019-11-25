# redis-timer-rs

## Usage

### SET
``` TIMER.SET <key> <cron fmt str> [<debug flg>] ```
- if success, return `OK`
- if not success, return `Error`

### GET
``` TIMER.GET <key> ```
- default
  - always time left (sec) as integer return
- on debug flg
  - detail info as array return
    1. time left (sec)
    2. next unix time which is limit
    3. cron format which registered

For example:
```
127.0.0.1:6379> TIMER.GET sample 1
1) (integer) 2              <--- left 2sec
2) (integer) 1574668140     <--- timeup unixtime
3) */1 * * * *              <--- sheduled as every 1 minutes later
```


### Use Case:
```
127.0.0.1:6379> TIMER.SET sample_job '*/1 * * * *'  <-- timeup per 1 minutes
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
2) (integer) 1574668200  <--- change to next time by timeup.
3) */1 * * * *
```
