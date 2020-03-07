#[doc = "Reader of register FLCTL_RDBRST_LEN"]
pub type R = crate::R<u32, super::FLCTL_RDBRST_LEN>;
#[doc = "Writer for register FLCTL_RDBRST_LEN"]
pub type W = crate::W<u32, super::FLCTL_RDBRST_LEN>;
#[doc = "Register FLCTL_RDBRST_LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_RDBRST_LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BURST_LENGTH`"]
pub type BURST_LENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BURST_LENGTH`"]
pub struct BURST_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&self) -> BURST_LENGTH_R {
        BURST_LENGTH_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&mut self) -> BURST_LENGTH_W {
        BURST_LENGTH_W { w: self }
    }
}
