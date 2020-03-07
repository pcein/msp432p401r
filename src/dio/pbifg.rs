#[doc = "Reader of register PBIFG"]
pub type R = crate::R<u16, super::PBIFG>;
#[doc = "Writer for register PBIFG"]
pub type W = crate::W<u16, super::PBIFG>;
#[doc = "Register PBIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PBIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IFG`"]
pub type P3IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3IFG`"]
pub struct P3IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4IFG`"]
pub type P4IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4IFG`"]
pub struct P4IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&self) -> P3IFG_R {
        P3IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&self) -> P4IFG_R {
        P4IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&mut self) -> P3IFG_W {
        P3IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&mut self) -> P4IFG_W {
        P4IFG_W { w: self }
    }
}
