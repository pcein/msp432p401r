#[doc = "Reader of register CRC16DI"]
pub type R = crate::R<u16, super::CRC16DI>;
#[doc = "Writer for register CRC16DI"]
pub type W = crate::W<u16, super::CRC16DI>;
#[doc = "Register CRC16DI `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC16DI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC16DI`"]
pub type CRC16DI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC16DI`"]
pub struct CRC16DI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16DI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&self) -> CRC16DI_R {
        CRC16DI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&mut self) -> CRC16DI_W {
        CRC16DI_W { w: self }
    }
}
