#[doc = "Reader of register CRC16INIRES"]
pub type R = crate::R<u16, super::CRC16INIRES>;
#[doc = "Writer for register CRC16INIRES"]
pub type W = crate::W<u16, super::CRC16INIRES>;
#[doc = "Register CRC16INIRES `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRC16INIRES {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CRC16INIRES`"]
pub type CRC16INIRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC16INIRES`"]
pub struct CRC16INIRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16INIRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 initialization and result"]
    #[inline(always)]
    pub fn crc16inires(&self) -> CRC16INIRES_R {
        CRC16INIRES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 initialization and result"]
    #[inline(always)]
    pub fn crc16inires(&mut self) -> CRC16INIRES_W {
        CRC16INIRES_W { w: self }
    }
}
