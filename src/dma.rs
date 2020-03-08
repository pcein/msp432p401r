#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Configuration Status"]
    pub dma_device_cfg: DMA_DEVICE_CFG,
    #[doc = "0x04 - Software Channel Trigger Register"]
    pub dma_sw_chtrig: DMA_SW_CHTRIG,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Channel n Source Configuration Register"]
    pub dma_ch_srccfg: [DMA_CH_SRCCFG; 32],
    _reserved3: [u8; 112usize],
    #[doc = "0x100 - Interrupt 1 Source Channel Configuration"]
    pub dma_int1_srccfg: DMA_INT1_SRCCFG,
    #[doc = "0x104 - Interrupt 2 Source Channel Configuration Register"]
    pub dma_int2_srccfg: DMA_INT2_SRCCFG,
    #[doc = "0x108 - Interrupt 3 Source Channel Configuration Register"]
    pub dma_int3_srccfg: DMA_INT3_SRCCFG,
    _reserved6: [u8; 4usize],
    #[doc = "0x110 - Interrupt 0 Source Channel Flag Register"]
    pub dma_int0_srcflg: DMA_INT0_SRCFLG,
    #[doc = "0x114 - Interrupt 0 Source Channel Clear Flag Register"]
    pub dma_int0_clrflg: DMA_INT0_CLRFLG,
    _reserved8: [u8; 3816usize],
    #[doc = "0x1000 - Status Register"]
    pub dma_stat: DMA_STAT,
    #[doc = "0x1004 - Configuration Register"]
    pub dma_cfg: DMA_CFG,
    #[doc = "0x1008 - Channel Control Data Base Pointer Register"]
    pub dma_ctlbase: DMA_CTLBASE,
    #[doc = "0x100c - Channel Alternate Control Data Base Pointer Register"]
    pub dma_altbase: DMA_ALTBASE,
    #[doc = "0x1010 - Channel Wait on Request Status Register"]
    pub dma_waitstat: DMA_WAITSTAT,
    #[doc = "0x1014 - Channel Software Request Register"]
    pub dma_swreq: DMA_SWREQ,
    #[doc = "0x1018 - Channel Useburst Set Register"]
    pub dma_useburstset: DMA_USEBURSTSET,
    #[doc = "0x101c - Channel Useburst Clear Register"]
    pub dma_useburstclr: DMA_USEBURSTCLR,
    #[doc = "0x1020 - Channel Request Mask Set Register"]
    pub dma_reqmaskset: DMA_REQMASKSET,
    #[doc = "0x1024 - Channel Request Mask Clear Register"]
    pub dma_reqmaskclr: DMA_REQMASKCLR,
    #[doc = "0x1028 - Channel Enable Set Register"]
    pub dma_enaset: DMA_ENASET,
    #[doc = "0x102c - Channel Enable Clear Register"]
    pub dma_enaclr: DMA_ENACLR,
    #[doc = "0x1030 - Channel Primary-Alternate Set Register"]
    pub dma_altset: DMA_ALTSET,
    #[doc = "0x1034 - Channel Primary-Alternate Clear Register"]
    pub dma_altclr: DMA_ALTCLR,
    #[doc = "0x1038 - Channel Priority Set Register"]
    pub dma_prioset: DMA_PRIOSET,
    #[doc = "0x103c - Channel Priority Clear Register"]
    pub dma_prioclr: DMA_PRIOCLR,
    _reserved24: [u8; 12usize],
    #[doc = "0x104c - Bus Error Clear Register"]
    pub dma_errclr: DMA_ERRCLR,
}
#[doc = "Device Configuration Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_device_cfg](dma_device_cfg) module"]
pub type DMA_DEVICE_CFG = crate::Reg<u32, _DMA_DEVICE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DEVICE_CFG;
#[doc = "`read()` method returns [dma_device_cfg::R](dma_device_cfg::R) reader structure"]
impl crate::Readable for DMA_DEVICE_CFG {}
#[doc = "Device Configuration Status"]
pub mod dma_device_cfg;
#[doc = "Software Channel Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sw_chtrig](dma_sw_chtrig) module"]
pub type DMA_SW_CHTRIG = crate::Reg<u32, _DMA_SW_CHTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SW_CHTRIG;
#[doc = "`read()` method returns [dma_sw_chtrig::R](dma_sw_chtrig::R) reader structure"]
impl crate::Readable for DMA_SW_CHTRIG {}
#[doc = "`write(|w| ..)` method takes [dma_sw_chtrig::W](dma_sw_chtrig::W) writer structure"]
impl crate::Writable for DMA_SW_CHTRIG {}
#[doc = "Software Channel Trigger Register"]
pub mod dma_sw_chtrig;
#[doc = "Channel n Source Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ch_srccfg](dma_ch_srccfg) module"]
pub type DMA_CH_SRCCFG = crate::Reg<u32, _DMA_CH_SRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CH_SRCCFG;
#[doc = "`read()` method returns [dma_ch_srccfg::R](dma_ch_srccfg::R) reader structure"]
impl crate::Readable for DMA_CH_SRCCFG {}
#[doc = "`write(|w| ..)` method takes [dma_ch_srccfg::W](dma_ch_srccfg::W) writer structure"]
impl crate::Writable for DMA_CH_SRCCFG {}
#[doc = "Channel n Source Configuration Register"]
pub mod dma_ch_srccfg;
#[doc = "Interrupt 1 Source Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int1_srccfg](dma_int1_srccfg) module"]
pub type DMA_INT1_SRCCFG = crate::Reg<u32, _DMA_INT1_SRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT1_SRCCFG;
#[doc = "`read()` method returns [dma_int1_srccfg::R](dma_int1_srccfg::R) reader structure"]
impl crate::Readable for DMA_INT1_SRCCFG {}
#[doc = "`write(|w| ..)` method takes [dma_int1_srccfg::W](dma_int1_srccfg::W) writer structure"]
impl crate::Writable for DMA_INT1_SRCCFG {}
#[doc = "Interrupt 1 Source Channel Configuration"]
pub mod dma_int1_srccfg;
#[doc = "Interrupt 2 Source Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int2_srccfg](dma_int2_srccfg) module"]
pub type DMA_INT2_SRCCFG = crate::Reg<u32, _DMA_INT2_SRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT2_SRCCFG;
#[doc = "`read()` method returns [dma_int2_srccfg::R](dma_int2_srccfg::R) reader structure"]
impl crate::Readable for DMA_INT2_SRCCFG {}
#[doc = "`write(|w| ..)` method takes [dma_int2_srccfg::W](dma_int2_srccfg::W) writer structure"]
impl crate::Writable for DMA_INT2_SRCCFG {}
#[doc = "Interrupt 2 Source Channel Configuration Register"]
pub mod dma_int2_srccfg;
#[doc = "Interrupt 3 Source Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int3_srccfg](dma_int3_srccfg) module"]
pub type DMA_INT3_SRCCFG = crate::Reg<u32, _DMA_INT3_SRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT3_SRCCFG;
#[doc = "`read()` method returns [dma_int3_srccfg::R](dma_int3_srccfg::R) reader structure"]
impl crate::Readable for DMA_INT3_SRCCFG {}
#[doc = "`write(|w| ..)` method takes [dma_int3_srccfg::W](dma_int3_srccfg::W) writer structure"]
impl crate::Writable for DMA_INT3_SRCCFG {}
#[doc = "Interrupt 3 Source Channel Configuration Register"]
pub mod dma_int3_srccfg;
#[doc = "Interrupt 0 Source Channel Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int0_srcflg](dma_int0_srcflg) module"]
pub type DMA_INT0_SRCFLG = crate::Reg<u32, _DMA_INT0_SRCFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT0_SRCFLG;
#[doc = "`read()` method returns [dma_int0_srcflg::R](dma_int0_srcflg::R) reader structure"]
impl crate::Readable for DMA_INT0_SRCFLG {}
#[doc = "Interrupt 0 Source Channel Flag Register"]
pub mod dma_int0_srcflg;
#[doc = "Interrupt 0 Source Channel Clear Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int0_clrflg](dma_int0_clrflg) module"]
pub type DMA_INT0_CLRFLG = crate::Reg<u32, _DMA_INT0_CLRFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT0_CLRFLG;
#[doc = "`write(|w| ..)` method takes [dma_int0_clrflg::W](dma_int0_clrflg::W) writer structure"]
impl crate::Writable for DMA_INT0_CLRFLG {}
#[doc = "Interrupt 0 Source Channel Clear Flag Register"]
pub mod dma_int0_clrflg;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_stat](dma_stat) module"]
pub type DMA_STAT = crate::Reg<u32, _DMA_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_STAT;
#[doc = "`read()` method returns [dma_stat::R](dma_stat::R) reader structure"]
impl crate::Readable for DMA_STAT {}
#[doc = "Status Register"]
pub mod dma_stat;
#[doc = "Configuration Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cfg](dma_cfg) module"]
pub type DMA_CFG = crate::Reg<u32, _DMA_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CFG;
#[doc = "`write(|w| ..)` method takes [dma_cfg::W](dma_cfg::W) writer structure"]
impl crate::Writable for DMA_CFG {}
#[doc = "Configuration Register"]
pub mod dma_cfg;
#[doc = "Channel Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctlbase](dma_ctlbase) module"]
pub type DMA_CTLBASE = crate::Reg<u32, _DMA_CTLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CTLBASE;
#[doc = "`read()` method returns [dma_ctlbase::R](dma_ctlbase::R) reader structure"]
impl crate::Readable for DMA_CTLBASE {}
#[doc = "`write(|w| ..)` method takes [dma_ctlbase::W](dma_ctlbase::W) writer structure"]
impl crate::Writable for DMA_CTLBASE {}
#[doc = "Channel Control Data Base Pointer Register"]
pub mod dma_ctlbase;
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altbase](dma_altbase) module"]
pub type DMA_ALTBASE = crate::Reg<u32, _DMA_ALTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ALTBASE;
#[doc = "`read()` method returns [dma_altbase::R](dma_altbase::R) reader structure"]
impl crate::Readable for DMA_ALTBASE {}
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod dma_altbase;
#[doc = "Channel Wait on Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_waitstat](dma_waitstat) module"]
pub type DMA_WAITSTAT = crate::Reg<u32, _DMA_WAITSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_WAITSTAT;
#[doc = "`read()` method returns [dma_waitstat::R](dma_waitstat::R) reader structure"]
impl crate::Readable for DMA_WAITSTAT {}
#[doc = "Channel Wait on Request Status Register"]
pub mod dma_waitstat;
#[doc = "Channel Software Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_swreq](dma_swreq) module"]
pub type DMA_SWREQ = crate::Reg<u32, _DMA_SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SWREQ;
#[doc = "`write(|w| ..)` method takes [dma_swreq::W](dma_swreq::W) writer structure"]
impl crate::Writable for DMA_SWREQ {}
#[doc = "Channel Software Request Register"]
pub mod dma_swreq;
#[doc = "Channel Useburst Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_useburstset](dma_useburstset) module"]
pub type DMA_USEBURSTSET = crate::Reg<u32, _DMA_USEBURSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_USEBURSTSET;
#[doc = "`read()` method returns [dma_useburstset::R](dma_useburstset::R) reader structure"]
impl crate::Readable for DMA_USEBURSTSET {}
#[doc = "`write(|w| ..)` method takes [dma_useburstset::W](dma_useburstset::W) writer structure"]
impl crate::Writable for DMA_USEBURSTSET {}
#[doc = "Channel Useburst Set Register"]
pub mod dma_useburstset;
#[doc = "Channel Useburst Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_useburstclr](dma_useburstclr) module"]
pub type DMA_USEBURSTCLR = crate::Reg<u32, _DMA_USEBURSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_USEBURSTCLR;
#[doc = "`write(|w| ..)` method takes [dma_useburstclr::W](dma_useburstclr::W) writer structure"]
impl crate::Writable for DMA_USEBURSTCLR {}
#[doc = "Channel Useburst Clear Register"]
pub mod dma_useburstclr;
#[doc = "Channel Request Mask Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_reqmaskset](dma_reqmaskset) module"]
pub type DMA_REQMASKSET = crate::Reg<u32, _DMA_REQMASKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_REQMASKSET;
#[doc = "`read()` method returns [dma_reqmaskset::R](dma_reqmaskset::R) reader structure"]
impl crate::Readable for DMA_REQMASKSET {}
#[doc = "`write(|w| ..)` method takes [dma_reqmaskset::W](dma_reqmaskset::W) writer structure"]
impl crate::Writable for DMA_REQMASKSET {}
#[doc = "Channel Request Mask Set Register"]
pub mod dma_reqmaskset;
#[doc = "Channel Request Mask Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_reqmaskclr](dma_reqmaskclr) module"]
pub type DMA_REQMASKCLR = crate::Reg<u32, _DMA_REQMASKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_REQMASKCLR;
#[doc = "`write(|w| ..)` method takes [dma_reqmaskclr::W](dma_reqmaskclr::W) writer structure"]
impl crate::Writable for DMA_REQMASKCLR {}
#[doc = "Channel Request Mask Clear Register"]
pub mod dma_reqmaskclr;
#[doc = "Channel Enable Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enaset](dma_enaset) module"]
pub type DMA_ENASET = crate::Reg<u32, _DMA_ENASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ENASET;
#[doc = "`read()` method returns [dma_enaset::R](dma_enaset::R) reader structure"]
impl crate::Readable for DMA_ENASET {}
#[doc = "`write(|w| ..)` method takes [dma_enaset::W](dma_enaset::W) writer structure"]
impl crate::Writable for DMA_ENASET {}
#[doc = "Channel Enable Set Register"]
pub mod dma_enaset;
#[doc = "Channel Enable Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enaclr](dma_enaclr) module"]
pub type DMA_ENACLR = crate::Reg<u32, _DMA_ENACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ENACLR;
#[doc = "`write(|w| ..)` method takes [dma_enaclr::W](dma_enaclr::W) writer structure"]
impl crate::Writable for DMA_ENACLR {}
#[doc = "Channel Enable Clear Register"]
pub mod dma_enaclr;
#[doc = "Channel Primary-Alternate Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altset](dma_altset) module"]
pub type DMA_ALTSET = crate::Reg<u32, _DMA_ALTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ALTSET;
#[doc = "`read()` method returns [dma_altset::R](dma_altset::R) reader structure"]
impl crate::Readable for DMA_ALTSET {}
#[doc = "`write(|w| ..)` method takes [dma_altset::W](dma_altset::W) writer structure"]
impl crate::Writable for DMA_ALTSET {}
#[doc = "Channel Primary-Alternate Set Register"]
pub mod dma_altset;
#[doc = "Channel Primary-Alternate Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altclr](dma_altclr) module"]
pub type DMA_ALTCLR = crate::Reg<u32, _DMA_ALTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ALTCLR;
#[doc = "`write(|w| ..)` method takes [dma_altclr::W](dma_altclr::W) writer structure"]
impl crate::Writable for DMA_ALTCLR {}
#[doc = "Channel Primary-Alternate Clear Register"]
pub mod dma_altclr;
#[doc = "Channel Priority Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_prioset](dma_prioset) module"]
pub type DMA_PRIOSET = crate::Reg<u32, _DMA_PRIOSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_PRIOSET;
#[doc = "`read()` method returns [dma_prioset::R](dma_prioset::R) reader structure"]
impl crate::Readable for DMA_PRIOSET {}
#[doc = "`write(|w| ..)` method takes [dma_prioset::W](dma_prioset::W) writer structure"]
impl crate::Writable for DMA_PRIOSET {}
#[doc = "Channel Priority Set Register"]
pub mod dma_prioset;
#[doc = "Channel Priority Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_prioclr](dma_prioclr) module"]
pub type DMA_PRIOCLR = crate::Reg<u32, _DMA_PRIOCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_PRIOCLR;
#[doc = "`write(|w| ..)` method takes [dma_prioclr::W](dma_prioclr::W) writer structure"]
impl crate::Writable for DMA_PRIOCLR {}
#[doc = "Channel Priority Clear Register"]
pub mod dma_prioclr;
#[doc = "Bus Error Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_errclr](dma_errclr) module"]
pub type DMA_ERRCLR = crate::Reg<u32, _DMA_ERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ERRCLR;
#[doc = "`read()` method returns [dma_errclr::R](dma_errclr::R) reader structure"]
impl crate::Readable for DMA_ERRCLR {}
#[doc = "`write(|w| ..)` method takes [dma_errclr::W](dma_errclr::W) writer structure"]
impl crate::Writable for DMA_ERRCLR {}
#[doc = "Bus Error Clear Register"]
pub mod dma_errclr;
