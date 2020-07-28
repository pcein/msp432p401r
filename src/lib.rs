#![doc = "Peripheral access API for MSP432P401R microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
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
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn PSS_IRQ();
    fn CS_IRQ();
    fn PCM_IRQ();
    fn WDT_A_IRQ();
    fn FPU_IRQ();
    fn FLCTL_IRQ();
    fn COMP_E0_IRQ();
    fn COMP_E1_IRQ();
    fn TA0_0_IRQ();
    fn TA0_N_IRQ();
    fn TA1_0_IRQ();
    fn TA1_N_IRQ();
    fn TA2_0_IRQ();
    fn TA2_N_IRQ();
    fn TA3_0_IRQ();
    fn TA3_N_IRQ();
    fn EUSCIA0_IRQ();
    fn EUSCIA1_IRQ();
    fn EUSCIA2_IRQ();
    fn EUSCIA3_IRQ();
    fn EUSCIB0_IRQ();
    fn EUSCIB1_IRQ();
    fn EUSCIB2_IRQ();
    fn EUSCIB3_IRQ();
    fn ADC14_IRQ();
    fn T32_INT1_IRQ();
    fn T32_INT2_IRQ();
    fn T32_INTC_IRQ();
    fn AES256_IRQ();
    fn RTC_C_IRQ();
    fn DMA_ERR_IRQ();
    fn DMA_INT3_IRQ();
    fn DMA_INT2_IRQ();
    fn DMA_INT1_IRQ();
    fn DMA_INT0_IRQ();
    fn PORT1_IRQ();
    fn PORT2_IRQ();
    fn PORT3_IRQ();
    fn PORT4_IRQ();
    fn PORT5_IRQ();
    fn PORT6_IRQ();
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
pub static __INTERRUPTS: [Vector; 41] = [
    Vector { _handler: PSS_IRQ },
    Vector { _handler: CS_IRQ },
    Vector { _handler: PCM_IRQ },
    Vector {
        _handler: WDT_A_IRQ,
    },
    Vector { _handler: FPU_IRQ },
    Vector {
        _handler: FLCTL_IRQ,
    },
    Vector {
        _handler: COMP_E0_IRQ,
    },
    Vector {
        _handler: COMP_E1_IRQ,
    },
    Vector {
        _handler: TA0_0_IRQ,
    },
    Vector {
        _handler: TA0_N_IRQ,
    },
    Vector {
        _handler: TA1_0_IRQ,
    },
    Vector {
        _handler: TA1_N_IRQ,
    },
    Vector {
        _handler: TA2_0_IRQ,
    },
    Vector {
        _handler: TA2_N_IRQ,
    },
    Vector {
        _handler: TA3_0_IRQ,
    },
    Vector {
        _handler: TA3_N_IRQ,
    },
    Vector {
        _handler: EUSCIA0_IRQ,
    },
    Vector {
        _handler: EUSCIA1_IRQ,
    },
    Vector {
        _handler: EUSCIA2_IRQ,
    },
    Vector {
        _handler: EUSCIA3_IRQ,
    },
    Vector {
        _handler: EUSCIB0_IRQ,
    },
    Vector {
        _handler: EUSCIB1_IRQ,
    },
    Vector {
        _handler: EUSCIB2_IRQ,
    },
    Vector {
        _handler: EUSCIB3_IRQ,
    },
    Vector {
        _handler: ADC14_IRQ,
    },
    Vector {
        _handler: T32_INT1_IRQ,
    },
    Vector {
        _handler: T32_INT2_IRQ,
    },
    Vector {
        _handler: T32_INTC_IRQ,
    },
    Vector {
        _handler: AES256_IRQ,
    },
    Vector {
        _handler: RTC_C_IRQ,
    },
    Vector {
        _handler: DMA_ERR_IRQ,
    },
    Vector {
        _handler: DMA_INT3_IRQ,
    },
    Vector {
        _handler: DMA_INT2_IRQ,
    },
    Vector {
        _handler: DMA_INT1_IRQ,
    },
    Vector {
        _handler: DMA_INT0_IRQ,
    },
    Vector {
        _handler: PORT1_IRQ,
    },
    Vector {
        _handler: PORT2_IRQ,
    },
    Vector {
        _handler: PORT3_IRQ,
    },
    Vector {
        _handler: PORT4_IRQ,
    },
    Vector {
        _handler: PORT5_IRQ,
    },
    Vector {
        _handler: PORT6_IRQ,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - PSS Interrupt"]
    PSS_IRQ = 0,
    #[doc = "1 - CS Interrupt"]
    CS_IRQ = 1,
    #[doc = "2 - PCM Interrupt"]
    PCM_IRQ = 2,
    #[doc = "3 - WDT_A Interrupt"]
    WDT_A_IRQ = 3,
    #[doc = "4 - FPU Interrupt"]
    FPU_IRQ = 4,
    #[doc = "5 - Flash Controller Interrupt"]
    FLCTL_IRQ = 5,
    #[doc = "6 - COMP_E0 Interrupt"]
    COMP_E0_IRQ = 6,
    #[doc = "7 - COMP_E1 Interrupt"]
    COMP_E1_IRQ = 7,
    #[doc = "8 - TA0_0 Interrupt"]
    TA0_0_IRQ = 8,
    #[doc = "9 - TA0_N Interrupt"]
    TA0_N_IRQ = 9,
    #[doc = "10 - TA1_0 Interrupt"]
    TA1_0_IRQ = 10,
    #[doc = "11 - TA1_N Interrupt"]
    TA1_N_IRQ = 11,
    #[doc = "12 - TA2_0 Interrupt"]
    TA2_0_IRQ = 12,
    #[doc = "13 - TA2_N Interrupt"]
    TA2_N_IRQ = 13,
    #[doc = "14 - TA3_0 Interrupt"]
    TA3_0_IRQ = 14,
    #[doc = "15 - TA3_N Interrupt"]
    TA3_N_IRQ = 15,
    #[doc = "16 - EUSCIA0 Interrupt"]
    EUSCIA0_IRQ = 16,
    #[doc = "17 - EUSCIA1 Interrupt"]
    EUSCIA1_IRQ = 17,
    #[doc = "18 - EUSCIA2 Interrupt"]
    EUSCIA2_IRQ = 18,
    #[doc = "19 - EUSCIA3 Interrupt"]
    EUSCIA3_IRQ = 19,
    #[doc = "20 - EUSCIB0 Interrupt"]
    EUSCIB0_IRQ = 20,
    #[doc = "21 - EUSCIB1 Interrupt"]
    EUSCIB1_IRQ = 21,
    #[doc = "22 - EUSCIB2 Interrupt"]
    EUSCIB2_IRQ = 22,
    #[doc = "23 - EUSCIB3 Interrupt"]
    EUSCIB3_IRQ = 23,
    #[doc = "24 - ADC14 Interrupt"]
    ADC14_IRQ = 24,
    #[doc = "25 - T32_INT1 Interrupt"]
    T32_INT1_IRQ = 25,
    #[doc = "26 - T32_INT2 Interrupt"]
    T32_INT2_IRQ = 26,
    #[doc = "27 - T32_INTC Interrupt"]
    T32_INTC_IRQ = 27,
    #[doc = "28 - AES256 Interrupt"]
    AES256_IRQ = 28,
    #[doc = "29 - RTC_C Interrupt"]
    RTC_C_IRQ = 29,
    #[doc = "30 - DMA_ERR Interrupt"]
    DMA_ERR_IRQ = 30,
    #[doc = "31 - DMA_INT3 Interrupt"]
    DMA_INT3_IRQ = 31,
    #[doc = "32 - DMA_INT2 Interrupt"]
    DMA_INT2_IRQ = 32,
    #[doc = "33 - DMA_INT1 Interrupt"]
    DMA_INT1_IRQ = 33,
    #[doc = "34 - DMA_INT0 Interrupt"]
    DMA_INT0_IRQ = 34,
    #[doc = "35 - Port1 Interrupt"]
    PORT1_IRQ = 35,
    #[doc = "36 - Port2 Interrupt"]
    PORT2_IRQ = 36,
    #[doc = "37 - Port3 Interrupt"]
    PORT3_IRQ = 37,
    #[doc = "38 - Port4 Interrupt"]
    PORT4_IRQ = 38,
    #[doc = "39 - Port5 Interrupt"]
    PORT5_IRQ = 39,
    #[doc = "40 - Port6 Interrupt"]
    PORT6_IRQ = 40,
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
#[doc = "TLV"]
pub struct TLV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TLV {}
impl TLV {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tlv::RegisterBlock {
        0x0020_1000 as *const _
    }
}
impl Deref for TLV {
    type Target = tlv::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TLV::ptr() }
    }
}
#[doc = "TLV"]
pub mod tlv;
#[doc = "TIMER_A0"]
pub struct TIMER_A0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A0 {}
impl TIMER_A0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIMER_A0 {
    type Target = timer_a0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_A0::ptr() }
    }
}
#[doc = "TIMER_A0"]
pub mod timer_a0;
#[doc = "TIMER_A1"]
pub struct TIMER_A1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A1 {}
impl TIMER_A1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a1::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIMER_A1 {
    type Target = timer_a1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_A1::ptr() }
    }
}
#[doc = "TIMER_A1"]
pub mod timer_a1;
#[doc = "TIMER_A2"]
pub struct TIMER_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A2 {}
impl TIMER_A2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a2::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIMER_A2 {
    type Target = timer_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_A2::ptr() }
    }
}
#[doc = "TIMER_A2"]
pub mod timer_a2;
#[doc = "TIMER_A3"]
pub struct TIMER_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A3 {}
impl TIMER_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a3::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIMER_A3 {
    type Target = timer_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_A3::ptr() }
    }
}
#[doc = "TIMER_A3"]
pub mod timer_a3;
#[doc = "EUSCI_A0"]
pub struct EUSCI_A0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A0 {}
impl EUSCI_A0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_a0::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for EUSCI_A0 {
    type Target = eusci_a0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A0::ptr() }
    }
}
#[doc = "EUSCI_A0"]
pub mod eusci_a0;
#[doc = "EUSCI_A1"]
pub struct EUSCI_A1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A1 {}
impl EUSCI_A1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_a1::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for EUSCI_A1 {
    type Target = eusci_a1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A1::ptr() }
    }
}
#[doc = "EUSCI_A1"]
pub mod eusci_a1;
#[doc = "EUSCI_A2"]
pub struct EUSCI_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A2 {}
impl EUSCI_A2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_a2::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for EUSCI_A2 {
    type Target = eusci_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A2::ptr() }
    }
}
#[doc = "EUSCI_A2"]
pub mod eusci_a2;
#[doc = "EUSCI_A3"]
pub struct EUSCI_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A3 {}
impl EUSCI_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_a3::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for EUSCI_A3 {
    type Target = eusci_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A3::ptr() }
    }
}
#[doc = "EUSCI_A3"]
pub mod eusci_a3;
#[doc = "EUSCI_B0"]
pub struct EUSCI_B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B0 {}
impl EUSCI_B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_b0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for EUSCI_B0 {
    type Target = eusci_b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B0::ptr() }
    }
}
#[doc = "EUSCI_B0"]
pub mod eusci_b0;
#[doc = "EUSCI_B1"]
pub struct EUSCI_B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B1 {}
impl EUSCI_B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_b1::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for EUSCI_B1 {
    type Target = eusci_b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B1::ptr() }
    }
}
#[doc = "EUSCI_B1"]
pub mod eusci_b1;
#[doc = "EUSCI_B2"]
pub struct EUSCI_B2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B2 {}
impl EUSCI_B2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_b2::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for EUSCI_B2 {
    type Target = eusci_b2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B2::ptr() }
    }
}
#[doc = "EUSCI_B2"]
pub mod eusci_b2;
#[doc = "EUSCI_B3"]
pub struct EUSCI_B3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B3 {}
impl EUSCI_B3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eusci_b3::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for EUSCI_B3 {
    type Target = eusci_b3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B3::ptr() }
    }
}
#[doc = "EUSCI_B3"]
pub mod eusci_b3;
#[doc = "REF_A"]
pub struct REF_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REF_A {}
impl REF_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ref_a::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for REF_A {
    type Target = ref_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*REF_A::ptr() }
    }
}
#[doc = "REF_A"]
pub mod ref_a;
#[doc = "COMP_E0"]
pub struct COMP_E0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP_E0 {}
impl COMP_E0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp_e0::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for COMP_E0 {
    type Target = comp_e0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP_E0::ptr() }
    }
}
#[doc = "COMP_E0"]
pub mod comp_e0;
#[doc = "COMP_E1"]
pub struct COMP_E1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP_E1 {}
impl COMP_E1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp_e1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for COMP_E1 {
    type Target = comp_e1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP_E1::ptr() }
    }
}
#[doc = "COMP_E1"]
pub mod comp_e1;
#[doc = "AES256"]
pub struct AES256 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES256 {}
impl AES256 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes256::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for AES256 {
    type Target = aes256::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES256::ptr() }
    }
}
#[doc = "AES256"]
pub mod aes256;
#[doc = "CRC32"]
pub struct CRC32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC32 {}
impl CRC32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc32::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for CRC32 {
    type Target = crc32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC32::ptr() }
    }
}
#[doc = "CRC32"]
pub mod crc32;
#[doc = "RTC_C"]
pub struct RTC_C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_C {}
impl RTC_C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_c::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for RTC_C {
    type Target = rtc_c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC_C::ptr() }
    }
}
#[doc = "RTC_C"]
pub mod rtc_c;
#[doc = "WDT_A"]
pub struct WDT_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_A {}
impl WDT_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt_a::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for WDT_A {
    type Target = wdt_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT_A::ptr() }
    }
}
#[doc = "WDT_A"]
pub mod wdt_a;
#[doc = "DIO"]
pub struct DIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DIO {}
impl DIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dio::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for DIO {
    type Target = dio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DIO::ptr() }
    }
}
#[doc = "DIO"]
pub mod dio;
#[doc = "PMAP"]
pub struct PMAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMAP {}
impl PMAP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmap::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for PMAP {
    type Target = pmap::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMAP::ptr() }
    }
}
#[doc = "PMAP"]
pub mod pmap;
#[doc = "CAPTIO0"]
pub struct CAPTIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPTIO0 {}
impl CAPTIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const captio0::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for CAPTIO0 {
    type Target = captio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPTIO0::ptr() }
    }
}
#[doc = "CAPTIO0"]
pub mod captio0;
#[doc = "CAPTIO1"]
pub struct CAPTIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPTIO1 {}
impl CAPTIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const captio1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for CAPTIO1 {
    type Target = captio1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPTIO1::ptr() }
    }
}
#[doc = "CAPTIO1"]
pub mod captio1;
#[doc = "TIMER32"]
pub struct TIMER32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER32 {}
impl TIMER32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer32::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for TIMER32 {
    type Target = timer32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER32::ptr() }
    }
}
#[doc = "TIMER32"]
pub mod timer32;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "PCM"]
pub struct PCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCM {}
impl PCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcm::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for PCM {
    type Target = pcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCM::ptr() }
    }
}
#[doc = "PCM"]
pub mod pcm;
#[doc = "CS"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS"]
pub mod cs;
#[doc = "PSS"]
pub struct PSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PSS {}
impl PSS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pss::RegisterBlock {
        0x4001_0800 as *const _
    }
}
impl Deref for PSS {
    type Target = pss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PSS::ptr() }
    }
}
#[doc = "PSS"]
pub mod pss;
#[doc = "FLCTL"]
pub struct FLCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLCTL {}
impl FLCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flctl::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for FLCTL {
    type Target = flctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLCTL::ptr() }
    }
}
#[doc = "FLCTL"]
pub mod flctl;
#[doc = "ADC14"]
pub struct ADC14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC14 {}
impl ADC14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc14::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for ADC14 {
    type Target = adc14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC14::ptr() }
    }
}
#[doc = "ADC14"]
pub mod adc14;
#[doc = "System Control Space for ARM core: SCnSCB, SCB, SysTick, NVIC, CoreDebug, MPU, FPU"]
pub struct SYSTEMCONTROLSPACE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROLSPACE {}
impl SYSTEMCONTROLSPACE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control_space::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROLSPACE {
    type Target = system_control_space::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROLSPACE::ptr() }
    }
}
#[doc = "System Control Space for ARM core: SCnSCB, SCB, SysTick, NVIC, CoreDebug, MPU, FPU"]
pub mod system_control_space;
#[doc = "RSTCTL"]
pub struct RSTCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTCTL {}
impl RSTCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstctl::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for RSTCTL {
    type Target = rstctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTCTL::ptr() }
    }
}
#[doc = "RSTCTL"]
pub mod rstctl;
#[doc = "SYSCTL"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctl::RegisterBlock {
        0xe004_3000 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "SYSCTL"]
