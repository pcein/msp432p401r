#[doc = "Reader of register PEOUT"]
pub type R = crate::R<u16, super::PEOUT>;
#[doc = "Writer for register PEOUT"]
pub type W = crate::W<u16, super::PEOUT>;
#[doc = "Register PEOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PEOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9OUT`"]
pub type P9OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9OUT`"]
pub struct P9OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10OUT`"]
pub type P10OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10OUT`"]
pub struct P10OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P10OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&self) -> P9OUT_R {
        P9OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&self) -> P10OUT_R {
        P10OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&mut self) -> P9OUT_W {
        P9OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&mut self) -> P10OUT_W {
        P10OUT_W { w: self }
    }
}
