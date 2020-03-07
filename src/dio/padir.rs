#[doc = "Reader of register PADIR"]
pub type R = crate::R<u16, super::PADIR>;
#[doc = "Writer for register PADIR"]
pub type W = crate::W<u16, super::PADIR>;
#[doc = "Register PADIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PADIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1DIR`"]
pub type P1DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1DIR`"]
pub struct P1DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2DIR`"]
pub type P2DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2DIR`"]
pub struct P2DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&self) -> P1DIR_R {
        P1DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&self) -> P2DIR_R {
        P2DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&mut self) -> P1DIR_W {
        P1DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&mut self) -> P2DIR_W {
        P2DIR_W { w: self }
    }
}
