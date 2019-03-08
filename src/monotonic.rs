use crate::ClockSource;

#[cfg(all(not(target_os = "macos"), not(target_os = "ios"), not(target_os = "windows")))]
#[derive(Clone)]
pub struct Monotonic;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "windows"))]
#[derive(Clone)]
pub struct Monotonic {
    numer: u64,
    denom: u64,
}

#[cfg(all(not(target_os = "macos"), not(target_os = "ios"), not(target_os = "windows")))]
impl Monotonic {
    pub fn new() -> Monotonic {
        Monotonic {}
    }
}

#[cfg(all(not(target_os = "macos"), not(target_os = "ios"), not(target_os = "windows")))]
impl ClockSource for Monotonic {
    fn now(&self) -> u64 {
        let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
        }
        (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
    }

    fn start(&self) -> u64 {
        self.now()
    }

    fn end(&self) -> u64 {
        self.now()
    }
}

#[cfg(target_os = "windows")]
impl Monotonic {
    pub fn new() -> Monotonic {
        use std::mem;
        use winapi::um::profileapi;
        use winapi::um::winnt::LARGE_INTEGER;

        let numer = unsafe {
            let mut freq = mem::uninitialized();
            debug_assert_eq!(mem::align_of::<LARGE_INTEGER>(), 8);
            let res = profileapi::QueryPerformanceFrequency(&mut freq);
            debug_assert_ne!(res, 0, "failed to query performance frequency: {}", res);
            *freq.QuadPart() as u64
        };
        let denom = 1_000_000_000;

        Monotonic { numer, denom }
    }
}

#[cfg(target_os = "windows")]
impl ClockSource for Monotonic {
    fn now(&self) -> u64 {
        use std::mem;
        use winapi::um::profileapi;
        use winapi::um::winnt::LARGE_INTEGER;

        let raw = unsafe {
            let mut count = mem::uninitialized();
            debug_assert_eq!(mem::align_of::<LARGE_INTEGER>(), 8);
            let res = profileapi::QueryPerformanceCounter(&mut count);
            debug_assert_ne!(res, 0, "failed to query performance counter: {}", res);
            *count.QuadPart() as u64
        };
        (raw * self.numer) / self.denom
    }

    fn start(&self) -> u64 {
        self.now()
    }

    fn end(&self) -> u64 {
        self.now()
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Monotonic {
    pub fn new() -> Monotonic {
        let mut info = libc::mach_timebase_info { numer: 0, denom: 0 };
        unsafe {
            libc::mach_timebase_info(&mut info);
        }

        Monotonic {
            numer: u64::from(info.numer),
            denom: u64::from(info.denom),
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl ClockSource for Monotonic {
    fn now(&self) -> u64 {
        let raw = unsafe { libc::mach_absolute_time() };
        (raw * self.numer) / self.denom
    }

    fn start(&self) -> u64 {
        self.now()
    }

    fn end(&self) -> u64 {
        self.now()
    }
}

impl Default for Monotonic {
    fn default() -> Self {
        Self::new()
    }
}
