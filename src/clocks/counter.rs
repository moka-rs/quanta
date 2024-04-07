#[derive(Clone, Debug, Default)]
pub struct Counter;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(miri)))]
impl Counter {
    pub fn now(&self) -> u64 {
        unsafe { ::core::arch::x86_64::_rdtsc() }
    }
}

#[cfg(all(target_arch = "aarch64", not(miri)))]
impl Counter {
    pub fn now(&self) -> u64 {
        let count: u64;

        unsafe {
            ::core::arch::asm!("mrs {}, cntvct_el0", out(reg) count);
        }

        count
    }
}

#[cfg(not(any(
    all(target_arch = "x86_64", target_feature = "sse2", not(miri)),
    all(target_arch = "aarch64", not(miri))
)))]
impl Counter {
    pub fn now(&self) -> u64 {
        panic!("can't use counter without TSC (x86_64) or system counter (ARM) support");
    }
}
