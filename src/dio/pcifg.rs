#[doc = "Reader of register PCIFG"]
pub type R = crate::R<u16, super::PCIFG>;
#[doc = "Writer for register PCIFG"]
pub type W = crate::W<u16, super::PCIFG>;
#[doc = "Register PCIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5IFG`"]
pub type P5IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5IFG`"]
pub struct P5IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6IFG`"]
pub type P6IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6IFG`"]
pub struct P6IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&self) -> P5IFG_R {
        P5IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&self) -> P6IFG_R {
        P6IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&mut self) -> P5IFG_W {
        P5IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&mut self) -> P6IFG_W {
        P6IFG_W { w: self }
    }
}
