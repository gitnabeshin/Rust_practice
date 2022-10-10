# Tokio plactice

Threading

## run

```
============ thread_test ================
  THREAD:[ThreadId(14)](No.0) cnt=0
  THREAD:[ThreadId(14)](No.0) cnt=1
  THREAD:[ThreadId(14)](No.0) cnt=2
  THREAD:[ThreadId(15)](No.1) cnt=0
  THREAD:[ThreadId(16)](No.2) cnt=0
  THREAD:[ThreadId(15)](No.1) cnt=1
  THREAD:[ThreadId(15)](No.1) cnt=2
  THREAD:[ThreadId(17)](No.3) cnt=0
  THREAD:[ThreadId(16)](No.2) cnt=1
  THREAD:[ThreadId(18)](No.4) cnt=0
  THREAD:[ThreadId(19)](No.5) cnt=0
  THREAD:[ThreadId(20)](No.6) cnt=0
  THREAD:[ThreadId(16)](No.2) cnt=2
  THREAD:[ThreadId(17)](No.3) cnt=1
  THREAD:[ThreadId(21)](No.7) cnt=0
  THREAD:[ThreadId(18)](No.4) cnt=1
  THREAD:[ThreadId(22)](No.8) cnt=0
  THREAD:[ThreadId(23)](No.9) cnt=0
  THREAD:[ThreadId(17)](No.3) cnt=2
  THREAD:[ThreadId(19)](No.5) cnt=1
  THREAD:[ThreadId(20)](No.6) cnt=1
  THREAD:[ThreadId(18)](No.4) cnt=2
  THREAD:[ThreadId(21)](No.7) cnt=1
  THREAD:[ThreadId(19)](No.5) cnt=2
  THREAD:[ThreadId(22)](No.8) cnt=1
  THREAD:[ThreadId(23)](No.9) cnt=1
  THREAD:[ThreadId(20)](No.6) cnt=2
  THREAD:[ThreadId(21)](No.7) cnt=2
  THREAD:[ThreadId(22)](No.8) cnt=2
  THREAD:[ThreadId(23)](No.9) cnt=2
============ tokio_spawn_test ================
   TOKIO:[ThreadId(3)](No.0) cnt=0
   TOKIO:[ThreadId(3)](No.0) cnt=1
   TOKIO:[ThreadId(3)](No.0) cnt=2
   TOKIO:[ThreadId(3)](No.1) cnt=0
   TOKIO:[ThreadId(3)](No.1) cnt=1
   TOKIO:[ThreadId(3)](No.2) cnt=0
   TOKIO:[ThreadId(12)](No.3) cnt=0
   TOKIO:[ThreadId(12)](No.1) cnt=2
   TOKIO:[ThreadId(12)](No.4) cnt=0
   TOKIO:[ThreadId(12)](No.2) cnt=1
   TOKIO:[ThreadId(12)](No.5) cnt=0
   TOKIO:[ThreadId(12)](No.3) cnt=1
   TOKIO:[ThreadId(12)](No.6) cnt=0
   TOKIO:[ThreadId(12)](No.2) cnt=2
   TOKIO:[ThreadId(12)](No.7) cnt=0
   TOKIO:[ThreadId(12)](No.8) cnt=0
   TOKIO:[ThreadId(12)](No.4) cnt=1
   TOKIO:[ThreadId(12)](No.9) cnt=0
   TOKIO:[ThreadId(12)](No.3) cnt=2
   TOKIO:[ThreadId(12)](No.5) cnt=1
   TOKIO:[ThreadId(12)](No.4) cnt=2
   TOKIO:[ThreadId(3)](No.6) cnt=1
   TOKIO:[ThreadId(12)](No.7) cnt=1
   TOKIO:[ThreadId(12)](No.5) cnt=2
   TOKIO:[ThreadId(12)](No.8) cnt=1
   TOKIO:[ThreadId(12)](No.9) cnt=1
   TOKIO:[ThreadId(12)](No.6) cnt=2
   TOKIO:[ThreadId(12)](No.7) cnt=2
   TOKIO:[ThreadId(12)](No.8) cnt=2
   TOKIO:[ThreadId(12)](No.9) cnt=2
```
