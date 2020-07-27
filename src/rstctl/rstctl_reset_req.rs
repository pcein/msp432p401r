#[doc = "Reader of register RSTCTL_RESET_REQ"]
pub type R = crate::R<u32, super::RSTCTL_RESET_REQ>;
#[doc = "Writer for register RSTCTL_RESET_REQ"]
pub type W = crate::W<u32, super::RSTCTL_RESET_REQ>;
#[doc = "Register RSTCTL_RESET_REQ `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCTL_RESET_REQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SOFT_REQ`"]
pub struct SOFT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_REQ_W<'a> {
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
#[doc = "Write proxy for field `HARD_REQ`"]
pub struct HARD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HARD_REQ_W<'a> {
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
#[doc = "Write proxy for field `RSTKEY`"]
pub struct RSTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 0 - Soft Reset request"]
    #[inline(always)]
    pub fn soft_req(&mut self) -> SOFT_REQ_W {
        SOFT_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Hard Reset request"]
    #[inline(always)]
    pub fn hard_req(&mut self) -> HARD_REQ_W {
        HARD_REQ_W { w: self }
    }
    #[doc = "Bits 8:15 - Write key to unlock reset request bits"]
    #[inline(always)]
    pub fn rstkey(&mut self) -> RSTKEY_W {
        RSTKEY_W { w: self }
    }
}
