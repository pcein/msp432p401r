#[doc = "Reader of register PBREN"]
pub type R = crate::R<u16, super::PBREN>;
#[doc = "Writer for register PBREN"]
pub type W = crate::W<u16, super::PBREN>;
#[doc = "Register PBREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PBREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3REN`"]
pub type P3REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3REN`"]
pub struct P3REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4REN`"]
pub type P4REN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4REN`"]
pub struct P4REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&self) -> P3REN_R {
        P3REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&self) -> P4REN_R {
        P4REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&mut self) -> P3REN_W {
        P3REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&mut self) -> P4REN_W {
        P4REN_W { w: self }
    }
}
