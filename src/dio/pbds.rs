#[doc = "Reader of register PBDS"]
pub type R = crate::R<u16, super::PBDS>;
#[doc = "Writer for register PBDS"]
pub type W = crate::W<u16, super::PBDS>;
#[doc = "Register PBDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PBDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3DS`"]
pub type P3DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3DS`"]
pub struct P3DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4DS`"]
pub type P4DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4DS`"]
pub struct P4DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&self) -> P3DS_R {
        P3DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&self) -> P4DS_R {
        P4DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&mut self) -> P3DS_W {
        P3DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&mut self) -> P4DS_W {
        P4DS_W { w: self }
    }
}
