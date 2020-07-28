#[doc = "Reader of register FLCTL_BANK0_INFO_WEPROT"]
pub type R = crate::R<u32, super::FLCTL_BANK0_INFO_WEPROT>;
#[doc = "Writer for register FLCTL_BANK0_INFO_WEPROT"]
pub type W = crate::W<u32, super::FLCTL_BANK0_INFO_WEPROT>;
#[doc = "Register FLCTL_BANK0_INFO_WEPROT `reset()`'s with value 0x03"]
impl crate::ResetValue for super::FLCTL_BANK0_INFO_WEPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `PROT0`"]
pub type PROT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT0`"]
pub struct PROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT0_W<'a> {
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
#[doc = "Reader of field `PROT1`"]
pub type PROT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT1`"]
pub struct PROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&self) -> PROT0_R {
        PROT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&mut self) -> PROT0_W {
        PROT0_W { w: self }
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W {
        PROT1_W { w: self }
    }
}
