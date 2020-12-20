//! FFI shim around the inline assembly in `inline.rs`.
//!
//! We use this file to precompile some assembly stubs into the static libraries you can find in
//! `bin`. Apps using the `cortex-m` crate then link against those static libraries and don't need
//! to build this file themselves.
//!
//! Nowadays the assembly stubs are no longer actual assembly files, but actually just this small
//! Rust crate that uses unstable inline assembly, coupled with the `xtask` tool to invoke rustc
//! and build the files.
//!
//! Precompiling this to a static lib allows users to call assembly routines from stable Rust, but
//! also perform [linker plugin LTO] with the precompiled artifacts to completely inline the
//! assembly routines into their code, which brings the "outline assembly" on par with "real" inline
//! assembly.
//!
//! For developers and contributors to `cortex-m`, this setup means that they don't have to install
//! any binutils, assembler, or C compiler to hack on the crate. All they need is to run `cargo
//! xtask assemble` to rebuild the archives from this file.
//!
//! Cool, right?
//!
//! # Rust version management
//!
//! Since inline assembly is still unstable, and we want to ensure that the created blobs are
//! up-to-date in CI, we have to pin the nightly version we use for this. The nightly toolchain is
//! stored in `asm-toolchain`.
//!
//! The `cargo xtask` automation will automatically install the `asm-toolchain` as well as all
//! Cortex-M targets needed to generate the blobs.
//!
//! [linker plugin LTO]: https://doc.rust-lang.org/stable/rustc/linker-plugin-lto.html

#![feature(asm, naked_functions)]
#![no_std]
#![crate_type = "staticlib"]
#![deny(warnings)]

mod inline;

macro_rules! shims {
    (
        $( fn $name:ident( $($arg:ident: $argty:ty),* ) $(-> $ret:ty)?; )+
    ) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $name(
                $($arg: $argty),*
            ) $(-> $ret)? {
                crate::inline::$name($($arg),*)
            }
        )+
    };
}

shims! {
    fn __nop();
    fn __wait();
    fn __break();
}

/// Primary entry point to all bare-metal PIC32 apps.
/// 
/// See "2.6 MCU Initialization" in the PIC32MX Family Data Sheet for more information.
/// 
/// This is part of the assembly stubs because it uses inline assembly to set the stack pointer,
/// and it must be a naked function (since the stack is not usable).
#[no_mangle]
#[naked]
pub unsafe extern "C" fn Reset() -> ! {
    asm!(
        // Initialize Global Pointer and Frame Pointer as zero.
        "move $gp, $zero",
        "move $fp, $zero",

        // The initial stack pointer value is provided by the linker script and depends on the
        // memory layout.
        "li $sp, _stack_start",

        // Jump to the Rust entry point, which will perform the remaining early initialization
        // (like copying `.data` and clearing `.bss` regions).
        "j RustEntry",

        options(noreturn)
    );
}

#[doc(hidden)]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// We *must* define a panic handler here, even though nothing here should ever be able to panic.
///
/// We prove that nothing will ever panic by calling a function that doesn't exist. If the panic
/// handler gets linked in, this causes a linker error. We always build this file with optimizations
/// enabled, but even without them the panic handler should never be linked in.
#[panic_handler]
#[link_section = "panic_handler_unreachable"]
fn panic(_: &core::panic::PanicInfo) -> ! {
    extern "C" {
        #[link_name = "cortex-m internal error: panic handler not optimized out, please file an \
        issue at https://github.com/rust-embedded/cortex-m"]
        fn __cortex_m_should_not_panic() -> !;
    }

    unsafe {
        __cortex_m_should_not_panic();
    }
}