pub mod sysctl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "TLV"]
    pub TLV: TLV,
    #[doc = "TIMER_A0"]
    pub TIMER_A0: TIMER_A0,
    #[doc = "TIMER_A1"]
    pub TIMER_A1: TIMER_A1,
    #[doc = "TIMER_A2"]
    pub TIMER_A2: TIMER_A2,
    #[doc = "TIMER_A3"]
    pub TIMER_A3: TIMER_A3,
    #[doc = "EUSCI_A0"]
    pub EUSCI_A0: EUSCI_A0,
    #[doc = "EUSCI_A1"]
    pub EUSCI_A1: EUSCI_A1,
    #[doc = "EUSCI_A2"]
    pub EUSCI_A2: EUSCI_A2,
    #[doc = "EUSCI_A3"]
    pub EUSCI_A3: EUSCI_A3,
    #[doc = "EUSCI_B0"]
    pub EUSCI_B0: EUSCI_B0,
    #[doc = "EUSCI_B1"]
    pub EUSCI_B1: EUSCI_B1,
    #[doc = "EUSCI_B2"]
    pub EUSCI_B2: EUSCI_B2,
    #[doc = "EUSCI_B3"]
    pub EUSCI_B3: EUSCI_B3,
    #[doc = "REF_A"]
    pub REF_A: REF_A,
    #[doc = "COMP_E0"]
    pub COMP_E0: COMP_E0,
    #[doc = "COMP_E1"]
    pub COMP_E1: COMP_E1,
    #[doc = "AES256"]
    pub AES256: AES256,
    #[doc = "CRC32"]
    pub CRC32: CRC32,
    #[doc = "RTC_C"]
    pub RTC_C: RTC_C,
    #[doc = "WDT_A"]
    pub WDT_A: WDT_A,
    #[doc = "DIO"]
    pub DIO: DIO,
    #[doc = "PMAP"]
    pub PMAP: PMAP,
    #[doc = "CAPTIO0"]
    pub CAPTIO0: CAPTIO0,
    #[doc = "CAPTIO1"]
    pub CAPTIO1: CAPTIO1,
    #[doc = "TIMER32"]
    pub TIMER32: TIMER32,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "PCM"]
    pub PCM: PCM,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "PSS"]
    pub PSS: PSS,
    #[doc = "FLCTL"]
    pub FLCTL: FLCTL,
    #[doc = "ADC14"]
    pub ADC14: ADC14,
    #[doc = "SYSTEMCONTROLSPACE"]
    pub SYSTEMCONTROLSPACE: SYSTEMCONTROLSPACE,
    #[doc = "RSTCTL"]
    pub RSTCTL: RSTCTL,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
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
            TLV: TLV {
                _marker: PhantomData,
            },
            TIMER_A0: TIMER_A0 {
                _marker: PhantomData,
            },
            TIMER_A1: TIMER_A1 {
                _marker: PhantomData,
            },
            TIMER_A2: TIMER_A2 {
                _marker: PhantomData,
            },
            TIMER_A3: TIMER_A3 {
                _marker: PhantomData,
            },
            EUSCI_A0: EUSCI_A0 {
                _marker: PhantomData,
            },
            EUSCI_A1: EUSCI_A1 {
                _marker: PhantomData,
            },
            EUSCI_A2: EUSCI_A2 {
                _marker: PhantomData,
            },
            EUSCI_A3: EUSCI_A3 {
                _marker: PhantomData,
            },
            EUSCI_B0: EUSCI_B0 {
                _marker: PhantomData,
            },
            EUSCI_B1: EUSCI_B1 {
                _marker: PhantomData,
            },
            EUSCI_B2: EUSCI_B2 {
                _marker: PhantomData,
            },
            EUSCI_B3: EUSCI_B3 {
                _marker: PhantomData,
            },
            REF_A: REF_A {
                _marker: PhantomData,
            },
            COMP_E0: COMP_E0 {
                _marker: PhantomData,
            },
            COMP_E1: COMP_E1 {
                _marker: PhantomData,
            },
            AES256: AES256 {
                _marker: PhantomData,
            },
            CRC32: CRC32 {
                _marker: PhantomData,
            },
            RTC_C: RTC_C {
                _marker: PhantomData,
            },
            WDT_A: WDT_A {
                _marker: PhantomData,
            },
            DIO: DIO {
                _marker: PhantomData,
            },
            PMAP: PMAP {
                _marker: PhantomData,
            },
            CAPTIO0: CAPTIO0 {
                _marker: PhantomData,
            },
            CAPTIO1: CAPTIO1 {
                _marker: PhantomData,
            },
            TIMER32: TIMER32 {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            PCM: PCM {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            PSS: PSS {
                _marker: PhantomData,
            },
            FLCTL: FLCTL {
                _marker: PhantomData,
            },
            ADC14: ADC14 {
                _marker: PhantomData,
            },
            SYSTEMCONTROLSPACE: SYSTEMCONTROLSPACE {
                _marker: PhantomData,
            },
            RSTCTL: RSTCTL {
                _marker: PhantomData,
            },
            SYSCTL: SYSCTL {
                _marker: PhantomData,
            },
        }
    }
}
