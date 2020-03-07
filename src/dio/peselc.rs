#[doc = "Reader of register PESELC"]
pub type R = crate::R<u16, super::PESELC>;
#[doc = "Writer for register PESELC"]
pub type W = crate::W<u16, super::PESELC>;
#[doc = "Register PESELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PESELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9SELC`"]
pub type P9SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9SELC`"]
pub struct P9SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P9SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10SELC`"]
pub type P10SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10SELC`"]
pub struct P10SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P10SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&self) -> P9SELC_R {
        P9SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&self) -> P10SELC_R {
        P10SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&mut self) -> P9SELC_W {
        P9SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&mut self) -> P10SELC_W {
        P10SELC_W { w: self }
    }
}
