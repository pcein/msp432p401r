#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cex_ctl0: CEXCTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cex_ctl1: CEXCTL1,
    #[doc = "0x04 - Comparator Control Register 2"]
    pub cex_ctl2: CEXCTL2,
    #[doc = "0x06 - Comparator Control Register 3"]
    pub cex_ctl3: CEXCTL3,
    _reserved4: [u8; 4usize],
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    pub cex_int: CEXINT,
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    pub cex_iv: CEXIV,
}
#[doc = "Comparator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl0](cex_ctl0) module"]
pub type CEXCTL0 = crate::Reg<u16, _CEXCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXCTL0;
#[doc = "`read()` method returns [cex_ctl0::R](cex_ctl0::R) reader structure"]
impl crate::Readable for CEXCTL0 {}
#[doc = "`write(|w| ..)` method takes [cex_ctl0::W](cex_ctl0::W) writer structure"]
impl crate::Writable for CEXCTL0 {}
#[doc = "Comparator Control Register 0"]
pub mod cex_ctl0;
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl1](cex_ctl1) module"]
pub type CEXCTL1 = crate::Reg<u16, _CEXCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXCTL1;
#[doc = "`read()` method returns [cex_ctl1::R](cex_ctl1::R) reader structure"]
impl crate::Readable for CEXCTL1 {}
#[doc = "`write(|w| ..)` method takes [cex_ctl1::W](cex_ctl1::W) writer structure"]
impl crate::Writable for CEXCTL1 {}
#[doc = "Comparator Control Register 1"]
pub mod cex_ctl1;
#[doc = "Comparator Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl2](cex_ctl2) module"]
pub type CEXCTL2 = crate::Reg<u16, _CEXCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXCTL2;
#[doc = "`read()` method returns [cex_ctl2::R](cex_ctl2::R) reader structure"]
impl crate::Readable for CEXCTL2 {}
#[doc = "`write(|w| ..)` method takes [cex_ctl2::W](cex_ctl2::W) writer structure"]
impl crate::Writable for CEXCTL2 {}
#[doc = "Comparator Control Register 2"]
pub mod cex_ctl2;
#[doc = "Comparator Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl3](cex_ctl3) module"]
pub type CEXCTL3 = crate::Reg<u16, _CEXCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXCTL3;
#[doc = "`read()` method returns [cex_ctl3::R](cex_ctl3::R) reader structure"]
impl crate::Readable for CEXCTL3 {}
#[doc = "`write(|w| ..)` method takes [cex_ctl3::W](cex_ctl3::W) writer structure"]
impl crate::Writable for CEXCTL3 {}
#[doc = "Comparator Control Register 3"]
pub mod cex_ctl3;
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_int](cex_int) module"]
pub type CEXINT = crate::Reg<u16, _CEXINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXINT;
#[doc = "`read()` method returns [cex_int::R](cex_int::R) reader structure"]
impl crate::Readable for CEXINT {}
#[doc = "`write(|w| ..)` method takes [cex_int::W](cex_int::W) writer structure"]
impl crate::Writable for CEXINT {}
#[doc = "Comparator Interrupt Control Register"]
pub mod cex_int;
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_iv](cex_iv) module"]
pub type CEXIV = crate::Reg<u16, _CEXIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEXIV;
#[doc = "`read()` method returns [cex_iv::R](cex_iv::R) reader structure"]
impl crate::Readable for CEXIV {}
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cex_iv;
