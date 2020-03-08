#[doc = "Reader of register PDSEL1"]
pub type R = crate::R<u16, super::PDSEL1>;
#[doc = "Writer for register PDSEL1"]
pub type W = crate::W<u16, super::PDSEL1>;
#[doc = "Register PDSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDSEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7SEL1`"]
pub type P7SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7SEL1`"]
pub struct P7SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8SEL1`"]
pub type P8SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8SEL1`"]
pub struct P8SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&self) -> P7SEL1_R {
        P7SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8SEL1_R {
        P8SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&mut self) -> P7SEL1_W {
        P7SEL1_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&mut self) -> P8SEL1_W {
        P8SEL1_W { w: self }
    }
}
