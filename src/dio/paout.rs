#[doc = "Reader of register PAOUT"]
pub type R = crate::R<u16, super::PAOUT>;
#[doc = "Writer for register PAOUT"]
pub type W = crate::W<u16, super::PAOUT>;
#[doc = "Register PAOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PAOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2OUT`"]
pub type P2OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2OUT`"]
pub struct P2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `P1OUT`"]
pub type P1OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1OUT`"]
pub struct P1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&self) -> P2OUT_R {
        P2OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&self) -> P1OUT_R {
        P1OUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&mut self) -> P2OUT_W {
        P2OUT_W { w: self }
    }
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&mut self) -> P1OUT_W {
        P1OUT_W { w: self }
    }
}
