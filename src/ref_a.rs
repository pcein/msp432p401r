#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - REF Control Register 0"]
    pub refctl0: REFCTL0,
}
#[doc = "REF Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctl0](refctl0) module"]
pub type REFCTL0 = crate::Reg<u16, _REFCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTL0;
#[doc = "`read()` method returns [refctl0::R](refctl0::R) reader structure"]
impl crate::Readable for REFCTL0 {}
#[doc = "`write(|w| ..)` method takes [refctl0::W](refctl0::W) writer structure"]
impl crate::Writable for REFCTL0 {}
#[doc = "REF Control Register 0"]
pub mod refctl0;
