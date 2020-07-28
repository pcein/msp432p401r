#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Mapping Key Register"]
    pub pmapkeyid: PMAPKEYID,
    #[doc = "0x02 - Port Mapping Control Register"]
    pub pmapctl: PMAPCTL,
    _reserved2: [u8; 4usize],
    #[doc = "0x08 - Port mapping register, P1.0 and P1.1"]
    pub p1map01: P1MAP01,
    #[doc = "0x0a - Port mapping register, P1.2 and P1.3"]
    pub p1map23: P1MAP23,
    #[doc = "0x0c - Port mapping register, P1.4 and P1.5"]
    pub p1map45: P1MAP45,
    #[doc = "0x0e - Port mapping register, P1.6 and P1.7"]
    pub p1map67: P1MAP67,
    #[doc = "0x10 - Port mapping register, P2.0 and P2.1"]
    pub p2map01: P2MAP01,
    #[doc = "0x12 - Port mapping register, P2.2 and P2.3"]
    pub p2map23: P2MAP23,
    #[doc = "0x14 - Port mapping register, P2.4 and P2.5"]
    pub p2map45: P2MAP45,
    #[doc = "0x16 - Port mapping register, P2.6 and P2.7"]
    pub p2map67: P2MAP67,
    #[doc = "0x18 - Port mapping register, P3.0 and P3.1"]
    pub p3map01: P3MAP01,
    #[doc = "0x1a - Port mapping register, P3.2 and P3.3"]
    pub p3map23: P3MAP23,
    #[doc = "0x1c - Port mapping register, P3.4 and P3.5"]
    pub p3map45: P3MAP45,
    #[doc = "0x1e - Port mapping register, P3.6 and P3.7"]
    pub p3map67: P3MAP67,
    #[doc = "0x20 - Port mapping register, P4.0 and P4.1"]
    pub p4map01: P4MAP01,
    #[doc = "0x22 - Port mapping register, P4.2 and P4.3"]
    pub p4map23: P4MAP23,
    #[doc = "0x24 - Port mapping register, P4.4 and P4.5"]
    pub p4map45: P4MAP45,
    #[doc = "0x26 - Port mapping register, P4.6 and P4.7"]
    pub p4map67: P4MAP67,
    #[doc = "0x28 - Port mapping register, P5.0 and P5.1"]
    pub p5map01: P5MAP01,
    #[doc = "0x2a - Port mapping register, P5.2 and P5.3"]
    pub p5map23: P5MAP23,
    #[doc = "0x2c - Port mapping register, P5.4 and P5.5"]
    pub p5map45: P5MAP45,
    #[doc = "0x2e - Port mapping register, P5.6 and P5.7"]
    pub p5map67: P5MAP67,
    #[doc = "0x30 - Port mapping register, P6.0 and P6.1"]
    pub p6map01: P6MAP01,
    #[doc = "0x32 - Port mapping register, P6.2 and P6.3"]
    pub p6map23: P6MAP23,
    #[doc = "0x34 - Port mapping register, P6.4 and P6.5"]
    pub p6map45: P6MAP45,
    #[doc = "0x36 - Port mapping register, P6.6 and P6.7"]
    pub p6map67: P6MAP67,
    #[doc = "0x38 - Port mapping register, P7.0 and P7.1"]
    pub p7map01: P7MAP01,
    #[doc = "0x3a - Port mapping register, P7.2 and P7.3"]
    pub p7map23: P7MAP23,
    #[doc = "0x3c - Port mapping register, P7.4 and P7.5"]
    pub p7map45: P7MAP45,
    #[doc = "0x3e - Port mapping register, P7.6 and P7.7"]
    pub p7map67: P7MAP67,
}
#[doc = "Port Mapping Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapkeyid](pmapkeyid) module"]
pub type PMAPKEYID = crate::Reg<u16, _PMAPKEYID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAPKEYID;
#[doc = "`read()` method returns [pmapkeyid::R](pmapkeyid::R) reader structure"]
impl crate::Readable for PMAPKEYID {}
#[doc = "`write(|w| ..)` method takes [pmapkeyid::W](pmapkeyid::W) writer structure"]
impl crate::Writable for PMAPKEYID {}
#[doc = "Port Mapping Key Register"]
pub mod pmapkeyid;
#[doc = "Port Mapping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapctl](pmapctl) module"]
pub type PMAPCTL = crate::Reg<u16, _PMAPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAPCTL;
#[doc = "`read()` method returns [pmapctl::R](pmapctl::R) reader structure"]
impl crate::Readable for PMAPCTL {}
#[doc = "`write(|w| ..)` method takes [pmapctl::W](pmapctl::W) writer structure"]
impl crate::Writable for PMAPCTL {}
#[doc = "Port Mapping Control Register"]
pub mod pmapctl;
#[doc = "Port mapping register, P1.0 and P1.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1map01](p1map01) module"]
pub type P1MAP01 = crate::Reg<u16, _P1MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1MAP01;
#[doc = "`read()` method returns [p1map01::R](p1map01::R) reader structure"]
impl crate::Readable for P1MAP01 {}
#[doc = "`write(|w| ..)` method takes [p1map01::W](p1map01::W) writer structure"]
impl crate::Writable for P1MAP01 {}
#[doc = "Port mapping register, P1.0 and P1.1"]
pub mod p1map01;
#[doc = "Port mapping register, P1.2 and P1.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1map23](p1map23) module"]
pub type P1MAP23 = crate::Reg<u16, _P1MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1MAP23;
#[doc = "`read()` method returns [p1map23::R](p1map23::R) reader structure"]
impl crate::Readable for P1MAP23 {}
#[doc = "`write(|w| ..)` method takes [p1map23::W](p1map23::W) writer structure"]
impl crate::Writable for P1MAP23 {}
#[doc = "Port mapping register, P1.2 and P1.3"]
pub mod p1map23;
#[doc = "Port mapping register, P1.4 and P1.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1map45](p1map45) module"]
pub type P1MAP45 = crate::Reg<u16, _P1MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1MAP45;
#[doc = "`read()` method returns [p1map45::R](p1map45::R) reader structure"]
impl crate::Readable for P1MAP45 {}
#[doc = "`write(|w| ..)` method takes [p1map45::W](p1map45::W) writer structure"]
impl crate::Writable for P1MAP45 {}
#[doc = "Port mapping register, P1.4 and P1.5"]
pub mod p1map45;
#[doc = "Port mapping register, P1.6 and P1.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1map67](p1map67) module"]
pub type P1MAP67 = crate::Reg<u16, _P1MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1MAP67;
#[doc = "`read()` method returns [p1map67::R](p1map67::R) reader structure"]
impl crate::Readable for P1MAP67 {}
#[doc = "`write(|w| ..)` method takes [p1map67::W](p1map67::W) writer structure"]
impl crate::Writable for P1MAP67 {}
#[doc = "Port mapping register, P1.6 and P1.7"]
pub mod p1map67;
#[doc = "Port mapping register, P2.0 and P2.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2map01](p2map01) module"]
pub type P2MAP01 = crate::Reg<u16, _P2MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2MAP01;
#[doc = "`read()` method returns [p2map01::R](p2map01::R) reader structure"]
impl crate::Readable for P2MAP01 {}
#[doc = "`write(|w| ..)` method takes [p2map01::W](p2map01::W) writer structure"]
impl crate::Writable for P2MAP01 {}
#[doc = "Port mapping register, P2.0 and P2.1"]
pub mod p2map01;
#[doc = "Port mapping register, P2.2 and P2.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2map23](p2map23) module"]
pub type P2MAP23 = crate::Reg<u16, _P2MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2MAP23;
#[doc = "`read()` method returns [p2map23::R](p2map23::R) reader structure"]
impl crate::Readable for P2MAP23 {}
#[doc = "`write(|w| ..)` method takes [p2map23::W](p2map23::W) writer structure"]
impl crate::Writable for P2MAP23 {}
#[doc = "Port mapping register, P2.2 and P2.3"]
pub mod p2map23;
#[doc = "Port mapping register, P2.4 and P2.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2map45](p2map45) module"]
pub type P2MAP45 = crate::Reg<u16, _P2MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2MAP45;
#[doc = "`read()` method returns [p2map45::R](p2map45::R) reader structure"]
impl crate::Readable for P2MAP45 {}
#[doc = "`write(|w| ..)` method takes [p2map45::W](p2map45::W) writer structure"]
impl crate::Writable for P2MAP45 {}
#[doc = "Port mapping register, P2.4 and P2.5"]
pub mod p2map45;
#[doc = "Port mapping register, P2.6 and P2.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2map67](p2map67) module"]
pub type P2MAP67 = crate::Reg<u16, _P2MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2MAP67;
#[doc = "`read()` method returns [p2map67::R](p2map67::R) reader structure"]
impl crate::Readable for P2MAP67 {}
#[doc = "`write(|w| ..)` method takes [p2map67::W](p2map67::W) writer structure"]
impl crate::Writable for P2MAP67 {}
#[doc = "Port mapping register, P2.6 and P2.7"]
pub mod p2map67;
#[doc = "Port mapping register, P3.0 and P3.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3map01](p3map01) module"]
pub type P3MAP01 = crate::Reg<u16, _P3MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3MAP01;
#[doc = "`read()` method returns [p3map01::R](p3map01::R) reader structure"]
impl crate::Readable for P3MAP01 {}
#[doc = "`write(|w| ..)` method takes [p3map01::W](p3map01::W) writer structure"]
impl crate::Writable for P3MAP01 {}
#[doc = "Port mapping register, P3.0 and P3.1"]
pub mod p3map01;
#[doc = "Port mapping register, P3.2 and P3.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3map23](p3map23) module"]
pub type P3MAP23 = crate::Reg<u16, _P3MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3MAP23;
#[doc = "`read()` method returns [p3map23::R](p3map23::R) reader structure"]
impl crate::Readable for P3MAP23 {}
#[doc = "`write(|w| ..)` method takes [p3map23::W](p3map23::W) writer structure"]
impl crate::Writable for P3MAP23 {}
#[doc = "Port mapping register, P3.2 and P3.3"]
pub mod p3map23;
#[doc = "Port mapping register, P3.4 and P3.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3map45](p3map45) module"]
pub type P3MAP45 = crate::Reg<u16, _P3MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3MAP45;
#[doc = "`read()` method returns [p3map45::R](p3map45::R) reader structure"]
impl crate::Readable for P3MAP45 {}
#[doc = "`write(|w| ..)` method takes [p3map45::W](p3map45::W) writer structure"]
impl crate::Writable for P3MAP45 {}
#[doc = "Port mapping register, P3.4 and P3.5"]
pub mod p3map45;
#[doc = "Port mapping register, P3.6 and P3.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3map67](p3map67) module"]
pub type P3MAP67 = crate::Reg<u16, _P3MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3MAP67;
#[doc = "`read()` method returns [p3map67::R](p3map67::R) reader structure"]
impl crate::Readable for P3MAP67 {}
#[doc = "`write(|w| ..)` method takes [p3map67::W](p3map67::W) writer structure"]
impl crate::Writable for P3MAP67 {}
#[doc = "Port mapping register, P3.6 and P3.7"]
pub mod p3map67;
#[doc = "Port mapping register, P4.0 and P4.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map01](p4map01) module"]
pub type P4MAP01 = crate::Reg<u16, _P4MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4MAP01;
#[doc = "`read()` method returns [p4map01::R](p4map01::R) reader structure"]
impl crate::Readable for P4MAP01 {}
#[doc = "`write(|w| ..)` method takes [p4map01::W](p4map01::W) writer structure"]
impl crate::Writable for P4MAP01 {}
#[doc = "Port mapping register, P4.0 and P4.1"]
pub mod p4map01;
#[doc = "Port mapping register, P4.2 and P4.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map23](p4map23) module"]
pub type P4MAP23 = crate::Reg<u16, _P4MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4MAP23;
#[doc = "`read()` method returns [p4map23::R](p4map23::R) reader structure"]
impl crate::Readable for P4MAP23 {}
#[doc = "`write(|w| ..)` method takes [p4map23::W](p4map23::W) writer structure"]
impl crate::Writable for P4MAP23 {}
#[doc = "Port mapping register, P4.2 and P4.3"]
pub mod p4map23;
#[doc = "Port mapping register, P4.4 and P4.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map45](p4map45) module"]
pub type P4MAP45 = crate::Reg<u16, _P4MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4MAP45;
#[doc = "`read()` method returns [p4map45::R](p4map45::R) reader structure"]
impl crate::Readable for P4MAP45 {}
#[doc = "`write(|w| ..)` method takes [p4map45::W](p4map45::W) writer structure"]
impl crate::Writable for P4MAP45 {}
#[doc = "Port mapping register, P4.4 and P4.5"]
pub mod p4map45;
#[doc = "Port mapping register, P4.6 and P4.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map67](p4map67) module"]
pub type P4MAP67 = crate::Reg<u16, _P4MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4MAP67;
#[doc = "`read()` method returns [p4map67::R](p4map67::R) reader structure"]
impl crate::Readable for P4MAP67 {}
#[doc = "`write(|w| ..)` method takes [p4map67::W](p4map67::W) writer structure"]
impl crate::Writable for P4MAP67 {}
#[doc = "Port mapping register, P4.6 and P4.7"]
pub mod p4map67;
#[doc = "Port mapping register, P5.0 and P5.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5map01](p5map01) module"]
pub type P5MAP01 = crate::Reg<u16, _P5MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5MAP01;
#[doc = "`read()` method returns [p5map01::R](p5map01::R) reader structure"]
impl crate::Readable for P5MAP01 {}
#[doc = "`write(|w| ..)` method takes [p5map01::W](p5map01::W) writer structure"]
impl crate::Writable for P5MAP01 {}
#[doc = "Port mapping register, P5.0 and P5.1"]
pub mod p5map01;
#[doc = "Port mapping register, P5.2 and P5.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5map23](p5map23) module"]
pub type P5MAP23 = crate::Reg<u16, _P5MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5MAP23;
#[doc = "`read()` method returns [p5map23::R](p5map23::R) reader structure"]
impl crate::Readable for P5MAP23 {}
#[doc = "`write(|w| ..)` method takes [p5map23::W](p5map23::W) writer structure"]
impl crate::Writable for P5MAP23 {}
#[doc = "Port mapping register, P5.2 and P5.3"]
pub mod p5map23;
#[doc = "Port mapping register, P5.4 and P5.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5map45](p5map45) module"]
pub type P5MAP45 = crate::Reg<u16, _P5MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5MAP45;
#[doc = "`read()` method returns [p5map45::R](p5map45::R) reader structure"]
impl crate::Readable for P5MAP45 {}
#[doc = "`write(|w| ..)` method takes [p5map45::W](p5map45::W) writer structure"]
impl crate::Writable for P5MAP45 {}
#[doc = "Port mapping register, P5.4 and P5.5"]
pub mod p5map45;
#[doc = "Port mapping register, P5.6 and P5.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5map67](p5map67) module"]
pub type P5MAP67 = crate::Reg<u16, _P5MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5MAP67;
#[doc = "`read()` method returns [p5map67::R](p5map67::R) reader structure"]
impl crate::Readable for P5MAP67 {}
#[doc = "`write(|w| ..)` method takes [p5map67::W](p5map67::W) writer structure"]
impl crate::Writable for P5MAP67 {}
#[doc = "Port mapping register, P5.6 and P5.7"]
pub mod p5map67;
#[doc = "Port mapping register, P6.0 and P6.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6map01](p6map01) module"]
pub type P6MAP01 = crate::Reg<u16, _P6MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6MAP01;
#[doc = "`read()` method returns [p6map01::R](p6map01::R) reader structure"]
impl crate::Readable for P6MAP01 {}
#[doc = "`write(|w| ..)` method takes [p6map01::W](p6map01::W) writer structure"]
impl crate::Writable for P6MAP01 {}
#[doc = "Port mapping register, P6.0 and P6.1"]
pub mod p6map01;
#[doc = "Port mapping register, P6.2 and P6.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6map23](p6map23) module"]
pub type P6MAP23 = crate::Reg<u16, _P6MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6MAP23;
#[doc = "`read()` method returns [p6map23::R](p6map23::R) reader structure"]
impl crate::Readable for P6MAP23 {}
#[doc = "`write(|w| ..)` method takes [p6map23::W](p6map23::W) writer structure"]
impl crate::Writable for P6MAP23 {}
#[doc = "Port mapping register, P6.2 and P6.3"]
pub mod p6map23;
#[doc = "Port mapping register, P6.4 and P6.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6map45](p6map45) module"]
pub type P6MAP45 = crate::Reg<u16, _P6MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6MAP45;
#[doc = "`read()` method returns [p6map45::R](p6map45::R) reader structure"]
impl crate::Readable for P6MAP45 {}
#[doc = "`write(|w| ..)` method takes [p6map45::W](p6map45::W) writer structure"]
impl crate::Writable for P6MAP45 {}
#[doc = "Port mapping register, P6.4 and P6.5"]
pub mod p6map45;
#[doc = "Port mapping register, P6.6 and P6.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6map67](p6map67) module"]
pub type P6MAP67 = crate::Reg<u16, _P6MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6MAP67;
#[doc = "`read()` method returns [p6map67::R](p6map67::R) reader structure"]
impl crate::Readable for P6MAP67 {}
#[doc = "`write(|w| ..)` method takes [p6map67::W](p6map67::W) writer structure"]
impl crate::Writable for P6MAP67 {}
#[doc = "Port mapping register, P6.6 and P6.7"]
pub mod p6map67;
#[doc = "Port mapping register, P7.0 and P7.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7map01](p7map01) module"]
pub type P7MAP01 = crate::Reg<u16, _P7MAP01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7MAP01;
#[doc = "`read()` method returns [p7map01::R](p7map01::R) reader structure"]
impl crate::Readable for P7MAP01 {}
#[doc = "`write(|w| ..)` method takes [p7map01::W](p7map01::W) writer structure"]
impl crate::Writable for P7MAP01 {}
#[doc = "Port mapping register, P7.0 and P7.1"]
pub mod p7map01;
#[doc = "Port mapping register, P7.2 and P7.3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7map23](p7map23) module"]
pub type P7MAP23 = crate::Reg<u16, _P7MAP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7MAP23;
#[doc = "`read()` method returns [p7map23::R](p7map23::R) reader structure"]
impl crate::Readable for P7MAP23 {}
#[doc = "`write(|w| ..)` method takes [p7map23::W](p7map23::W) writer structure"]
impl crate::Writable for P7MAP23 {}
#[doc = "Port mapping register, P7.2 and P7.3"]
pub mod p7map23;
#[doc = "Port mapping register, P7.4 and P7.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7map45](p7map45) module"]
pub type P7MAP45 = crate::Reg<u16, _P7MAP45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7MAP45;
#[doc = "`read()` method returns [p7map45::R](p7map45::R) reader structure"]
impl crate::Readable for P7MAP45 {}
#[doc = "`write(|w| ..)` method takes [p7map45::W](p7map45::W) writer structure"]
impl crate::Writable for P7MAP45 {}
#[doc = "Port mapping register, P7.4 and P7.5"]
pub mod p7map45;
#[doc = "Port mapping register, P7.6 and P7.7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7map67](p7map67) module"]
pub type P7MAP67 = crate::Reg<u16, _P7MAP67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7MAP67;
#[doc = "`read()` method returns [p7map67::R](p7map67::R) reader structure"]
impl crate::Readable for P7MAP67 {}
#[doc = "`write(|w| ..)` method takes [p7map67::W](p7map67::W) writer structure"]
impl crate::Writable for P7MAP67 {}
#[doc = "Port mapping register, P7.6 and P7.7"]
pub mod p7map67;
