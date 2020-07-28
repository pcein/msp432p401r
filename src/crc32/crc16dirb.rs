#[doc = "Reader of register CRC16DIRB"]
pub type R = crate::R<u16, super::CRC16DIRB>;
#[doc = "Writer for register CRC16DIRB"]
pub type W = crate::W<u16, super::CRC16DIRB>;
#[doc = "Register CRC16DIRB `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC16DIRB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC16DIRB`"]
pub type CRC16DIRB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC16DIRB`"]
pub struct CRC16DIRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16DIRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&self) -> CRC16DIRB_R {
        CRC16DIRB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&mut self) -> CRC16DIRB_W {
        CRC16DIRB_W { w: self }
    }
}
