#[doc = "Reader of register PCSEL0"]
pub type R = crate::R<u16, super::PCSEL0>;
#[doc = "Writer for register PCSEL0"]
pub type W = crate::W<u16, super::PCSEL0>;
#[doc = "Register PCSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCSEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5SEL0`"]
pub type P5SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5SEL0`"]
pub struct P5SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6SEL0`"]
pub type P6SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6SEL0`"]
pub struct P6SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&self) -> P5SEL0_R {
        P5SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&self) -> P6SEL0_R {
        P6SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&mut self) -> P5SEL0_W {
        P5SEL0_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&mut self) -> P6SEL0_W {
        P6SEL0_W { w: self }
    }
}
