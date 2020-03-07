#[doc = "Reader of register PCDIR"]
pub type R = crate::R<u16, super::PCDIR>;
#[doc = "Writer for register PCDIR"]
pub type W = crate::W<u16, super::PCDIR>;
#[doc = "Register PCDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5DIR`"]
pub type P5DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5DIR`"]
pub struct P5DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6DIR`"]
pub type P6DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6DIR`"]
pub struct P6DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&self) -> P5DIR_R {
        P5DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&self) -> P6DIR_R {
        P6DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&mut self) -> P5DIR_W {
        P5DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&mut self) -> P6DIR_W {
        P6DIR_W { w: self }
    }
}
