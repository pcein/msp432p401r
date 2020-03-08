#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    pub rtcctl0: RTCCTL0,
    #[doc = "0x02 - RTCCTL13 Register"]
    pub rtcctl13: RTCCTL13,
    #[doc = "0x04 - RTCOCAL Register"]
    pub rtcocal: RTCOCAL,
    #[doc = "0x06 - RTCTCMP Register"]
    pub rtctcmp: RTCTCMP,
    #[doc = "0x08 - Real-Time Clock Prescale Timer 0 Control Register"]
    pub rtcps0ctl: RTCPS0CTL,
    #[doc = "0x0a - Real-Time Clock Prescale Timer 1 Control Register"]
    pub rtcps1ctl: RTCPS1CTL,
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    pub rtcps: RTCPS,
    #[doc = "0x0e - Real-Time Clock Interrupt Vector Register"]
    pub rtciv: RTCIV,
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    pub rtctim0: RTCTIM0,
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    pub rtctim1: RTCTIM1,
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    pub rtcdate: RTCDATE,
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    pub rtcyear: RTCYEAR,
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    pub rtcaminhr: RTCAMINHR,
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    pub rtcadowday: RTCADOWDAY,
    #[doc = "0x1c - Binary-to-BCD Conversion Register"]
    pub rtcbin2bcd: RTCBIN2BCD,
    #[doc = "0x1e - BCD-to-Binary Conversion Register"]
    pub rtcbcd2bin: RTCBCD2BIN,
}
#[doc = "RTCCTL0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl0](rtcctl0) module"]
pub type RTCCTL0 = crate::Reg<u16, _RTCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL0;
#[doc = "`read()` method returns [rtcctl0::R](rtcctl0::R) reader structure"]
impl crate::Readable for RTCCTL0 {}
#[doc = "`write(|w| ..)` method takes [rtcctl0::W](rtcctl0::W) writer structure"]
impl crate::Writable for RTCCTL0 {}
#[doc = "RTCCTL0 Register"]
pub mod rtcctl0;
#[doc = "RTCCTL13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl13](rtcctl13) module"]
pub type RTCCTL13 = crate::Reg<u16, _RTCCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL13;
#[doc = "`read()` method returns [rtcctl13::R](rtcctl13::R) reader structure"]
impl crate::Readable for RTCCTL13 {}
#[doc = "`write(|w| ..)` method takes [rtcctl13::W](rtcctl13::W) writer structure"]
impl crate::Writable for RTCCTL13 {}
#[doc = "RTCCTL13 Register"]
pub mod rtcctl13;
#[doc = "RTCOCAL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](rtcocal) module"]
pub type RTCOCAL = crate::Reg<u16, _RTCOCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCOCAL;
#[doc = "`read()` method returns [rtcocal::R](rtcocal::R) reader structure"]
impl crate::Readable for RTCOCAL {}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](rtcocal::W) writer structure"]
impl crate::Writable for RTCOCAL {}
#[doc = "RTCOCAL Register"]
pub mod rtcocal;
#[doc = "RTCTCMP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctcmp](rtctcmp) module"]
pub type RTCTCMP = crate::Reg<u16, _RTCTCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTCMP;
#[doc = "`read()` method returns [rtctcmp::R](rtctcmp::R) reader structure"]
impl crate::Readable for RTCTCMP {}
#[doc = "`write(|w| ..)` method takes [rtctcmp::W](rtctcmp::W) writer structure"]
impl crate::Writable for RTCTCMP {}
#[doc = "RTCTCMP Register"]
pub mod rtctcmp;
#[doc = "Real-Time Clock Prescale Timer 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps0ctl](rtcps0ctl) module"]
pub type RTCPS0CTL = crate::Reg<u16, _RTCPS0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS0CTL;
#[doc = "`read()` method returns [rtcps0ctl::R](rtcps0ctl::R) reader structure"]
impl crate::Readable for RTCPS0CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps0ctl::W](rtcps0ctl::W) writer structure"]
impl crate::Writable for RTCPS0CTL {}
#[doc = "Real-Time Clock Prescale Timer 0 Control Register"]
pub mod rtcps0ctl;
#[doc = "Real-Time Clock Prescale Timer 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](rtcps1ctl) module"]
pub type RTCPS1CTL = crate::Reg<u16, _RTCPS1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS1CTL;
#[doc = "`read()` method returns [rtcps1ctl::R](rtcps1ctl::R) reader structure"]
impl crate::Readable for RTCPS1CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](rtcps1ctl::W) writer structure"]
impl crate::Writable for RTCPS1CTL {}
#[doc = "Real-Time Clock Prescale Timer 1 Control Register"]
pub mod rtcps1ctl;
#[doc = "Real-Time Clock Prescale Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps](rtcps) module"]
pub type RTCPS = crate::Reg<u16, _RTCPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS;
#[doc = "`read()` method returns [rtcps::R](rtcps::R) reader structure"]
impl crate::Readable for RTCPS {}
#[doc = "`write(|w| ..)` method takes [rtcps::W](rtcps::W) writer structure"]
impl crate::Writable for RTCPS {}
#[doc = "Real-Time Clock Prescale Timer Counter Register"]
pub mod rtcps;
#[doc = "Real-Time Clock Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](rtciv) module"]
pub type RTCIV = crate::Reg<u16, _RTCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCIV;
#[doc = "`read()` method returns [rtciv::R](rtciv::R) reader structure"]
impl crate::Readable for RTCIV {}
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCTIM0 Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim0](rtctim0) module"]
pub type RTCTIM0 = crate::Reg<u16, _RTCTIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM0;
#[doc = "`read()` method returns [rtctim0::R](rtctim0::R) reader structure"]
impl crate::Readable for RTCTIM0 {}
#[doc = "`write(|w| ..)` method takes [rtctim0::W](rtctim0::W) writer structure"]
impl crate::Writable for RTCTIM0 {}
#[doc = "RTCTIM0 Register Hexadecimal Format"]
pub mod rtctim0;
#[doc = "Real-Time Clock Hour, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim1](rtctim1) module"]
pub type RTCTIM1 = crate::Reg<u16, _RTCTIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM1;
#[doc = "`read()` method returns [rtctim1::R](rtctim1::R) reader structure"]
impl crate::Readable for RTCTIM1 {}
#[doc = "`write(|w| ..)` method takes [rtctim1::W](rtctim1::W) writer structure"]
impl crate::Writable for RTCTIM1 {}
#[doc = "Real-Time Clock Hour, Day of Week"]
pub mod rtctim1;
#[doc = "RTCDATE - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdate](rtcdate) module"]
pub type RTCDATE = crate::Reg<u16, _RTCDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATE;
#[doc = "`read()` method returns [rtcdate::R](rtcdate::R) reader structure"]
impl crate::Readable for RTCDATE {}
#[doc = "`write(|w| ..)` method takes [rtcdate::W](rtcdate::W) writer structure"]
impl crate::Writable for RTCDATE {}
#[doc = "RTCDATE - Hexadecimal Format"]
pub mod rtcdate;
#[doc = "RTCYEAR Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcyear](rtcyear) module"]
pub type RTCYEAR = crate::Reg<u16, _RTCYEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCYEAR;
#[doc = "`read()` method returns [rtcyear::R](rtcyear::R) reader structure"]
impl crate::Readable for RTCYEAR {}
#[doc = "`write(|w| ..)` method takes [rtcyear::W](rtcyear::W) writer structure"]
impl crate::Writable for RTCYEAR {}
#[doc = "RTCYEAR Register Hexadecimal Format"]
pub mod rtcyear;
#[doc = "RTCMINHR - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaminhr](rtcaminhr) module"]
pub type RTCAMINHR = crate::Reg<u16, _RTCAMINHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCAMINHR;
#[doc = "`read()` method returns [rtcaminhr::R](rtcaminhr::R) reader structure"]
impl crate::Readable for RTCAMINHR {}
#[doc = "`write(|w| ..)` method takes [rtcaminhr::W](rtcaminhr::W) writer structure"]
impl crate::Writable for RTCAMINHR {}
#[doc = "RTCMINHR - Hexadecimal Format"]
pub mod rtcaminhr;
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadowday](rtcadowday) module"]
pub type RTCADOWDAY = crate::Reg<u16, _RTCADOWDAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCADOWDAY;
#[doc = "`read()` method returns [rtcadowday::R](rtcadowday::R) reader structure"]
impl crate::Readable for RTCADOWDAY {}
#[doc = "`write(|w| ..)` method takes [rtcadowday::W](rtcadowday::W) writer structure"]
impl crate::Writable for RTCADOWDAY {}
#[doc = "RTCADOWDAY - Hexadecimal Format"]
pub mod rtcadowday;
#[doc = "Binary-to-BCD Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcbin2bcd](rtcbin2bcd) module"]
pub type RTCBIN2BCD = crate::Reg<u16, _RTCBIN2BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCBIN2BCD;
#[doc = "`read()` method returns [rtcbin2bcd::R](rtcbin2bcd::R) reader structure"]
impl crate::Readable for RTCBIN2BCD {}
#[doc = "`write(|w| ..)` method takes [rtcbin2bcd::W](rtcbin2bcd::W) writer structure"]
impl crate::Writable for RTCBIN2BCD {}
#[doc = "Binary-to-BCD Conversion Register"]
pub mod rtcbin2bcd;
#[doc = "BCD-to-Binary Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcbcd2bin](rtcbcd2bin) module"]
pub type RTCBCD2BIN = crate::Reg<u16, _RTCBCD2BIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCBCD2BIN;
#[doc = "`read()` method returns [rtcbcd2bin::R](rtcbcd2bin::R) reader structure"]
impl crate::Readable for RTCBCD2BIN {}
#[doc = "`write(|w| ..)` method takes [rtcbcd2bin::W](rtcbcd2bin::W) writer structure"]
impl crate::Writable for RTCBCD2BIN {}
#[doc = "BCD-to-Binary Conversion Register"]
pub mod rtcbcd2bin;
