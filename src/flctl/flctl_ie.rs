#[doc = "Reader of register FLCTL_IE"]
pub type R = crate::R<u32, super::FLCTL_IE>;
#[doc = "Writer for register FLCTL_IE"]
pub type W = crate::W<u32, super::FLCTL_IE>;
#[doc = "Register FLCTL_IE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDBRST`"]
pub type RDBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDBRST`"]
pub struct RDBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RDBRST_W<'a> {
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
#[doc = "Reader of field `AVPRE`"]
pub type AVPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVPRE`"]
pub struct AVPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPRE_W<'a> {
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
#[doc = "Reader of field `AVPST`"]
pub type AVPST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVPST`"]
pub struct AVPST_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRG`"]
pub type PRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRG`"]
pub struct PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PRGB`"]
pub type PRGB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRGB`"]
pub struct PRGB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BMRK`"]
pub type BMRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMRK`"]
pub struct BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMRK_W<'a> {
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
#[doc = "Reader of field `PRG_ERR`"]
pub type PRG_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRG_ERR`"]
pub struct PRG_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRG_ERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&self) -> AVPRE_R {
        AVPRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&self) -> AVPST_R {
        AVPST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&self) -> PRG_R {
        PRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&self) -> PRGB_R {
        PRGB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&self) -> BMRK_R {
        BMRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&self) -> PRG_ERR_R {
        PRG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W {
        RDBRST_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W {
        AVPRE_W { w: self }
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W {
        AVPST_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W {
        PRG_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W {
        PRGB_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W {
        BMRK_W { w: self }
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PRG_ERR_W {
        PRG_ERR_W { w: self }
    }
}
