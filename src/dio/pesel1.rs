#[doc = "Reader of register PESEL1"]
pub type R = crate::R<u16, super::PESEL1>;
#[doc = "Writer for register PESEL1"]
pub type W = crate::W<u16, super::PESEL1>;
#[doc = "Register PESEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PESEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9SEL1`"]
pub type P9SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9SEL1`"]
pub struct P9SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10SEL1`"]
pub type P10SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10SEL1`"]
pub struct P10SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P10SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&self) -> P9SEL1_R {
        P9SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&self) -> P10SEL1_R {
        P10SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&mut self) -> P9SEL1_W {
        P9SEL1_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&mut self) -> P10SEL1_W {
        P10SEL1_W { w: self }
    }
}
