#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 14usize],
    #[doc = "0x0e - Capacitive Touch IO x Control Register"]
    pub captiox_ctl: CAPTIOXCTL,
}
#[doc = "Capacitive Touch IO x Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captiox_ctl](captiox_ctl) module"]
pub type CAPTIOXCTL = crate::Reg<u16, _CAPTIOXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTIOXCTL;
#[doc = "`read()` method returns [captiox_ctl::R](captiox_ctl::R) reader structure"]
impl crate::Readable for CAPTIOXCTL {}
#[doc = "`write(|w| ..)` method takes [captiox_ctl::W](captiox_ctl::W) writer structure"]
impl crate::Writable for CAPTIOXCTL {}
#[doc = "Capacitive Touch IO x Control Register"]
pub mod captiox_ctl;
