#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Input for CRC32 Signature Computation"]
    pub crc32di: CRC32DI,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Data In Reverse for CRC32 Computation"]
    pub crc32dirb: CRC32DIRB,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - CRC32 Initialization and Result, lower 16 bits"]
    pub crc32inires_lo: CRC32INIRES_LO,
    #[doc = "0x0a - CRC32 Initialization and Result, upper 16 bits"]
    pub crc32inires_hi: CRC32INIRES_HI,
    #[doc = "0x0c - CRC32 Result Reverse, lower 16 bits"]
    pub crc32resr_lo: CRC32RESR_LO,
    #[doc = "0x0e - CRC32 Result Reverse, Upper 16 bits"]
    pub crc32resr_hi: CRC32RESR_HI,
    #[doc = "0x10 - Data Input for CRC16 computation"]
    pub crc16di: CRC16DI,
    _reserved7: [u8; 2usize],
    #[doc = "0x14 - CRC16 Data In Reverse"]
    pub crc16dirb: CRC16DIRB,
    _reserved8: [u8; 2usize],
    #[doc = "0x18 - CRC16 Initialization and Result register"]
    pub crc16inires: CRC16INIRES,
    _reserved9: [u8; 4usize],
    #[doc = "0x1e - CRC16 Result Reverse"]
    pub crc16resr: CRC16RESR,
}
#[doc = "Data Input for CRC32 Signature Computation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32di](crc32di) module"]
pub type CRC32DI = crate::Reg<u16, _CRC32DI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DI;
#[doc = "`read()` method returns [crc32di::R](crc32di::R) reader structure"]
impl crate::Readable for CRC32DI {}
#[doc = "`write(|w| ..)` method takes [crc32di::W](crc32di::W) writer structure"]
impl crate::Writable for CRC32DI {}
#[doc = "Data Input for CRC32 Signature Computation"]
pub mod crc32di;
#[doc = "Data In Reverse for CRC32 Computation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dirb](crc32dirb) module"]
pub type CRC32DIRB = crate::Reg<u16, _CRC32DIRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DIRB;
#[doc = "`read()` method returns [crc32dirb::R](crc32dirb::R) reader structure"]
impl crate::Readable for CRC32DIRB {}
#[doc = "`write(|w| ..)` method takes [crc32dirb::W](crc32dirb::W) writer structure"]
impl crate::Writable for CRC32DIRB {}
#[doc = "Data In Reverse for CRC32 Computation"]
pub mod crc32dirb;
#[doc = "CRC32 Initialization and Result, lower 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32inires_lo](crc32inires_lo) module"]
pub type CRC32INIRES_LO = crate::Reg<u16, _CRC32INIRES_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32INIRES_LO;
#[doc = "`read()` method returns [crc32inires_lo::R](crc32inires_lo::R) reader structure"]
impl crate::Readable for CRC32INIRES_LO {}
#[doc = "`write(|w| ..)` method takes [crc32inires_lo::W](crc32inires_lo::W) writer structure"]
impl crate::Writable for CRC32INIRES_LO {}
#[doc = "CRC32 Initialization and Result, lower 16 bits"]
pub mod crc32inires_lo;
#[doc = "CRC32 Initialization and Result, upper 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32inires_hi](crc32inires_hi) module"]
pub type CRC32INIRES_HI = crate::Reg<u16, _CRC32INIRES_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32INIRES_HI;
#[doc = "`read()` method returns [crc32inires_hi::R](crc32inires_hi::R) reader structure"]
impl crate::Readable for CRC32INIRES_HI {}
#[doc = "`write(|w| ..)` method takes [crc32inires_hi::W](crc32inires_hi::W) writer structure"]
impl crate::Writable for CRC32INIRES_HI {}
#[doc = "CRC32 Initialization and Result, upper 16 bits"]
pub mod crc32inires_hi;
#[doc = "CRC32 Result Reverse, lower 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32resr_lo](crc32resr_lo) module"]
pub type CRC32RESR_LO = crate::Reg<u16, _CRC32RESR_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32RESR_LO;
#[doc = "`read()` method returns [crc32resr_lo::R](crc32resr_lo::R) reader structure"]
impl crate::Readable for CRC32RESR_LO {}
#[doc = "`write(|w| ..)` method takes [crc32resr_lo::W](crc32resr_lo::W) writer structure"]
impl crate::Writable for CRC32RESR_LO {}
#[doc = "CRC32 Result Reverse, lower 16 bits"]
pub mod crc32resr_lo;
#[doc = "CRC32 Result Reverse, Upper 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32resr_hi](crc32resr_hi) module"]
pub type CRC32RESR_HI = crate::Reg<u16, _CRC32RESR_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32RESR_HI;
#[doc = "`read()` method returns [crc32resr_hi::R](crc32resr_hi::R) reader structure"]
impl crate::Readable for CRC32RESR_HI {}
#[doc = "`write(|w| ..)` method takes [crc32resr_hi::W](crc32resr_hi::W) writer structure"]
impl crate::Writable for CRC32RESR_HI {}
#[doc = "CRC32 Result Reverse, Upper 16 bits"]
pub mod crc32resr_hi;
#[doc = "Data Input for CRC16 computation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16di](crc16di) module"]
pub type CRC16DI = crate::Reg<u16, _CRC16DI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16DI;
#[doc = "`read()` method returns [crc16di::R](crc16di::R) reader structure"]
impl crate::Readable for CRC16DI {}
#[doc = "`write(|w| ..)` method takes [crc16di::W](crc16di::W) writer structure"]
impl crate::Writable for CRC16DI {}
#[doc = "Data Input for CRC16 computation"]
pub mod crc16di;
#[doc = "CRC16 Data In Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16dirb](crc16dirb) module"]
pub type CRC16DIRB = crate::Reg<u16, _CRC16DIRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16DIRB;
#[doc = "`read()` method returns [crc16dirb::R](crc16dirb::R) reader structure"]
impl crate::Readable for CRC16DIRB {}
#[doc = "`write(|w| ..)` method takes [crc16dirb::W](crc16dirb::W) writer structure"]
impl crate::Writable for CRC16DIRB {}
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirb;
#[doc = "CRC16 Initialization and Result register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16inires](crc16inires) module"]
pub type CRC16INIRES = crate::Reg<u16, _CRC16INIRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16INIRES;
#[doc = "`read()` method returns [crc16inires::R](crc16inires::R) reader structure"]
impl crate::Readable for CRC16INIRES {}
#[doc = "`write(|w| ..)` method takes [crc16inires::W](crc16inires::W) writer structure"]
impl crate::Writable for CRC16INIRES {}
#[doc = "CRC16 Initialization and Result register"]
pub mod crc16inires;
#[doc = "CRC16 Result Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16resr](crc16resr) module"]
pub type CRC16RESR = crate::Reg<u16, _CRC16RESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16RESR;
#[doc = "`read()` method returns [crc16resr::R](crc16resr::R) reader structure"]
impl crate::Readable for CRC16RESR {}
#[doc = "`write(|w| ..)` method takes [crc16resr::W](crc16resr::W) writer structure"]
impl crate::Writable for CRC16RESR {}
#[doc = "CRC16 Result Reverse"]
pub mod crc16resr;
