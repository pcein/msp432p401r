#[doc = "Reader of register RTCBIN2BCD"]
pub type R = crate::R<u16, super::RTCBIN2BCD>;
#[doc = "Writer for register RTCBIN2BCD"]
pub type W = crate::W<u16, super::RTCBIN2BCD>;
#[doc = "Register RTCBIN2BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCBIN2BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BIN2BCD`"]
pub type BIN2BCD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BIN2BCD`"]
pub struct BIN2BCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BIN2BCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&self) -> BIN2BCD_R {
        BIN2BCD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&mut self) -> BIN2BCD_W {
        BIN2BCD_W { w: self }
    }
}
