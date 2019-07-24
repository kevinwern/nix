use nix::time::{clock_getres, clock_gettime, clock_settime, ClockId};
use nix::Error;
use nix::errno::Errno;
use nix::sys::time::TimeSpec;
use libc::timespec;

#[test]
pub fn test_clock_getres() {
    clock_getres(ClockId::CLOCK_REALTIME).unwrap();
}

#[test]
pub fn test_clock_gettime() {
    let res1 = clock_gettime(ClockId::CLOCK_REALTIME).unwrap();
    let res2 = clock_gettime(ClockId::CLOCK_REALTIME).unwrap();
    assert!(res1 < res2);
}

#[test]
pub fn test_clock_settime() {
    require_capability!(CAP_SYS_MODULE);
    let ts = TimeSpec::from(timespec{tv_sec: 10000000, tv_nsec: 100});
    let res = clock_settime(ClockId::CLOCK_REALTIME, ts).unwrap();
}

#[test]
pub fn test_clock_settime_err() {
    require_capability!(CAP_SYS_MODULE);
    let ts = TimeSpec::from(timespec{tv_sec: 10000000, tv_nsec: 100});
    let err = clock_settime(ClockId::CLOCK_MONOTONIC, ts).unwrap_err();
    assert_eq!(err, Error::Sys(Errno::EINVAL));
}
