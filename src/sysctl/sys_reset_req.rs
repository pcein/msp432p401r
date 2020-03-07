#[doc = "Reader of register SYS_RESET_REQ"]
pub type R = crate::R<u32, super::SYS_RESET_REQ>;
#[doc = "Writer for register SYS_RESET_REQ"]
pub type W = crate::W<u32, super::SYS_RESET_REQ>;
#[doc = "Register SYS_RESET_REQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_RESET_REQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `POR`"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
impl R {}
impl W {
    #[doc = "Bit 0 - Generate POR"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - Generate Reboot_Reset"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W {
        REBOOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Write key"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WKEY_W {
        WKEY_W { w: self }
    }
}
