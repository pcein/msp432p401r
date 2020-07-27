#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 1 Load Register"]
    pub t32load1: T32LOAD1,
    #[doc = "0x04 - Timer 1 Current Value Register"]
    pub t32value1: T32VALUE1,
    #[doc = "0x08 - Timer 1 Timer Control Register"]
    pub t32control1: T32CONTROL1,
    #[doc = "0x0c - Timer 1 Interrupt Clear Register"]
    pub t32intclr1: T32INTCLR1,
    #[doc = "0x10 - Timer 1 Raw Interrupt Status Register"]
    pub t32ris1: T32RIS1,
    #[doc = "0x14 - Timer 1 Interrupt Status Register"]
    pub t32mis1: T32MIS1,
    #[doc = "0x18 - Timer 1 Background Load Register"]
    pub t32bgload1: T32BGLOAD1,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Timer 2 Load Register"]
    pub t32load2: T32LOAD2,
    #[doc = "0x24 - Timer 2 Current Value Register"]
    pub t32value2: T32VALUE2,
    #[doc = "0x28 - Timer 2 Timer Control Register"]
    pub t32control2: T32CONTROL2,
    #[doc = "0x2c - Timer 2 Interrupt Clear Register"]
    pub t32intclr2: T32INTCLR2,
    #[doc = "0x30 - Timer 2 Raw Interrupt Status Register"]
    pub t32ris2: T32RIS2,
    #[doc = "0x34 - Timer 2 Interrupt Status Register"]
    pub t32mis2: T32MIS2,
    #[doc = "0x38 - Timer 2 Background Load Register"]
    pub t32bgload2: T32BGLOAD2,
}
#[doc = "Timer 1 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32load1](t32load1) module"]
pub type T32LOAD1 = crate::Reg<u32, _T32LOAD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32LOAD1;
#[doc = "`read()` method returns [t32load1::R](t32load1::R) reader structure"]
impl crate::Readable for T32LOAD1 {}
#[doc = "`write(|w| ..)` method takes [t32load1::W](t32load1::W) writer structure"]
impl crate::Writable for T32LOAD1 {}
#[doc = "Timer 1 Load Register"]
pub mod t32load1;
#[doc = "Timer 1 Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32value1](t32value1) module"]
pub type T32VALUE1 = crate::Reg<u32, _T32VALUE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32VALUE1;
#[doc = "`read()` method returns [t32value1::R](t32value1::R) reader structure"]
impl crate::Readable for T32VALUE1 {}
#[doc = "Timer 1 Current Value Register"]
pub mod t32value1;
#[doc = "Timer 1 Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32control1](t32control1) module"]
pub type T32CONTROL1 = crate::Reg<u32, _T32CONTROL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32CONTROL1;
#[doc = "`read()` method returns [t32control1::R](t32control1::R) reader structure"]
impl crate::Readable for T32CONTROL1 {}
#[doc = "`write(|w| ..)` method takes [t32control1::W](t32control1::W) writer structure"]
impl crate::Writable for T32CONTROL1 {}
#[doc = "Timer 1 Timer Control Register"]
pub mod t32control1;
#[doc = "Timer 1 Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32intclr1](t32intclr1) module"]
pub type T32INTCLR1 = crate::Reg<u32, _T32INTCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32INTCLR1;
#[doc = "`write(|w| ..)` method takes [t32intclr1::W](t32intclr1::W) writer structure"]
impl crate::Writable for T32INTCLR1 {}
#[doc = "Timer 1 Interrupt Clear Register"]
pub mod t32intclr1;
#[doc = "Timer 1 Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32ris1](t32ris1) module"]
pub type T32RIS1 = crate::Reg<u32, _T32RIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32RIS1;
#[doc = "`read()` method returns [t32ris1::R](t32ris1::R) reader structure"]
impl crate::Readable for T32RIS1 {}
#[doc = "Timer 1 Raw Interrupt Status Register"]
pub mod t32ris1;
#[doc = "Timer 1 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32mis1](t32mis1) module"]
pub type T32MIS1 = crate::Reg<u32, _T32MIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32MIS1;
#[doc = "`read()` method returns [t32mis1::R](t32mis1::R) reader structure"]
impl crate::Readable for T32MIS1 {}
#[doc = "Timer 1 Interrupt Status Register"]
pub mod t32mis1;
#[doc = "Timer 1 Background Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32bgload1](t32bgload1) module"]
pub type T32BGLOAD1 = crate::Reg<u32, _T32BGLOAD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32BGLOAD1;
#[doc = "`read()` method returns [t32bgload1::R](t32bgload1::R) reader structure"]
impl crate::Readable for T32BGLOAD1 {}
#[doc = "`write(|w| ..)` method takes [t32bgload1::W](t32bgload1::W) writer structure"]
impl crate::Writable for T32BGLOAD1 {}
#[doc = "Timer 1 Background Load Register"]
pub mod t32bgload1;
#[doc = "Timer 2 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32load2](t32load2) module"]
pub type T32LOAD2 = crate::Reg<u32, _T32LOAD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32LOAD2;
#[doc = "`read()` method returns [t32load2::R](t32load2::R) reader structure"]
impl crate::Readable for T32LOAD2 {}
#[doc = "`write(|w| ..)` method takes [t32load2::W](t32load2::W) writer structure"]
impl crate::Writable for T32LOAD2 {}
#[doc = "Timer 2 Load Register"]
pub mod t32load2;
#[doc = "Timer 2 Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32value2](t32value2) module"]
pub type T32VALUE2 = crate::Reg<u32, _T32VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32VALUE2;
#[doc = "`read()` method returns [t32value2::R](t32value2::R) reader structure"]
impl crate::Readable for T32VALUE2 {}
#[doc = "Timer 2 Current Value Register"]
pub mod t32value2;
#[doc = "Timer 2 Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32control2](t32control2) module"]
pub type T32CONTROL2 = crate::Reg<u32, _T32CONTROL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32CONTROL2;
#[doc = "`read()` method returns [t32control2::R](t32control2::R) reader structure"]
impl crate::Readable for T32CONTROL2 {}
#[doc = "`write(|w| ..)` method takes [t32control2::W](t32control2::W) writer structure"]
impl crate::Writable for T32CONTROL2 {}
#[doc = "Timer 2 Timer Control Register"]
pub mod t32control2;
#[doc = "Timer 2 Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32intclr2](t32intclr2) module"]
pub type T32INTCLR2 = crate::Reg<u32, _T32INTCLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32INTCLR2;
#[doc = "`write(|w| ..)` method takes [t32intclr2::W](t32intclr2::W) writer structure"]
impl crate::Writable for T32INTCLR2 {}
#[doc = "Timer 2 Interrupt Clear Register"]
pub mod t32intclr2;
#[doc = "Timer 2 Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32ris2](t32ris2) module"]
pub type T32RIS2 = crate::Reg<u32, _T32RIS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32RIS2;
#[doc = "`read()` method returns [t32ris2::R](t32ris2::R) reader structure"]
impl crate::Readable for T32RIS2 {}
#[doc = "Timer 2 Raw Interrupt Status Register"]
pub mod t32ris2;
#[doc = "Timer 2 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32mis2](t32mis2) module"]
pub type T32MIS2 = crate::Reg<u32, _T32MIS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32MIS2;
#[doc = "`read()` method returns [t32mis2::R](t32mis2::R) reader structure"]
impl crate::Readable for T32MIS2 {}
#[doc = "Timer 2 Interrupt Status Register"]
pub mod t32mis2;
#[doc = "Timer 2 Background Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32bgload2](t32bgload2) module"]
pub type T32BGLOAD2 = crate::Reg<u32, _T32BGLOAD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T32BGLOAD2;
#[doc = "`read()` method returns [t32bgload2::R](t32bgload2::R) reader structure"]
impl crate::Readable for T32BGLOAD2 {}
#[doc = "`write(|w| ..)` method takes [t32bgload2::W](t32bgload2::W) writer structure"]
impl crate::Writable for T32BGLOAD2 {}
#[doc = "Timer 2 Background Load Register"]
pub mod t32bgload2;
