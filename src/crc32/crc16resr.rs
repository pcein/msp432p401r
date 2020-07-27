#[doc = "Reader of register CRC16RESR"]
pub type R = crate::R<u16, super::CRC16RESR>;
#[doc = "Writer for register CRC16RESR"]
pub type W = crate::W<u16, super::CRC16RESR>;
#[doc = "Register CRC16RESR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRC16RESR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CRC16RESR`"]
pub type CRC16RESR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC16RESR`"]
pub struct CRC16RESR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16RESR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&self) -> CRC16RESR_R {
        CRC16RESR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&mut self) -> CRC16RESR_W {
        CRC16RESR_W { w: self }
    }
}
