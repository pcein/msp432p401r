#[doc = "Reader of register PAIFG"]
pub type R = crate::R<u16, super::PAIFG>;
#[doc = "Writer for register PAIFG"]
pub type W = crate::W<u16, super::PAIFG>;
#[doc = "Register PAIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PAIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IFG`"]
pub type P1IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1IFG`"]
pub struct P1IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2IFG`"]
pub type P2IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2IFG`"]
pub struct P2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&self) -> P1IFG_R {
        P1IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&self) -> P2IFG_R {
        P2IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&mut self) -> P1IFG_W {
        P1IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&mut self) -> P2IFG_W {
        P2IFG_W { w: self }
    }
}
