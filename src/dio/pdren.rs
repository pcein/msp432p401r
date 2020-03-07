#[doc = "Reader of register PDREN"]
pub type R = crate::R<u16, super::PDREN>;
#[doc = "Writer for register PDREN"]
pub type W = crate::W<u16, super::PDREN>;
#[doc = "Register PDREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PDREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7REN`"]
pub type P7REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7REN`"]
pub struct P7REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8REN`"]
pub type P8REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8REN`"]
pub struct P8REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&self) -> P7REN_R {
        P7REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&self) -> P8REN_R {
        P8REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&mut self) -> P7REN_W {
        P7REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&mut self) -> P8REN_W {
        P8REN_W { w: self }
    }
}
