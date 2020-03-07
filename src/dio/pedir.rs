#[doc = "Reader of register PEDIR"]
pub type R = crate::R<u16, super::PEDIR>;
#[doc = "Writer for register PEDIR"]
pub type W = crate::W<u16, super::PEDIR>;
#[doc = "Register PEDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PEDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9DIR`"]
pub type P9DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9DIR`"]
pub struct P9DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10DIR`"]
pub type P10DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10DIR`"]
pub struct P10DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P10DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&self) -> P9DIR_R {
        P9DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&self) -> P10DIR_R {
        P10DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&mut self) -> P9DIR_W {
        P9DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&mut self) -> P10DIR_W {
        P10DIR_W { w: self }
    }
}
