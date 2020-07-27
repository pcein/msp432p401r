#[doc = "Reader of register DMA_INT1_SRCCFG"]
pub type R = crate::R<u32, super::DMA_INT1_SRCCFG>;
#[doc = "Writer for register DMA_INT1_SRCCFG"]
pub type W = crate::W<u32, super::DMA_INT1_SRCCFG>;
#[doc = "Register DMA_INT1_SRCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT1_SRCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_SRC`"]
pub type INT_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_SRC`"]
pub struct INT_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&self) -> INT_SRC_R {
        INT_SRC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enables DMA_INT1 mapping"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&mut self) -> INT_SRC_W {
        INT_SRC_W { w: self }
    }
    #[doc = "Bit 5 - Enables DMA_INT1 mapping"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
