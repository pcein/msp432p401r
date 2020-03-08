#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub tax_ctl: TAXCTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub tax_cctl: [TAXCCTL; 5],
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - TimerA register"]
    pub tax_r: TAXR,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub tax_ccr: [TAXCCR; 5],
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub tax_ex0: TAXEX0,
    _reserved5: [u8; 12usize],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub tax_iv: TAXIV,
}
#[doc = "TimerAx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ctl](tax_ctl) module"]
pub type TAXCTL = crate::Reg<u16, _TAXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXCTL;
#[doc = "`read()` method returns [tax_ctl::R](tax_ctl::R) reader structure"]
impl crate::Readable for TAXCTL {}
#[doc = "`write(|w| ..)` method takes [tax_ctl::W](tax_ctl::W) writer structure"]
impl crate::Writable for TAXCTL {}
#[doc = "TimerAx Control Register"]
pub mod tax_ctl;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_cctl](tax_cctl) module"]
pub type TAXCCTL = crate::Reg<u16, _TAXCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXCCTL;
#[doc = "`read()` method returns [tax_cctl::R](tax_cctl::R) reader structure"]
impl crate::Readable for TAXCCTL {}
#[doc = "`write(|w| ..)` method takes [tax_cctl::W](tax_cctl::W) writer structure"]
impl crate::Writable for TAXCCTL {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod tax_cctl;
#[doc = "TimerA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_r](tax_r) module"]
pub type TAXR = crate::Reg<u16, _TAXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXR;
#[doc = "`read()` method returns [tax_r::R](tax_r::R) reader structure"]
impl crate::Readable for TAXR {}
#[doc = "`write(|w| ..)` method takes [tax_r::W](tax_r::W) writer structure"]
impl crate::Writable for TAXR {}
#[doc = "TimerA register"]
pub mod tax_r;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ccr](tax_ccr) module"]
pub type TAXCCR = crate::Reg<u16, _TAXCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXCCR;
#[doc = "`read()` method returns [tax_ccr::R](tax_ccr::R) reader structure"]
impl crate::Readable for TAXCCR {}
#[doc = "`write(|w| ..)` method takes [tax_ccr::W](tax_ccr::W) writer structure"]
impl crate::Writable for TAXCCR {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod tax_ccr;
#[doc = "TimerAx Expansion 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ex0](tax_ex0) module"]
pub type TAXEX0 = crate::Reg<u16, _TAXEX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXEX0;
#[doc = "`read()` method returns [tax_ex0::R](tax_ex0::R) reader structure"]
impl crate::Readable for TAXEX0 {}
#[doc = "`write(|w| ..)` method takes [tax_ex0::W](tax_ex0::W) writer structure"]
impl crate::Writable for TAXEX0 {}
#[doc = "TimerAx Expansion 0 Register"]
pub mod tax_ex0;
#[doc = "TimerAx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_iv](tax_iv) module"]
pub type TAXIV = crate::Reg<u16, _TAXIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAXIV;
#[doc = "`read()` method returns [tax_iv::R](tax_iv::R) reader structure"]
impl crate::Readable for TAXIV {}
#[doc = "TimerAx Interrupt Vector Register"]
pub mod tax_iv;
