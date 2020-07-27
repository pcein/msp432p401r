#[doc = "Reader of register PDIES"]
pub type R = crate::R<u16, super::PDIES>;
#[doc = "Writer for register PDIES"]
pub type W = crate::W<u16, super::PDIES>;
#[doc = "Register PDIES `reset()`'s with value 0"]
impl crate::ResetValue for super::PDIES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7IES`"]
pub type P7IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P7IES`"]
pub struct P7IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P8IES`"]
pub type P8IES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P8IES`"]
pub struct P8IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&self) -> P7IES_R {
        P7IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&self) -> P8IES_R {
        P8IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&mut self) -> P7IES_W {
        P7IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&mut self) -> P8IES_W {
        P8IES_W { w: self }
    }
}
