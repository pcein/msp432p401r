#[doc = "Reader of register PCIE"]
pub type R = crate::R<u16, super::PCIE>;
#[doc = "Writer for register PCIE"]
pub type W = crate::W<u16, super::PCIE>;
#[doc = "Register PCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PCIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5IE`"]
pub type P5IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5IE`"]
pub struct P5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6IE`"]
pub type P6IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6IE`"]
pub struct P6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&self) -> P5IE_R {
        P5IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&self) -> P6IE_R {
        P6IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&mut self) -> P5IE_W {
        P5IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&mut self) -> P6IE_W {
        P6IE_W { w: self }
    }
}
