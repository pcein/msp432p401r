#[doc = "Reader of register PASEL0"]
pub type R = crate::R<u16, super::PASEL0>;
#[doc = "Writer for register PASEL0"]
pub type W = crate::W<u16, super::PASEL0>;
#[doc = "Register PASEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PASEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SEL0`"]
pub type P1SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1SEL0`"]
pub struct P1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2SEL0`"]
pub type P2SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2SEL0`"]
pub struct P2SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Select 0"]
    #[inline(always)]
    pub fn p1sel0(&self) -> P1SEL0_R {
        P1SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Select 0"]
    #[inline(always)]
    pub fn p2sel0(&self) -> P2SEL0_R {
        P2SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Select 0"]
    #[inline(always)]
    pub fn p1sel0(&mut self) -> P1SEL0_W {
        P1SEL0_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Select 0"]
    #[inline(always)]
    pub fn p2sel0(&mut self) -> P2SEL0_W {
        P2SEL0_W { w: self }
    }
}
