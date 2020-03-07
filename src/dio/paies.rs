#[doc = "Reader of register PAIES"]
pub type R = crate::R<u16, super::PAIES>;
#[doc = "Writer for register PAIES"]
pub type W = crate::W<u16, super::PAIES>;
#[doc = "Register PAIES `reset()`'s with value 0"]
impl crate::ResetValue for super::PAIES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IES`"]
pub type P1IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1IES`"]
pub struct P1IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2IES`"]
pub type P2IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2IES`"]
pub struct P2IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&self) -> P1IES_R {
        P1IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&self) -> P2IES_R {
        P2IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&mut self) -> P1IES_W {
        P1IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&mut self) -> P2IES_W {
        P2IES_W { w: self }
    }
}
