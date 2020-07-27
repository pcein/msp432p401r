#[doc = "Reader of register FLCTL_ERASE_SECTADDR"]
pub type R = crate::R<u32, super::FLCTL_ERASE_SECTADDR>;
#[doc = "Writer for register FLCTL_ERASE_SECTADDR"]
pub type W = crate::W<u32, super::FLCTL_ERASE_SECTADDR>;
#[doc = "Register FLCTL_ERASE_SECTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_ERASE_SECTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECT_ADDRESS`"]
pub type SECT_ADDRESS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SECT_ADDRESS`"]
pub struct SECT_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECT_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&self) -> SECT_ADDRESS_R {
        SECT_ADDRESS_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&mut self) -> SECT_ADDRESS_W {
        SECT_ADDRESS_W { w: self }
    }
}
