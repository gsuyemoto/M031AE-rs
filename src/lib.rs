#![doc = "Peripheral access API for M031AE_V1 microcontrollers (generated using svd2rust v0.19.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#[cfg(feature = "rt")]
extern crate cortex_m;
extern crate cortex_m_rt;

pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        match self {}
    }
}
#[doc = "CLK Register Map"]
pub struct CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLK {}
impl CLK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const clk::RegisterBlock = 0x4000_0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clk::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CLK {
    type Target = clk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK").finish()
    }
}
#[doc = "CLK Register Map"]
pub mod clk;
#[doc = "SYS Register Map"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sys::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS").finish()
    }
}
#[doc = "SYS Register Map"]
pub mod sys;
#[doc = "SYST_SCR Register Map"]
pub struct SCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCS {}
impl SCS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scs::RegisterBlock = 0xe000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCS {
    type Target = scs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCS").finish()
    }
}
#[doc = "SYST_SCR Register Map"]
pub mod scs;
#[doc = "NMI Register Map"]
pub struct NMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NMI {}
impl NMI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const nmi::RegisterBlock = 0x4000_0300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nmi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for NMI {
    type Target = nmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for NMI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMI").finish()
    }
}
#[doc = "NMI Register Map"]
pub mod nmi;
#[doc = "FMC Register Map"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fmc::RegisterBlock = 0x4000_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC").finish()
    }
}
#[doc = "FMC Register Map"]
pub mod fmc;
#[doc = "GPIO Register Map"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO Register Map"]
pub mod gpio;
#[doc = "PDMA Register Map"]
pub struct PDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA {}
impl PDMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pdma::RegisterBlock = 0x4000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PDMA {
    type Target = pdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PDMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMA").finish()
    }
}
#[doc = "PDMA Register Map"]
pub mod pdma;
#[doc = "TIMER Register Map"]
pub struct TMR01 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR01 {}
impl TMR01 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmr01::RegisterBlock = 0x4005_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr01::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMR01 {
    type Target = tmr01::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMR01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR01").finish()
    }
}
#[doc = "TIMER Register Map"]
pub mod tmr01;
#[doc = "TIMER Register Map"]
pub struct TMR23 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR23 {}
impl TMR23 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmr23::RegisterBlock = 0x4005_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr23::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMR23 {
    type Target = tmr23::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMR23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR23").finish()
    }
}
#[doc = "TIMER Register Map"]
pub mod tmr23;
#[doc = "WDT Register Map"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x4004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "WDT Register Map"]
pub mod wdt;
#[doc = "WWDT Register Map"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wwdt::RegisterBlock = 0x4004_0100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WWDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDT").finish()
    }
}
#[doc = "WWDT Register Map"]
pub mod wwdt;
#[doc = "RTC Register Map"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4004_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "RTC Register Map"]
pub mod rtc;
#[doc = "BPWM Register Map"]
pub struct BPWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPWM0 {}
impl BPWM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bpwm0::RegisterBlock = 0x4005_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bpwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BPWM0 {
    type Target = bpwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BPWM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPWM0").finish()
    }
}
#[doc = "BPWM Register Map"]
pub mod bpwm0;
#[doc = "BPWM Register Map"]
pub struct BPWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPWM1 {}
impl BPWM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bpwm1::RegisterBlock = 0x4005_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bpwm1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BPWM1 {
    type Target = bpwm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BPWM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPWM1").finish()
    }
}
#[doc = "BPWM Register Map"]
pub mod bpwm1;
#[doc = "PWM Register Map"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm0::RegisterBlock = 0x4005_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0").finish()
    }
}
#[doc = "PWM Register Map"]
pub mod pwm0;
#[doc = "PWM Register Map"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm1::RegisterBlock = 0x4005_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1").finish()
    }
}
#[doc = "PWM Register Map"]
pub mod pwm1;
#[doc = "UART Register Map"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4007_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart0;
#[doc = "UART Register Map"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart1::RegisterBlock = 0x4007_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart1;
#[doc = "UART Register Map"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart2::RegisterBlock = 0x4007_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART2").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart2;
#[doc = "UART Register Map"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart3::RegisterBlock = 0x4007_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART3").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart3;
#[doc = "UART Register Map"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart4::RegisterBlock = 0x4007_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart4::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART4").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart4;
#[doc = "UART Register Map"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart5::RegisterBlock = 0x4007_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart5::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART5 {
    type Target = uart5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART5").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart5;
