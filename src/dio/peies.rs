#[doc = "Reader of register PEIES"]
pub type R = crate::R<u16, super::PEIES>;
#[doc = "Writer for register PEIES"]
pub type W = crate::W<u16, super::PEIES>;
#[doc = "Register PEIES `reset()`'s with value 0"]
impl crate::ResetValue for super::PEIES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9IES`"]
pub type P9IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9IES`"]
pub struct P9IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10IES`"]
pub type P10IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10IES`"]
pub struct P10IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&self) -> P9IES_R {
        P9IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&self) -> P10IES_R {
        P10IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&mut self) -> P9IES_W {
        P9IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&mut self) -> P10IES_W {
        P10IES_W { w: self }
    }
}
