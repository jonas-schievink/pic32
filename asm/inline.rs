//! Inline assembly implementing the routines exposed in `cortex_m::asm`.
//!
//! If the `inline-asm` feature is enabled, these functions will be directly called by the
//! `cortex-m` wrappers. Otherwise, `cortex-m` links against them via prebuilt archives.
//!
//! All of these functions should be blanket-`unsafe`. `cortex-m` provides safe wrappers where
//! applicable.

#[inline(always)]
pub unsafe fn __nop() {
    asm!("nop");
}

#[inline(always)]
pub unsafe fn __wait() {
    asm!("wait");
}

#[inline(always)]
pub unsafe fn __break() {
    asm!("break");
}
