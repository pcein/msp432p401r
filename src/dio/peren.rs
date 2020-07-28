#[doc = "Reader of register PEREN"]
pub type R = crate::R<u16, super::PEREN>;
#[doc = "Writer for register PEREN"]
pub type W = crate::W<u16, super::PEREN>;
#[doc = "Register PEREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PEREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9REN`"]
pub type P9REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9REN`"]
pub struct P9REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10REN`"]
pub type P10REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10REN`"]
pub struct P10REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P10REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&self) -> P9REN_R {
        P9REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&self) -> P10REN_R {
        P10REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&mut self) -> P9REN_W {
        P9REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&mut self) -> P10REN_W {
        P10REN_W { w: self }
    }
}
