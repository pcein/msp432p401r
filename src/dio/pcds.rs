#[doc = "Reader of register PCDS"]
pub type R = crate::R<u16, super::PCDS>;
#[doc = "Writer for register PCDS"]
pub type W = crate::W<u16, super::PCDS>;
#[doc = "Register PCDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PCDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5DS`"]
pub type P5DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5DS`"]
pub struct P5DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6DS`"]
pub type P6DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6DS`"]
pub struct P6DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&self) -> P5DS_R {
        P5DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&self) -> P6DS_R {
        P6DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&mut self) -> P5DS_W {
        P5DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&mut self) -> P6DS_W {
        P6DS_W { w: self }
    }
}
