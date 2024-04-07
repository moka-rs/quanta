#[allow(dead_code)]
#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(miri)))]
pub fn has_counter_support() -> bool {
    let cpuid = raw_cpuid::CpuId::new();
    let has_invariant_tsc = cpuid
        .get_advanced_power_mgmt_info()
        .map_or(false, |apm| apm.has_invariant_tsc());
    let has_rdtscp = cpuid
        .get_extended_processor_and_feature_identifiers()
        .map_or(false, |epf| epf.has_rdtscp());

    has_invariant_tsc && has_rdtscp
}

#[cfg(all(target_arch = "aarch64", not(miri)))]
pub fn has_counter_support() -> bool {
    // AArch64 implies ARMv8 or above, where the system counter is always present.
    true
}

#[allow(dead_code)]
#[cfg(not(any(
    all(target_arch = "x86_64", target_feature = "sse2", not(miri)),
    all(target_arch = "aarch64", not(miri))
)))]
pub fn has_counter_support() -> bool {
    false
}
