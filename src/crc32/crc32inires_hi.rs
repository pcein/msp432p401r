#[doc = "Reader of register CRC32INIRES_HI"]
pub type R = crate::R<u16, super::CRC32INIRES_HI>;
#[doc = "Writer for register CRC32INIRES_HI"]
pub type W = crate::W<u16, super::CRC32INIRES_HI>;
#[doc = "Register CRC32INIRES_HI `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC32INIRES_HI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC32INIRES_HI`"]
pub type CRC32INIRES_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC32INIRES_HI`"]
pub struct CRC32INIRES_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32INIRES_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&self) -> CRC32INIRES_HI_R {
        CRC32INIRES_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&mut self) -> CRC32INIRES_HI_W {
        CRC32INIRES_HI_W { w: self }
    }
}
