#[doc = "Reader of register PDDIR"]
pub type R = crate::R<u16, super::PDDIR>;
#[doc = "Writer for register PDDIR"]
pub type W = crate::W<u16, super::PDDIR>;
#[doc = "Register PDDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7DIR`"]
pub type P7DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7DIR`"]
pub struct P7DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8DIR`"]
pub type P8DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8DIR`"]
pub struct P8DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&self) -> P7DIR_R {
        P7DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&self) -> P8DIR_R {
        P8DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&mut self) -> P7DIR_W {
        P7DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&mut self) -> P8DIR_W {
        P8DIR_W { w: self }
    }
}