#[doc = "UART Register Map"]
pub struct UART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART6 {}
impl UART6 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart6::RegisterBlock = 0x4007_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart6::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART6 {
    type Target = uart6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART6").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart6;
#[doc = "UART Register Map"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart7::RegisterBlock = 0x4007_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart7::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART7 {
    type Target = uart7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART7").finish()
    }
}
#[doc = "UART Register Map"]
pub mod uart7;
#[doc = "SPI Register Map"]
pub struct SPIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIX {}
impl SPIX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spix::RegisterBlock = 0x4006_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spix::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPIX {
    type Target = spix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPIX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIX").finish()
    }
}
#[doc = "SPI Register Map"]
pub mod spix;
#[doc = "QSPI Register Map"]
pub struct QSPIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPIX {}
impl QSPIX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const qspix::RegisterBlock = 0x4006_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspix::RegisterBlock {
        Self::PTR
    }
}
impl Deref for QSPIX {
    type Target = qspix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for QSPIX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QSPIX").finish()
    }
}
#[doc = "QSPI Register Map"]
pub mod qspix;
#[doc = "I2C Register Map"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2C Register Map"]
pub mod i2c0;
#[doc = "I2C Register Map"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4008_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
#[doc = "I2C Register Map"]
pub mod i2c1;
#[doc = "USCIUART Register Map"]
pub struct UUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UUART0 {}
impl UUART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uuart0::RegisterBlock = 0x400d_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uuart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UUART0 {
    type Target = uuart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UUART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUART0").finish()
    }
}
#[doc = "USCIUART Register Map"]
pub mod uuart0;
#[doc = "USCIUART Register Map"]
pub struct UUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UUART1 {}
impl UUART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uuart1::RegisterBlock = 0x400d_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uuart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UUART1 {
    type Target = uuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UUART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUART1").finish()
    }
}
#[doc = "USCIUART Register Map"]
pub mod uuart1;
#[doc = "USCISPI Register Map"]
pub struct USPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USPI0 {}
impl USPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uspi0::RegisterBlock = 0x400d_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uspi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USPI0 {
    type Target = uspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USPI0").finish()
    }
}
#[doc = "USCISPI Register Map"]
pub mod uspi0;
#[doc = "USCISPI Register Map"]
pub struct USPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USPI1 {}
impl USPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uspi1::RegisterBlock = 0x400d_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uspi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USPI1 {
    type Target = uspi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USPI1").finish()
    }
}
#[doc = "USCISPI Register Map"]
pub mod uspi1;
#[doc = "UI2CI2C Register Map"]
pub struct UI2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UI2C0 {}
impl UI2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ui2c0::RegisterBlock = 0x400d_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ui2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UI2C0 {
    type Target = ui2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UI2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UI2C0").finish()
    }
}
#[doc = "UI2CI2C Register Map"]
pub mod ui2c0;
#[doc = "UI2CI2C Register Map"]
pub struct UI2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UI2C1 {}
impl UI2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ui2c1::RegisterBlock = 0x400d_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ui2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UI2C1 {
    type Target = ui2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UI2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UI2C1").finish()
    }
}
#[doc = "UI2CI2C Register Map"]
pub mod ui2c1;
#[doc = "EBI Register Map"]
pub struct EBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EBI {}
impl EBI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ebi::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ebi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EBI {
    type Target = ebi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EBI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EBI").finish()
    }
}
#[doc = "EBI Register Map"]
pub mod ebi;
#[doc = "USBD Register Map"]
pub struct USBD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBD {}
impl USBD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usbd::RegisterBlock = 0x400c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbd::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USBD {
    type Target = usbd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USBD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBD").finish()
    }
}
#[doc = "USBD Register Map"]
pub mod usbd;
#[doc = "CRC Register Map"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x4003_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "CRC Register Map"]
pub mod crc;
#[doc = "HDIV Register Map"]
pub struct HDIV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HDIV {}
impl HDIV {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hdiv::RegisterBlock = 0x4001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hdiv::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HDIV {
    type Target = hdiv::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDIV").finish()
    }
}
#[doc = "HDIV Register Map"]
pub mod hdiv;
#[doc = "ADC Register Map"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4004_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "ADC Register Map"]
pub mod adc;
#[doc = "ACMP Register Map"]
pub struct ACMP01 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP01 {}
impl ACMP01 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const acmp01::RegisterBlock = 0x4004_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp01::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ACMP01 {
    type Target = acmp01::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ACMP01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACMP01").finish()
    }
}
#[doc = "ACMP Register Map"]
pub mod acmp01;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CLK"]
    pub CLK: CLK,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "SCS"]
    pub SCS: SCS,
    #[doc = "NMI"]
    pub NMI: NMI,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PDMA"]
    pub PDMA: PDMA,
    #[doc = "TMR01"]
    pub TMR01: TMR01,
    #[doc = "TMR23"]
    pub TMR23: TMR23,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "BPWM0"]
    pub BPWM0: BPWM0,
    #[doc = "BPWM1"]
    pub BPWM1: BPWM1,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "UART6"]
    pub UART6: UART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "SPIX"]
    pub SPIX: SPIX,
    #[doc = "QSPIX"]
    pub QSPIX: QSPIX,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "UUART0"]
    pub UUART0: UUART0,
    #[doc = "UUART1"]
    pub UUART1: UUART1,
    #[doc = "USPI0"]
    pub USPI0: USPI0,
    #[doc = "USPI1"]
    pub USPI1: USPI1,
    #[doc = "UI2C0"]
    pub UI2C0: UI2C0,
    #[doc = "UI2C1"]
    pub UI2C1: UI2C1,
    #[doc = "EBI"]
    pub EBI: EBI,
    #[doc = "USBD"]
    pub USBD: USBD,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "HDIV"]
    pub HDIV: HDIV,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "ACMP01"]
    pub ACMP01: ACMP01,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CLK: CLK {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            SCS: SCS {
                _marker: PhantomData,
            },
            NMI: NMI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PDMA: PDMA {
                _marker: PhantomData,
            },
            TMR01: TMR01 {
                _marker: PhantomData,
            },
            TMR23: TMR23 {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            BPWM0: BPWM0 {
                _marker: PhantomData,
            },
            BPWM1: BPWM1 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            UART6: UART6 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            SPIX: SPIX {
                _marker: PhantomData,
            },
            QSPIX: QSPIX {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            UUART0: UUART0 {
                _marker: PhantomData,
            },
            UUART1: UUART1 {
                _marker: PhantomData,
            },
            USPI0: USPI0 {
                _marker: PhantomData,
            },
            USPI1: USPI1 {
                _marker: PhantomData,
            },
            UI2C0: UI2C0 {
                _marker: PhantomData,
            },
            UI2C1: UI2C1 {
                _marker: PhantomData,
            },
            EBI: EBI {
                _marker: PhantomData,
            },
            USBD: USBD {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            HDIV: HDIV {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            ACMP01: ACMP01 {
                _marker: PhantomData,
            },
        }
    }
}
