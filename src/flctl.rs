#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Status Register"]
    pub flctl_power_stat: FLCTL_POWER_STAT,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Bank0 Read Control Register"]
    pub flctl_bank0_rdctl: FLCTL_BANK0_RDCTL,
    #[doc = "0x14 - Bank1 Read Control Register"]
    pub flctl_bank1_rdctl: FLCTL_BANK1_RDCTL,
    _reserved3: [u8; 8usize],
    #[doc = "0x20 - Read Burst/Compare Control and Status Register"]
    pub flctl_rdbrst_ctlstat: FLCTL_RDBRST_CTLSTAT,
    #[doc = "0x24 - Read Burst/Compare Start Address Register"]
    pub flctl_rdbrst_startaddr: FLCTL_RDBRST_STARTADDR,
    #[doc = "0x28 - Read Burst/Compare Length Register"]
    pub flctl_rdbrst_len: FLCTL_RDBRST_LEN,
    _reserved6: [u8; 16usize],
    #[doc = "0x3c - Read Burst/Compare Fail Address Register"]
    pub flctl_rdbrst_failaddr: FLCTL_RDBRST_FAILADDR,
    #[doc = "0x40 - Read Burst/Compare Fail Count Register"]
    pub flctl_rdbrst_failcnt: FLCTL_RDBRST_FAILCNT,
    _reserved8: [u8; 12usize],
    #[doc = "0x50 - Program Control and Status Register"]
    pub flctl_prg_ctlstat: FLCTL_PRG_CTLSTAT,
    #[doc = "0x54 - Program Burst Control and Status Register"]
    pub flctl_prgbrst_ctlstat: FLCTL_PRGBRST_CTLSTAT,
    #[doc = "0x58 - Program Burst Start Address Register"]
    pub flctl_prgbrst_startaddr: FLCTL_PRGBRST_STARTADDR,
    _reserved11: [u8; 4usize],
    #[doc = "0x60 - Program Burst Data0 Register0"]
    pub flctl_prgbrst_data0_0: FLCTL_PRGBRST_DATA0_0,
    #[doc = "0x64 - Program Burst Data0 Register1"]
    pub flctl_prgbrst_data0_1: FLCTL_PRGBRST_DATA0_1,
    #[doc = "0x68 - Program Burst Data0 Register2"]
    pub flctl_prgbrst_data0_2: FLCTL_PRGBRST_DATA0_2,
    #[doc = "0x6c - Program Burst Data0 Register3"]
    pub flctl_prgbrst_data0_3: FLCTL_PRGBRST_DATA0_3,
    #[doc = "0x70 - Program Burst Data1 Register0"]
    pub flctl_prgbrst_data1_0: FLCTL_PRGBRST_DATA1_0,
    #[doc = "0x74 - Program Burst Data1 Register1"]
    pub flctl_prgbrst_data1_1: FLCTL_PRGBRST_DATA1_1,
    #[doc = "0x78 - Program Burst Data1 Register2"]
    pub flctl_prgbrst_data1_2: FLCTL_PRGBRST_DATA1_2,
    #[doc = "0x7c - Program Burst Data1 Register3"]
    pub flctl_prgbrst_data1_3: FLCTL_PRGBRST_DATA1_3,
    #[doc = "0x80 - Program Burst Data2 Register0"]
    pub flctl_prgbrst_data2_0: FLCTL_PRGBRST_DATA2_0,
    #[doc = "0x84 - Program Burst Data2 Register1"]
    pub flctl_prgbrst_data2_1: FLCTL_PRGBRST_DATA2_1,
    #[doc = "0x88 - Program Burst Data2 Register2"]
    pub flctl_prgbrst_data2_2: FLCTL_PRGBRST_DATA2_2,
    #[doc = "0x8c - Program Burst Data2 Register3"]
    pub flctl_prgbrst_data2_3: FLCTL_PRGBRST_DATA2_3,
    #[doc = "0x90 - Program Burst Data3 Register0"]
    pub flctl_prgbrst_data3_0: FLCTL_PRGBRST_DATA3_0,
    #[doc = "0x94 - Program Burst Data3 Register1"]
    pub flctl_prgbrst_data3_1: FLCTL_PRGBRST_DATA3_1,
    #[doc = "0x98 - Program Burst Data3 Register2"]
    pub flctl_prgbrst_data3_2: FLCTL_PRGBRST_DATA3_2,
    #[doc = "0x9c - Program Burst Data3 Register3"]
    pub flctl_prgbrst_data3_3: FLCTL_PRGBRST_DATA3_3,
    #[doc = "0xa0 - Erase Control and Status Register"]
    pub flctl_erase_ctlstat: FLCTL_ERASE_CTLSTAT,
    #[doc = "0xa4 - Erase Sector Address Register"]
    pub flctl_erase_sectaddr: FLCTL_ERASE_SECTADDR,
    _reserved29: [u8; 8usize],
    #[doc = "0xb0 - Information Memory Bank0 Write/Erase Protection Register"]
    pub flctl_bank0_info_weprot: FLCTL_BANK0_INFO_WEPROT,
    #[doc = "0xb4 - Main Memory Bank0 Write/Erase Protection Register"]
    pub flctl_bank0_main_weprot: FLCTL_BANK0_MAIN_WEPROT,
    _reserved31: [u8; 8usize],
    #[doc = "0xc0 - Information Memory Bank1 Write/Erase Protection Register"]
    pub flctl_bank1_info_weprot: FLCTL_BANK1_INFO_WEPROT,
    #[doc = "0xc4 - Main Memory Bank1 Write/Erase Protection Register"]
    pub flctl_bank1_main_weprot: FLCTL_BANK1_MAIN_WEPROT,
    _reserved33: [u8; 8usize],
    #[doc = "0xd0 - Benchmark Control and Status Register"]
    pub flctl_bmrk_ctlstat: FLCTL_BMRK_CTLSTAT,
    #[doc = "0xd4 - Benchmark Instruction Fetch Count Register"]
    pub flctl_bmrk_ifetch: FLCTL_BMRK_IFETCH,
    #[doc = "0xd8 - Benchmark Data Read Count Register"]
    pub flctl_bmrk_dread: FLCTL_BMRK_DREAD,
    #[doc = "0xdc - Benchmark Count Compare Register"]
    pub flctl_bmrk_cmp: FLCTL_BMRK_CMP,
    _reserved37: [u8; 16usize],
    #[doc = "0xf0 - Interrupt Flag Register"]
    pub flctl_ifg: FLCTL_IFG,
    #[doc = "0xf4 - Interrupt Enable Register"]
    pub flctl_ie: FLCTL_IE,
    #[doc = "0xf8 - Clear Interrupt Flag Register"]
    pub flctl_clrifg: FLCTL_CLRIFG,
    #[doc = "0xfc - Set Interrupt Flag Register"]
    pub flctl_setifg: FLCTL_SETIFG,
    #[doc = "0x100 - Read Timing Control Register"]
    pub flctl_read_timctl: FLCTL_READ_TIMCTL,
    #[doc = "0x104 - Read Margin Timing Control Register"]
    pub flctl_readmargin_timctl: FLCTL_READMARGIN_TIMCTL,
    #[doc = "0x108 - Program Verify Timing Control Register"]
    pub flctl_prgver_timctl: FLCTL_PRGVER_TIMCTL,
    #[doc = "0x10c - Erase Verify Timing Control Register"]
    pub flctl_ersver_timctl: FLCTL_ERSVER_TIMCTL,
    #[doc = "0x110 - Leakage Verify Timing Control Register"]
    pub flctl_lkgver_timctl: FLCTL_LKGVER_TIMCTL,
    #[doc = "0x114 - Program Timing Control Register"]
    pub flctl_program_timctl: FLCTL_PROGRAM_TIMCTL,
    #[doc = "0x118 - Erase Timing Control Register"]
    pub flctl_erase_timctl: FLCTL_ERASE_TIMCTL,
    #[doc = "0x11c - Mass Erase Timing Control Register"]
    pub flctl_masserase_timctl: FLCTL_MASSERASE_TIMCTL,
    #[doc = "0x120 - Burst Program Timing Control Register"]
    pub flctl_burstprg_timctl: FLCTL_BURSTPRG_TIMCTL,
}
#[doc = "Power Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_power_stat](flctl_power_stat) module"]
pub type FLCTL_POWER_STAT = crate::Reg<u32, _FLCTL_POWER_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_POWER_STAT;
#[doc = "`read()` method returns [flctl_power_stat::R](flctl_power_stat::R) reader structure"]
impl crate::Readable for FLCTL_POWER_STAT {}
#[doc = "Power Status Register"]
pub mod flctl_power_stat;
#[doc = "Bank0 Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_rdctl](flctl_bank0_rdctl) module"]
pub type FLCTL_BANK0_RDCTL = crate::Reg<u32, _FLCTL_BANK0_RDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK0_RDCTL;
#[doc = "`read()` method returns [flctl_bank0_rdctl::R](flctl_bank0_rdctl::R) reader structure"]
impl crate::Readable for FLCTL_BANK0_RDCTL {}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_rdctl::W](flctl_bank0_rdctl::W) writer structure"]
impl crate::Writable for FLCTL_BANK0_RDCTL {}
#[doc = "Bank0 Read Control Register"]
pub mod flctl_bank0_rdctl;
#[doc = "Bank1 Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_rdctl](flctl_bank1_rdctl) module"]
pub type FLCTL_BANK1_RDCTL = crate::Reg<u32, _FLCTL_BANK1_RDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK1_RDCTL;
#[doc = "`read()` method returns [flctl_bank1_rdctl::R](flctl_bank1_rdctl::R) reader structure"]
impl crate::Readable for FLCTL_BANK1_RDCTL {}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_rdctl::W](flctl_bank1_rdctl::W) writer structure"]
impl crate::Writable for FLCTL_BANK1_RDCTL {}
#[doc = "Bank1 Read Control Register"]
pub mod flctl_bank1_rdctl;
#[doc = "Read Burst/Compare Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_ctlstat](flctl_rdbrst_ctlstat) module"]
pub type FLCTL_RDBRST_CTLSTAT = crate::Reg<u32, _FLCTL_RDBRST_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_RDBRST_CTLSTAT;
#[doc = "`read()` method returns [flctl_rdbrst_ctlstat::R](flctl_rdbrst_ctlstat::R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_ctlstat::W](flctl_rdbrst_ctlstat::W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_CTLSTAT {}
#[doc = "Read Burst/Compare Control and Status Register"]
pub mod flctl_rdbrst_ctlstat;
#[doc = "Read Burst/Compare Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_startaddr](flctl_rdbrst_startaddr) module"]
pub type FLCTL_RDBRST_STARTADDR = crate::Reg<u32, _FLCTL_RDBRST_STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_RDBRST_STARTADDR;
#[doc = "`read()` method returns [flctl_rdbrst_startaddr::R](flctl_rdbrst_startaddr::R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_STARTADDR {}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_startaddr::W](flctl_rdbrst_startaddr::W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_STARTADDR {}
#[doc = "Read Burst/Compare Start Address Register"]
pub mod flctl_rdbrst_startaddr;
#[doc = "Read Burst/Compare Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_len](flctl_rdbrst_len) module"]
pub type FLCTL_RDBRST_LEN = crate::Reg<u32, _FLCTL_RDBRST_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_RDBRST_LEN;
#[doc = "`read()` method returns [flctl_rdbrst_len::R](flctl_rdbrst_len::R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_LEN {}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_len::W](flctl_rdbrst_len::W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_LEN {}
#[doc = "Read Burst/Compare Length Register"]
pub mod flctl_rdbrst_len;
#[doc = "Read Burst/Compare Fail Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_failaddr](flctl_rdbrst_failaddr) module"]
pub type FLCTL_RDBRST_FAILADDR = crate::Reg<u32, _FLCTL_RDBRST_FAILADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_RDBRST_FAILADDR;
#[doc = "`read()` method returns [flctl_rdbrst_failaddr::R](flctl_rdbrst_failaddr::R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_FAILADDR {}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_failaddr::W](flctl_rdbrst_failaddr::W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_FAILADDR {}
#[doc = "Read Burst/Compare Fail Address Register"]
pub mod flctl_rdbrst_failaddr;
#[doc = "Read Burst/Compare Fail Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_failcnt](flctl_rdbrst_failcnt) module"]
pub type FLCTL_RDBRST_FAILCNT = crate::Reg<u32, _FLCTL_RDBRST_FAILCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_RDBRST_FAILCNT;
#[doc = "`read()` method returns [flctl_rdbrst_failcnt::R](flctl_rdbrst_failcnt::R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_FAILCNT {}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_failcnt::W](flctl_rdbrst_failcnt::W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_FAILCNT {}
#[doc = "Read Burst/Compare Fail Count Register"]
pub mod flctl_rdbrst_failcnt;
#[doc = "Program Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prg_ctlstat](flctl_prg_ctlstat) module"]
pub type FLCTL_PRG_CTLSTAT = crate::Reg<u32, _FLCTL_PRG_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRG_CTLSTAT;
#[doc = "`read()` method returns [flctl_prg_ctlstat::R](flctl_prg_ctlstat::R) reader structure"]
impl crate::Readable for FLCTL_PRG_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [flctl_prg_ctlstat::W](flctl_prg_ctlstat::W) writer structure"]
impl crate::Writable for FLCTL_PRG_CTLSTAT {}
#[doc = "Program Control and Status Register"]
pub mod flctl_prg_ctlstat;
#[doc = "Program Burst Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_ctlstat](flctl_prgbrst_ctlstat) module"]
pub type FLCTL_PRGBRST_CTLSTAT = crate::Reg<u32, _FLCTL_PRGBRST_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_CTLSTAT;
#[doc = "`read()` method returns [flctl_prgbrst_ctlstat::R](flctl_prgbrst_ctlstat::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_ctlstat::W](flctl_prgbrst_ctlstat::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_CTLSTAT {}
#[doc = "Program Burst Control and Status Register"]
pub mod flctl_prgbrst_ctlstat;
#[doc = "Program Burst Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_startaddr](flctl_prgbrst_startaddr) module"]
pub type FLCTL_PRGBRST_STARTADDR = crate::Reg<u32, _FLCTL_PRGBRST_STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_STARTADDR;
#[doc = "`read()` method returns [flctl_prgbrst_startaddr::R](flctl_prgbrst_startaddr::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_STARTADDR {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_startaddr::W](flctl_prgbrst_startaddr::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_STARTADDR {}
#[doc = "Program Burst Start Address Register"]
pub mod flctl_prgbrst_startaddr;
#[doc = "Program Burst Data0 Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data0_0](flctl_prgbrst_data0_0) module"]
pub type FLCTL_PRGBRST_DATA0_0 = crate::Reg<u32, _FLCTL_PRGBRST_DATA0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA0_0;
#[doc = "`read()` method returns [flctl_prgbrst_data0_0::R](flctl_prgbrst_data0_0::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA0_0 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data0_0::W](flctl_prgbrst_data0_0::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA0_0 {}
#[doc = "Program Burst Data0 Register0"]
pub mod flctl_prgbrst_data0_0;
#[doc = "Program Burst Data0 Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data0_1](flctl_prgbrst_data0_1) module"]
pub type FLCTL_PRGBRST_DATA0_1 = crate::Reg<u32, _FLCTL_PRGBRST_DATA0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA0_1;
#[doc = "`read()` method returns [flctl_prgbrst_data0_1::R](flctl_prgbrst_data0_1::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA0_1 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data0_1::W](flctl_prgbrst_data0_1::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA0_1 {}
#[doc = "Program Burst Data0 Register1"]
pub mod flctl_prgbrst_data0_1;
#[doc = "Program Burst Data0 Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data0_2](flctl_prgbrst_data0_2) module"]
pub type FLCTL_PRGBRST_DATA0_2 = crate::Reg<u32, _FLCTL_PRGBRST_DATA0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA0_2;
#[doc = "`read()` method returns [flctl_prgbrst_data0_2::R](flctl_prgbrst_data0_2::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA0_2 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data0_2::W](flctl_prgbrst_data0_2::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA0_2 {}
#[doc = "Program Burst Data0 Register2"]
pub mod flctl_prgbrst_data0_2;
#[doc = "Program Burst Data0 Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data0_3](flctl_prgbrst_data0_3) module"]
pub type FLCTL_PRGBRST_DATA0_3 = crate::Reg<u32, _FLCTL_PRGBRST_DATA0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA0_3;
#[doc = "`read()` method returns [flctl_prgbrst_data0_3::R](flctl_prgbrst_data0_3::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA0_3 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data0_3::W](flctl_prgbrst_data0_3::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA0_3 {}
#[doc = "Program Burst Data0 Register3"]
pub mod flctl_prgbrst_data0_3;
#[doc = "Program Burst Data1 Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data1_0](flctl_prgbrst_data1_0) module"]
pub type FLCTL_PRGBRST_DATA1_0 = crate::Reg<u32, _FLCTL_PRGBRST_DATA1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA1_0;
#[doc = "`read()` method returns [flctl_prgbrst_data1_0::R](flctl_prgbrst_data1_0::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA1_0 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data1_0::W](flctl_prgbrst_data1_0::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA1_0 {}
#[doc = "Program Burst Data1 Register0"]
pub mod flctl_prgbrst_data1_0;
#[doc = "Program Burst Data1 Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data1_1](flctl_prgbrst_data1_1) module"]
pub type FLCTL_PRGBRST_DATA1_1 = crate::Reg<u32, _FLCTL_PRGBRST_DATA1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA1_1;
#[doc = "`read()` method returns [flctl_prgbrst_data1_1::R](flctl_prgbrst_data1_1::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA1_1 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data1_1::W](flctl_prgbrst_data1_1::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA1_1 {}
#[doc = "Program Burst Data1 Register1"]
pub mod flctl_prgbrst_data1_1;
#[doc = "Program Burst Data1 Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data1_2](flctl_prgbrst_data1_2) module"]
pub type FLCTL_PRGBRST_DATA1_2 = crate::Reg<u32, _FLCTL_PRGBRST_DATA1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA1_2;
#[doc = "`read()` method returns [flctl_prgbrst_data1_2::R](flctl_prgbrst_data1_2::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA1_2 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data1_2::W](flctl_prgbrst_data1_2::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA1_2 {}
#[doc = "Program Burst Data1 Register2"]
pub mod flctl_prgbrst_data1_2;
#[doc = "Program Burst Data1 Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data1_3](flctl_prgbrst_data1_3) module"]
pub type FLCTL_PRGBRST_DATA1_3 = crate::Reg<u32, _FLCTL_PRGBRST_DATA1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA1_3;
#[doc = "`read()` method returns [flctl_prgbrst_data1_3::R](flctl_prgbrst_data1_3::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA1_3 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data1_3::W](flctl_prgbrst_data1_3::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA1_3 {}
#[doc = "Program Burst Data1 Register3"]
pub mod flctl_prgbrst_data1_3;
#[doc = "Program Burst Data2 Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data2_0](flctl_prgbrst_data2_0) module"]
pub type FLCTL_PRGBRST_DATA2_0 = crate::Reg<u32, _FLCTL_PRGBRST_DATA2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA2_0;
#[doc = "`read()` method returns [flctl_prgbrst_data2_0::R](flctl_prgbrst_data2_0::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA2_0 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data2_0::W](flctl_prgbrst_data2_0::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA2_0 {}
#[doc = "Program Burst Data2 Register0"]
pub mod flctl_prgbrst_data2_0;
#[doc = "Program Burst Data2 Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data2_1](flctl_prgbrst_data2_1) module"]
pub type FLCTL_PRGBRST_DATA2_1 = crate::Reg<u32, _FLCTL_PRGBRST_DATA2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA2_1;
#[doc = "`read()` method returns [flctl_prgbrst_data2_1::R](flctl_prgbrst_data2_1::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA2_1 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data2_1::W](flctl_prgbrst_data2_1::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA2_1 {}
#[doc = "Program Burst Data2 Register1"]
pub mod flctl_prgbrst_data2_1;
#[doc = "Program Burst Data2 Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data2_2](flctl_prgbrst_data2_2) module"]
pub type FLCTL_PRGBRST_DATA2_2 = crate::Reg<u32, _FLCTL_PRGBRST_DATA2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA2_2;
#[doc = "`read()` method returns [flctl_prgbrst_data2_2::R](flctl_prgbrst_data2_2::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA2_2 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data2_2::W](flctl_prgbrst_data2_2::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA2_2 {}
#[doc = "Program Burst Data2 Register2"]
pub mod flctl_prgbrst_data2_2;
#[doc = "Program Burst Data2 Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data2_3](flctl_prgbrst_data2_3) module"]
pub type FLCTL_PRGBRST_DATA2_3 = crate::Reg<u32, _FLCTL_PRGBRST_DATA2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA2_3;
#[doc = "`read()` method returns [flctl_prgbrst_data2_3::R](flctl_prgbrst_data2_3::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA2_3 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data2_3::W](flctl_prgbrst_data2_3::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA2_3 {}
#[doc = "Program Burst Data2 Register3"]
pub mod flctl_prgbrst_data2_3;
#[doc = "Program Burst Data3 Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data3_0](flctl_prgbrst_data3_0) module"]
pub type FLCTL_PRGBRST_DATA3_0 = crate::Reg<u32, _FLCTL_PRGBRST_DATA3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA3_0;
#[doc = "`read()` method returns [flctl_prgbrst_data3_0::R](flctl_prgbrst_data3_0::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA3_0 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data3_0::W](flctl_prgbrst_data3_0::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA3_0 {}
#[doc = "Program Burst Data3 Register0"]
pub mod flctl_prgbrst_data3_0;
#[doc = "Program Burst Data3 Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data3_1](flctl_prgbrst_data3_1) module"]
pub type FLCTL_PRGBRST_DATA3_1 = crate::Reg<u32, _FLCTL_PRGBRST_DATA3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA3_1;
#[doc = "`read()` method returns [flctl_prgbrst_data3_1::R](flctl_prgbrst_data3_1::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA3_1 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data3_1::W](flctl_prgbrst_data3_1::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA3_1 {}
#[doc = "Program Burst Data3 Register1"]
pub mod flctl_prgbrst_data3_1;
#[doc = "Program Burst Data3 Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data3_2](flctl_prgbrst_data3_2) module"]
pub type FLCTL_PRGBRST_DATA3_2 = crate::Reg<u32, _FLCTL_PRGBRST_DATA3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA3_2;
#[doc = "`read()` method returns [flctl_prgbrst_data3_2::R](flctl_prgbrst_data3_2::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA3_2 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data3_2::W](flctl_prgbrst_data3_2::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA3_2 {}
#[doc = "Program Burst Data3 Register2"]
pub mod flctl_prgbrst_data3_2;
#[doc = "Program Burst Data3 Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data3_3](flctl_prgbrst_data3_3) module"]
pub type FLCTL_PRGBRST_DATA3_3 = crate::Reg<u32, _FLCTL_PRGBRST_DATA3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGBRST_DATA3_3;
#[doc = "`read()` method returns [flctl_prgbrst_data3_3::R](flctl_prgbrst_data3_3::R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA3_3 {}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data3_3::W](flctl_prgbrst_data3_3::W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA3_3 {}
#[doc = "Program Burst Data3 Register3"]
pub mod flctl_prgbrst_data3_3;
#[doc = "Erase Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_ctlstat](flctl_erase_ctlstat) module"]
pub type FLCTL_ERASE_CTLSTAT = crate::Reg<u32, _FLCTL_ERASE_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_ERASE_CTLSTAT;
#[doc = "`read()` method returns [flctl_erase_ctlstat::R](flctl_erase_ctlstat::R) reader structure"]
impl crate::Readable for FLCTL_ERASE_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [flctl_erase_ctlstat::W](flctl_erase_ctlstat::W) writer structure"]
impl crate::Writable for FLCTL_ERASE_CTLSTAT {}
#[doc = "Erase Control and Status Register"]
pub mod flctl_erase_ctlstat;
#[doc = "Erase Sector Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_sectaddr](flctl_erase_sectaddr) module"]
pub type FLCTL_ERASE_SECTADDR = crate::Reg<u32, _FLCTL_ERASE_SECTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_ERASE_SECTADDR;
#[doc = "`read()` method returns [flctl_erase_sectaddr::R](flctl_erase_sectaddr::R) reader structure"]
impl crate::Readable for FLCTL_ERASE_SECTADDR {}
#[doc = "`write(|w| ..)` method takes [flctl_erase_sectaddr::W](flctl_erase_sectaddr::W) writer structure"]
impl crate::Writable for FLCTL_ERASE_SECTADDR {}
#[doc = "Erase Sector Address Register"]
pub mod flctl_erase_sectaddr;
#[doc = "Information Memory Bank0 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_info_weprot](flctl_bank0_info_weprot) module"]
pub type FLCTL_BANK0_INFO_WEPROT = crate::Reg<u32, _FLCTL_BANK0_INFO_WEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK0_INFO_WEPROT;
#[doc = "`read()` method returns [flctl_bank0_info_weprot::R](flctl_bank0_info_weprot::R) reader structure"]
impl crate::Readable for FLCTL_BANK0_INFO_WEPROT {}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_info_weprot::W](flctl_bank0_info_weprot::W) writer structure"]
impl crate::Writable for FLCTL_BANK0_INFO_WEPROT {}
#[doc = "Information Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_info_weprot;
#[doc = "Main Memory Bank0 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot](flctl_bank0_main_weprot) module"]
pub type FLCTL_BANK0_MAIN_WEPROT = crate::Reg<u32, _FLCTL_BANK0_MAIN_WEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK0_MAIN_WEPROT;
#[doc = "`read()` method returns [flctl_bank0_main_weprot::R](flctl_bank0_main_weprot::R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT {}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot::W](flctl_bank0_main_weprot::W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT {}
#[doc = "Main Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_main_weprot;
#[doc = "Information Memory Bank1 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_info_weprot](flctl_bank1_info_weprot) module"]
pub type FLCTL_BANK1_INFO_WEPROT = crate::Reg<u32, _FLCTL_BANK1_INFO_WEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK1_INFO_WEPROT;
#[doc = "`read()` method returns [flctl_bank1_info_weprot::R](flctl_bank1_info_weprot::R) reader structure"]
impl crate::Readable for FLCTL_BANK1_INFO_WEPROT {}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_info_weprot::W](flctl_bank1_info_weprot::W) writer structure"]
impl crate::Writable for FLCTL_BANK1_INFO_WEPROT {}
#[doc = "Information Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_info_weprot;
#[doc = "Main Memory Bank1 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_main_weprot](flctl_bank1_main_weprot) module"]
pub type FLCTL_BANK1_MAIN_WEPROT = crate::Reg<u32, _FLCTL_BANK1_MAIN_WEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BANK1_MAIN_WEPROT;
#[doc = "`read()` method returns [flctl_bank1_main_weprot::R](flctl_bank1_main_weprot::R) reader structure"]
impl crate::Readable for FLCTL_BANK1_MAIN_WEPROT {}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_main_weprot::W](flctl_bank1_main_weprot::W) writer structure"]
impl crate::Writable for FLCTL_BANK1_MAIN_WEPROT {}
#[doc = "Main Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_main_weprot;
#[doc = "Benchmark Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_ctlstat](flctl_bmrk_ctlstat) module"]
pub type FLCTL_BMRK_CTLSTAT = crate::Reg<u32, _FLCTL_BMRK_CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BMRK_CTLSTAT;
#[doc = "`read()` method returns [flctl_bmrk_ctlstat::R](flctl_bmrk_ctlstat::R) reader structure"]
impl crate::Readable for FLCTL_BMRK_CTLSTAT {}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_ctlstat::W](flctl_bmrk_ctlstat::W) writer structure"]
impl crate::Writable for FLCTL_BMRK_CTLSTAT {}
#[doc = "Benchmark Control and Status Register"]
pub mod flctl_bmrk_ctlstat;
#[doc = "Benchmark Instruction Fetch Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_ifetch](flctl_bmrk_ifetch) module"]
pub type FLCTL_BMRK_IFETCH = crate::Reg<u32, _FLCTL_BMRK_IFETCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BMRK_IFETCH;
#[doc = "`read()` method returns [flctl_bmrk_ifetch::R](flctl_bmrk_ifetch::R) reader structure"]
impl crate::Readable for FLCTL_BMRK_IFETCH {}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_ifetch::W](flctl_bmrk_ifetch::W) writer structure"]
impl crate::Writable for FLCTL_BMRK_IFETCH {}
#[doc = "Benchmark Instruction Fetch Count Register"]
pub mod flctl_bmrk_ifetch;
#[doc = "Benchmark Data Read Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_dread](flctl_bmrk_dread) module"]
pub type FLCTL_BMRK_DREAD = crate::Reg<u32, _FLCTL_BMRK_DREAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BMRK_DREAD;
#[doc = "`read()` method returns [flctl_bmrk_dread::R](flctl_bmrk_dread::R) reader structure"]
impl crate::Readable for FLCTL_BMRK_DREAD {}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_dread::W](flctl_bmrk_dread::W) writer structure"]
impl crate::Writable for FLCTL_BMRK_DREAD {}
#[doc = "Benchmark Data Read Count Register"]
pub mod flctl_bmrk_dread;
#[doc = "Benchmark Count Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_cmp](flctl_bmrk_cmp) module"]
pub type FLCTL_BMRK_CMP = crate::Reg<u32, _FLCTL_BMRK_CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BMRK_CMP;
#[doc = "`read()` method returns [flctl_bmrk_cmp::R](flctl_bmrk_cmp::R) reader structure"]
impl crate::Readable for FLCTL_BMRK_CMP {}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_cmp::W](flctl_bmrk_cmp::W) writer structure"]
impl crate::Writable for FLCTL_BMRK_CMP {}
#[doc = "Benchmark Count Compare Register"]
pub mod flctl_bmrk_cmp;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ifg](flctl_ifg) module"]
pub type FLCTL_IFG = crate::Reg<u32, _FLCTL_IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_IFG;
#[doc = "`read()` method returns [flctl_ifg::R](flctl_ifg::R) reader structure"]
impl crate::Readable for FLCTL_IFG {}
#[doc = "Interrupt Flag Register"]
pub mod flctl_ifg;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ie](flctl_ie) module"]
pub type FLCTL_IE = crate::Reg<u32, _FLCTL_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_IE;
#[doc = "`read()` method returns [flctl_ie::R](flctl_ie::R) reader structure"]
impl crate::Readable for FLCTL_IE {}
#[doc = "`write(|w| ..)` method takes [flctl_ie::W](flctl_ie::W) writer structure"]
impl crate::Writable for FLCTL_IE {}
#[doc = "Interrupt Enable Register"]
pub mod flctl_ie;
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_clrifg](flctl_clrifg) module"]
pub type FLCTL_CLRIFG = crate::Reg<u32, _FLCTL_CLRIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_CLRIFG;
#[doc = "`write(|w| ..)` method takes [flctl_clrifg::W](flctl_clrifg::W) writer structure"]
impl crate::Writable for FLCTL_CLRIFG {}
#[doc = "Clear Interrupt Flag Register"]
pub mod flctl_clrifg;
#[doc = "Set Interrupt Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_setifg](flctl_setifg) module"]
pub type FLCTL_SETIFG = crate::Reg<u32, _FLCTL_SETIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_SETIFG;
#[doc = "`write(|w| ..)` method takes [flctl_setifg::W](flctl_setifg::W) writer structure"]
impl crate::Writable for FLCTL_SETIFG {}
#[doc = "Set Interrupt Flag Register"]
pub mod flctl_setifg;
#[doc = "Read Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_read_timctl](flctl_read_timctl) module"]
pub type FLCTL_READ_TIMCTL = crate::Reg<u32, _FLCTL_READ_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_READ_TIMCTL;
#[doc = "`read()` method returns [flctl_read_timctl::R](flctl_read_timctl::R) reader structure"]
impl crate::Readable for FLCTL_READ_TIMCTL {}
#[doc = "Read Timing Control Register"]
pub mod flctl_read_timctl;
#[doc = "Read Margin Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_readmargin_timctl](flctl_readmargin_timctl) module"]
pub type FLCTL_READMARGIN_TIMCTL = crate::Reg<u32, _FLCTL_READMARGIN_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_READMARGIN_TIMCTL;
#[doc = "`read()` method returns [flctl_readmargin_timctl::R](flctl_readmargin_timctl::R) reader structure"]
impl crate::Readable for FLCTL_READMARGIN_TIMCTL {}
#[doc = "Read Margin Timing Control Register"]
pub mod flctl_readmargin_timctl;
#[doc = "Program Verify Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgver_timctl](flctl_prgver_timctl) module"]
pub type FLCTL_PRGVER_TIMCTL = crate::Reg<u32, _FLCTL_PRGVER_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PRGVER_TIMCTL;
#[doc = "`read()` method returns [flctl_prgver_timctl::R](flctl_prgver_timctl::R) reader structure"]
impl crate::Readable for FLCTL_PRGVER_TIMCTL {}
#[doc = "Program Verify Timing Control Register"]
pub mod flctl_prgver_timctl;
#[doc = "Erase Verify Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ersver_timctl](flctl_ersver_timctl) module"]
pub type FLCTL_ERSVER_TIMCTL = crate::Reg<u32, _FLCTL_ERSVER_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_ERSVER_TIMCTL;
#[doc = "`read()` method returns [flctl_ersver_timctl::R](flctl_ersver_timctl::R) reader structure"]
impl crate::Readable for FLCTL_ERSVER_TIMCTL {}
#[doc = "Erase Verify Timing Control Register"]
pub mod flctl_ersver_timctl;
#[doc = "Leakage Verify Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_lkgver_timctl](flctl_lkgver_timctl) module"]
pub type FLCTL_LKGVER_TIMCTL = crate::Reg<u32, _FLCTL_LKGVER_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_LKGVER_TIMCTL;
#[doc = "`read()` method returns [flctl_lkgver_timctl::R](flctl_lkgver_timctl::R) reader structure"]
impl crate::Readable for FLCTL_LKGVER_TIMCTL {}
#[doc = "Leakage Verify Timing Control Register"]
pub mod flctl_lkgver_timctl;
#[doc = "Program Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_program_timctl](flctl_program_timctl) module"]
pub type FLCTL_PROGRAM_TIMCTL = crate::Reg<u32, _FLCTL_PROGRAM_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_PROGRAM_TIMCTL;
#[doc = "`read()` method returns [flctl_program_timctl::R](flctl_program_timctl::R) reader structure"]
impl crate::Readable for FLCTL_PROGRAM_TIMCTL {}
#[doc = "Program Timing Control Register"]
pub mod flctl_program_timctl;
#[doc = "Erase Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_timctl](flctl_erase_timctl) module"]
pub type FLCTL_ERASE_TIMCTL = crate::Reg<u32, _FLCTL_ERASE_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_ERASE_TIMCTL;
#[doc = "`read()` method returns [flctl_erase_timctl::R](flctl_erase_timctl::R) reader structure"]
impl crate::Readable for FLCTL_ERASE_TIMCTL {}
#[doc = "Erase Timing Control Register"]
pub mod flctl_erase_timctl;
#[doc = "Mass Erase Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_masserase_timctl](flctl_masserase_timctl) module"]
pub type FLCTL_MASSERASE_TIMCTL = crate::Reg<u32, _FLCTL_MASSERASE_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_MASSERASE_TIMCTL;
#[doc = "`read()` method returns [flctl_masserase_timctl::R](flctl_masserase_timctl::R) reader structure"]
impl crate::Readable for FLCTL_MASSERASE_TIMCTL {}
#[doc = "Mass Erase Timing Control Register"]
pub mod flctl_masserase_timctl;
#[doc = "Burst Program Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_burstprg_timctl](flctl_burstprg_timctl) module"]
pub type FLCTL_BURSTPRG_TIMCTL = crate::Reg<u32, _FLCTL_BURSTPRG_TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLCTL_BURSTPRG_TIMCTL;
#[doc = "`read()` method returns [flctl_burstprg_timctl::R](flctl_burstprg_timctl::R) reader structure"]
impl crate::Readable for FLCTL_BURSTPRG_TIMCTL {}
#[doc = "Burst Program Timing Control Register"]
pub mod flctl_burstprg_timctl;
