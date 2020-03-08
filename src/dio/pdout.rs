#[doc = "Reader of register PDOUT"]
pub type R = crate::R<u16, super::PDOUT>;
#[doc = "Writer for register PDOUT"]
pub type W = crate::W<u16, super::PDOUT>;
#[doc = "Register PDOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PDOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7OUT`"]
pub type P7OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7OUT`"]
pub struct P7OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8OUT`"]
pub type P8OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8OUT`"]
pub struct P8OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&self) -> P7OUT_R {
        P7OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&self) -> P8OUT_R {
        P8OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&mut self) -> P7OUT_W {
        P7OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&mut self) -> P8OUT_W {
        P8OUT_W { w: self }
    }
}
