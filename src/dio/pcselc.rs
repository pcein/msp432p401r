#[doc = "Reader of register PCSELC"]
pub type R = crate::R<u16, super::PCSELC>;
#[doc = "Writer for register PCSELC"]
pub type W = crate::W<u16, super::PCSELC>;
#[doc = "Register PCSELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PCSELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5SELC`"]
pub type P5SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5SELC`"]
pub struct P5SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6SELC`"]
pub type P6SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6SELC`"]
pub struct P6SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&self) -> P5SELC_R {
        P5SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&self) -> P6SELC_R {
        P6SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&mut self) -> P5SELC_W {
        P5SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&mut self) -> P6SELC_W {
        P6SELC_W { w: self }
    }
}
