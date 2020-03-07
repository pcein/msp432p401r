#[doc = "Writer for register FLCTL_SETIFG"]
pub type W = crate::W<u32, super::FLCTL_SETIFG>;
#[doc = "Register FLCTL_SETIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_SETIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W {
        RDBRST_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W {
        AVPRE_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W {
        AVPST_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W {
        PRG_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W {
        PRGB_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 8 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W {
        BMRK_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PRG_ERR_W {
        PRG_ERR_W { w: self }
    }
}
