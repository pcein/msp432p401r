#[doc = "Reader of register FLCTL_RDBRST_FAILCNT"]
pub type R = crate::R<u32, super::FLCTL_RDBRST_FAILCNT>;
#[doc = "Writer for register FLCTL_RDBRST_FAILCNT"]
pub type W = crate::W<u32, super::FLCTL_RDBRST_FAILCNT>;
#[doc = "Register FLCTL_RDBRST_FAILCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_RDBRST_FAILCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAIL_COUNT`"]
pub type FAIL_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FAIL_COUNT`"]
pub struct FAIL_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&self) -> FAIL_COUNT_R {
        FAIL_COUNT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&mut self) -> FAIL_COUNT_W {
        FAIL_COUNT_W { w: self }
    }
}
