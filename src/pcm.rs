#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    pub pcmctl0: PCMCTL0,
    #[doc = "0x04 - Control 1 Register"]
    pub pcmctl1: PCMCTL1,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub pcmie: PCMIE,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub pcmifg: PCMIFG,
    #[doc = "0x10 - Clear Interrupt Flag Register"]
    pub pcmclrifg: PCMCLRIFG,
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmctl0](pcmctl0) module"]
pub type PCMCTL0 = crate::Reg<u32, _PCMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCMCTL0;
#[doc = "`read()` method returns [pcmctl0::R](pcmctl0::R) reader structure"]
impl crate::Readable for PCMCTL0 {}
#[doc = "`write(|w| ..)` method takes [pcmctl0::W](pcmctl0::W) writer structure"]
impl crate::Writable for PCMCTL0 {}
#[doc = "Control 0 Register"]
pub mod pcmctl0;
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmctl1](pcmctl1) module"]
pub type PCMCTL1 = crate::Reg<u32, _PCMCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCMCTL1;
#[doc = "`read()` method returns [pcmctl1::R](pcmctl1::R) reader structure"]
impl crate::Readable for PCMCTL1 {}
#[doc = "`write(|w| ..)` method takes [pcmctl1::W](pcmctl1::W) writer structure"]
impl crate::Writable for PCMCTL1 {}
#[doc = "Control 1 Register"]
pub mod pcmctl1;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmie](pcmie) module"]
pub type PCMIE = crate::Reg<u32, _PCMIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCMIE;
#[doc = "`read()` method returns [pcmie::R](pcmie::R) reader structure"]
impl crate::Readable for PCMIE {}
#[doc = "`write(|w| ..)` method takes [pcmie::W](pcmie::W) writer structure"]
impl crate::Writable for PCMIE {}
#[doc = "Interrupt Enable Register"]
pub mod pcmie;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmifg](pcmifg) module"]
pub type PCMIFG = crate::Reg<u32, _PCMIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCMIFG;
#[doc = "`read()` method returns [pcmifg::R](pcmifg::R) reader structure"]
impl crate::Readable for PCMIFG {}
#[doc = "Interrupt Flag Register"]
pub mod pcmifg;
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmclrifg](pcmclrifg) module"]
pub type PCMCLRIFG = crate::Reg<u32, _PCMCLRIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCMCLRIFG;
#[doc = "`write(|w| ..)` method takes [pcmclrifg::W](pcmclrifg::W) writer structure"]
impl crate::Writable for PCMCLRIFG {}
#[doc = "Clear Interrupt Flag Register"]
pub mod pcmclrifg;
