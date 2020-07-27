#[doc = "Reader of register PDDS"]
pub type R = crate::R<u16, super::PDDS>;
#[doc = "Writer for register PDDS"]
pub type W = crate::W<u16, super::PDDS>;
#[doc = "Register PDDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PDDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7DS`"]
pub type P7DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7DS`"]
pub struct P7DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8DS`"]
pub type P8DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8DS`"]
pub struct P8DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&self) -> P7DS_R {
        P7DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&self) -> P8DS_R {
        P8DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&mut self) -> P7DS_W {
        P7DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&mut self) -> P8DS_W {
        P8DS_W { w: self }
    }
}
