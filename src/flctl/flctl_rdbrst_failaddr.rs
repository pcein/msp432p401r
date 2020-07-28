#[doc = "Reader of register FLCTL_RDBRST_FAILADDR"]
pub type R = crate::R<u32, super::FLCTL_RDBRST_FAILADDR>;
#[doc = "Writer for register FLCTL_RDBRST_FAILADDR"]
pub type W = crate::W<u32, super::FLCTL_RDBRST_FAILADDR>;
#[doc = "Register FLCTL_RDBRST_FAILADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_RDBRST_FAILADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAIL_ADDRESS`"]
pub type FAIL_ADDRESS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FAIL_ADDRESS`"]
pub struct FAIL_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Reflects address of last failed compare"]
    #[inline(always)]
    pub fn fail_address(&self) -> FAIL_ADDRESS_R {
        FAIL_ADDRESS_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Reflects address of last failed compare"]
    #[inline(always)]
    pub fn fail_address(&mut self) -> FAIL_ADDRESS_W {
        FAIL_ADDRESS_W { w: self }
    }
}
