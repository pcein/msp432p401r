#[doc = "Reader of register PDIE"]
pub type R = crate::R<u16, super::PDIE>;
#[doc = "Writer for register PDIE"]
pub type W = crate::W<u16, super::PDIE>;
#[doc = "Register PDIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PDIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7IE`"]
pub type P7IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7IE`"]
pub struct P7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8IE`"]
pub type P8IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8IE`"]
pub struct P8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&self) -> P7IE_R {
        P7IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&self) -> P8IE_R {
        P8IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&mut self) -> P7IE_W {
        P7IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&mut self) -> P8IE_W {
        P8IE_W { w: self }
    }
}
