#[doc = "Reader of register PAIE"]
pub type R = crate::R<u16, super::PAIE>;
#[doc = "Writer for register PAIE"]
pub type W = crate::W<u16, super::PAIE>;
#[doc = "Register PAIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PAIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IE`"]
pub type P1IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1IE`"]
pub struct P1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2IE`"]
pub type P2IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2IE`"]
pub struct P2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&self) -> P1IE_R {
        P1IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&self) -> P2IE_R {
        P2IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&mut self) -> P1IE_W {
        P1IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&mut self) -> P2IE_W {
        P2IE_W { w: self }
    }
}
