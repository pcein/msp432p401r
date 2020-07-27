#[doc = "Reader of register FLCTL_BANK1_MAIN_WEPROT"]
pub type R = crate::R<u32, super::FLCTL_BANK1_MAIN_WEPROT>;
#[doc = "Writer for register FLCTL_BANK1_MAIN_WEPROT"]
pub type W = crate::W<u32, super::FLCTL_BANK1_MAIN_WEPROT>;
#[doc = "Register FLCTL_BANK1_MAIN_WEPROT `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FLCTL_BANK1_MAIN_WEPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
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
#[doc = "Reader of field `PROT2`"]
pub type PROT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT2`"]
pub struct PROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT2_W<'a> {
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
#[doc = "Reader of field `PROT3`"]
pub type PROT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT3`"]
pub struct PROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT3_W<'a> {
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
#[doc = "Reader of field `PROT4`"]
pub type PROT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT4`"]
pub struct PROT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT4_W<'a> {
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
#[doc = "Reader of field `PROT5`"]
pub type PROT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT5`"]
pub struct PROT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT5_W<'a> {
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
#[doc = "Reader of field `PROT6`"]
pub type PROT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT6`"]
pub struct PROT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PROT7`"]
pub type PROT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT7`"]
pub struct PROT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PROT8`"]
pub type PROT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT8`"]
pub struct PROT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT8_W<'a> {
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
#[doc = "Reader of field `PROT9`"]
pub type PROT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT9`"]
pub struct PROT9_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT9_W<'a> {
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
#[doc = "Reader of field `PROT10`"]
pub type PROT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT10`"]
pub struct PROT10_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT10_W<'a> {
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
#[doc = "Reader of field `PROT11`"]
pub type PROT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT11`"]
pub struct PROT11_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PROT12`"]
pub type PROT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT12`"]
pub struct PROT12_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PROT13`"]
pub type PROT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT13`"]
pub struct PROT13_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PROT14`"]
pub type PROT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT14`"]
pub struct PROT14_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PROT15`"]
pub type PROT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT15`"]
pub struct PROT15_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PROT16`"]
pub type PROT16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT16`"]
pub struct PROT16_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PROT17`"]
pub type PROT17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT17`"]
pub struct PROT17_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PROT18`"]
pub type PROT18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT18`"]
pub struct PROT18_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PROT19`"]
pub type PROT19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT19`"]
pub struct PROT19_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PROT20`"]
pub type PROT20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT20`"]
pub struct PROT20_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PROT21`"]
pub type PROT21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT21`"]
pub struct PROT21_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PROT22`"]
pub type PROT22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT22`"]
pub struct PROT22_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PROT23`"]
pub type PROT23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT23`"]
pub struct PROT23_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PROT24`"]
pub type PROT24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT24`"]
pub struct PROT24_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PROT25`"]
pub type PROT25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT25`"]
pub struct PROT25_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PROT26`"]
pub type PROT26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT26`"]
pub struct PROT26_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PROT27`"]
pub type PROT27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT27`"]
pub struct PROT27_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PROT28`"]
pub type PROT28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT28`"]
pub struct PROT28_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PROT29`"]
pub type PROT29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT29`"]
pub struct PROT29_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PROT30`"]
pub type PROT30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT30`"]
pub struct PROT30_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PROT31`"]
pub type PROT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT31`"]
pub struct PROT31_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&self) -> PROT0_R {
        PROT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase operations"]
    #[inline(always)]
    pub fn prot2(&self) -> PROT2_R {
        PROT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase operations"]
    #[inline(always)]
    pub fn prot3(&self) -> PROT3_R {
        PROT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase operations"]
    #[inline(always)]
    pub fn prot4(&self) -> PROT4_R {
        PROT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase operations"]
    #[inline(always)]
    pub fn prot5(&self) -> PROT5_R {
        PROT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase operations"]
    #[inline(always)]
    pub fn prot6(&self) -> PROT6_R {
        PROT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase operations"]
    #[inline(always)]
    pub fn prot7(&self) -> PROT7_R {
        PROT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase operations"]
    #[inline(always)]
    pub fn prot8(&self) -> PROT8_R {
        PROT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase operations"]
    #[inline(always)]
    pub fn prot9(&self) -> PROT9_R {
        PROT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase operations"]
    #[inline(always)]
    pub fn prot10(&self) -> PROT10_R {
        PROT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase operations"]
    #[inline(always)]
    pub fn prot11(&self) -> PROT11_R {
        PROT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase operations"]
    #[inline(always)]
    pub fn prot12(&self) -> PROT12_R {
        PROT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase operations"]
    #[inline(always)]
    pub fn prot13(&self) -> PROT13_R {
        PROT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase operations"]
    #[inline(always)]
    pub fn prot14(&self) -> PROT14_R {
        PROT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase operations"]
    #[inline(always)]
    pub fn prot15(&self) -> PROT15_R {
        PROT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase operations"]
    #[inline(always)]
    pub fn prot16(&self) -> PROT16_R {
        PROT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase operations"]
    #[inline(always)]
    pub fn prot17(&self) -> PROT17_R {
        PROT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase operations"]
    #[inline(always)]
    pub fn prot18(&self) -> PROT18_R {
        PROT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase operations"]
    #[inline(always)]
    pub fn prot19(&self) -> PROT19_R {
        PROT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase operations"]
    #[inline(always)]
    pub fn prot20(&self) -> PROT20_R {
        PROT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase operations"]
    #[inline(always)]
    pub fn prot21(&self) -> PROT21_R {
        PROT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase operations"]
    #[inline(always)]
    pub fn prot22(&self) -> PROT22_R {
        PROT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase operations"]
    #[inline(always)]
    pub fn prot23(&self) -> PROT23_R {
        PROT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase operations"]
    #[inline(always)]
    pub fn prot24(&self) -> PROT24_R {
        PROT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase operations"]
    #[inline(always)]
    pub fn prot25(&self) -> PROT25_R {
        PROT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase operations"]
    #[inline(always)]
    pub fn prot26(&self) -> PROT26_R {
        PROT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase operations"]
    #[inline(always)]
    pub fn prot27(&self) -> PROT27_R {
        PROT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase operations"]
    #[inline(always)]
    pub fn prot28(&self) -> PROT28_R {
        PROT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase operations"]
    #[inline(always)]
    pub fn prot29(&self) -> PROT29_R {
        PROT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase operations"]
    #[inline(always)]
    pub fn prot30(&self) -> PROT30_R {
        PROT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase operations"]
    #[inline(always)]
    pub fn prot31(&self) -> PROT31_R {
        PROT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&mut self) -> PROT0_W {
        PROT0_W { w: self }
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W {
        PROT1_W { w: self }
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase operations"]
    #[inline(always)]
    pub fn prot2(&mut self) -> PROT2_W {
        PROT2_W { w: self }
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase operations"]
    #[inline(always)]
    pub fn prot3(&mut self) -> PROT3_W {
        PROT3_W { w: self }
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase operations"]
    #[inline(always)]
    pub fn prot4(&mut self) -> PROT4_W {
        PROT4_W { w: self }
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase operations"]
    #[inline(always)]
    pub fn prot5(&mut self) -> PROT5_W {
        PROT5_W { w: self }
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase operations"]
    #[inline(always)]
    pub fn prot6(&mut self) -> PROT6_W {
        PROT6_W { w: self }
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase operations"]
    #[inline(always)]
    pub fn prot7(&mut self) -> PROT7_W {
        PROT7_W { w: self }
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase operations"]
    #[inline(always)]
    pub fn prot8(&mut self) -> PROT8_W {
        PROT8_W { w: self }
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase operations"]
    #[inline(always)]
    pub fn prot9(&mut self) -> PROT9_W {
        PROT9_W { w: self }
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase operations"]
    #[inline(always)]
    pub fn prot10(&mut self) -> PROT10_W {
        PROT10_W { w: self }
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase operations"]
    #[inline(always)]
    pub fn prot11(&mut self) -> PROT11_W {
        PROT11_W { w: self }
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase operations"]
    #[inline(always)]
    pub fn prot12(&mut self) -> PROT12_W {
        PROT12_W { w: self }
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase operations"]
    #[inline(always)]
    pub fn prot13(&mut self) -> PROT13_W {
        PROT13_W { w: self }
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase operations"]
    #[inline(always)]
    pub fn prot14(&mut self) -> PROT14_W {
        PROT14_W { w: self }
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase operations"]
    #[inline(always)]
    pub fn prot15(&mut self) -> PROT15_W {
        PROT15_W { w: self }
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase operations"]
    #[inline(always)]
    pub fn prot16(&mut self) -> PROT16_W {
        PROT16_W { w: self }
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase operations"]
    #[inline(always)]
    pub fn prot17(&mut self) -> PROT17_W {
        PROT17_W { w: self }
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase operations"]
    #[inline(always)]
    pub fn prot18(&mut self) -> PROT18_W {
        PROT18_W { w: self }
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase operations"]
    #[inline(always)]
    pub fn prot19(&mut self) -> PROT19_W {
        PROT19_W { w: self }
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase operations"]
    #[inline(always)]
    pub fn prot20(&mut self) -> PROT20_W {
        PROT20_W { w: self }
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase operations"]
    #[inline(always)]
    pub fn prot21(&mut self) -> PROT21_W {
        PROT21_W { w: self }
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase operations"]
    #[inline(always)]
    pub fn prot22(&mut self) -> PROT22_W {
        PROT22_W { w: self }
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase operations"]
    #[inline(always)]
    pub fn prot23(&mut self) -> PROT23_W {
        PROT23_W { w: self }
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase operations"]
    #[inline(always)]
    pub fn prot24(&mut self) -> PROT24_W {
        PROT24_W { w: self }
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase operations"]
    #[inline(always)]
    pub fn prot25(&mut self) -> PROT25_W {
        PROT25_W { w: self }
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase operations"]
    #[inline(always)]
    pub fn prot26(&mut self) -> PROT26_W {
        PROT26_W { w: self }
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase operations"]
    #[inline(always)]
    pub fn prot27(&mut self) -> PROT27_W {
        PROT27_W { w: self }
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase operations"]
    #[inline(always)]
    pub fn prot28(&mut self) -> PROT28_W {
        PROT28_W { w: self }
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase operations"]
    #[inline(always)]
    pub fn prot29(&mut self) -> PROT29_W {
        PROT29_W { w: self }
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase operations"]
    #[inline(always)]
    pub fn prot30(&mut self) -> PROT30_W {
        PROT30_W { w: self }
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase operations"]
    #[inline(always)]
    pub fn prot31(&mut self) -> PROT31_W {
        PROT31_W { w: self }
    }
}
