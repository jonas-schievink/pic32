//! Miscellaneous assembly instructions

// When inline assembly is enabled, pull in the assembly routines here. `call_asm!` will invoke
// these routines.
#[cfg(feature = "inline-asm")]
#[path = "../asm/inline.rs"]
pub(crate) mod inline;

/// `nop` – A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    call_asm!(__nop());
}

/// `wait` – Waits for an interrupt.
#[inline]
pub fn wait() {
    call_asm!(__wait())
}

/// `break` – Breakpoint.
#[inline]
pub fn breakpoint() {
    call_asm!(__break())
}
