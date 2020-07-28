#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    pub ucbx_ctlw0: UCBXCTLW0,
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucbx_ctlw1: UCBXCTLW1,
    _reserved2: [u8; 2usize],
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    pub ucbx_brw: UCBXBRW,
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    pub ucbx_statw: UCBXSTATW,
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucbx_tbcnt: UCBXTBCNT,
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    pub ucbx_rxbuf: UCBXRXBUF,
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    pub ucbx_txbuf: UCBXTXBUF,
    _reserved7: [u8; 4usize],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucbx_i2coa0: UCBXI2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucbx_i2coa1: UCBXI2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucbx_i2coa2: UCBXI2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucbx_i2coa3: UCBXI2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucbx_addrx: UCBXADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucbx_addmask: UCBXADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucbx_i2csa: UCBXI2CSA,
    _reserved14: [u8; 8usize],
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    pub ucbx_ie: UCBXIE,
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    pub ucbx_ifg: UCBXIFG,
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    pub ucbx_iv: UCBXIV,
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ctlw0](ucbx_ctlw0) module"]
pub type UCBXCTLW0 = crate::Reg<u16, _UCBXCTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXCTLW0;
#[doc = "`read()` method returns [ucbx_ctlw0::R](ucbx_ctlw0::R) reader structure"]
impl crate::Readable for UCBXCTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucbx_ctlw0::W](ucbx_ctlw0::W) writer structure"]
impl crate::Writable for UCBXCTLW0 {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucbx_ctlw0;
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ctlw1](ucbx_ctlw1) module"]
pub type UCBXCTLW1 = crate::Reg<u16, _UCBXCTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXCTLW1;
#[doc = "`read()` method returns [ucbx_ctlw1::R](ucbx_ctlw1::R) reader structure"]
impl crate::Readable for UCBXCTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucbx_ctlw1::W](ucbx_ctlw1::W) writer structure"]
impl crate::Writable for UCBXCTLW1 {}
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucbx_ctlw1;
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_brw](ucbx_brw) module"]
pub type UCBXBRW = crate::Reg<u16, _UCBXBRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXBRW;
#[doc = "`read()` method returns [ucbx_brw::R](ucbx_brw::R) reader structure"]
impl crate::Readable for UCBXBRW {}
#[doc = "`write(|w| ..)` method takes [ucbx_brw::W](ucbx_brw::W) writer structure"]
impl crate::Writable for UCBXBRW {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucbx_brw;
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_statw](ucbx_statw) module"]
pub type UCBXSTATW = crate::Reg<u16, _UCBXSTATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXSTATW;
#[doc = "`read()` method returns [ucbx_statw::R](ucbx_statw::R) reader structure"]
impl crate::Readable for UCBXSTATW {}
#[doc = "`write(|w| ..)` method takes [ucbx_statw::W](ucbx_statw::W) writer structure"]
impl crate::Writable for UCBXSTATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucbx_statw;
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_tbcnt](ucbx_tbcnt) module"]
pub type UCBXTBCNT = crate::Reg<u16, _UCBXTBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXTBCNT;
#[doc = "`read()` method returns [ucbx_tbcnt::R](ucbx_tbcnt::R) reader structure"]
impl crate::Readable for UCBXTBCNT {}
#[doc = "`write(|w| ..)` method takes [ucbx_tbcnt::W](ucbx_tbcnt::W) writer structure"]
impl crate::Writable for UCBXTBCNT {}
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucbx_tbcnt;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_rxbuf](ucbx_rxbuf) module"]
pub type UCBXRXBUF = crate::Reg<u16, _UCBXRXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXRXBUF;
#[doc = "`read()` method returns [ucbx_rxbuf::R](ucbx_rxbuf::R) reader structure"]
impl crate::Readable for UCBXRXBUF {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucbx_rxbuf;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_txbuf](ucbx_txbuf) module"]
pub type UCBXTXBUF = crate::Reg<u16, _UCBXTXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXTXBUF;
#[doc = "`read()` method returns [ucbx_txbuf::R](ucbx_txbuf::R) reader structure"]
impl crate::Readable for UCBXTXBUF {}
#[doc = "`write(|w| ..)` method takes [ucbx_txbuf::W](ucbx_txbuf::W) writer structure"]
impl crate::Writable for UCBXTXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucbx_txbuf;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa0](ucbx_i2coa0) module"]
pub type UCBXI2COA0 = crate::Reg<u16, _UCBXI2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXI2COA0;
#[doc = "`read()` method returns [ucbx_i2coa0::R](ucbx_i2coa0::R) reader structure"]
impl crate::Readable for UCBXI2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa0::W](ucbx_i2coa0::W) writer structure"]
impl crate::Writable for UCBXI2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucbx_i2coa0;
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa1](ucbx_i2coa1) module"]
pub type UCBXI2COA1 = crate::Reg<u16, _UCBXI2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXI2COA1;
#[doc = "`read()` method returns [ucbx_i2coa1::R](ucbx_i2coa1::R) reader structure"]
impl crate::Readable for UCBXI2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa1::W](ucbx_i2coa1::W) writer structure"]
impl crate::Writable for UCBXI2COA1 {}
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucbx_i2coa1;
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa2](ucbx_i2coa2) module"]
pub type UCBXI2COA2 = crate::Reg<u16, _UCBXI2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXI2COA2;
#[doc = "`read()` method returns [ucbx_i2coa2::R](ucbx_i2coa2::R) reader structure"]
impl crate::Readable for UCBXI2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa2::W](ucbx_i2coa2::W) writer structure"]
impl crate::Writable for UCBXI2COA2 {}
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucbx_i2coa2;
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa3](ucbx_i2coa3) module"]
pub type UCBXI2COA3 = crate::Reg<u16, _UCBXI2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXI2COA3;
#[doc = "`read()` method returns [ucbx_i2coa3::R](ucbx_i2coa3::R) reader structure"]
impl crate::Readable for UCBXI2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa3::W](ucbx_i2coa3::W) writer structure"]
impl crate::Writable for UCBXI2COA3 {}
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucbx_i2coa3;
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_addrx](ucbx_addrx) module"]
pub type UCBXADDRX = crate::Reg<u16, _UCBXADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXADDRX;
#[doc = "`read()` method returns [ucbx_addrx::R](ucbx_addrx::R) reader structure"]
impl crate::Readable for UCBXADDRX {}
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucbx_addrx;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_addmask](ucbx_addmask) module"]
pub type UCBXADDMASK = crate::Reg<u16, _UCBXADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXADDMASK;
#[doc = "`read()` method returns [ucbx_addmask::R](ucbx_addmask::R) reader structure"]
impl crate::Readable for UCBXADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucbx_addmask::W](ucbx_addmask::W) writer structure"]
impl crate::Writable for UCBXADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucbx_addmask;
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2csa](ucbx_i2csa) module"]
pub type UCBXI2CSA = crate::Reg<u16, _UCBXI2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXI2CSA;
#[doc = "`read()` method returns [ucbx_i2csa::R](ucbx_i2csa::R) reader structure"]
impl crate::Readable for UCBXI2CSA {}
#[doc = "`write(|w| ..)` method takes [ucbx_i2csa::W](ucbx_i2csa::W) writer structure"]
impl crate::Writable for UCBXI2CSA {}
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucbx_i2csa;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ie](ucbx_ie) module"]
pub type UCBXIE = crate::Reg<u16, _UCBXIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXIE;
#[doc = "`read()` method returns [ucbx_ie::R](ucbx_ie::R) reader structure"]
impl crate::Readable for UCBXIE {}
#[doc = "`write(|w| ..)` method takes [ucbx_ie::W](ucbx_ie::W) writer structure"]
impl crate::Writable for UCBXIE {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucbx_ie;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ifg](ucbx_ifg) module"]
pub type UCBXIFG = crate::Reg<u16, _UCBXIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXIFG;
#[doc = "`read()` method returns [ucbx_ifg::R](ucbx_ifg::R) reader structure"]
impl crate::Readable for UCBXIFG {}
#[doc = "`write(|w| ..)` method takes [ucbx_ifg::W](ucbx_ifg::W) writer structure"]
impl crate::Writable for UCBXIFG {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucbx_ifg;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_iv](ucbx_iv) module"]
pub type UCBXIV = crate::Reg<u16, _UCBXIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCBXIV;
#[doc = "`read()` method returns [ucbx_iv::R](ucbx_iv::R) reader structure"]
impl crate::Readable for UCBXIV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucbx_iv;
