#[doc = "Reader of register PEDS"]
pub type R = crate::R<u16, super::PEDS>;
#[doc = "Writer for register PEDS"]
pub type W = crate::W<u16, super::PEDS>;
#[doc = "Register PEDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PEDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9DS`"]
pub type P9DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P9DS`"]
pub struct P9DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P10DS`"]
pub type P10DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P10DS`"]
pub struct P10DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P10DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&self) -> P9DS_R {
        P9DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&self) -> P10DS_R {
        P10DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&mut self) -> P9DS_W {
        P9DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&mut self) -> P10DS_W {
        P10DS_W { w: self }
    }
}
