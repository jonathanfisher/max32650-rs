#![doc = "Peripheral access API for MAX32650 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT0();
    fn USB();
    fn RTC();
    fn TRNG();
    fn TMR0();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn TMR5();
    fn I2C0();
    fn UART0();
    fn UART1();
    fn SPI0();
    fn SPI17Y1();
    fn SPI17Y2();
    fn ADC();
    fn FLASH_CONTROLLER();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn CRYPTO_ENGINE();
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn UART2();
    fn I2C1();
    fn SPIXFC();
    fn WDT1();
    fn GPIO3();
    fn PT();
    fn SMARTDMA();
    fn HPB();
    fn SDHC();
    fn ONEWIRE();
    fn DMA4();
    fn DMA5();
    fn DMA6();
    fn DMA7();
    fn DMA8();
    fn DMA9();
    fn DMA10();
    fn DMA11();
    fn DMA12();
    fn DMA13();
    fn DMA14();
    fn DMA15();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 80] = [
    Vector { _reserved: 0 },
    Vector { _handler: WDT0 },
    Vector { _handler: USB },
    Vector { _handler: RTC },
    Vector { _handler: TRNG },
    Vector { _handler: TMR0 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: TMR5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI17Y1 },
    Vector { _handler: SPI17Y2 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: FLASH_CONTROLLER,
    },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector {
        _handler: CRYPTO_ENGINE,
    },
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART2 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector { _handler: SPIXFC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT1 },
    Vector { _handler: GPIO3 },
    Vector { _handler: PT },
    Vector { _handler: SMARTDMA },
    Vector { _handler: HPB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDHC },
    Vector { _handler: ONEWIRE },
    Vector { _handler: DMA4 },
    Vector { _handler: DMA5 },
    Vector { _handler: DMA6 },
    Vector { _handler: DMA7 },
    Vector { _handler: DMA8 },
    Vector { _handler: DMA9 },
    Vector { _handler: DMA10 },
    Vector { _handler: DMA11 },
    Vector { _handler: DMA12 },
    Vector { _handler: DMA13 },
    Vector { _handler: DMA14 },
    Vector { _handler: DMA15 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "1 - WDT0 IRQ"]
    WDT0 = 1,
    #[doc = "2 - USB IRQ"]
    USB = 2,
    #[doc = "3 - RTC interrupt."]
    RTC = 3,
    #[doc = "4 - TRNG interrupt."]
    TRNG = 4,
    #[doc = "5 - TMR0 IRQ"]
    TMR0 = 5,
    #[doc = "6 - TMR1 IRQ"]
    TMR1 = 6,
    #[doc = "7 - TMR2 IRQ"]
    TMR2 = 7,
    #[doc = "8 - TMR3 IRQ"]
    TMR3 = 8,
    #[doc = "9 - TMR4 IRQ"]
    TMR4 = 9,
    #[doc = "10 - TMR5 IRQ"]
    TMR5 = 10,
    #[doc = "13 - I2C0 IRQ"]
    I2C0 = 13,
    #[doc = "14 - UART0 IRQ"]
    UART0 = 14,
    #[doc = "15 - UART1 IRQ"]
    UART1 = 15,
    #[doc = "16 - SPI0 IRQ"]
    SPI0 = 16,
    #[doc = "17 - SPI17Y1 IRQ"]
    SPI17Y1 = 17,
    #[doc = "18 - SPI17Y2 IRQ"]
    SPI17Y2 = 18,
    #[doc = "20 - ADC IRQ"]
    ADC = 20,
    #[doc = "23 - Flash Controller interrupt."]
    FLASH_CONTROLLER = 23,
    #[doc = "24 - GPIO0 interrupt."]
    GPIO0 = 24,
    #[doc = "25 - GPIO1 IRQ"]
    GPIO1 = 25,
    #[doc = "26 - GPIO2 IRQ"]
    GPIO2 = 26,
    #[doc = "27 - Crypto Engine interrupt."]
    CRYPTO_ENGINE = 27,
    #[doc = "28 - DMA Channel 0 IRQ"]
    DMA0 = 28,
    #[doc = "29 - DMA Channel 1 IRQ"]
    DMA1 = 29,
    #[doc = "30 - DMA Channel 2 IRQ"]
    DMA2 = 30,
    #[doc = "31 - DMA Channel 3 IRQ"]
    DMA3 = 31,
    #[doc = "34 - UART2 IRQ"]
    UART2 = 34,
    #[doc = "36 - I2C1 IRQ"]
    I2C1 = 36,
    #[doc = "38 - SPIXFC IRQ"]
    SPIXFC = 38,
    #[doc = "57 - WDT1 IRQ"]
    WDT1 = 57,
    #[doc = "58 - GPIO3 IRQ"]
    GPIO3 = 58,
    #[doc = "59 - Pulse Train IRQ"]
    PT = 59,
    #[doc = "60 - Smart DMA interrupt."]
    SMARTDMA = 60,
    #[doc = "61 - HPB Interrupt."]
    HPB = 61,
    #[doc = "66 - SDHC IRQ"]
    SDHC = 66,
    #[doc = "67 - 1-Wire IRQ"]
    ONEWIRE = 67,
    #[doc = "68 - DMA Channel 4 IRQ"]
    DMA4 = 68,
    #[doc = "69 - DMA Channel 5 IRQ"]
    DMA5 = 69,
    #[doc = "70 - DMA Channel 6 IRQ"]
    DMA6 = 70,
    #[doc = "71 - DMA Channel 7 IRQ"]
    DMA7 = 71,
    #[doc = "72 - DMA Channel 8 IRQ"]
    DMA8 = 72,
    #[doc = "73 - DMA Channel 9 IRQ"]
    DMA9 = 73,
    #[doc = "74 - DMA Channel 10 IRQ"]
    DMA10 = 74,
    #[doc = "75 - DMA Channel 11 IRQ"]
    DMA11 = 75,
    #[doc = "76 - DMA Channel 12 IRQ"]
    DMA12 = 76,
    #[doc = "77 - DMA Channel 13 IRQ"]
    DMA13 = 77,
    #[doc = "78 - DMA Channel 14 IRQ"]
    DMA14 = 78,
    #[doc = "79 - DMA Channel 15 IRQ"]
    DMA15 = 79,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "10-bit Analog to Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "10-bit Analog to Digital Converter"]
pub mod adc;
#[doc = "Battery-Backed Function Control."]
pub struct BBFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BBFC {}
impl BBFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bbfc::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for BBFC {
    type Target = bbfc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BBFC::ptr() }
    }
}
#[doc = "Battery-Backed Function Control."]
pub mod bbfc;
#[doc = "Battery-Backed Registers."]
pub struct BBSIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BBSIR {}
impl BBSIR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bbsir::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for BBSIR {
    type Target = bbsir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BBSIR::ptr() }
    }
}
#[doc = "Battery-Backed Registers."]
pub mod bbsir;
#[doc = "The Cryptographic Accelerator used to assist the computationally intensive operations of several common algorithms."]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "The Cryptographic Accelerator used to assist the computationally intensive operations of several common algorithms."]
pub mod crypto;
#[doc = "External Memory Cache Controller Registers."]
pub struct EMCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMCC {}
impl EMCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emcc::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for EMCC {
    type Target = emcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMCC::ptr() }
    }
}
#[doc = "External Memory Cache Controller Registers."]
pub mod emcc;
#[doc = "DMA Controller Fully programmable, chaining capable DMA channels."]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA Controller Fully programmable, chaining capable DMA channels."]
pub mod dma;
#[doc = "Flash Memory Control."]
pub struct FLC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLC {}
impl FLC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flc::RegisterBlock {
        0x4002_9000 as *const _
    }
}
impl Deref for FLC {
    type Target = flc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLC::ptr() }
    }
}
#[doc = "Flash Memory Control."]
pub mod flc;
#[doc = "Global Control Registers."]
pub struct GCR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCR {}
impl GCR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gcr::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for GCR {
    type Target = gcr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCR::ptr() }
    }
}
#[doc = "Global Control Registers."]
pub mod gcr;
#[doc = "Individual I/O for each GPIO"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO0::ptr() }
    }
}
#[doc = "Individual I/O for each GPIO"]
pub mod gpio0;
#[doc = "Individual I/O for each GPIO 1"]
pub struct GPIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO1 {}
impl GPIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for GPIO1 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO1::ptr() }
    }
}
#[doc = "Individual I/O for each GPIO 2"]
pub struct GPIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO2 {}
impl GPIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for GPIO2 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO2::ptr() }
    }
}
#[doc = "Individual I/O for each GPIO 3"]
pub struct GPIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO3 {}
impl GPIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for GPIO3 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO3::ptr() }
    }
}
#[doc = "HyperBus."]
pub struct HPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HPB {}
impl HPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hpb::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for HPB {
    type Target = hpb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HPB::ptr() }
    }
}
#[doc = "HyperBus."]
pub mod hpb;
#[doc = "Inter-Integrated Circuit."]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4001_d000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit."]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit. 1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4001_e000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Instruction Cache Controller Registers"]
pub struct ICC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC0 {}
impl ICC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icc0::RegisterBlock {
        0x4002_a000 as *const _
    }
}
impl Deref for ICC0 {
    type Target = icc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICC0::ptr() }
    }
}
#[doc = "Instruction Cache Controller Registers"]
pub mod icc0;
#[doc = "Instruction Cache Controller Registers 1"]
pub struct ICC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC1 {}
impl ICC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icc0::RegisterBlock {
        0x4002_f000 as *const _
    }
}
impl Deref for ICC1 {
    type Target = icc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICC1::ptr() }
    }
}
#[doc = "Non Battery-Backed Function Control."]
pub struct NBBFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NBBFC {}
impl NBBFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nbbfc::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for NBBFC {
    type Target = nbbfc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NBBFC::ptr() }
    }
}
#[doc = "Non Battery-Backed Function Control."]
pub mod nbbfc;
#[doc = "One Time Programmable Memory controller."]
pub struct OTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTP {}
impl OTP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otp::RegisterBlock {
        0x4004_1000 as *const _
    }
}
impl Deref for OTP {
    type Target = otp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTP::ptr() }
    }
}
#[doc = "One Time Programmable Memory controller."]
pub mod otp;
#[doc = "1-Wire Master Interface."]
pub struct OWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OWM {}
impl OWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const owm::RegisterBlock {
        0x4003_d000 as *const _
    }
}
impl Deref for OWM {
    type Target = owm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OWM::ptr() }
    }
}
#[doc = "1-Wire Master Interface."]
pub mod owm;
#[doc = "Pulse Train Generation"]
pub struct PTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTG {}
impl PTG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ptg::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for PTG {
    type Target = ptg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PTG::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub mod ptg;
#[doc = "Pulse Train"]
pub struct PT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT {}
impl PT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c020 as *const _
    }
}
impl Deref for PT {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT::ptr() }
    }
}
#[doc = "Pulse Train"]
pub mod pt;
#[doc = "Pulse Train 1"]
pub struct PT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT1 {}
impl PT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c040 as *const _
    }
}
impl Deref for PT1 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT1::ptr() }
    }
}
#[doc = "Pulse Train 2"]
pub struct PT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT2 {}
impl PT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c060 as *const _
    }
}
impl Deref for PT2 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT2::ptr() }
    }
}
#[doc = "Pulse Train 3"]
pub struct PT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT3 {}
impl PT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c080 as *const _
    }
}
impl Deref for PT3 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT3::ptr() }
    }
}
#[doc = "Pulse Train 4"]
pub struct PT4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT4 {}
impl PT4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c0a0 as *const _
    }
}
impl Deref for PT4 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT4::ptr() }
    }
}
#[doc = "Pulse Train 5"]
pub struct PT5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT5 {}
impl PT5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c0c0 as *const _
    }
}
impl Deref for PT5 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT5::ptr() }
    }
}
#[doc = "Pulse Train 6"]
pub struct PT6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT6 {}
impl PT6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c0e0 as *const _
    }
}
impl Deref for PT6 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT6::ptr() }
    }
}
#[doc = "Pulse Train 7"]
pub struct PT7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT7 {}
impl PT7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c100 as *const _
    }
}
impl Deref for PT7 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT7::ptr() }
    }
}
#[doc = "Pulse Train 8"]
pub struct PT8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT8 {}
impl PT8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c120 as *const _
    }
}
impl Deref for PT8 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT8::ptr() }
    }
}
#[doc = "Pulse Train 9"]
pub struct PT9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT9 {}
impl PT9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c140 as *const _
    }
}
impl Deref for PT9 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT9::ptr() }
    }
}
#[doc = "Pulse Train 10"]
pub struct PT10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT10 {}
impl PT10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c160 as *const _
    }
}
impl Deref for PT10 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT10::ptr() }
    }
}
#[doc = "Pulse Train 11"]
pub struct PT11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT11 {}
impl PT11 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c180 as *const _
    }
}
impl Deref for PT11 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT11::ptr() }
    }
}
#[doc = "Pulse Train 12"]
pub struct PT12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT12 {}
impl PT12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c1a0 as *const _
    }
}
impl Deref for PT12 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT12::ptr() }
    }
}
#[doc = "Pulse Train 13"]
pub struct PT13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT13 {}
impl PT13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c1c0 as *const _
    }
}
impl Deref for PT13 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT13::ptr() }
    }
}
#[doc = "Pulse Train 14"]
pub struct PT14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT14 {}
impl PT14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c1e0 as *const _
    }
}
impl Deref for PT14 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT14::ptr() }
    }
}
#[doc = "Pulse Train 15"]
pub struct PT15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT15 {}
impl PT15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pt::RegisterBlock {
        0x4003_c200 as *const _
    }
}
impl Deref for PT15 {
    type Target = pt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PT15::ptr() }
    }
}
#[doc = "Power Sequencer / Low Power Control Register."]
pub struct PWRSEQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRSEQ {}
impl PWRSEQ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwrseq::RegisterBlock {
        0x4000_6800 as *const _
    }
}
impl Deref for PWRSEQ {
    type Target = pwrseq::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWRSEQ::ptr() }
    }
}
#[doc = "Power Sequencer / Low Power Control Register."]
pub mod pwrseq;
#[doc = "Real Time Clock and Alarm."]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock and Alarm."]
pub mod rtc;
#[doc = "SDHC/SDIO Controller"]
pub struct SDHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDHC {}
impl SDHC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdhc::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for SDHC {
    type Target = sdhc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDHC::ptr() }
    }
}
#[doc = "SDHC/SDIO Controller"]
pub mod sdhc;
#[doc = "Smart DMA"]
pub struct SDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMA {}
impl SDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdma::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for SDMA {
    type Target = sdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMA::ptr() }
    }
}
#[doc = "Smart DMA"]
pub mod sdma;
#[doc = "The Semaphore peripheral allows multiple cores in a system to cooperate when accessing shred resources. The peripheral contains eight semaphores that can be atomically set and cleared. It is left to the discretion of the software architect to decide how and when the semaphores are used and how they are allocated. Existing hardware does not have to be modified for this type of cooperative sharing, and the use of semaphores is exclusively within the software domain."]
pub struct SEMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEMA {}
impl SEMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sema::RegisterBlock {
        0x4003_e000 as *const _
    }
}
impl Deref for SEMA {
    type Target = sema::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEMA::ptr() }
    }
}
#[doc = "The Semaphore peripheral allows multiple cores in a system to cooperate when accessing shred resources. The peripheral contains eight semaphores that can be atomically set and cleared. It is left to the discretion of the software architect to decide how and when the semaphores are used and how they are allocated. Existing hardware does not have to be modified for this type of cooperative sharing, and the use of semaphores is exclusively within the software domain."]
pub mod sema;
#[doc = "System Initialization Registers."]
pub struct SIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIR {}
impl SIR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sir::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for SIR {
    type Target = sir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIR::ptr() }
    }
}
#[doc = "System Initialization Registers."]
pub mod sir;
#[doc = "The Security Monitor block used to monitor system threat conditions."]
pub struct SMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMON {}
impl SMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smon::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SMON {
    type Target = smon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMON::ptr() }
    }
}
#[doc = "The Security Monitor block used to monitor system threat conditions."]
pub mod smon;
#[doc = "SPID peripheral."]
pub struct SPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPID {}
impl SPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spid::RegisterBlock {
        0x4003_a000 as *const _
    }
}
impl Deref for SPID {
    type Target = spid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPID::ptr() }
    }
}
#[doc = "SPID peripheral."]
pub mod spid;
#[doc = "SPI XiP Flash Configuration Controller"]
pub struct SPIXFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIXFC {}
impl SPIXFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spixfc::RegisterBlock {
        0x4002_7000 as *const _
    }
}
impl Deref for SPIXFC {
    type Target = spixfc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIXFC::ptr() }
    }
}
#[doc = "SPI XiP Flash Configuration Controller"]
pub mod spixfc;
#[doc = "SPI XiP Master Controller FIFO."]
pub struct SPIXC_FIFO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIXC_FIFO {}
impl SPIXC_FIFO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spixc_fifo::RegisterBlock {
        0x400b_c000 as *const _
    }
}
impl Deref for SPIXC_FIFO {
    type Target = spixc_fifo::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIXC_FIFO::ptr() }
    }
}
#[doc = "SPI XiP Master Controller FIFO."]
pub mod spixc_fifo;
#[doc = "SPIXF Master"]
pub struct SPIXF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIXF {}
impl SPIXF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spixf::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for SPIXF {
    type Target = spixf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIXF::ptr() }
    }
}
#[doc = "SPIXF Master"]
pub mod spixf;
#[doc = "SPI peripheral."]
pub struct SPI17Y {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI17Y {}
impl SPI17Y {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi17y::RegisterBlock {
        0x4004_6000 as *const _
    }
}
impl Deref for SPI17Y {
    type Target = spi17y::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI17Y::ptr() }
    }
}
#[doc = "SPI peripheral."]
pub mod spi17y;
#[doc = "SPI peripheral. 1"]
pub struct SPI17Y1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI17Y1 {}
impl SPI17Y1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi17y::RegisterBlock {
        0x4004_7000 as *const _
    }
}
impl Deref for SPI17Y1 {
    type Target = spi17y::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI17Y1::ptr() }
    }
}
#[doc = "SPI peripheral. 2"]
pub struct SPI17Y2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI17Y2 {}
impl SPI17Y2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi17y::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for SPI17Y2 {
    type Target = spi17y::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI17Y2::ptr() }
    }
}
#[doc = "Serial Peripheral Interface."]
pub struct SPIMSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIMSS {}
impl SPIMSS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spimss::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for SPIMSS {
    type Target = spimss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIMSS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface."]
pub mod spimss;
#[doc = "Color LCD Controller"]
pub struct CLCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLCD {}
impl CLCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clcd::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for CLCD {
    type Target = clcd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLCD::ptr() }
    }
}
#[doc = "Color LCD Controller"]
pub mod clcd;
#[doc = "32-bit reloadable timer that can be used for timing and event counting."]
pub struct TMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR0 {}
impl TMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TMR0 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR0::ptr() }
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting."]
pub mod tmr0;
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 1"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 2"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 3"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 4"]
pub struct TMR4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR4 {}
impl TMR4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TMR4 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR4::ptr() }
    }
}
#[doc = "32-bit reloadable timer that can be used for timing and event counting. 5"]
pub struct TMR5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR5 {}
impl TMR5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for TMR5 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR5::ptr() }
    }
}
#[doc = "Random Number Generator."]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x400b_5000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "Random Number Generator."]
pub mod trng;
#[doc = "UART"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4004_2000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART"]
pub mod uart0;
#[doc = "UART 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4004_3000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART 2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "USB 2.0 High-speed Controller."]
pub struct USBHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHS {}
impl USBHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhs::RegisterBlock {
        0x400b_1000 as *const _
    }
}
impl Deref for USBHS {
    type Target = usbhs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHS::ptr() }
    }
}
#[doc = "USB 2.0 High-speed Controller."]
pub mod usbhs;
#[doc = "Watchdog Timer 0"]
pub struct WDT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0 {}
impl WDT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for WDT0 {
    type Target = wdt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT0::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub mod wdt0;
#[doc = "Watchdog Timer 0 1"]
pub struct WDT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT1 {}
impl WDT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for WDT1 {
    type Target = wdt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT1::ptr() }
    }
}
#[doc = "AES Keys."]
pub struct AES_KEY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES_KEY {}
impl AES_KEY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes_key::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for AES_KEY {
    type Target = aes_key::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES_KEY::ptr() }
    }
}
#[doc = "AES Keys."]
pub mod aes_key;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "BBFC"]
    pub BBFC: BBFC,
    #[doc = "BBSIR"]
    pub BBSIR: BBSIR,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "EMCC"]
    pub EMCC: EMCC,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FLC"]
    pub FLC: FLC,
    #[doc = "GCR"]
    pub GCR: GCR,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
    #[doc = "HPB"]
    pub HPB: HPB,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "ICC0"]
    pub ICC0: ICC0,
    #[doc = "ICC1"]
    pub ICC1: ICC1,
    #[doc = "NBBFC"]
    pub NBBFC: NBBFC,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "OWM"]
    pub OWM: OWM,
    #[doc = "PTG"]
    pub PTG: PTG,
    #[doc = "PT"]
    pub PT: PT,
    #[doc = "PT1"]
    pub PT1: PT1,
    #[doc = "PT2"]
    pub PT2: PT2,
    #[doc = "PT3"]
    pub PT3: PT3,
    #[doc = "PT4"]
    pub PT4: PT4,
    #[doc = "PT5"]
    pub PT5: PT5,
    #[doc = "PT6"]
    pub PT6: PT6,
    #[doc = "PT7"]
    pub PT7: PT7,
    #[doc = "PT8"]
    pub PT8: PT8,
    #[doc = "PT9"]
    pub PT9: PT9,
    #[doc = "PT10"]
    pub PT10: PT10,
    #[doc = "PT11"]
    pub PT11: PT11,
    #[doc = "PT12"]
    pub PT12: PT12,
    #[doc = "PT13"]
    pub PT13: PT13,
    #[doc = "PT14"]
    pub PT14: PT14,
    #[doc = "PT15"]
    pub PT15: PT15,
    #[doc = "PWRSEQ"]
    pub PWRSEQ: PWRSEQ,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDHC"]
    pub SDHC: SDHC,
    #[doc = "SDMA"]
    pub SDMA: SDMA,
    #[doc = "SEMA"]
    pub SEMA: SEMA,
    #[doc = "SIR"]
    pub SIR: SIR,
    #[doc = "SMON"]
    pub SMON: SMON,
    #[doc = "SPID"]
    pub SPID: SPID,
    #[doc = "SPIXFC"]
    pub SPIXFC: SPIXFC,
    #[doc = "SPIXC_FIFO"]
    pub SPIXC_FIFO: SPIXC_FIFO,
    #[doc = "SPIXF"]
    pub SPIXF: SPIXF,
    #[doc = "SPI17Y"]
    pub SPI17Y: SPI17Y,
    #[doc = "SPI17Y1"]
    pub SPI17Y1: SPI17Y1,
    #[doc = "SPI17Y2"]
    pub SPI17Y2: SPI17Y2,
    #[doc = "SPIMSS"]
    pub SPIMSS: SPIMSS,
    #[doc = "CLCD"]
    pub CLCD: CLCD,
    #[doc = "TMR0"]
    pub TMR0: TMR0,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "TMR4"]
    pub TMR4: TMR4,
    #[doc = "TMR5"]
    pub TMR5: TMR5,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "USBHS"]
    pub USBHS: USBHS,
    #[doc = "WDT0"]
    pub WDT0: WDT0,
    #[doc = "WDT1"]
    pub WDT1: WDT1,
    #[doc = "AES_KEY"]
    pub AES_KEY: AES_KEY,
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
            ADC: ADC {
                _marker: PhantomData,
            },
            BBFC: BBFC {
                _marker: PhantomData,
            },
            BBSIR: BBSIR {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            EMCC: EMCC {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FLC: FLC {
                _marker: PhantomData,
            },
            GCR: GCR {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
            HPB: HPB {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            ICC0: ICC0 {
                _marker: PhantomData,
            },
            ICC1: ICC1 {
                _marker: PhantomData,
            },
            NBBFC: NBBFC {
                _marker: PhantomData,
            },
            OTP: OTP {
                _marker: PhantomData,
            },
            OWM: OWM {
                _marker: PhantomData,
            },
            PTG: PTG {
                _marker: PhantomData,
            },
            PT: PT {
                _marker: PhantomData,
            },
            PT1: PT1 {
                _marker: PhantomData,
            },
            PT2: PT2 {
                _marker: PhantomData,
            },
            PT3: PT3 {
                _marker: PhantomData,
            },
            PT4: PT4 {
                _marker: PhantomData,
            },
            PT5: PT5 {
                _marker: PhantomData,
            },
            PT6: PT6 {
                _marker: PhantomData,
            },
            PT7: PT7 {
                _marker: PhantomData,
            },
            PT8: PT8 {
                _marker: PhantomData,
            },
            PT9: PT9 {
                _marker: PhantomData,
            },
            PT10: PT10 {
                _marker: PhantomData,
            },
            PT11: PT11 {
                _marker: PhantomData,
            },
            PT12: PT12 {
                _marker: PhantomData,
            },
            PT13: PT13 {
                _marker: PhantomData,
            },
            PT14: PT14 {
                _marker: PhantomData,
            },
            PT15: PT15 {
                _marker: PhantomData,
            },
            PWRSEQ: PWRSEQ {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SDHC: SDHC {
                _marker: PhantomData,
            },
            SDMA: SDMA {
                _marker: PhantomData,
            },
            SEMA: SEMA {
                _marker: PhantomData,
            },
            SIR: SIR {
                _marker: PhantomData,
            },
            SMON: SMON {
                _marker: PhantomData,
            },
            SPID: SPID {
                _marker: PhantomData,
            },
            SPIXFC: SPIXFC {
                _marker: PhantomData,
            },
            SPIXC_FIFO: SPIXC_FIFO {
                _marker: PhantomData,
            },
            SPIXF: SPIXF {
                _marker: PhantomData,
            },
            SPI17Y: SPI17Y {
                _marker: PhantomData,
            },
            SPI17Y1: SPI17Y1 {
                _marker: PhantomData,
            },
            SPI17Y2: SPI17Y2 {
                _marker: PhantomData,
            },
            SPIMSS: SPIMSS {
                _marker: PhantomData,
            },
            CLCD: CLCD {
                _marker: PhantomData,
            },
            TMR0: TMR0 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            TMR4: TMR4 {
                _marker: PhantomData,
            },
            TMR5: TMR5 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
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
            USBHS: USBHS {
                _marker: PhantomData,
            },
            WDT0: WDT0 {
                _marker: PhantomData,
            },
            WDT1: WDT1 {
                _marker: PhantomData,
            },
            AES_KEY: AES_KEY {
                _marker: PhantomData,
            },
        }
    }
}
