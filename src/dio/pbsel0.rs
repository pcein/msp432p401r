#[doc = "Reader of register PBSEL0"]
pub type R = crate::R<u16, super::PBSEL0>;
#[doc = "Writer for register PBSEL0"]
pub type W = crate::W<u16, super::PBSEL0>;
#[doc = "Register PBSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PBSEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4SEL0`"]
pub type P4SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4SEL0`"]
pub struct P4SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `P3SEL0`"]
pub type P3SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3SEL0`"]
pub struct P3SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&self) -> P4SEL0_R {
        P4SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&mut self) -> P4SEL0_W {
        P4SEL0_W { w: self }
    }
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3SEL0_W {
        P3SEL0_W { w: self }
    }
}
