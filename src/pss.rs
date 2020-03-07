#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key Register"]
    pub psskey: PSSKEY,
    #[doc = "0x04 - Control 0 Register"]
    pub pssctl0: PSSCTL0,
    _reserved2: [u8; 44usize],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub pssie: PSSIE,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub pssifg: PSSIFG,
    #[doc = "0x3c - Clear Interrupt Flag Register"]
    pub pssclrifg: PSSCLRIFG,
}
#[doc = "Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psskey](psskey) module"]
pub type PSSKEY = crate::Reg<u32, _PSSKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSKEY;
#[doc = "`read()` method returns [psskey::R](psskey::R) reader structure"]
impl crate::Readable for PSSKEY {}
#[doc = "`write(|w| ..)` method takes [psskey::W](psskey::W) writer structure"]
impl crate::Writable for PSSKEY {}
#[doc = "Key Register"]
pub mod psskey;
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssctl0](pssctl0) module"]
pub type PSSCTL0 = crate::Reg<u32, _PSSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSCTL0;
#[doc = "`read()` method returns [pssctl0::R](pssctl0::R) reader structure"]
impl crate::Readable for PSSCTL0 {}
#[doc = "`write(|w| ..)` method takes [pssctl0::W](pssctl0::W) writer structure"]
impl crate::Writable for PSSCTL0 {}
#[doc = "Control 0 Register"]
pub mod pssctl0;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssie](pssie) module"]
pub type PSSIE = crate::Reg<u32, _PSSIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSIE;
#[doc = "`read()` method returns [pssie::R](pssie::R) reader structure"]
impl crate::Readable for PSSIE {}
#[doc = "`write(|w| ..)` method takes [pssie::W](pssie::W) writer structure"]
impl crate::Writable for PSSIE {}
#[doc = "Interrupt Enable Register"]
pub mod pssie;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssifg](pssifg) module"]
pub type PSSIFG = crate::Reg<u32, _PSSIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSIFG;
#[doc = "`read()` method returns [pssifg::R](pssifg::R) reader structure"]
impl crate::Readable for PSSIFG {}
#[doc = "Interrupt Flag Register"]
pub mod pssifg;
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssclrifg](pssclrifg) module"]
pub type PSSCLRIFG = crate::Reg<u32, _PSSCLRIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSCLRIFG;
#[doc = "`read()` method returns [pssclrifg::R](pssclrifg::R) reader structure"]
impl crate::Readable for PSSCLRIFG {}
#[doc = "`write(|w| ..)` method takes [pssclrifg::W](pssclrifg::W) writer structure"]
impl crate::Writable for PSSCLRIFG {}
#[doc = "Clear Interrupt Flag Register"]
pub mod pssclrifg;
