#[doc = "Reader of register PBIE"]
pub type R = crate::R<u16, super::PBIE>;
#[doc = "Writer for register PBIE"]
pub type W = crate::W<u16, super::PBIE>;
#[doc = "Register PBIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PBIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IE`"]
pub type P3IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3IE`"]
pub struct P3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4IE`"]
pub type P4IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4IE`"]
pub struct P4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&self) -> P3IE_R {
        P3IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&self) -> P4IE_R {
        P4IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&mut self) -> P3IE_W {
        P3IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&mut self) -> P4IE_W {
        P4IE_W { w: self }
    }
}
