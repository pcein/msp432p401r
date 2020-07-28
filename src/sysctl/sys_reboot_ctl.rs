#[doc = "Reader of register SYS_REBOOT_CTL"]
pub type R = crate::R<u32, super::SYS_REBOOT_CTL>;
#[doc = "Writer for register SYS_REBOOT_CTL"]
pub type W = crate::W<u32, super::SYS_REBOOT_CTL>;
#[doc = "Register SYS_REBOOT_CTL `reset()`'s with value 0xfe"]
impl crate::ResetValue for super::SYS_REBOOT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfe
    }
}
#[doc = "Reader of field `REBOOT`"]
pub type REBOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REBOOT`"]
pub struct REBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> REBOOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `WKEY`"]
pub struct WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W {
        REBOOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Key to enable writes to bit 0"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WKEY_W {
        WKEY_W { w: self }
    }
}
