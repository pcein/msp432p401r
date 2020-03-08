#[doc = "Reader of register PBSELC"]
pub type R = crate::R<u16, super::PBSELC>;
#[doc = "Writer for register PBSELC"]
pub type W = crate::W<u16, super::PBSELC>;
#[doc = "Register PBSELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PBSELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3SELC`"]
pub type P3SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3SELC`"]
pub struct P3SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4SELC`"]
pub type P4SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4SELC`"]
pub struct P4SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&self) -> P3SELC_R {
        P3SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&self) -> P4SELC_R {
        P4SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&mut self) -> P3SELC_W {
        P3SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&mut self) -> P4SELC_W {
        P4SELC_W { w: self }
    }
}
