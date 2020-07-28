#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TLV Checksum"]
    pub tlv_checksum: TLV_CHECKSUM,
    #[doc = "0x04 - Device Info Tag"]
    pub device_info_tag: DEVICE_INFO_TAG,
    #[doc = "0x08 - Device Info Length"]
    pub device_info_len: DEVICE_INFO_LEN,
    #[doc = "0x0c - Device ID"]
    pub device_id: DEVICE_ID,
    #[doc = "0x10 - HW Revision"]
    pub hwrev: HWREV,
    #[doc = "0x14 - Boot Code Revision"]
    pub bcrev: BCREV,
    #[doc = "0x18 - ROM Driver Library Revision"]
    pub rom_drvlib_rev: ROM_DRVLIB_REV,
    #[doc = "0x1c - Die Record Tag"]
    pub die_rec_tag: DIE_REC_TAG,
    #[doc = "0x20 - Die Record Length"]
    pub die_rec_len: DIE_REC_LEN,
    #[doc = "0x24 - Die X-Position"]
    pub die_xpos: DIE_XPOS,
    #[doc = "0x28 - Die Y-Position"]
    pub die_ypos: DIE_YPOS,
    #[doc = "0x2c - Wafer ID"]
    pub wafer_id: WAFER_ID,
    #[doc = "0x30 - Lot ID"]
    pub lot_id: LOT_ID,
    #[doc = "0x34 - Reserved"]
    pub reserved0: RESERVED0,
    #[doc = "0x38 - Reserved"]
    pub reserved1: RESERVED1,
    #[doc = "0x3c - Reserved"]
    pub reserved2: RESERVED2,
    #[doc = "0x40 - Test Results"]
    pub test_results: TEST_RESULTS,
    #[doc = "0x44 - Clock System Calibration Tag"]
    pub cs_cal_tag: CS_CAL_TAG,
    #[doc = "0x48 - Clock System Calibration Length"]
    pub cs_cal_len: CS_CAL_LEN,
    #[doc = "0x4c - DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
    pub dcoir_fcal_rsel04: DCOIR_FCAL_RSEL04,
    #[doc = "0x50 - DCO IR mode: Frequency calibration for DCORSEL 5"]
    pub dcoir_fcal_rsel5: DCOIR_FCAL_RSEL5,
    #[doc = "0x54 - Reserved"]
    pub reserved3: RESERVED3,
    #[doc = "0x58 - Reserved"]
    pub reserved4: RESERVED4,
    #[doc = "0x5c - Reserved"]
    pub reserved5: RESERVED5,
    #[doc = "0x60 - Reserved"]
    pub reserved6: RESERVED6,
    #[doc = "0x64 - DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
    pub dcoir_constk_rsel04: DCOIR_CONSTK_RSEL04,
    #[doc = "0x68 - DCO IR mode: DCO Constant (K) for DCORSEL 5"]
    pub dcoir_constk_rsel5: DCOIR_CONSTK_RSEL5,
    #[doc = "0x6c - DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
    pub dcoer_fcal_rsel04: DCOER_FCAL_RSEL04,
    #[doc = "0x70 - DCO ER mode: Frequency calibration for DCORSEL 5"]
    pub dcoer_fcal_rsel5: DCOER_FCAL_RSEL5,
    #[doc = "0x74 - Reserved"]
    pub reserved7: RESERVED7,
    #[doc = "0x78 - Reserved"]
    pub reserved8: RESERVED8,
    #[doc = "0x7c - Reserved"]
    pub reserved9: RESERVED9,
    #[doc = "0x80 - Reserved"]
    pub reserved10: RESERVED10,
    #[doc = "0x84 - DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
    pub dcoer_constk_rsel04: DCOER_CONSTK_RSEL04,
    #[doc = "0x88 - DCO ER mode: DCO Constant (K) for DCORSEL 5"]
    pub dcoer_constk_rsel5: DCOER_CONSTK_RSEL5,
    #[doc = "0x8c - ADC14 Calibration Tag"]
    pub adc14_cal_tag: ADC14_CAL_TAG,
    #[doc = "0x90 - ADC14 Calibration Length"]
    pub adc14_cal_len: ADC14_CAL_LEN,
    #[doc = "0x94 - ADC Gain Factor"]
    pub adc_gain_factor: ADC_GAIN_FACTOR,
    #[doc = "0x98 - ADC Offset"]
    pub adc_offset: ADC_OFFSET,
    #[doc = "0x9c - Reserved"]
    pub reserved11: RESERVED11,
    #[doc = "0xa0 - Reserved"]
    pub reserved12: RESERVED12,
    #[doc = "0xa4 - Reserved"]
    pub reserved13: RESERVED13,
    #[doc = "0xa8 - Reserved"]
    pub reserved14: RESERVED14,
    #[doc = "0xac - Reserved"]
    pub reserved15: RESERVED15,
    #[doc = "0xb0 - Reserved"]
    pub reserved16: RESERVED16,
    #[doc = "0xb4 - Reserved"]
    pub reserved17: RESERVED17,
    #[doc = "0xb8 - Reserved"]
    pub reserved18: RESERVED18,
    #[doc = "0xbc - Reserved"]
    pub reserved19: RESERVED19,
    #[doc = "0xc0 - Reserved"]
    pub reserved20: RESERVED20,
    #[doc = "0xc4 - Reserved"]
    pub reserved21: RESERVED21,
    #[doc = "0xc8 - Reserved"]
    pub reserved22: RESERVED22,
    #[doc = "0xcc - Reserved"]
    pub reserved23: RESERVED23,
    #[doc = "0xd0 - Reserved"]
    pub reserved24: RESERVED24,
    #[doc = "0xd4 - Reserved"]
    pub reserved25: RESERVED25,
    #[doc = "0xd8 - Reserved"]
    pub reserved26: RESERVED26,
    #[doc = "0xdc - ADC14 1.2V Reference Temp. Sensor 30C"]
    pub adc14_ref1p2v_ts30c: ADC14_REF1P2V_TS30C,
    #[doc = "0xe0 - ADC14 1.2V Reference Temp. Sensor 85C"]
    pub adc14_ref1p2v_ts85c: ADC14_REF1P2V_TS85C,
    #[doc = "0xe4 - ADC14 1.45V Reference Temp. Sensor 30C"]
    pub adc14_ref1p45v_ts30c: ADC14_REF1P45V_TS30C,
    #[doc = "0xe8 - ADC14 1.45V Reference Temp. Sensor 85C"]
    pub adc14_ref1p45v_ts85c: ADC14_REF1P45V_TS85C,
    #[doc = "0xec - ADC14 2.5V Reference Temp. Sensor 30C"]
    pub adc14_ref2p5v_ts30c: ADC14_REF2P5V_TS30C,
    #[doc = "0xf0 - ADC14 2.5V Reference Temp. Sensor 85C"]
    pub adc14_ref2p5v_ts85c: ADC14_REF2P5V_TS85C,
    #[doc = "0xf4 - REF Calibration Tag"]
    pub ref_cal_tag: REF_CAL_TAG,
    #[doc = "0xf8 - REF Calibration Length"]
    pub ref_cal_len: REF_CAL_LEN,
    #[doc = "0xfc - REF 1.2V Reference"]
    pub ref_1p2v: REF_1P2V,
    #[doc = "0x100 - REF 1.45V Reference"]
    pub ref_1p45v: REF_1P45V,
    #[doc = "0x104 - REF 2.5V Reference"]
    pub ref_2p5v: REF_2P5V,
    #[doc = "0x108 - Flash Info Tag"]
    pub flash_info_tag: FLASH_INFO_TAG,
    #[doc = "0x10c - Flash Info Length"]
    pub flash_info_len: FLASH_INFO_LEN,
    #[doc = "0x110 - Flash Maximum Programming Pulses"]
    pub flash_max_prog_pulses: FLASH_MAX_PROG_PULSES,
    #[doc = "0x114 - Flash Maximum Erase Pulses"]
    pub flash_max_erase_pulses: FLASH_MAX_ERASE_PULSES,
    #[doc = "0x118 - 128-bit Random Number Tag"]
    pub random_num_tag: RANDOM_NUM_TAG,
    #[doc = "0x11c - 128-bit Random Number Length"]
    pub random_num_len: RANDOM_NUM_LEN,
    #[doc = "0x120 - 32-bit Random Number 1"]
    pub random_num_1: RANDOM_NUM_1,
    #[doc = "0x124 - 32-bit Random Number 2"]
    pub random_num_2: RANDOM_NUM_2,
    #[doc = "0x128 - 32-bit Random Number 3"]
    pub random_num_3: RANDOM_NUM_3,
    #[doc = "0x12c - 32-bit Random Number 4"]
    pub random_num_4: RANDOM_NUM_4,
    #[doc = "0x130 - BSL Configuration Tag"]
    pub bsl_cfg_tag: BSL_CFG_TAG,
    #[doc = "0x134 - BSL Configuration Length"]
    pub bsl_cfg_len: BSL_CFG_LEN,
    #[doc = "0x138 - BSL Peripheral Interface Selection"]
    pub bsl_periphif_sel: BSL_PERIPHIF_SEL,
    #[doc = "0x13c - BSL Port Interface Configuration for UART"]
    pub bsl_portif_cfg_uart: BSL_PORTIF_CFG_UART,
    #[doc = "0x140 - BSL Port Interface Configuration for SPI"]
    pub bsl_portif_cfg_spi: BSL_PORTIF_CFG_SPI,
    #[doc = "0x144 - BSL Port Interface Configuration for I2C"]
    pub bsl_portif_cfg_i2c: BSL_PORTIF_CFG_I2C,
    #[doc = "0x148 - TLV End Word"]
    pub tlv_end: TLV_END,
}
#[doc = "TLV Checksum\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_checksum](tlv_checksum) module"]
pub type TLV_CHECKSUM = crate::Reg<u32, _TLV_CHECKSUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_CHECKSUM;
#[doc = "`read()` method returns [tlv_checksum::R](tlv_checksum::R) reader structure"]
impl crate::Readable for TLV_CHECKSUM {}
#[doc = "TLV Checksum"]
pub mod tlv_checksum;
#[doc = "Device Info Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_info_tag](device_info_tag) module"]
pub type DEVICE_INFO_TAG = crate::Reg<u32, _DEVICE_INFO_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_INFO_TAG;
#[doc = "`read()` method returns [device_info_tag::R](device_info_tag::R) reader structure"]
impl crate::Readable for DEVICE_INFO_TAG {}
#[doc = "Device Info Tag"]
pub mod device_info_tag;
#[doc = "Device Info Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_info_len](device_info_len) module"]
pub type DEVICE_INFO_LEN = crate::Reg<u32, _DEVICE_INFO_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_INFO_LEN;
#[doc = "`read()` method returns [device_info_len::R](device_info_len::R) reader structure"]
impl crate::Readable for DEVICE_INFO_LEN {}
#[doc = "Device Info Length"]
pub mod device_info_len;
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id](device_id) module"]
pub type DEVICE_ID = crate::Reg<u32, _DEVICE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID;
#[doc = "`read()` method returns [device_id::R](device_id::R) reader structure"]
impl crate::Readable for DEVICE_ID {}
#[doc = "Device ID"]
pub mod device_id;
#[doc = "HW Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrev](hwrev) module"]
pub type HWREV = crate::Reg<u32, _HWREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWREV;
#[doc = "`read()` method returns [hwrev::R](hwrev::R) reader structure"]
impl crate::Readable for HWREV {}
#[doc = "HW Revision"]
pub mod hwrev;
#[doc = "Boot Code Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcrev](bcrev) module"]
pub type BCREV = crate::Reg<u32, _BCREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCREV;
#[doc = "`read()` method returns [bcrev::R](bcrev::R) reader structure"]
impl crate::Readable for BCREV {}
#[doc = "Boot Code Revision"]
pub mod bcrev;
#[doc = "ROM Driver Library Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_drvlib_rev](rom_drvlib_rev) module"]
pub type ROM_DRVLIB_REV = crate::Reg<u32, _ROM_DRVLIB_REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_DRVLIB_REV;
#[doc = "`read()` method returns [rom_drvlib_rev::R](rom_drvlib_rev::R) reader structure"]
impl crate::Readable for ROM_DRVLIB_REV {}
#[doc = "ROM Driver Library Revision"]
pub mod rom_drvlib_rev;
#[doc = "Die Record Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_rec_tag](die_rec_tag) module"]
pub type DIE_REC_TAG = crate::Reg<u32, _DIE_REC_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIE_REC_TAG;
#[doc = "`read()` method returns [die_rec_tag::R](die_rec_tag::R) reader structure"]
impl crate::Readable for DIE_REC_TAG {}
#[doc = "Die Record Tag"]
pub mod die_rec_tag;
#[doc = "Die Record Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_rec_len](die_rec_len) module"]
pub type DIE_REC_LEN = crate::Reg<u32, _DIE_REC_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIE_REC_LEN;
#[doc = "`read()` method returns [die_rec_len::R](die_rec_len::R) reader structure"]
impl crate::Readable for DIE_REC_LEN {}
#[doc = "Die Record Length"]
pub mod die_rec_len;
#[doc = "Die X-Position\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_xpos](die_xpos) module"]
pub type DIE_XPOS = crate::Reg<u32, _DIE_XPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIE_XPOS;
#[doc = "`read()` method returns [die_xpos::R](die_xpos::R) reader structure"]
impl crate::Readable for DIE_XPOS {}
#[doc = "Die X-Position"]
pub mod die_xpos;
#[doc = "Die Y-Position\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_ypos](die_ypos) module"]
pub type DIE_YPOS = crate::Reg<u32, _DIE_YPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIE_YPOS;
#[doc = "`read()` method returns [die_ypos::R](die_ypos::R) reader structure"]
impl crate::Readable for DIE_YPOS {}
#[doc = "Die Y-Position"]
pub mod die_ypos;
#[doc = "Wafer ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wafer_id](wafer_id) module"]
pub type WAFER_ID = crate::Reg<u32, _WAFER_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAFER_ID;
#[doc = "`read()` method returns [wafer_id::R](wafer_id::R) reader structure"]
impl crate::Readable for WAFER_ID {}
#[doc = "Wafer ID"]
pub mod wafer_id;
#[doc = "Lot ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lot_id](lot_id) module"]
pub type LOT_ID = crate::Reg<u32, _LOT_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOT_ID;
#[doc = "`read()` method returns [lot_id::R](lot_id::R) reader structure"]
impl crate::Readable for LOT_ID {}
#[doc = "Lot ID"]
pub mod lot_id;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved0](reserved0) module"]
pub type RESERVED0 = crate::Reg<u32, _RESERVED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED0;
#[doc = "`read()` method returns [reserved0::R](reserved0::R) reader structure"]
impl crate::Readable for RESERVED0 {}
#[doc = "Reserved"]
pub mod reserved0;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved1](reserved1) module"]
pub type RESERVED1 = crate::Reg<u32, _RESERVED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED1;
#[doc = "`read()` method returns [reserved1::R](reserved1::R) reader structure"]
impl crate::Readable for RESERVED1 {}
#[doc = "Reserved"]
pub mod reserved1;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved2](reserved2) module"]
pub type RESERVED2 = crate::Reg<u32, _RESERVED2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED2;
#[doc = "`read()` method returns [reserved2::R](reserved2::R) reader structure"]
impl crate::Readable for RESERVED2 {}
#[doc = "Reserved"]
pub mod reserved2;
#[doc = "Test Results\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_results](test_results) module"]
pub type TEST_RESULTS = crate::Reg<u32, _TEST_RESULTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST_RESULTS;
#[doc = "`read()` method returns [test_results::R](test_results::R) reader structure"]
impl crate::Readable for TEST_RESULTS {}
#[doc = "Test Results"]
pub mod test_results;
#[doc = "Clock System Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs_cal_tag](cs_cal_tag) module"]
pub type CS_CAL_TAG = crate::Reg<u32, _CS_CAL_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS_CAL_TAG;
#[doc = "`read()` method returns [cs_cal_tag::R](cs_cal_tag::R) reader structure"]
impl crate::Readable for CS_CAL_TAG {}
#[doc = "Clock System Calibration Tag"]
pub mod cs_cal_tag;
#[doc = "Clock System Calibration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs_cal_len](cs_cal_len) module"]
pub type CS_CAL_LEN = crate::Reg<u32, _CS_CAL_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS_CAL_LEN;
#[doc = "`read()` method returns [cs_cal_len::R](cs_cal_len::R) reader structure"]
impl crate::Readable for CS_CAL_LEN {}
#[doc = "Clock System Calibration Length"]
pub mod cs_cal_len;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_fcal_rsel04](dcoir_fcal_rsel04) module"]
pub type DCOIR_FCAL_RSEL04 = crate::Reg<u32, _DCOIR_FCAL_RSEL04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOIR_FCAL_RSEL04;
#[doc = "`read()` method returns [dcoir_fcal_rsel04::R](dcoir_fcal_rsel04::R) reader structure"]
impl crate::Readable for DCOIR_FCAL_RSEL04 {}
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoir_fcal_rsel04;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_fcal_rsel5](dcoir_fcal_rsel5) module"]
pub type DCOIR_FCAL_RSEL5 = crate::Reg<u32, _DCOIR_FCAL_RSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOIR_FCAL_RSEL5;
#[doc = "`read()` method returns [dcoir_fcal_rsel5::R](dcoir_fcal_rsel5::R) reader structure"]
impl crate::Readable for DCOIR_FCAL_RSEL5 {}
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 5"]
pub mod dcoir_fcal_rsel5;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved3](reserved3) module"]
pub type RESERVED3 = crate::Reg<u32, _RESERVED3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED3;
#[doc = "`read()` method returns [reserved3::R](reserved3::R) reader structure"]
impl crate::Readable for RESERVED3 {}
#[doc = "Reserved"]
pub mod reserved3;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved4](reserved4) module"]
pub type RESERVED4 = crate::Reg<u32, _RESERVED4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED4;
#[doc = "`read()` method returns [reserved4::R](reserved4::R) reader structure"]
impl crate::Readable for RESERVED4 {}
#[doc = "Reserved"]
pub mod reserved4;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved5](reserved5) module"]
pub type RESERVED5 = crate::Reg<u32, _RESERVED5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED5;
#[doc = "`read()` method returns [reserved5::R](reserved5::R) reader structure"]
impl crate::Readable for RESERVED5 {}
#[doc = "Reserved"]
pub mod reserved5;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved6](reserved6) module"]
pub type RESERVED6 = crate::Reg<u32, _RESERVED6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED6;
#[doc = "`read()` method returns [reserved6::R](reserved6::R) reader structure"]
impl crate::Readable for RESERVED6 {}
#[doc = "Reserved"]
pub mod reserved6;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_constk_rsel04](dcoir_constk_rsel04) module"]
pub type DCOIR_CONSTK_RSEL04 = crate::Reg<u32, _DCOIR_CONSTK_RSEL04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOIR_CONSTK_RSEL04;
#[doc = "`read()` method returns [dcoir_constk_rsel04::R](dcoir_constk_rsel04::R) reader structure"]
impl crate::Readable for DCOIR_CONSTK_RSEL04 {}
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoir_constk_rsel04;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_constk_rsel5](dcoir_constk_rsel5) module"]
pub type DCOIR_CONSTK_RSEL5 = crate::Reg<u32, _DCOIR_CONSTK_RSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOIR_CONSTK_RSEL5;
#[doc = "`read()` method returns [dcoir_constk_rsel5::R](dcoir_constk_rsel5::R) reader structure"]
impl crate::Readable for DCOIR_CONSTK_RSEL5 {}
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoir_constk_rsel5;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoer_fcal_rsel04](dcoer_fcal_rsel04) module"]
pub type DCOER_FCAL_RSEL04 = crate::Reg<u32, _DCOER_FCAL_RSEL04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOER_FCAL_RSEL04;
#[doc = "`read()` method returns [dcoer_fcal_rsel04::R](dcoer_fcal_rsel04::R) reader structure"]
impl crate::Readable for DCOER_FCAL_RSEL04 {}
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoer_fcal_rsel04;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoer_fcal_rsel5](dcoer_fcal_rsel5) module"]
pub type DCOER_FCAL_RSEL5 = crate::Reg<u32, _DCOER_FCAL_RSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOER_FCAL_RSEL5;
#[doc = "`read()` method returns [dcoer_fcal_rsel5::R](dcoer_fcal_rsel5::R) reader structure"]
impl crate::Readable for DCOER_FCAL_RSEL5 {}
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 5"]
pub mod dcoer_fcal_rsel5;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved7](reserved7) module"]
pub type RESERVED7 = crate::Reg<u32, _RESERVED7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED7;
#[doc = "`read()` method returns [reserved7::R](reserved7::R) reader structure"]
impl crate::Readable for RESERVED7 {}
#[doc = "Reserved"]
pub mod reserved7;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved8](reserved8) module"]
pub type RESERVED8 = crate::Reg<u32, _RESERVED8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED8;
#[doc = "`read()` method returns [reserved8::R](reserved8::R) reader structure"]
impl crate::Readable for RESERVED8 {}
#[doc = "Reserved"]
pub mod reserved8;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved9](reserved9) module"]
pub type RESERVED9 = crate::Reg<u32, _RESERVED9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED9;
#[doc = "`read()` method returns [reserved9::R](reserved9::R) reader structure"]
impl crate::Readable for RESERVED9 {}
#[doc = "Reserved"]
pub mod reserved9;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved10](reserved10) module"]
pub type RESERVED10 = crate::Reg<u32, _RESERVED10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED10;
#[doc = "`read()` method returns [reserved10::R](reserved10::R) reader structure"]
impl crate::Readable for RESERVED10 {}
#[doc = "Reserved"]
pub mod reserved10;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoer_constk_rsel04](dcoer_constk_rsel04) module"]
pub type DCOER_CONSTK_RSEL04 = crate::Reg<u32, _DCOER_CONSTK_RSEL04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOER_CONSTK_RSEL04;
#[doc = "`read()` method returns [dcoer_constk_rsel04::R](dcoer_constk_rsel04::R) reader structure"]
impl crate::Readable for DCOER_CONSTK_RSEL04 {}
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoer_constk_rsel04;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoer_constk_rsel5](dcoer_constk_rsel5) module"]
pub type DCOER_CONSTK_RSEL5 = crate::Reg<u32, _DCOER_CONSTK_RSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOER_CONSTK_RSEL5;
#[doc = "`read()` method returns [dcoer_constk_rsel5::R](dcoer_constk_rsel5::R) reader structure"]
impl crate::Readable for DCOER_CONSTK_RSEL5 {}
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoer_constk_rsel5;
#[doc = "ADC14 Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_cal_tag](adc14_cal_tag) module"]
pub type ADC14_CAL_TAG = crate::Reg<u32, _ADC14_CAL_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_CAL_TAG;
#[doc = "`read()` method returns [adc14_cal_tag::R](adc14_cal_tag::R) reader structure"]
impl crate::Readable for ADC14_CAL_TAG {}
#[doc = "ADC14 Calibration Tag"]
pub mod adc14_cal_tag;
#[doc = "ADC14 Calibration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_cal_len](adc14_cal_len) module"]
pub type ADC14_CAL_LEN = crate::Reg<u32, _ADC14_CAL_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_CAL_LEN;
#[doc = "`read()` method returns [adc14_cal_len::R](adc14_cal_len::R) reader structure"]
impl crate::Readable for ADC14_CAL_LEN {}
#[doc = "ADC14 Calibration Length"]
pub mod adc14_cal_len;
#[doc = "ADC Gain Factor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_gain_factor](adc_gain_factor) module"]
pub type ADC_GAIN_FACTOR = crate::Reg<u32, _ADC_GAIN_FACTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_GAIN_FACTOR;
#[doc = "`read()` method returns [adc_gain_factor::R](adc_gain_factor::R) reader structure"]
impl crate::Readable for ADC_GAIN_FACTOR {}
#[doc = "ADC Gain Factor"]
pub mod adc_gain_factor;
#[doc = "ADC Offset\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_offset](adc_offset) module"]
pub type ADC_OFFSET = crate::Reg<u32, _ADC_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFFSET;
#[doc = "`read()` method returns [adc_offset::R](adc_offset::R) reader structure"]
impl crate::Readable for ADC_OFFSET {}
#[doc = "ADC Offset"]
pub mod adc_offset;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved11](reserved11) module"]
pub type RESERVED11 = crate::Reg<u32, _RESERVED11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED11;
#[doc = "`read()` method returns [reserved11::R](reserved11::R) reader structure"]
impl crate::Readable for RESERVED11 {}
#[doc = "Reserved"]
pub mod reserved11;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved12](reserved12) module"]
pub type RESERVED12 = crate::Reg<u32, _RESERVED12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED12;
#[doc = "`read()` method returns [reserved12::R](reserved12::R) reader structure"]
impl crate::Readable for RESERVED12 {}
#[doc = "Reserved"]
pub mod reserved12;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved13](reserved13) module"]
pub type RESERVED13 = crate::Reg<u32, _RESERVED13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED13;
#[doc = "`read()` method returns [reserved13::R](reserved13::R) reader structure"]
impl crate::Readable for RESERVED13 {}
#[doc = "Reserved"]
pub mod reserved13;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved14](reserved14) module"]
pub type RESERVED14 = crate::Reg<u32, _RESERVED14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED14;
#[doc = "`read()` method returns [reserved14::R](reserved14::R) reader structure"]
impl crate::Readable for RESERVED14 {}
#[doc = "Reserved"]
pub mod reserved14;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved15](reserved15) module"]
pub type RESERVED15 = crate::Reg<u32, _RESERVED15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED15;
#[doc = "`read()` method returns [reserved15::R](reserved15::R) reader structure"]
impl crate::Readable for RESERVED15 {}
#[doc = "Reserved"]
pub mod reserved15;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved16](reserved16) module"]
pub type RESERVED16 = crate::Reg<u32, _RESERVED16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED16;
#[doc = "`read()` method returns [reserved16::R](reserved16::R) reader structure"]
impl crate::Readable for RESERVED16 {}
#[doc = "Reserved"]
pub mod reserved16;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved17](reserved17) module"]
pub type RESERVED17 = crate::Reg<u32, _RESERVED17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED17;
#[doc = "`read()` method returns [reserved17::R](reserved17::R) reader structure"]
impl crate::Readable for RESERVED17 {}
#[doc = "Reserved"]
pub mod reserved17;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved18](reserved18) module"]
pub type RESERVED18 = crate::Reg<u32, _RESERVED18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED18;
#[doc = "`read()` method returns [reserved18::R](reserved18::R) reader structure"]
impl crate::Readable for RESERVED18 {}
#[doc = "Reserved"]
pub mod reserved18;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved19](reserved19) module"]
pub type RESERVED19 = crate::Reg<u32, _RESERVED19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED19;
#[doc = "`read()` method returns [reserved19::R](reserved19::R) reader structure"]
impl crate::Readable for RESERVED19 {}
#[doc = "Reserved"]
pub mod reserved19;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved20](reserved20) module"]
pub type RESERVED20 = crate::Reg<u32, _RESERVED20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED20;
#[doc = "`read()` method returns [reserved20::R](reserved20::R) reader structure"]
impl crate::Readable for RESERVED20 {}
#[doc = "Reserved"]
pub mod reserved20;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved21](reserved21) module"]
pub type RESERVED21 = crate::Reg<u32, _RESERVED21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED21;
#[doc = "`read()` method returns [reserved21::R](reserved21::R) reader structure"]
impl crate::Readable for RESERVED21 {}
#[doc = "Reserved"]
pub mod reserved21;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved22](reserved22) module"]
pub type RESERVED22 = crate::Reg<u32, _RESERVED22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED22;
#[doc = "`read()` method returns [reserved22::R](reserved22::R) reader structure"]
impl crate::Readable for RESERVED22 {}
#[doc = "Reserved"]
pub mod reserved22;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved23](reserved23) module"]
pub type RESERVED23 = crate::Reg<u32, _RESERVED23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED23;
#[doc = "`read()` method returns [reserved23::R](reserved23::R) reader structure"]
impl crate::Readable for RESERVED23 {}
#[doc = "Reserved"]
pub mod reserved23;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved24](reserved24) module"]
pub type RESERVED24 = crate::Reg<u32, _RESERVED24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED24;
#[doc = "`read()` method returns [reserved24::R](reserved24::R) reader structure"]
impl crate::Readable for RESERVED24 {}
#[doc = "Reserved"]
pub mod reserved24;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved25](reserved25) module"]
pub type RESERVED25 = crate::Reg<u32, _RESERVED25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED25;
#[doc = "`read()` method returns [reserved25::R](reserved25::R) reader structure"]
impl crate::Readable for RESERVED25 {}
#[doc = "Reserved"]
pub mod reserved25;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved26](reserved26) module"]
pub type RESERVED26 = crate::Reg<u32, _RESERVED26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED26;
#[doc = "`read()` method returns [reserved26::R](reserved26::R) reader structure"]
impl crate::Readable for RESERVED26 {}
#[doc = "Reserved"]
pub mod reserved26;
#[doc = "ADC14 1.2V Reference Temp. Sensor 30C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p2v_ts30c](adc14_ref1p2v_ts30c) module"]
pub type ADC14_REF1P2V_TS30C = crate::Reg<u32, _ADC14_REF1P2V_TS30C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF1P2V_TS30C;
#[doc = "`read()` method returns [adc14_ref1p2v_ts30c::R](adc14_ref1p2v_ts30c::R) reader structure"]
impl crate::Readable for ADC14_REF1P2V_TS30C {}
#[doc = "ADC14 1.2V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p2v_ts30c;
#[doc = "ADC14 1.2V Reference Temp. Sensor 85C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p2v_ts85c](adc14_ref1p2v_ts85c) module"]
pub type ADC14_REF1P2V_TS85C = crate::Reg<u32, _ADC14_REF1P2V_TS85C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF1P2V_TS85C;
#[doc = "`read()` method returns [adc14_ref1p2v_ts85c::R](adc14_ref1p2v_ts85c::R) reader structure"]
impl crate::Readable for ADC14_REF1P2V_TS85C {}
#[doc = "ADC14 1.2V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p2v_ts85c;
#[doc = "ADC14 1.45V Reference Temp. Sensor 30C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p45v_ts30c](adc14_ref1p45v_ts30c) module"]
pub type ADC14_REF1P45V_TS30C = crate::Reg<u32, _ADC14_REF1P45V_TS30C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF1P45V_TS30C;
#[doc = "`read()` method returns [adc14_ref1p45v_ts30c::R](adc14_ref1p45v_ts30c::R) reader structure"]
impl crate::Readable for ADC14_REF1P45V_TS30C {}
#[doc = "ADC14 1.45V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p45v_ts30c;
#[doc = "ADC14 1.45V Reference Temp. Sensor 85C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p45v_ts85c](adc14_ref1p45v_ts85c) module"]
pub type ADC14_REF1P45V_TS85C = crate::Reg<u32, _ADC14_REF1P45V_TS85C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF1P45V_TS85C;
#[doc = "`read()` method returns [adc14_ref1p45v_ts85c::R](adc14_ref1p45v_ts85c::R) reader structure"]
impl crate::Readable for ADC14_REF1P45V_TS85C {}
#[doc = "ADC14 1.45V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p45v_ts85c;
#[doc = "ADC14 2.5V Reference Temp. Sensor 30C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref2p5v_ts30c](adc14_ref2p5v_ts30c) module"]
pub type ADC14_REF2P5V_TS30C = crate::Reg<u32, _ADC14_REF2P5V_TS30C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF2P5V_TS30C;
#[doc = "`read()` method returns [adc14_ref2p5v_ts30c::R](adc14_ref2p5v_ts30c::R) reader structure"]
impl crate::Readable for ADC14_REF2P5V_TS30C {}
#[doc = "ADC14 2.5V Reference Temp. Sensor 30C"]
pub mod adc14_ref2p5v_ts30c;
#[doc = "ADC14 2.5V Reference Temp. Sensor 85C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref2p5v_ts85c](adc14_ref2p5v_ts85c) module"]
pub type ADC14_REF2P5V_TS85C = crate::Reg<u32, _ADC14_REF2P5V_TS85C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC14_REF2P5V_TS85C;
#[doc = "`read()` method returns [adc14_ref2p5v_ts85c::R](adc14_ref2p5v_ts85c::R) reader structure"]
impl crate::Readable for ADC14_REF2P5V_TS85C {}
#[doc = "ADC14 2.5V Reference Temp. Sensor 85C"]
pub mod adc14_ref2p5v_ts85c;
#[doc = "REF Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cal_tag](ref_cal_tag) module"]
pub type REF_CAL_TAG = crate::Reg<u32, _REF_CAL_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_CAL_TAG;
#[doc = "`read()` method returns [ref_cal_tag::R](ref_cal_tag::R) reader structure"]
impl crate::Readable for REF_CAL_TAG {}
#[doc = "REF Calibration Tag"]
pub mod ref_cal_tag;
#[doc = "REF Calibration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cal_len](ref_cal_len) module"]
pub type REF_CAL_LEN = crate::Reg<u32, _REF_CAL_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_CAL_LEN;
#[doc = "`read()` method returns [ref_cal_len::R](ref_cal_len::R) reader structure"]
impl crate::Readable for REF_CAL_LEN {}
#[doc = "REF Calibration Length"]
pub mod ref_cal_len;
#[doc = "REF 1.2V Reference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_1p2v](ref_1p2v) module"]
pub type REF_1P2V = crate::Reg<u32, _REF_1P2V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_1P2V;
#[doc = "`read()` method returns [ref_1p2v::R](ref_1p2v::R) reader structure"]
impl crate::Readable for REF_1P2V {}
#[doc = "REF 1.2V Reference"]
pub mod ref_1p2v;
#[doc = "REF 1.45V Reference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_1p45v](ref_1p45v) module"]
pub type REF_1P45V = crate::Reg<u32, _REF_1P45V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_1P45V;
#[doc = "`read()` method returns [ref_1p45v::R](ref_1p45v::R) reader structure"]
impl crate::Readable for REF_1P45V {}
#[doc = "REF 1.45V Reference"]
pub mod ref_1p45v;
#[doc = "REF 2.5V Reference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_2p5v](ref_2p5v) module"]
pub type REF_2P5V = crate::Reg<u32, _REF_2P5V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_2P5V;
#[doc = "`read()` method returns [ref_2p5v::R](ref_2p5v::R) reader structure"]
impl crate::Readable for REF_2P5V {}
#[doc = "REF 2.5V Reference"]
pub mod ref_2p5v;
#[doc = "Flash Info Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_info_tag](flash_info_tag) module"]
pub type FLASH_INFO_TAG = crate::Reg<u32, _FLASH_INFO_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_INFO_TAG;
#[doc = "`read()` method returns [flash_info_tag::R](flash_info_tag::R) reader structure"]
impl crate::Readable for FLASH_INFO_TAG {}
#[doc = "Flash Info Tag"]
pub mod flash_info_tag;
#[doc = "Flash Info Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_info_len](flash_info_len) module"]
pub type FLASH_INFO_LEN = crate::Reg<u32, _FLASH_INFO_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_INFO_LEN;
#[doc = "`read()` method returns [flash_info_len::R](flash_info_len::R) reader structure"]
impl crate::Readable for FLASH_INFO_LEN {}
#[doc = "Flash Info Length"]
pub mod flash_info_len;
#[doc = "Flash Maximum Programming Pulses\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_max_prog_pulses](flash_max_prog_pulses) module"]
pub type FLASH_MAX_PROG_PULSES = crate::Reg<u32, _FLASH_MAX_PROG_PULSES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_MAX_PROG_PULSES;
#[doc = "`read()` method returns [flash_max_prog_pulses::R](flash_max_prog_pulses::R) reader structure"]
impl crate::Readable for FLASH_MAX_PROG_PULSES {}
#[doc = "Flash Maximum Programming Pulses"]
pub mod flash_max_prog_pulses;
#[doc = "Flash Maximum Erase Pulses\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_max_erase_pulses](flash_max_erase_pulses) module"]
pub type FLASH_MAX_ERASE_PULSES = crate::Reg<u32, _FLASH_MAX_ERASE_PULSES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_MAX_ERASE_PULSES;
#[doc = "`read()` method returns [flash_max_erase_pulses::R](flash_max_erase_pulses::R) reader structure"]
impl crate::Readable for FLASH_MAX_ERASE_PULSES {}
#[doc = "Flash Maximum Erase Pulses"]
pub mod flash_max_erase_pulses;
#[doc = "128-bit Random Number Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_tag](random_num_tag) module"]
pub type RANDOM_NUM_TAG = crate::Reg<u32, _RANDOM_NUM_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_TAG;
#[doc = "`read()` method returns [random_num_tag::R](random_num_tag::R) reader structure"]
impl crate::Readable for RANDOM_NUM_TAG {}
#[doc = "128-bit Random Number Tag"]
pub mod random_num_tag;
#[doc = "128-bit Random Number Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_len](random_num_len) module"]
pub type RANDOM_NUM_LEN = crate::Reg<u32, _RANDOM_NUM_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_LEN;
#[doc = "`read()` method returns [random_num_len::R](random_num_len::R) reader structure"]
impl crate::Readable for RANDOM_NUM_LEN {}
#[doc = "128-bit Random Number Length"]
pub mod random_num_len;
#[doc = "32-bit Random Number 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_1](random_num_1) module"]
pub type RANDOM_NUM_1 = crate::Reg<u32, _RANDOM_NUM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_1;
#[doc = "`read()` method returns [random_num_1::R](random_num_1::R) reader structure"]
impl crate::Readable for RANDOM_NUM_1 {}
#[doc = "32-bit Random Number 1"]
pub mod random_num_1;
#[doc = "32-bit Random Number 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_2](random_num_2) module"]
pub type RANDOM_NUM_2 = crate::Reg<u32, _RANDOM_NUM_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_2;
#[doc = "`read()` method returns [random_num_2::R](random_num_2::R) reader structure"]
impl crate::Readable for RANDOM_NUM_2 {}
#[doc = "32-bit Random Number 2"]
pub mod random_num_2;
#[doc = "32-bit Random Number 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_3](random_num_3) module"]
pub type RANDOM_NUM_3 = crate::Reg<u32, _RANDOM_NUM_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_3;
#[doc = "`read()` method returns [random_num_3::R](random_num_3::R) reader structure"]
impl crate::Readable for RANDOM_NUM_3 {}
#[doc = "32-bit Random Number 3"]
pub mod random_num_3;
#[doc = "32-bit Random Number 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_4](random_num_4) module"]
pub type RANDOM_NUM_4 = crate::Reg<u32, _RANDOM_NUM_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUM_4;
#[doc = "`read()` method returns [random_num_4::R](random_num_4::R) reader structure"]
impl crate::Readable for RANDOM_NUM_4 {}
#[doc = "32-bit Random Number 4"]
pub mod random_num_4;
#[doc = "BSL Configuration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_cfg_tag](bsl_cfg_tag) module"]
pub type BSL_CFG_TAG = crate::Reg<u32, _BSL_CFG_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_CFG_TAG;
#[doc = "`read()` method returns [bsl_cfg_tag::R](bsl_cfg_tag::R) reader structure"]
impl crate::Readable for BSL_CFG_TAG {}
#[doc = "BSL Configuration Tag"]
pub mod bsl_cfg_tag;
#[doc = "BSL Configuration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_cfg_len](bsl_cfg_len) module"]
pub type BSL_CFG_LEN = crate::Reg<u32, _BSL_CFG_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_CFG_LEN;
#[doc = "`read()` method returns [bsl_cfg_len::R](bsl_cfg_len::R) reader structure"]
impl crate::Readable for BSL_CFG_LEN {}
#[doc = "BSL Configuration Length"]
pub mod bsl_cfg_len;
#[doc = "BSL Peripheral Interface Selection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_periphif_sel](bsl_periphif_sel) module"]
pub type BSL_PERIPHIF_SEL = crate::Reg<u32, _BSL_PERIPHIF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_PERIPHIF_SEL;
#[doc = "`read()` method returns [bsl_periphif_sel::R](bsl_periphif_sel::R) reader structure"]
impl crate::Readable for BSL_PERIPHIF_SEL {}
#[doc = "BSL Peripheral Interface Selection"]
pub mod bsl_periphif_sel;
#[doc = "BSL Port Interface Configuration for UART\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_portif_cfg_uart](bsl_portif_cfg_uart) module"]
pub type BSL_PORTIF_CFG_UART = crate::Reg<u32, _BSL_PORTIF_CFG_UART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_PORTIF_CFG_UART;
#[doc = "`read()` method returns [bsl_portif_cfg_uart::R](bsl_portif_cfg_uart::R) reader structure"]
impl crate::Readable for BSL_PORTIF_CFG_UART {}
#[doc = "BSL Port Interface Configuration for UART"]
pub mod bsl_portif_cfg_uart;
#[doc = "BSL Port Interface Configuration for SPI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_portif_cfg_spi](bsl_portif_cfg_spi) module"]
pub type BSL_PORTIF_CFG_SPI = crate::Reg<u32, _BSL_PORTIF_CFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_PORTIF_CFG_SPI;
#[doc = "`read()` method returns [bsl_portif_cfg_spi::R](bsl_portif_cfg_spi::R) reader structure"]
impl crate::Readable for BSL_PORTIF_CFG_SPI {}
#[doc = "BSL Port Interface Configuration for SPI"]
pub mod bsl_portif_cfg_spi;
#[doc = "BSL Port Interface Configuration for I2C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_portif_cfg_i2c](bsl_portif_cfg_i2c) module"]
pub type BSL_PORTIF_CFG_I2C = crate::Reg<u32, _BSL_PORTIF_CFG_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSL_PORTIF_CFG_I2C;
#[doc = "`read()` method returns [bsl_portif_cfg_i2c::R](bsl_portif_cfg_i2c::R) reader structure"]
impl crate::Readable for BSL_PORTIF_CFG_I2C {}
#[doc = "BSL Port Interface Configuration for I2C"]
pub mod bsl_portif_cfg_i2c;
#[doc = "TLV End Word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_end](tlv_end) module"]
pub type TLV_END = crate::Reg<u32, _TLV_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_END;
#[doc = "`read()` method returns [tlv_end::R](tlv_end::R) reader structure"]
impl crate::Readable for TLV_END {}
#[doc = "TLV End Word"]
pub mod tlv_end;
