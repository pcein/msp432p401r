#[doc = "Reader of register PDSELC"]
pub type R = crate::R<u16, super::PDSELC>;
#[doc = "Writer for register PDSELC"]
pub type W = crate::W<u16, super::PDSELC>;
#[doc = "Register PDSELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PDSELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7SELC`"]
pub type P7SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7SELC`"]
pub struct P7SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8SELC`"]
pub type P8SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8SELC`"]
pub struct P8SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&self) -> P7SELC_R {
        P7SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&self) -> P8SELC_R {
        P8SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&mut self) -> P7SELC_W {
        P7SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&mut self) -> P8SELC_W {
        P8SELC_W { w: self }
    }
}
