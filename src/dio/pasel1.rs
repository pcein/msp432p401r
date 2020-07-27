#[doc = "Reader of register PASEL1"]
pub type R = crate::R<u16, super::PASEL1>;
#[doc = "Writer for register PASEL1"]
pub type W = crate::W<u16, super::PASEL1>;
#[doc = "Register PASEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PASEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SEL1`"]
pub type P1SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1SEL1`"]
pub struct P1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2SEL1`"]
pub type P2SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2SEL1`"]
pub struct P2SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Select 1"]
    #[inline(always)]
    pub fn p1sel1(&self) -> P1SEL1_R {
        P1SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Select 1"]
    #[inline(always)]
    pub fn p2sel1(&self) -> P2SEL1_R {
        P2SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Select 1"]
    #[inline(always)]
    pub fn p1sel1(&mut self) -> P1SEL1_W {
        P1SEL1_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Select 1"]
    #[inline(always)]
    pub fn p2sel1(&mut self) -> P2SEL1_W {
        P2SEL1_W { w: self }
    }
}
