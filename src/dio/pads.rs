#[doc = "Reader of register PADS"]
pub type R = crate::R<u16, super::PADS>;
#[doc = "Writer for register PADS"]
pub type W = crate::W<u16, super::PADS>;
#[doc = "Register PADS `reset()`'s with value 0"]
impl crate::ResetValue for super::PADS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1DS`"]
pub type P1DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P1DS`"]
pub struct P1DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P2DS`"]
pub type P2DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P2DS`"]
pub struct P2DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&self) -> P1DS_R {
        P1DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&self) -> P2DS_R {
        P2DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&mut self) -> P1DS_W {
        P1DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&mut self) -> P2DS_W {
        P2DS_W { w: self }
    }
}
