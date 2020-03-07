#[doc = "Reader of register PAREN"]
pub type R = crate::R<u16, super::PAREN>;
#[doc = "Writer for register PAREN"]
pub type W = crate::W<u16, super::PAREN>;
#[doc = "Register PAREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PAREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1REN`"]
pub type P1REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1REN`"]
pub struct P1REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2REN`"]
pub type P2REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2REN`"]
pub struct P2REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&self) -> P1REN_R {
        P1REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&self) -> P2REN_R {
        P2REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&mut self) -> P1REN_W {
        P1REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&mut self) -> P2REN_W {
        P2REN_W { w: self }
    }
}
