#[doc = "Reader of register SYS_RESET_STATOVER"]
pub type R = crate::R<u32, super::SYS_RESET_STATOVER>;
#[doc = "Writer for register SYS_RESET_STATOVER"]
pub type W = crate::W<u32, super::SYS_RESET_STATOVER>;
#[doc = "Register SYS_RESET_STATOVER `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_RESET_STATOVER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFT`"]
pub type SOFT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HARD`"]
pub type HARD_R = crate::R<bool, bool>;
#[doc = "Reader of field `REBOOT`"]
pub type REBOOT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFT_OVER`"]
pub type SOFT_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFT_OVER`"]
pub struct SOFT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `HARD_OVER`"]
pub type HARD_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HARD_OVER`"]
pub struct HARD_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> HARD_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RBT_OVER`"]
pub type RBT_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBT_OVER`"]
pub struct RBT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RBT_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if SOFT Reset is active"]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if HARD Reset is active"]
    #[inline(always)]
    pub fn hard(&self) -> HARD_R {
        HARD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if Reboot Reset is active"]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&self) -> SOFT_OVER_R {
        SOFT_OVER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&self) -> HARD_OVER_R {
        HARD_OVER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&self) -> RBT_OVER_R {
        RBT_OVER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&mut self) -> SOFT_OVER_W {
        SOFT_OVER_W { w: self }
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&mut self) -> HARD_OVER_W {
        HARD_OVER_W { w: self }
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&mut self) -> RBT_OVER_W {
        RBT_OVER_W { w: self }
    }
}
