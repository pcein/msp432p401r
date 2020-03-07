#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Accelerator Control Register 0"]
    pub aesactl0: AESACTL0,
    #[doc = "0x02 - AES Accelerator Control Register 1"]
    pub aesactl1: AESACTL1,
    #[doc = "0x04 - AES Accelerator Status Register"]
    pub aesastat: AESASTAT,
    #[doc = "0x06 - AES Accelerator Key Register"]
    pub aesakey: AESAKEY,
    #[doc = "0x08 - AES Accelerator Data In Register"]
    pub aesadin: AESADIN,
    #[doc = "0x0a - AES Accelerator Data Out Register"]
    pub aesadout: AESADOUT,
    #[doc = "0x0c - AES Accelerator XORed Data In Register"]
    pub aesaxdin: AESAXDIN,
    #[doc = "0x0e - AES Accelerator XORed Data In Register"]
    pub aesaxin: AESAXIN,
}
#[doc = "AES Accelerator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl0](aesactl0) module"]
pub type AESACTL0 = crate::Reg<u16, _AESACTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESACTL0;
#[doc = "`read()` method returns [aesactl0::R](aesactl0::R) reader structure"]
impl crate::Readable for AESACTL0 {}
#[doc = "`write(|w| ..)` method takes [aesactl0::W](aesactl0::W) writer structure"]
impl crate::Writable for AESACTL0 {}
#[doc = "AES Accelerator Control Register 0"]
pub mod aesactl0;
#[doc = "AES Accelerator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl1](aesactl1) module"]
pub type AESACTL1 = crate::Reg<u16, _AESACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESACTL1;
#[doc = "`read()` method returns [aesactl1::R](aesactl1::R) reader structure"]
impl crate::Readable for AESACTL1 {}
#[doc = "`write(|w| ..)` method takes [aesactl1::W](aesactl1::W) writer structure"]
impl crate::Writable for AESACTL1 {}
#[doc = "AES Accelerator Control Register 1"]
pub mod aesactl1;
#[doc = "AES Accelerator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesastat](aesastat) module"]
pub type AESASTAT = crate::Reg<u16, _AESASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESASTAT;
#[doc = "`read()` method returns [aesastat::R](aesastat::R) reader structure"]
impl crate::Readable for AESASTAT {}
#[doc = "`write(|w| ..)` method takes [aesastat::W](aesastat::W) writer structure"]
impl crate::Writable for AESASTAT {}
#[doc = "AES Accelerator Status Register"]
pub mod aesastat;
#[doc = "AES Accelerator Key Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesakey](aesakey) module"]
pub type AESAKEY = crate::Reg<u16, _AESAKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAKEY;
#[doc = "`write(|w| ..)` method takes [aesakey::W](aesakey::W) writer structure"]
impl crate::Writable for AESAKEY {}
#[doc = "AES Accelerator Key Register"]
pub mod aesakey;
#[doc = "AES Accelerator Data In Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadin](aesadin) module"]
pub type AESADIN = crate::Reg<u16, _AESADIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESADIN;
#[doc = "`write(|w| ..)` method takes [aesadin::W](aesadin::W) writer structure"]
impl crate::Writable for AESADIN {}
#[doc = "AES Accelerator Data In Register"]
pub mod aesadin;
#[doc = "AES Accelerator Data Out Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadout](aesadout) module"]
pub type AESADOUT = crate::Reg<u16, _AESADOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESADOUT;
#[doc = "`write(|w| ..)` method takes [aesadout::W](aesadout::W) writer structure"]
impl crate::Writable for AESADOUT {}
#[doc = "AES Accelerator Data Out Register"]
pub mod aesadout;
#[doc = "AES Accelerator XORed Data In Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxdin](aesaxdin) module"]
pub type AESAXDIN = crate::Reg<u16, _AESAXDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAXDIN;
#[doc = "`write(|w| ..)` method takes [aesaxdin::W](aesaxdin::W) writer structure"]
impl crate::Writable for AESAXDIN {}
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxdin;
#[doc = "AES Accelerator XORed Data In Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxin](aesaxin) module"]
pub type AESAXIN = crate::Reg<u16, _AESAXIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAXIN;
#[doc = "`write(|w| ..)` method takes [aesaxin::W](aesaxin::W) writer structure"]
impl crate::Writable for AESAXIN {}
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxin;
