#[doc = "Reader of register PASELC"]
pub type R = crate::R<u16, super::PASELC>;
#[doc = "Writer for register PASELC"]
pub type W = crate::W<u16, super::PASELC>;
#[doc = "Register PASELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PASELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SELC`"]
pub type P1SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1SELC`"]
pub struct P1SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2SELC`"]
pub type P2SELC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2SELC`"]
pub struct P2SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&self) -> P1SELC_R {
        P1SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&self) -> P2SELC_R {
        P2SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&mut self) -> P1SELC_W {
        P1SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&mut self) -> P2SELC_W {
        P2SELC_W { w: self }
    }
}
