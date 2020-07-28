#[doc = "Reader of register PBDIR"]
pub type R = crate::R<u16, super::PBDIR>;
#[doc = "Writer for register PBDIR"]
pub type W = crate::W<u16, super::PBDIR>;
#[doc = "Register PBDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PBDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3DIR`"]
pub type P3DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3DIR`"]
pub struct P3DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4DIR`"]
pub type P4DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4DIR`"]
pub struct P4DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&self) -> P3DIR_R {
        P3DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&self) -> P4DIR_R {
        P4DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&mut self) -> P3DIR_W {
        P3DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&mut self) -> P4DIR_W {
        P4DIR_W { w: self }
    }
}
