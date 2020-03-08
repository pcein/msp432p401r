#[doc = "Reader of register PBIES"]
pub type R = crate::R<u16, super::PBIES>;
#[doc = "Writer for register PBIES"]
pub type W = crate::W<u16, super::PBIES>;
#[doc = "Register PBIES `reset()`'s with value 0"]
impl crate::ResetValue for super::PBIES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IES`"]
pub type P3IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3IES`"]
pub struct P3IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4IES`"]
pub type P4IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4IES`"]
pub struct P4IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&self) -> P3IES_R {
        P3IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&self) -> P4IES_R {
        P4IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&mut self) -> P3IES_W {
        P3IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&mut self) -> P4IES_W {
        P4IES_W { w: self }
    }
}
