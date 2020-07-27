#[doc = "Reader of register PEIE"]
pub type R = crate::R<u16, super::PEIE>;
#[doc = "Writer for register PEIE"]
pub type W = crate::W<u16, super::PEIE>;
#[doc = "Register PEIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PEIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9IE`"]
pub type P9IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9IE`"]
pub struct P9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10IE`"]
pub type P10IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10IE`"]
pub struct P10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&self) -> P9IE_R {
        P9IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&self) -> P10IE_R {
        P10IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&mut self) -> P9IE_W {
        P9IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&mut self) -> P10IE_W {
        P10IE_W { w: self }
    }
}
