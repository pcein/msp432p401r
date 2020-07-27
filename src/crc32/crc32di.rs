#[doc = "Reader of register CRC32DI"]
pub type R = crate::R<u16, super::CRC32DI>;
#[doc = "Writer for register CRC32DI"]
pub type W = crate::W<u16, super::CRC32DI>;
#[doc = "Register CRC32DI `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC32DI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC32DI`"]
pub type CRC32DI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC32DI`"]
pub struct CRC32DI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32DI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data input register"]
    #[inline(always)]
    pub fn crc32di(&self) -> CRC32DI_R {
        CRC32DI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data input register"]
    #[inline(always)]
    pub fn crc32di(&mut self) -> CRC32DI_W {
        CRC32DI_W { w: self }
    }
}
