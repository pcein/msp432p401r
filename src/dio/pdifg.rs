#[doc = "Reader of register PDIFG"]
pub type R = crate::R<u16, super::PDIFG>;
#[doc = "Writer for register PDIFG"]
pub type W = crate::W<u16, super::PDIFG>;
#[doc = "Register PDIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PDIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7IFG`"]
pub type P7IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7IFG`"]
pub struct P7IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8IFG`"]
pub type P8IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8IFG`"]
pub struct P8IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&self) -> P7IFG_R {
        P7IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&self) -> P8IFG_R {
        P8IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&mut self) -> P7IFG_W {
        P7IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&mut self) -> P8IFG_W {
        P8IFG_W { w: self }
    }
}
