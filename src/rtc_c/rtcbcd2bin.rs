#[doc = "Reader of register RTCBCD2BIN"]
pub type R = crate::R<u16, super::RTCBCD2BIN>;
#[doc = "Writer for register RTCBCD2BIN"]
pub type W = crate::W<u16, super::RTCBCD2BIN>;
#[doc = "Register RTCBCD2BIN `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCBCD2BIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCD2BIN`"]
pub type BCD2BIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BCD2BIN`"]
pub struct BCD2BIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCD2BIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&self) -> BCD2BIN_R {
        BCD2BIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&mut self) -> BCD2BIN_W {
        BCD2BIN_W { w: self }
    }
}
