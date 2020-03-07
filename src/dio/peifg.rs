#[doc = "Reader of register PEIFG"]
pub type R = crate::R<u16, super::PEIFG>;
#[doc = "Writer for register PEIFG"]
pub type W = crate::W<u16, super::PEIFG>;
#[doc = "Register PEIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PEIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9IFG`"]
pub type P9IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9IFG`"]
pub struct P9IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10IFG`"]
pub type P10IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10IFG`"]
pub struct P10IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&self) -> P9IFG_R {
        P9IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&self) -> P10IFG_R {
        P10IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&mut self) -> P9IFG_W {
        P9IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&mut self) -> P10IFG_W {
        P10IFG_W { w: self }
    }
}
