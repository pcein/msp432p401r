#[doc = "Reader of register PBOUT"]
pub type R = crate::R<u16, super::PBOUT>;
#[doc = "Writer for register PBOUT"]
pub type W = crate::W<u16, super::PBOUT>;
#[doc = "Register PBOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PBOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3OUT`"]
pub type P3OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P3OUT`"]
pub struct P3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P4OUT`"]
pub type P4OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P4OUT`"]
pub struct P4OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&self) -> P3OUT_R {
        P3OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&self) -> P4OUT_R {
        P4OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&mut self) -> P3OUT_W {
        P3OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&mut self) -> P4OUT_W {
        P4OUT_W { w: self }
    }
}
