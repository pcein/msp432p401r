#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key Register"]
    pub cskey: CSKEY,
    #[doc = "0x04 - Control 0 Register"]
    pub csctl0: CSCTL0,
    #[doc = "0x08 - Control 1 Register"]
    pub csctl1: CSCTL1,
    #[doc = "0x0c - Control 2 Register"]
    pub csctl2: CSCTL2,
    #[doc = "0x10 - Control 3 Register"]
    pub csctl3: CSCTL3,
    _reserved5: [u8; 28usize],
    #[doc = "0x30 - Clock Enable Register"]
    pub csclken: CSCLKEN,
    #[doc = "0x34 - Status Register"]
    pub csstat: CSSTAT,
    _reserved7: [u8; 8usize],
    #[doc = "0x40 - Interrupt Enable Register"]
    pub csie: CSIE,
    _reserved8: [u8; 4usize],
    #[doc = "0x48 - Interrupt Flag Register"]
    pub csifg: CSIFG,
    _reserved9: [u8; 4usize],
    #[doc = "0x50 - Clear Interrupt Flag Register"]
    pub csclrifg: CSCLRIFG,
    _reserved10: [u8; 4usize],
    #[doc = "0x58 - Set Interrupt Flag Register"]
    pub cssetifg: CSSETIFG,
    _reserved11: [u8; 4usize],
    #[doc = "0x60 - DCO External Resistor Cailbration 0 Register"]
    pub csdcoercal0: CSDCOERCAL0,
    #[doc = "0x64 - DCO External Resistor Calibration 1 Register"]
    pub csdcoercal1: CSDCOERCAL1,
}
#[doc = "Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cskey](cskey) module"]
pub type CSKEY = crate::Reg<u32, _CSKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSKEY;
#[doc = "`read()` method returns [cskey::R](cskey::R) reader structure"]
impl crate::Readable for CSKEY {}
#[doc = "`write(|w| ..)` method takes [cskey::W](cskey::W) writer structure"]
impl crate::Writable for CSKEY {}
#[doc = "Key Register"]
pub mod cskey;
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl0](csctl0) module"]
pub type CSCTL0 = crate::Reg<u32, _CSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL0;
#[doc = "`read()` method returns [csctl0::R](csctl0::R) reader structure"]
impl crate::Readable for CSCTL0 {}
#[doc = "`write(|w| ..)` method takes [csctl0::W](csctl0::W) writer structure"]
impl crate::Writable for CSCTL0 {}
#[doc = "Control 0 Register"]
pub mod csctl0;
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl1](csctl1) module"]
pub type CSCTL1 = crate::Reg<u32, _CSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL1;
#[doc = "`read()` method returns [csctl1::R](csctl1::R) reader structure"]
impl crate::Readable for CSCTL1 {}
#[doc = "`write(|w| ..)` method takes [csctl1::W](csctl1::W) writer structure"]
impl crate::Writable for CSCTL1 {}
#[doc = "Control 1 Register"]
pub mod csctl1;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl2](csctl2) module"]
pub type CSCTL2 = crate::Reg<u32, _CSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL2;
#[doc = "`read()` method returns [csctl2::R](csctl2::R) reader structure"]
impl crate::Readable for CSCTL2 {}
#[doc = "`write(|w| ..)` method takes [csctl2::W](csctl2::W) writer structure"]
impl crate::Writable for CSCTL2 {}
#[doc = "Control 2 Register"]
pub mod csctl2;
#[doc = "Control 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl3](csctl3) module"]
pub type CSCTL3 = crate::Reg<u32, _CSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL3;
#[doc = "`read()` method returns [csctl3::R](csctl3::R) reader structure"]
impl crate::Readable for CSCTL3 {}
#[doc = "`write(|w| ..)` method takes [csctl3::W](csctl3::W) writer structure"]
impl crate::Writable for CSCTL3 {}
#[doc = "Control 3 Register"]
pub mod csctl3;
#[doc = "Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csclken](csclken) module"]
pub type CSCLKEN = crate::Reg<u32, _CSCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCLKEN;
#[doc = "`read()` method returns [csclken::R](csclken::R) reader structure"]
impl crate::Readable for CSCLKEN {}
#[doc = "`write(|w| ..)` method takes [csclken::W](csclken::W) writer structure"]
impl crate::Writable for CSCLKEN {}
#[doc = "Clock Enable Register"]
pub mod csclken;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csstat](csstat) module"]
pub type CSSTAT = crate::Reg<u32, _CSSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSSTAT;
#[doc = "`read()` method returns [csstat::R](csstat::R) reader structure"]
impl crate::Readable for CSSTAT {}
#[doc = "Status Register"]
pub mod csstat;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csie](csie) module"]
pub type CSIE = crate::Reg<u32, _CSIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIE;
#[doc = "`read()` method returns [csie::R](csie::R) reader structure"]
impl crate::Readable for CSIE {}
#[doc = "`write(|w| ..)` method takes [csie::W](csie::W) writer structure"]
impl crate::Writable for CSIE {}
#[doc = "Interrupt Enable Register"]
pub mod csie;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csifg](csifg) module"]
pub type CSIFG = crate::Reg<u32, _CSIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIFG;
#[doc = "`read()` method returns [csifg::R](csifg::R) reader structure"]
impl crate::Readable for CSIFG {}
#[doc = "Interrupt Flag Register"]
pub mod csifg;
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csclrifg](csclrifg) module"]
pub type CSCLRIFG = crate::Reg<u32, _CSCLRIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCLRIFG;
#[doc = "`write(|w| ..)` method takes [csclrifg::W](csclrifg::W) writer structure"]
impl crate::Writable for CSCLRIFG {}
#[doc = "Clear Interrupt Flag Register"]
pub mod csclrifg;
#[doc = "Set Interrupt Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cssetifg](cssetifg) module"]
pub type CSSETIFG = crate::Reg<u32, _CSSETIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSSETIFG;
#[doc = "`write(|w| ..)` method takes [cssetifg::W](cssetifg::W) writer structure"]
impl crate::Writable for CSSETIFG {}
#[doc = "Set Interrupt Flag Register"]
pub mod cssetifg;
#[doc = "DCO External Resistor Cailbration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdcoercal0](csdcoercal0) module"]
pub type CSDCOERCAL0 = crate::Reg<u32, _CSDCOERCAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSDCOERCAL0;
#[doc = "`read()` method returns [csdcoercal0::R](csdcoercal0::R) reader structure"]
impl crate::Readable for CSDCOERCAL0 {}
#[doc = "`write(|w| ..)` method takes [csdcoercal0::W](csdcoercal0::W) writer structure"]
impl crate::Writable for CSDCOERCAL0 {}
#[doc = "DCO External Resistor Cailbration 0 Register"]
pub mod csdcoercal0;
#[doc = "DCO External Resistor Calibration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdcoercal1](csdcoercal1) module"]
pub type CSDCOERCAL1 = crate::Reg<u32, _CSDCOERCAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSDCOERCAL1;
#[doc = "`read()` method returns [csdcoercal1::R](csdcoercal1::R) reader structure"]
impl crate::Readable for CSDCOERCAL1 {}
#[doc = "`write(|w| ..)` method takes [csdcoercal1::W](csdcoercal1::W) writer structure"]
impl crate::Writable for CSDCOERCAL1 {}
#[doc = "DCO External Resistor Calibration 1 Register"]
pub mod csdcoercal1;
