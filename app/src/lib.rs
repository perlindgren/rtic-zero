#![no_std]

// Backend, you need to select one.
use rtic_zero_cortex_m as rtic_arch;
// Alternative backend
// use rtic_zero_cortex_m as rtic_arch;

// Machine generated, DO NOT TOUCH!
pub mod rtic_zero_codegen;
pub use rtic_zero_codegen::*;

// here you can add your own library stuff
pub struct A {}
