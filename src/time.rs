use sys::time::TimeSpec;
use {Result, Errno};
use libc;

libc_enum! {
    #[repr(i32)]
    pub enum ClockId {
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_BOOTTIME,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_BOOTTIME_ALARM,
        CLOCK_MONOTONIC,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_MONOTONIC_COARSE,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_MONOTONIC_FAST,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_MONOTONIC_PRECISE,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_MONOTONIC_RAW,
        #[cfg(any(target_os = "fuchsia",
                  target_env = "uclibc",
                  target_os = "macos",
                  target_os = "ios",
                  target_os = "freebsd",
                  target_os = "dragonfly",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_PROCESS_CPUTIME_ID,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_PROF,
        CLOCK_REALTIME,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_REALTIME_ALARM,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten"))]
        CLOCK_REALTIME_COARSE,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_REALTIME_FAST,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_REALTIME_PRECISE,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_SECOND,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "emscripten",
                  all(target_os = "linux", target_env = "musl")))]
        CLOCK_SGI_CYCLE,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "emscripten",
                  all(target_os = "linux", target_env = "musl")))]
        CLOCK_TAI,
        #[cfg(any(target_os = "fuchsia",
                  target_os = "ios",
                  target_os = "macos",
                  target_os = "freebsd",
                  target_os = "dragonfly",
                  target_os = "linux",
                  target_os = "android",
                  target_os = "emscripten",
                  target_env = "uclibc"))]
        CLOCK_THREAD_CPUTIME_ID,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_UPTIME,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_UPTIME_FAST,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_UPTIME_PRECISE,
        #[cfg(any(target_os = "freebsd",
                  target_os = "dragonfly"))]
        CLOCK_VIRTUAL,
    }
}

pub fn clock_getres(clk_id: ClockId) -> Result<TimeSpec> {
    let mut c_time = libc::timespec {tv_sec: 0, tv_nsec: 0};
    let res = unsafe { libc::clock_getres(clk_id as libc::c_int, &mut c_time) };
    Errno::result(res)?;
    Ok(TimeSpec::from(c_time))
}

pub fn clock_gettime(clk_id: ClockId) -> Result<TimeSpec> {
    let mut c_time = libc::timespec {tv_sec: 0, tv_nsec: 0};
    let res = unsafe { libc::clock_gettime(clk_id as libc::c_int, &mut c_time) };
    Errno::result(res)?;
    Ok(TimeSpec::from(c_time))
}

pub fn clock_settime(clk_id: ClockId, timespec: TimeSpec) -> Result<()> {
    let res = unsafe { libc::clock_settime(clk_id as libc::c_int, timespec.as_ref()) };
    Errno::result(res).map(drop)
}

