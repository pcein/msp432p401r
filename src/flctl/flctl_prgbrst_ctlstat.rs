#[doc = "Reader of register FLCTL_PRGBRST_CTLSTAT"]
pub type R = crate::R<u32, super::FLCTL_PRGBRST_CTLSTAT>;
#[doc = "Writer for register FLCTL_PRGBRST_CTLSTAT"]
pub type W = crate::W<u32, super::FLCTL_PRGBRST_CTLSTAT>;
#[doc = "Register FLCTL_PRGBRST_CTLSTAT `reset()`'s with value 0xc0"]
impl crate::ResetValue for super::FLCTL_PRGBRST_CTLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc0
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Type of memory that burst program is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Main Memory"]
    TYPE_0 = 0,
    #[doc = "1: Information Memory"]
    TYPE_1 = 1,
    #[doc = "3: Engineering Memory"]
    TYPE_3 = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPE_A::TYPE_0),
            1 => Val(TYPE_A::TYPE_1),
            3 => Val(TYPE_A::TYPE_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_0`"]
    #[inline(always)]
    pub fn is_type_0(&self) -> bool {
        *self == TYPE_A::TYPE_0
    }
    #[doc = "Checks if the value of the field is `TYPE_1`"]
    #[inline(always)]
    pub fn is_type_1(&self) -> bool {
        *self == TYPE_A::TYPE_1
    }
    #[doc = "Checks if the value of the field is `TYPE_3`"]
    #[inline(always)]
    pub fn is_type_3(&self) -> bool {
        *self == TYPE_A::TYPE_3
    }
}
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn type_0(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn type_1(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn type_3(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Length of burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: No burst operation"]
    LEN_0 = 0,
    #[doc = "1: 1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_1 = 1,
    #[doc = "2: 2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_2 = 2,
    #[doc = "3: 3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_3 = 3,
    #[doc = "4: 4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_4 = 4,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u8, LEN_A>;
impl LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEN_A::LEN_0),
            1 => Val(LEN_A::LEN_1),
            2 => Val(LEN_A::LEN_2),
            3 => Val(LEN_A::LEN_3),
            4 => Val(LEN_A::LEN_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEN_0`"]
    #[inline(always)]
    pub fn is_len_0(&self) -> bool {
        *self == LEN_A::LEN_0
    }
    #[doc = "Checks if the value of the field is `LEN_1`"]
    #[inline(always)]
    pub fn is_len_1(&self) -> bool {
        *self == LEN_A::LEN_1
    }
    #[doc = "Checks if the value of the field is `LEN_2`"]
    #[inline(always)]
    pub fn is_len_2(&self) -> bool {
        *self == LEN_A::LEN_2
    }
    #[doc = "Checks if the value of the field is `LEN_3`"]
    #[inline(always)]
    pub fn is_len_3(&self) -> bool {
        *self == LEN_A::LEN_3
    }
    #[doc = "Checks if the value of the field is `LEN_4`"]
    #[inline(always)]
    pub fn is_len_4(&self) -> bool {
        *self == LEN_A::LEN_4
    }
}
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No burst operation"]
    #[inline(always)]
    pub fn len_0(self) -> &'a mut W {
        self.variant(LEN_A::LEN_0)
    }
    #[doc = "1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_1(self) -> &'a mut W {
        self.variant(LEN_A::LEN_1)
    }
    #[doc = "2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_2(self) -> &'a mut W {
        self.variant(LEN_A::LEN_2)
    }
    #[doc = "3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_3(self) -> &'a mut W {
        self.variant(LEN_A::LEN_3)
    }
    #[doc = "4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_4(self) -> &'a mut W {
        self.variant(LEN_A::LEN_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Auto-Verify operation before the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_PRE_A {
    #[doc = "0: No program verify operations carried out"]
    AUTO_PRE_0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify after the Burst Program Operation"]
    AUTO_PRE_1 = 1,
}
impl From<AUTO_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_PRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO_PRE`"]
pub type AUTO_PRE_R = crate::R<bool, AUTO_PRE_A>;
impl AUTO_PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_PRE_A {
        match self.bits {
            false => AUTO_PRE_A::AUTO_PRE_0,
            true => AUTO_PRE_A::AUTO_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_PRE_0`"]
    #[inline(always)]
    pub fn is_auto_pre_0(&self) -> bool {
        *self == AUTO_PRE_A::AUTO_PRE_0
    }
    #[doc = "Checks if the value of the field is `AUTO_PRE_1`"]
    #[inline(always)]
    pub fn is_auto_pre_1(&self) -> bool {
        *self == AUTO_PRE_A::AUTO_PRE_1
    }
}
#[doc = "Write proxy for field `AUTO_PRE`"]
pub struct AUTO_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_PRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pre_0(self) -> &'a mut W {
        self.variant(AUTO_PRE_A::AUTO_PRE_0)
    }
    #[doc = "Causes an automatic Burst Program Verify after the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pre_1(self) -> &'a mut W {
        self.variant(AUTO_PRE_A::AUTO_PRE_1)
    }
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
#[doc = "Auto-Verify operation after the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_PST_A {
    #[doc = "0: No program verify operations carried out"]
    AUTO_PST_0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify before the Burst Program Operation"]
    AUTO_PST_1 = 1,
}
impl From<AUTO_PST_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_PST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO_PST`"]
pub type AUTO_PST_R = crate::R<bool, AUTO_PST_A>;
impl AUTO_PST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_PST_A {
        match self.bits {
            false => AUTO_PST_A::AUTO_PST_0,
            true => AUTO_PST_A::AUTO_PST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_PST_0`"]
    #[inline(always)]
    pub fn is_auto_pst_0(&self) -> bool {
        *self == AUTO_PST_A::AUTO_PST_0
    }
    #[doc = "Checks if the value of the field is `AUTO_PST_1`"]
    #[inline(always)]
    pub fn is_auto_pst_1(&self) -> bool {
        *self == AUTO_PST_A::AUTO_PST_1
    }
}
#[doc = "Write proxy for field `AUTO_PST`"]
pub struct AUTO_PST_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_PST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_PST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pst_0(self) -> &'a mut W {
        self.variant(AUTO_PST_A::AUTO_PST_0)
    }
    #[doc = "Causes an automatic Burst Program Verify before the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pst_1(self) -> &'a mut W {
        self.variant(AUTO_PST_A::AUTO_PST_1)
    }
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
#[doc = "Status of a Burst Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_STATUS_A {
    #[doc = "0: Idle (Burst not active)"]
    BURST_STATUS_0 = 0,
    #[doc = "1: Burst program started but pending"]
    BURST_STATUS_1 = 1,
    #[doc = "2: Burst active, with 1st 128 bit word being written into Flash"]
    BURST_STATUS_2 = 2,
    #[doc = "3: Burst active, with 2nd 128 bit word being written into Flash"]
    BURST_STATUS_3 = 3,
    #[doc = "4: Burst active, with 3rd 128 bit word being written into Flash"]
    BURST_STATUS_4 = 4,
    #[doc = "5: Burst active, with 4th 128 bit word being written into Flash"]
    BURST_STATUS_5 = 5,
    #[doc = "7: Burst Complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BURST_STATUS_7 = 7,
}
impl From<BURST_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURST_STATUS`"]
pub type BURST_STATUS_R = crate::R<u8, BURST_STATUS_A>;
impl BURST_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BURST_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BURST_STATUS_A::BURST_STATUS_0),
            1 => Val(BURST_STATUS_A::BURST_STATUS_1),
            2 => Val(BURST_STATUS_A::BURST_STATUS_2),
            3 => Val(BURST_STATUS_A::BURST_STATUS_3),
            4 => Val(BURST_STATUS_A::BURST_STATUS_4),
            5 => Val(BURST_STATUS_A::BURST_STATUS_5),
            7 => Val(BURST_STATUS_A::BURST_STATUS_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_0`"]
    #[inline(always)]
    pub fn is_burst_status_0(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_0
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_1`"]
    #[inline(always)]
    pub fn is_burst_status_1(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_1
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_2`"]
    #[inline(always)]
    pub fn is_burst_status_2(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_2
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_3`"]
    #[inline(always)]
    pub fn is_burst_status_3(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_3
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_4`"]
    #[inline(always)]
    pub fn is_burst_status_4(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_4
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_5`"]
    #[inline(always)]
    pub fn is_burst_status_5(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_5
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_7`"]
    #[inline(always)]
    pub fn is_burst_status_7(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_7
    }
}
#[doc = "Reader of field `PRE_ERR`"]
pub type PRE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PST_ERR`"]
pub type PST_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDR_ERR`"]
pub type ADDR_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_STAT`"]
pub struct CLR_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_STAT_W<'a> {
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
impl R {
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&self) -> AUTO_PRE_R {
        AUTO_PRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&self) -> AUTO_PST_R {
        AUTO_PST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Status of a Burst Operation"]
    #[inline(always)]
    pub fn burst_status(&self) -> BURST_STATUS_R {
        BURST_STATUS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Burst Operation encountered preprogram auto-verify errors"]
    #[inline(always)]
    pub fn pre_err(&self) -> PRE_ERR_R {
        PRE_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Burst Operation encountered postprogram auto-verify errors"]
    #[inline(always)]
    pub fn pst_err(&self) -> PST_ERR_R {
        PST_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Burst Operation was terminated due to attempted program of reserved memory"]
    #[inline(always)]
    pub fn addr_err(&self) -> ADDR_ERR_R {
        ADDR_ERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger start of burst program operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&mut self) -> AUTO_PRE_W {
        AUTO_PRE_W { w: self }
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&mut self) -> AUTO_PST_W {
        AUTO_PST_W { w: self }
    }
    #[doc = "Bit 23 - Clear status bits 21-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> CLR_STAT_W {
        CLR_STAT_W { w: self }
    }
}
