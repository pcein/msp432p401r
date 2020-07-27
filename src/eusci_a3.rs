#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    pub ucax_ctlw0: UCAXCTLW0,
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub ucax_ctlw1: UCAXCTLW1,
    _reserved2: [u8; 2usize],
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    pub ucax_brw: UCAXBRW,
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub ucax_mctlw: UCAXMCTLW,
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    pub ucax_statw: UCAXSTATW,
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    pub ucax_rxbuf: UCAXRXBUF,
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    pub ucax_txbuf: UCAXTXBUF,
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub ucax_abctl: UCAXABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub ucax_irctl: UCAXIRCTL,
    _reserved9: [u8; 6usize],
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    pub ucax_ie: UCAXIE,
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    pub ucax_ifg: UCAXIFG,
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    pub ucax_iv: UCAXIV,
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ctlw0](ucax_ctlw0) module"]
pub type UCAXCTLW0 = crate::Reg<u16, _UCAXCTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXCTLW0;
#[doc = "`read()` method returns [ucax_ctlw0::R](ucax_ctlw0::R) reader structure"]
impl crate::Readable for UCAXCTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucax_ctlw0::W](ucax_ctlw0::W) writer structure"]
impl crate::Writable for UCAXCTLW0 {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod ucax_ctlw0;
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ctlw1](ucax_ctlw1) module"]
pub type UCAXCTLW1 = crate::Reg<u16, _UCAXCTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXCTLW1;
#[doc = "`read()` method returns [ucax_ctlw1::R](ucax_ctlw1::R) reader structure"]
impl crate::Readable for UCAXCTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucax_ctlw1::W](ucax_ctlw1::W) writer structure"]
impl crate::Writable for UCAXCTLW1 {}
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod ucax_ctlw1;
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_brw](ucax_brw) module"]
pub type UCAXBRW = crate::Reg<u16, _UCAXBRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXBRW;
#[doc = "`read()` method returns [ucax_brw::R](ucax_brw::R) reader structure"]
impl crate::Readable for UCAXBRW {}
#[doc = "`write(|w| ..)` method takes [ucax_brw::W](ucax_brw::W) writer structure"]
impl crate::Writable for UCAXBRW {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod ucax_brw;
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_mctlw](ucax_mctlw) module"]
pub type UCAXMCTLW = crate::Reg<u16, _UCAXMCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXMCTLW;
#[doc = "`read()` method returns [ucax_mctlw::R](ucax_mctlw::R) reader structure"]
impl crate::Readable for UCAXMCTLW {}
#[doc = "`write(|w| ..)` method takes [ucax_mctlw::W](ucax_mctlw::W) writer structure"]
impl crate::Writable for UCAXMCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod ucax_mctlw;
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_statw](ucax_statw) module"]
pub type UCAXSTATW = crate::Reg<u16, _UCAXSTATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXSTATW;
#[doc = "`read()` method returns [ucax_statw::R](ucax_statw::R) reader structure"]
impl crate::Readable for UCAXSTATW {}
#[doc = "`write(|w| ..)` method takes [ucax_statw::W](ucax_statw::W) writer structure"]
impl crate::Writable for UCAXSTATW {}
#[doc = "eUSCI_Ax Status Register"]
pub mod ucax_statw;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_rxbuf](ucax_rxbuf) module"]
pub type UCAXRXBUF = crate::Reg<u16, _UCAXRXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXRXBUF;
#[doc = "`read()` method returns [ucax_rxbuf::R](ucax_rxbuf::R) reader structure"]
impl crate::Readable for UCAXRXBUF {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod ucax_rxbuf;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_txbuf](ucax_txbuf) module"]
pub type UCAXTXBUF = crate::Reg<u16, _UCAXTXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXTXBUF;
#[doc = "`read()` method returns [ucax_txbuf::R](ucax_txbuf::R) reader structure"]
impl crate::Readable for UCAXTXBUF {}
#[doc = "`write(|w| ..)` method takes [ucax_txbuf::W](ucax_txbuf::W) writer structure"]
impl crate::Writable for UCAXTXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod ucax_txbuf;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_abctl](ucax_abctl) module"]
pub type UCAXABCTL = crate::Reg<u16, _UCAXABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXABCTL;
#[doc = "`read()` method returns [ucax_abctl::R](ucax_abctl::R) reader structure"]
impl crate::Readable for UCAXABCTL {}
#[doc = "`write(|w| ..)` method takes [ucax_abctl::W](ucax_abctl::W) writer structure"]
impl crate::Writable for UCAXABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod ucax_abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_irctl](ucax_irctl) module"]
pub type UCAXIRCTL = crate::Reg<u16, _UCAXIRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXIRCTL;
#[doc = "`read()` method returns [ucax_irctl::R](ucax_irctl::R) reader structure"]
impl crate::Readable for UCAXIRCTL {}
#[doc = "`write(|w| ..)` method takes [ucax_irctl::W](ucax_irctl::W) writer structure"]
impl crate::Writable for UCAXIRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod ucax_irctl;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ie](ucax_ie) module"]
pub type UCAXIE = crate::Reg<u16, _UCAXIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXIE;
#[doc = "`read()` method returns [ucax_ie::R](ucax_ie::R) reader structure"]
impl crate::Readable for UCAXIE {}
#[doc = "`write(|w| ..)` method takes [ucax_ie::W](ucax_ie::W) writer structure"]
impl crate::Writable for UCAXIE {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod ucax_ie;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ifg](ucax_ifg) module"]
pub type UCAXIFG = crate::Reg<u16, _UCAXIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXIFG;
#[doc = "`read()` method returns [ucax_ifg::R](ucax_ifg::R) reader structure"]
impl crate::Readable for UCAXIFG {}
#[doc = "`write(|w| ..)` method takes [ucax_ifg::W](ucax_ifg::W) writer structure"]
impl crate::Writable for UCAXIFG {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod ucax_ifg;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_iv](ucax_iv) module"]
pub type UCAXIV = crate::Reg<u16, _UCAXIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCAXIV;
#[doc = "`read()` method returns [ucax_iv::R](ucax_iv::R) reader structure"]
impl crate::Readable for UCAXIV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod ucax_iv;
