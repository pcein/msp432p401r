#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reboot Control Register"]
    pub sys_reboot_ctl: SYS_REBOOT_CTL,
    #[doc = "0x04 - NMI Control and Status Register"]
    pub sys_nmi_ctlstat: SYS_NMI_CTLSTAT,
    #[doc = "0x08 - Watchdog Reset Control Register"]
    pub sys_wdtreset_ctl: SYS_WDTRESET_CTL,
    #[doc = "0x0c - Peripheral Halt Control Register"]
    pub sys_perihalt_ctl: SYS_PERIHALT_CTL,
    #[doc = "0x10 - SRAM Size Register"]
    pub sys_sram_size: SYS_SRAM_SIZE,
    #[doc = "0x14 - SRAM Bank Enable Register"]
    pub sys_sram_banken: SYS_SRAM_BANKEN,
    #[doc = "0x18 - SRAM Bank Retention Control Register"]
    pub sys_sram_bankret: SYS_SRAM_BANKRET,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Flash Size Register"]
    pub sys_flash_size: SYS_FLASH_SIZE,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - Digital I/O Glitch Filter Control Register"]
    pub sys_dio_gltflt_ctl: SYS_DIO_GLTFLT_CTL,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - IP Protected Secure Zone Data Access Unlock Register"]
    pub sys_secdata_unlock: SYS_SECDATA_UNLOCK,
    _reserved10: [u8; 4028usize],
    #[doc = "0x1000 - Master Unlock Register"]
    pub sys_master_unlock: SYS_MASTER_UNLOCK,
    #[doc = "0x1004 - Boot Override Request Register"]
    pub sys_bootover_req: [SYS_BOOTOVER_REQ; 2],
    #[doc = "0x100c - Boot Override Acknowledge Register"]
    pub sys_bootover_ack: SYS_BOOTOVER_ACK,
    #[doc = "0x1010 - Reset Request Register"]
    pub sys_reset_req: SYS_RESET_REQ,
    #[doc = "0x1014 - Reset Status and Override Register"]
    pub sys_reset_statover: SYS_RESET_STATOVER,
    _reserved15: [u8; 8usize],
    #[doc = "0x1020 - System Status Register"]
    pub sys_system_stat: SYS_SYSTEM_STAT,
}
#[doc = "Reboot Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reboot_ctl](sys_reboot_ctl) module"]
pub type SYS_REBOOT_CTL = crate::Reg<u32, _SYS_REBOOT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_REBOOT_CTL;
#[doc = "`read()` method returns [sys_reboot_ctl::R](sys_reboot_ctl::R) reader structure"]
impl crate::Readable for SYS_REBOOT_CTL {}
#[doc = "`write(|w| ..)` method takes [sys_reboot_ctl::W](sys_reboot_ctl::W) writer structure"]
impl crate::Writable for SYS_REBOOT_CTL {}
#[doc = "Reboot Control Register"]
pub mod sys_reboot_ctl;
#[doc = "NMI Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_nmi_ctlstat](sys_nmi_ctlstat) module"]
pub type SYS_NMI_CTLSTAT = crate::Reg<u32, _SYS_NMI_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_NMI_CTLSTAT;
#[doc = "`read()` method returns [sys_nmi_ctlstat::R](sys_nmi_ctlstat::R) reader structure"]
impl crate::Readable for SYS_NMI_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [sys_nmi_ctlstat::W](sys_nmi_ctlstat::W) writer structure"]
impl crate::Writable for SYS_NMI_CTLSTAT {}
#[doc = "NMI Control and Status Register"]
pub mod sys_nmi_ctlstat;
#[doc = "Watchdog Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_wdtreset_ctl](sys_wdtreset_ctl) module"]
pub type SYS_WDTRESET_CTL = crate::Reg<u32, _SYS_WDTRESET_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_WDTRESET_CTL;
#[doc = "`read()` method returns [sys_wdtreset_ctl::R](sys_wdtreset_ctl::R) reader structure"]
impl crate::Readable for SYS_WDTRESET_CTL {}
#[doc = "`write(|w| ..)` method takes [sys_wdtreset_ctl::W](sys_wdtreset_ctl::W) writer structure"]
impl crate::Writable for SYS_WDTRESET_CTL {}
#[doc = "Watchdog Reset Control Register"]
pub mod sys_wdtreset_ctl;
#[doc = "Peripheral Halt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_perihalt_ctl](sys_perihalt_ctl) module"]
pub type SYS_PERIHALT_CTL = crate::Reg<u32, _SYS_PERIHALT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_PERIHALT_CTL;
#[doc = "`read()` method returns [sys_perihalt_ctl::R](sys_perihalt_ctl::R) reader structure"]
impl crate::Readable for SYS_PERIHALT_CTL {}
#[doc = "`write(|w| ..)` method takes [sys_perihalt_ctl::W](sys_perihalt_ctl::W) writer structure"]
impl crate::Writable for SYS_PERIHALT_CTL {}
#[doc = "Peripheral Halt Control Register"]
pub mod sys_perihalt_ctl;
#[doc = "SRAM Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_size](sys_sram_size) module"]
pub type SYS_SRAM_SIZE = crate::Reg<u32, _SYS_SRAM_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_SRAM_SIZE;
#[doc = "`read()` method returns [sys_sram_size::R](sys_sram_size::R) reader structure"]
impl crate::Readable for SYS_SRAM_SIZE {}
#[doc = "SRAM Size Register"]
pub mod sys_sram_size;
#[doc = "SRAM Bank Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken](sys_sram_banken) module"]
pub type SYS_SRAM_BANKEN = crate::Reg<u32, _SYS_SRAM_BANKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_SRAM_BANKEN;
#[doc = "`read()` method returns [sys_sram_banken::R](sys_sram_banken::R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN {}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken::W](sys_sram_banken::W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN {}
#[doc = "SRAM Bank Enable Register"]
pub mod sys_sram_banken;
#[doc = "SRAM Bank Retention Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_bankret](sys_sram_bankret) module"]
pub type SYS_SRAM_BANKRET = crate::Reg<u32, _SYS_SRAM_BANKRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_SRAM_BANKRET;
#[doc = "`read()` method returns [sys_sram_bankret::R](sys_sram_bankret::R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKRET {}
#[doc = "`write(|w| ..)` method takes [sys_sram_bankret::W](sys_sram_bankret::W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKRET {}
#[doc = "SRAM Bank Retention Control Register"]
pub mod sys_sram_bankret;
#[doc = "Flash Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_flash_size](sys_flash_size) module"]
pub type SYS_FLASH_SIZE = crate::Reg<u32, _SYS_FLASH_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_FLASH_SIZE;
#[doc = "`read()` method returns [sys_flash_size::R](sys_flash_size::R) reader structure"]
impl crate::Readable for SYS_FLASH_SIZE {}
#[doc = "Flash Size Register"]
pub mod sys_flash_size;
#[doc = "Digital I/O Glitch Filter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_dio_gltflt_ctl](sys_dio_gltflt_ctl) module"]
pub type SYS_DIO_GLTFLT_CTL = crate::Reg<u32, _SYS_DIO_GLTFLT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_DIO_GLTFLT_CTL;
#[doc = "`read()` method returns [sys_dio_gltflt_ctl::R](sys_dio_gltflt_ctl::R) reader structure"]
impl crate::Readable for SYS_DIO_GLTFLT_CTL {}
#[doc = "`write(|w| ..)` method takes [sys_dio_gltflt_ctl::W](sys_dio_gltflt_ctl::W) writer structure"]
impl crate::Writable for SYS_DIO_GLTFLT_CTL {}
#[doc = "Digital I/O Glitch Filter Control Register"]
pub mod sys_dio_gltflt_ctl;
#[doc = "IP Protected Secure Zone Data Access Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_secdata_unlock](sys_secdata_unlock) module"]
pub type SYS_SECDATA_UNLOCK = crate::Reg<u32, _SYS_SECDATA_UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_SECDATA_UNLOCK;
#[doc = "`read()` method returns [sys_secdata_unlock::R](sys_secdata_unlock::R) reader structure"]
impl crate::Readable for SYS_SECDATA_UNLOCK {}
#[doc = "`write(|w| ..)` method takes [sys_secdata_unlock::W](sys_secdata_unlock::W) writer structure"]
impl crate::Writable for SYS_SECDATA_UNLOCK {}
#[doc = "IP Protected Secure Zone Data Access Unlock Register"]
pub mod sys_secdata_unlock;
#[doc = "Master Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_master_unlock](sys_master_unlock) module"]
pub type SYS_MASTER_UNLOCK = crate::Reg<u32, _SYS_MASTER_UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_MASTER_UNLOCK;
#[doc = "`read()` method returns [sys_master_unlock::R](sys_master_unlock::R) reader structure"]
impl crate::Readable for SYS_MASTER_UNLOCK {}
#[doc = "`write(|w| ..)` method takes [sys_master_unlock::W](sys_master_unlock::W) writer structure"]
impl crate::Writable for SYS_MASTER_UNLOCK {}
#[doc = "Master Unlock Register"]
pub mod sys_master_unlock;
#[doc = "Boot Override Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_bootover_req](sys_bootover_req) module"]
pub type SYS_BOOTOVER_REQ = crate::Reg<u32, _SYS_BOOTOVER_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_BOOTOVER_REQ;
#[doc = "`read()` method returns [sys_bootover_req::R](sys_bootover_req::R) reader structure"]
impl crate::Readable for SYS_BOOTOVER_REQ {}
#[doc = "`write(|w| ..)` method takes [sys_bootover_req::W](sys_bootover_req::W) writer structure"]
impl crate::Writable for SYS_BOOTOVER_REQ {}
#[doc = "Boot Override Request Register"]
pub mod sys_bootover_req;
#[doc = "Boot Override Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_bootover_ack](sys_bootover_ack) module"]
pub type SYS_BOOTOVER_ACK = crate::Reg<u32, _SYS_BOOTOVER_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_BOOTOVER_ACK;
#[doc = "`read()` method returns [sys_bootover_ack::R](sys_bootover_ack::R) reader structure"]
impl crate::Readable for SYS_BOOTOVER_ACK {}
#[doc = "`write(|w| ..)` method takes [sys_bootover_ack::W](sys_bootover_ack::W) writer structure"]
impl crate::Writable for SYS_BOOTOVER_ACK {}
#[doc = "Boot Override Acknowledge Register"]
pub mod sys_bootover_ack;
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reset_req](sys_reset_req) module"]
pub type SYS_RESET_REQ = crate::Reg<u32, _SYS_RESET_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_RESET_REQ;
#[doc = "`read()` method returns [sys_reset_req::R](sys_reset_req::R) reader structure"]
impl crate::Readable for SYS_RESET_REQ {}
#[doc = "`write(|w| ..)` method takes [sys_reset_req::W](sys_reset_req::W) writer structure"]
impl crate::Writable for SYS_RESET_REQ {}
#[doc = "Reset Request Register"]
pub mod sys_reset_req;
#[doc = "Reset Status and Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reset_statover](sys_reset_statover) module"]
pub type SYS_RESET_STATOVER = crate::Reg<u32, _SYS_RESET_STATOVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_RESET_STATOVER;
#[doc = "`read()` method returns [sys_reset_statover::R](sys_reset_statover::R) reader structure"]
impl crate::Readable for SYS_RESET_STATOVER {}
#[doc = "`write(|w| ..)` method takes [sys_reset_statover::W](sys_reset_statover::W) writer structure"]
impl crate::Writable for SYS_RESET_STATOVER {}
#[doc = "Reset Status and Override Register"]
pub mod sys_reset_statover;
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_system_stat](sys_system_stat) module"]
pub type SYS_SYSTEM_STAT = crate::Reg<u32, _SYS_SYSTEM_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_SYSTEM_STAT;
#[doc = "`read()` method returns [sys_system_stat::R](sys_system_stat::R) reader structure"]
impl crate::Readable for SYS_SYSTEM_STAT {}
#[doc = "System Status Register"]
pub mod sys_system_stat;
