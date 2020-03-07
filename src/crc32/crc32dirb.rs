#[doc = "Reader of register CRC32DIRB"]
pub type R = crate::R<u16, super::CRC32DIRB>;
#[doc = "Writer for register CRC32DIRB"]
pub type W = crate::W<u16, super::CRC32DIRB>;
#[doc = "Register CRC32DIRB `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC32DIRB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC32DIRB`"]
pub type CRC32DIRB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC32DIRB`"]
pub struct CRC32DIRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32DIRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&self) -> CRC32DIRB_R {
        CRC32DIRB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&mut self) -> CRC32DIRB_W {
        CRC32DIRB_W { w: self }
    }
}
