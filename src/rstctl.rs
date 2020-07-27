#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Request Register"]
    pub rstctl_reset_req: RSTCTL_RESET_REQ,
    #[doc = "0x04 - Hard Reset Status Register"]
    pub rstctl_hardreset_stat: RSTCTL_HARDRESET_STAT,
    #[doc = "0x08 - Hard Reset Status Clear Register"]
    pub rstctl_hardreset_clr: RSTCTL_HARDRESET_CLR,
    #[doc = "0x0c - Hard Reset Status Set Register"]
    pub rstctl_hardreset_set: RSTCTL_HARDRESET_SET,
    #[doc = "0x10 - Soft Reset Status Register"]
    pub rstctl_softreset_stat: RSTCTL_SOFTRESET_STAT,
    #[doc = "0x14 - Soft Reset Status Clear Register"]
    pub rstctl_softreset_clr: RSTCTL_SOFTRESET_CLR,
    #[doc = "0x18 - Soft Reset Status Set Register"]
    pub rstctl_softreset_set: RSTCTL_SOFTRESET_SET,
    _reserved7: [u8; 228usize],
    #[doc = "0x100 - PSS Reset Status Register"]
    pub rstctl_pssreset_stat: RSTCTL_PSSRESET_STAT,
    #[doc = "0x104 - PSS Reset Status Clear Register"]
    pub rstctl_pssreset_clr: RSTCTL_PSSRESET_CLR,
    #[doc = "0x108 - PCM Reset Status Register"]
    pub rstctl_pcmreset_stat: RSTCTL_PCMRESET_STAT,
    #[doc = "0x10c - PCM Reset Status Clear Register"]
    pub rstctl_pcmreset_clr: RSTCTL_PCMRESET_CLR,
    #[doc = "0x110 - Pin Reset Status Register"]
    pub rstctl_pinreset_stat: RSTCTL_PINRESET_STAT,
    #[doc = "0x114 - Pin Reset Status Clear Register"]
    pub rstctl_pinreset_clr: RSTCTL_PINRESET_CLR,
    #[doc = "0x118 - Reboot Reset Status Register"]
    pub rstctl_rebootreset_stat: RSTCTL_REBOOTRESET_STAT,
    #[doc = "0x11c - Reboot Reset Status Clear Register"]
    pub rstctl_rebootreset_clr: RSTCTL_REBOOTRESET_CLR,
    #[doc = "0x120 - CS Reset Status Register"]
    pub rstctl_csreset_stat: RSTCTL_CSRESET_STAT,
    #[doc = "0x124 - CS Reset Status Clear Register"]
    pub rstctl_csreset_clr: RSTCTL_CSRESET_CLR,
}
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_reset_req](rstctl_reset_req) module"]
pub type RSTCTL_RESET_REQ = crate::Reg<u32, _RSTCTL_RESET_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_RESET_REQ;
#[doc = "`read()` method returns [rstctl_reset_req::R](rstctl_reset_req::R) reader structure"]
impl crate::Readable for RSTCTL_RESET_REQ {}
#[doc = "`write(|w| ..)` method takes [rstctl_reset_req::W](rstctl_reset_req::W) writer structure"]
impl crate::Writable for RSTCTL_RESET_REQ {}
#[doc = "Reset Request Register"]
pub mod rstctl_reset_req;
#[doc = "Hard Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_hardreset_stat](rstctl_hardreset_stat) module"]
pub type RSTCTL_HARDRESET_STAT = crate::Reg<u32, _RSTCTL_HARDRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_HARDRESET_STAT;
#[doc = "`read()` method returns [rstctl_hardreset_stat::R](rstctl_hardreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_HARDRESET_STAT {}
#[doc = "Hard Reset Status Register"]
pub mod rstctl_hardreset_stat;
#[doc = "Hard Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_hardreset_clr](rstctl_hardreset_clr) module"]
pub type RSTCTL_HARDRESET_CLR = crate::Reg<u32, _RSTCTL_HARDRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_HARDRESET_CLR;
#[doc = "`read()` method returns [rstctl_hardreset_clr::R](rstctl_hardreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_HARDRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_hardreset_clr::W](rstctl_hardreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_HARDRESET_CLR {}
#[doc = "Hard Reset Status Clear Register"]
pub mod rstctl_hardreset_clr;
#[doc = "Hard Reset Status Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_hardreset_set](rstctl_hardreset_set) module"]
pub type RSTCTL_HARDRESET_SET = crate::Reg<u32, _RSTCTL_HARDRESET_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_HARDRESET_SET;
#[doc = "`read()` method returns [rstctl_hardreset_set::R](rstctl_hardreset_set::R) reader structure"]
impl crate::Readable for RSTCTL_HARDRESET_SET {}
#[doc = "`write(|w| ..)` method takes [rstctl_hardreset_set::W](rstctl_hardreset_set::W) writer structure"]
impl crate::Writable for RSTCTL_HARDRESET_SET {}
#[doc = "Hard Reset Status Set Register"]
pub mod rstctl_hardreset_set;
#[doc = "Soft Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_softreset_stat](rstctl_softreset_stat) module"]
pub type RSTCTL_SOFTRESET_STAT = crate::Reg<u32, _RSTCTL_SOFTRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_SOFTRESET_STAT;
#[doc = "`read()` method returns [rstctl_softreset_stat::R](rstctl_softreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_SOFTRESET_STAT {}
#[doc = "Soft Reset Status Register"]
pub mod rstctl_softreset_stat;
#[doc = "Soft Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_softreset_clr](rstctl_softreset_clr) module"]
pub type RSTCTL_SOFTRESET_CLR = crate::Reg<u32, _RSTCTL_SOFTRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_SOFTRESET_CLR;
#[doc = "`read()` method returns [rstctl_softreset_clr::R](rstctl_softreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_SOFTRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_softreset_clr::W](rstctl_softreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_SOFTRESET_CLR {}
#[doc = "Soft Reset Status Clear Register"]
pub mod rstctl_softreset_clr;
#[doc = "Soft Reset Status Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_softreset_set](rstctl_softreset_set) module"]
pub type RSTCTL_SOFTRESET_SET = crate::Reg<u32, _RSTCTL_SOFTRESET_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_SOFTRESET_SET;
#[doc = "`read()` method returns [rstctl_softreset_set::R](rstctl_softreset_set::R) reader structure"]
impl crate::Readable for RSTCTL_SOFTRESET_SET {}
#[doc = "`write(|w| ..)` method takes [rstctl_softreset_set::W](rstctl_softreset_set::W) writer structure"]
impl crate::Writable for RSTCTL_SOFTRESET_SET {}
#[doc = "Soft Reset Status Set Register"]
pub mod rstctl_softreset_set;
#[doc = "PSS Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pssreset_stat](rstctl_pssreset_stat) module"]
pub type RSTCTL_PSSRESET_STAT = crate::Reg<u32, _RSTCTL_PSSRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PSSRESET_STAT;
#[doc = "`read()` method returns [rstctl_pssreset_stat::R](rstctl_pssreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_PSSRESET_STAT {}
#[doc = "PSS Reset Status Register"]
pub mod rstctl_pssreset_stat;
#[doc = "PSS Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pssreset_clr](rstctl_pssreset_clr) module"]
pub type RSTCTL_PSSRESET_CLR = crate::Reg<u32, _RSTCTL_PSSRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PSSRESET_CLR;
#[doc = "`read()` method returns [rstctl_pssreset_clr::R](rstctl_pssreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_PSSRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_pssreset_clr::W](rstctl_pssreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_PSSRESET_CLR {}
#[doc = "PSS Reset Status Clear Register"]
pub mod rstctl_pssreset_clr;
#[doc = "PCM Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pcmreset_stat](rstctl_pcmreset_stat) module"]
pub type RSTCTL_PCMRESET_STAT = crate::Reg<u32, _RSTCTL_PCMRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PCMRESET_STAT;
#[doc = "`read()` method returns [rstctl_pcmreset_stat::R](rstctl_pcmreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_PCMRESET_STAT {}
#[doc = "PCM Reset Status Register"]
pub mod rstctl_pcmreset_stat;
#[doc = "PCM Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pcmreset_clr](rstctl_pcmreset_clr) module"]
pub type RSTCTL_PCMRESET_CLR = crate::Reg<u32, _RSTCTL_PCMRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PCMRESET_CLR;
#[doc = "`read()` method returns [rstctl_pcmreset_clr::R](rstctl_pcmreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_PCMRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_pcmreset_clr::W](rstctl_pcmreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_PCMRESET_CLR {}
#[doc = "PCM Reset Status Clear Register"]
pub mod rstctl_pcmreset_clr;
#[doc = "Pin Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pinreset_stat](rstctl_pinreset_stat) module"]
pub type RSTCTL_PINRESET_STAT = crate::Reg<u32, _RSTCTL_PINRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PINRESET_STAT;
#[doc = "`read()` method returns [rstctl_pinreset_stat::R](rstctl_pinreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_PINRESET_STAT {}
#[doc = "Pin Reset Status Register"]
pub mod rstctl_pinreset_stat;
#[doc = "Pin Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pinreset_clr](rstctl_pinreset_clr) module"]
pub type RSTCTL_PINRESET_CLR = crate::Reg<u32, _RSTCTL_PINRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_PINRESET_CLR;
#[doc = "`read()` method returns [rstctl_pinreset_clr::R](rstctl_pinreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_PINRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_pinreset_clr::W](rstctl_pinreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_PINRESET_CLR {}
#[doc = "Pin Reset Status Clear Register"]
pub mod rstctl_pinreset_clr;
#[doc = "Reboot Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_rebootreset_stat](rstctl_rebootreset_stat) module"]
pub type RSTCTL_REBOOTRESET_STAT = crate::Reg<u32, _RSTCTL_REBOOTRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_REBOOTRESET_STAT;
#[doc = "`read()` method returns [rstctl_rebootreset_stat::R](rstctl_rebootreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_REBOOTRESET_STAT {}
#[doc = "Reboot Reset Status Register"]
pub mod rstctl_rebootreset_stat;
#[doc = "Reboot Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_rebootreset_clr](rstctl_rebootreset_clr) module"]
pub type RSTCTL_REBOOTRESET_CLR = crate::Reg<u32, _RSTCTL_REBOOTRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_REBOOTRESET_CLR;
#[doc = "`read()` method returns [rstctl_rebootreset_clr::R](rstctl_rebootreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_REBOOTRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_rebootreset_clr::W](rstctl_rebootreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_REBOOTRESET_CLR {}
#[doc = "Reboot Reset Status Clear Register"]
pub mod rstctl_rebootreset_clr;
#[doc = "CS Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_csreset_stat](rstctl_csreset_stat) module"]
pub type RSTCTL_CSRESET_STAT = crate::Reg<u32, _RSTCTL_CSRESET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_CSRESET_STAT;
#[doc = "`read()` method returns [rstctl_csreset_stat::R](rstctl_csreset_stat::R) reader structure"]
impl crate::Readable for RSTCTL_CSRESET_STAT {}
#[doc = "CS Reset Status Register"]
pub mod rstctl_csreset_stat;
#[doc = "CS Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_csreset_clr](rstctl_csreset_clr) module"]
pub type RSTCTL_CSRESET_CLR = crate::Reg<u32, _RSTCTL_CSRESET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCTL_CSRESET_CLR;
#[doc = "`read()` method returns [rstctl_csreset_clr::R](rstctl_csreset_clr::R) reader structure"]
impl crate::Readable for RSTCTL_CSRESET_CLR {}
#[doc = "`write(|w| ..)` method takes [rstctl_csreset_clr::W](rstctl_csreset_clr::W) writer structure"]
impl crate::Writable for RSTCTL_CSRESET_CLR {}
#[doc = "CS Reset Status Clear Register"]
pub mod rstctl_csreset_clr;
