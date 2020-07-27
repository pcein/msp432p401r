#[doc = "Reader of register UCBxBRW"]
pub type R = crate::R<u16, super::UCBXBRW>;
#[doc = "Writer for register UCBxBRW"]
pub type W = crate::W<u16, super::UCBXBRW>;
#[doc = "Register UCBxBRW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCBXBRW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCBR`"]
pub type UCBR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UCBR`"]
pub struct UCBR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&self) -> UCBR_R {
        UCBR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&mut self) -> UCBR_W {
        UCBR_W { w: self }
    }
}
