#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    pub adc14ctl0: ADC14CTL0,
    #[doc = "0x04 - Control 1 Register"]
    pub adc14ctl1: ADC14CTL1,
    #[doc = "0x08 - Window Comparator Low Threshold 0 Register"]
    pub adc14lo0: ADC14LO0,
    #[doc = "0x0c - Window Comparator High Threshold 0 Register"]
    pub adc14hi0: ADC14HI0,
    #[doc = "0x10 - Window Comparator Low Threshold 1 Register"]
    pub adc14lo1: ADC14LO1,
    #[doc = "0x14 - Window Comparator High Threshold 1 Register"]
    pub adc14hi1: ADC14HI1,
    #[doc = "0x18 - Conversion Memory Control Register"]
    pub adc14mctl: [ADC14MCTL; 32],
    #[doc = "0x98 - Conversion Memory Register"]
    pub adc14mem: [ADC14MEM; 32],
    _reserved8: [u8; 36usize],
    #[doc = "0x13c - Interrupt Enable 0 Register"]
    pub adc14ier0: ADC14IER0,
    #[doc = "0x140 - Interrupt Enable 1 Register"]
    pub adc14ier1: ADC14IER1,
    #[doc = "0x144 - Interrupt Flag 0 Register"]
    pub adc14ifgr0: ADC14IFGR0,
    #[doc = "0x148 - Interrupt Flag 1 Register"]
    pub adc14ifgr1: ADC14IFGR1,
    #[doc = "0x14c - Clear Interrupt Flag 0 Register"]
    pub adc14clrifgr0: ADC14CLRIFGR0,
    #[doc = "0x150 - Clear Interrupt Flag 1 Register"]
    pub adc14clrifgr1: ADC14CLRIFGR1,
    #[doc = "0x154 - Interrupt Vector Register"]
    pub adc14iv: ADC14IV,
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ctl0](adc14ctl0) module"]
pub type ADC14CTL0 = crate::Reg<u32, _ADC14CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14CTL0;
#[doc = "`read()` method returns [adc14ctl0::R](adc14ctl0::R) reader structure"]
impl crate::Readable for ADC14CTL0 {}
#[doc = "`write(|w| ..)` method takes [adc14ctl0::W](adc14ctl0::W) writer structure"]
impl crate::Writable for ADC14CTL0 {}
#[doc = "Control 0 Register"]
pub mod adc14ctl0;
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ctl1](adc14ctl1) module"]
pub type ADC14CTL1 = crate::Reg<u32, _ADC14CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14CTL1;
#[doc = "`read()` method returns [adc14ctl1::R](adc14ctl1::R) reader structure"]
impl crate::Readable for ADC14CTL1 {}
#[doc = "`write(|w| ..)` method takes [adc14ctl1::W](adc14ctl1::W) writer structure"]
impl crate::Writable for ADC14CTL1 {}
#[doc = "Control 1 Register"]
pub mod adc14ctl1;
#[doc = "Window Comparator Low Threshold 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14lo0](adc14lo0) module"]
pub type ADC14LO0 = crate::Reg<u32, _ADC14LO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14LO0;
#[doc = "`read()` method returns [adc14lo0::R](adc14lo0::R) reader structure"]
impl crate::Readable for ADC14LO0 {}
#[doc = "`write(|w| ..)` method takes [adc14lo0::W](adc14lo0::W) writer structure"]
impl crate::Writable for ADC14LO0 {}
#[doc = "Window Comparator Low Threshold 0 Register"]
pub mod adc14lo0;
#[doc = "Window Comparator High Threshold 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14hi0](adc14hi0) module"]
pub type ADC14HI0 = crate::Reg<u32, _ADC14HI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14HI0;
#[doc = "`read()` method returns [adc14hi0::R](adc14hi0::R) reader structure"]
impl crate::Readable for ADC14HI0 {}
#[doc = "`write(|w| ..)` method takes [adc14hi0::W](adc14hi0::W) writer structure"]
impl crate::Writable for ADC14HI0 {}
#[doc = "Window Comparator High Threshold 0 Register"]
pub mod adc14hi0;
#[doc = "Window Comparator Low Threshold 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14lo1](adc14lo1) module"]
pub type ADC14LO1 = crate::Reg<u32, _ADC14LO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14LO1;
#[doc = "`read()` method returns [adc14lo1::R](adc14lo1::R) reader structure"]
impl crate::Readable for ADC14LO1 {}
#[doc = "`write(|w| ..)` method takes [adc14lo1::W](adc14lo1::W) writer structure"]
impl crate::Writable for ADC14LO1 {}
#[doc = "Window Comparator Low Threshold 1 Register"]
pub mod adc14lo1;
#[doc = "Window Comparator High Threshold 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14hi1](adc14hi1) module"]
pub type ADC14HI1 = crate::Reg<u32, _ADC14HI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14HI1;
#[doc = "`read()` method returns [adc14hi1::R](adc14hi1::R) reader structure"]
impl crate::Readable for ADC14HI1 {}
#[doc = "`write(|w| ..)` method takes [adc14hi1::W](adc14hi1::W) writer structure"]
impl crate::Writable for ADC14HI1 {}
#[doc = "Window Comparator High Threshold 1 Register"]
pub mod adc14hi1;
#[doc = "Conversion Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14mctl](adc14mctl) module"]
pub type ADC14MCTL = crate::Reg<u32, _ADC14MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14MCTL;
#[doc = "`read()` method returns [adc14mctl::R](adc14mctl::R) reader structure"]
impl crate::Readable for ADC14MCTL {}
#[doc = "`write(|w| ..)` method takes [adc14mctl::W](adc14mctl::W) writer structure"]
impl crate::Writable for ADC14MCTL {}
#[doc = "Conversion Memory Control Register"]
pub mod adc14mctl;
#[doc = "Conversion Memory Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14mem](adc14mem) module"]
pub type ADC14MEM = crate::Reg<u32, _ADC14MEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14MEM;
#[doc = "`read()` method returns [adc14mem::R](adc14mem::R) reader structure"]
impl crate::Readable for ADC14MEM {}
#[doc = "`write(|w| ..)` method takes [adc14mem::W](adc14mem::W) writer structure"]
impl crate::Writable for ADC14MEM {}
#[doc = "Conversion Memory Register"]
pub mod adc14mem;
#[doc = "Interrupt Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ier0](adc14ier0) module"]
pub type ADC14IER0 = crate::Reg<u32, _ADC14IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14IER0;
#[doc = "`read()` method returns [adc14ier0::R](adc14ier0::R) reader structure"]
impl crate::Readable for ADC14IER0 {}
#[doc = "`write(|w| ..)` method takes [adc14ier0::W](adc14ier0::W) writer structure"]
impl crate::Writable for ADC14IER0 {}
#[doc = "Interrupt Enable 0 Register"]
pub mod adc14ier0;
#[doc = "Interrupt Enable 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ier1](adc14ier1) module"]
pub type ADC14IER1 = crate::Reg<u32, _ADC14IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14IER1;
#[doc = "`read()` method returns [adc14ier1::R](adc14ier1::R) reader structure"]
impl crate::Readable for ADC14IER1 {}
#[doc = "`write(|w| ..)` method takes [adc14ier1::W](adc14ier1::W) writer structure"]
impl crate::Writable for ADC14IER1 {}
#[doc = "Interrupt Enable 1 Register"]
pub mod adc14ier1;
#[doc = "Interrupt Flag 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ifgr0](adc14ifgr0) module"]
pub type ADC14IFGR0 = crate::Reg<u32, _ADC14IFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14IFGR0;
#[doc = "`read()` method returns [adc14ifgr0::R](adc14ifgr0::R) reader structure"]
impl crate::Readable for ADC14IFGR0 {}
#[doc = "Interrupt Flag 0 Register"]
pub mod adc14ifgr0;
#[doc = "Interrupt Flag 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ifgr1](adc14ifgr1) module"]
pub type ADC14IFGR1 = crate::Reg<u32, _ADC14IFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14IFGR1;
#[doc = "`read()` method returns [adc14ifgr1::R](adc14ifgr1::R) reader structure"]
impl crate::Readable for ADC14IFGR1 {}
#[doc = "Interrupt Flag 1 Register"]
pub mod adc14ifgr1;
#[doc = "Clear Interrupt Flag 0 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14clrifgr0](adc14clrifgr0) module"]
pub type ADC14CLRIFGR0 = crate::Reg<u32, _ADC14CLRIFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14CLRIFGR0;
#[doc = "`write(|w| ..)` method takes [adc14clrifgr0::W](adc14clrifgr0::W) writer structure"]
impl crate::Writable for ADC14CLRIFGR0 {}
#[doc = "Clear Interrupt Flag 0 Register"]
pub mod adc14clrifgr0;
#[doc = "Clear Interrupt Flag 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14clrifgr1](adc14clrifgr1) module"]
pub type ADC14CLRIFGR1 = crate::Reg<u32, _ADC14CLRIFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14CLRIFGR1;
#[doc = "`read()` method returns [adc14clrifgr1::R](adc14clrifgr1::R) reader structure"]
impl crate::Readable for ADC14CLRIFGR1 {}
#[doc = "`write(|w| ..)` method takes [adc14clrifgr1::W](adc14clrifgr1::W) writer structure"]
impl crate::Writable for ADC14CLRIFGR1 {}
#[doc = "Clear Interrupt Flag 1 Register"]
pub mod adc14clrifgr1;
#[doc = "Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14iv](adc14iv) module"]
pub type ADC14IV = crate::Reg<u32, _ADC14IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14IV;
#[doc = "`read()` method returns [adc14iv::R](adc14iv::R) reader structure"]
impl crate::Readable for ADC14IV {}
#[doc = "`write(|w| ..)` method takes [adc14iv::W](adc14iv::W) writer structure"]
impl crate::Writable for ADC14IV {}
#[doc = "Interrupt Vector Register"]
pub mod adc14iv;
