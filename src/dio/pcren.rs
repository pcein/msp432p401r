#[doc = "Reader of register PCREN"]
pub type R = crate::R<u16, super::PCREN>;
#[doc = "Writer for register PCREN"]
pub type W = crate::W<u16, super::PCREN>;
#[doc = "Register PCREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PCREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5REN`"]
pub type P5REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5REN`"]
pub struct P5REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6REN`"]
pub type P6REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6REN`"]
pub struct P6REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&self) -> P5REN_R {
        P5REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&self) -> P6REN_R {
        P6REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&mut self) -> P5REN_W {
        P5REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&mut self) -> P6REN_W {
        P6REN_W { w: self }
    }
}
