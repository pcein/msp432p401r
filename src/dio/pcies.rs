#[doc = "Reader of register PCIES"]
pub type R = crate::R<u16, super::PCIES>;
#[doc = "Writer for register PCIES"]
pub type W = crate::W<u16, super::PCIES>;
#[doc = "Register PCIES `reset()`'s with value 0"]
impl crate::ResetValue for super::PCIES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5IES`"]
pub type P5IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5IES`"]
pub struct P5IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6IES`"]
pub type P6IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6IES`"]
pub struct P6IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&self) -> P5IES_R {
        P5IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&self) -> P6IES_R {
        P6IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&mut self) -> P5IES_W {
        P5IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&mut self) -> P6IES_W {
        P6IES_W { w: self }
    }
}
