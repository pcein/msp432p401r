#[doc = "Reader of register FLCTL_PRG_CTLSTAT"]
pub type R = crate::R<u32, super::FLCTL_PRG_CTLSTAT>;
#[doc = "Writer for register FLCTL_PRG_CTLSTAT"]
pub type W = crate::W<u32, super::FLCTL_PRG_CTLSTAT>;
#[doc = "Register FLCTL_PRG_CTLSTAT `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::FLCTL_PRG_CTLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Master control for all word program operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Word program operation disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Word program operation enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLE_A::ENABLE_1
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word program operation disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Word program operation enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Write immediate mode. Starts program operation immediately on each write to the Flash"]
    MODE_0 = 0,
    #[doc = "1: Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write immediate mode. Starts program operation immediately on each write to the Flash"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Controls automatic pre program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_PRE_A {
    #[doc = "0: No pre program verification"]
    VER_PRE_0 = 0,
    #[doc = "1: Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VER_PRE_1 = 1,
}
impl From<VER_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: VER_PRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VER_PRE`"]
pub type VER_PRE_R = crate::R<bool, VER_PRE_A>;
impl VER_PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_PRE_A {
        match self.bits {
            false => VER_PRE_A::VER_PRE_0,
            true => VER_PRE_A::VER_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VER_PRE_0`"]
    #[inline(always)]
    pub fn is_ver_pre_0(&self) -> bool {
        *self == VER_PRE_A::VER_PRE_0
    }
    #[doc = "Checks if the value of the field is `VER_PRE_1`"]
    #[inline(always)]
    pub fn is_ver_pre_1(&self) -> bool {
        *self == VER_PRE_A::VER_PRE_1
    }
}
#[doc = "Write proxy for field `VER_PRE`"]
pub struct VER_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VER_PRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No pre program verification"]
    #[inline(always)]
    pub fn ver_pre_0(self) -> &'a mut W {
        self.variant(VER_PRE_A::VER_PRE_0)
    }
    #[doc = "Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pre_1(self) -> &'a mut W {
        self.variant(VER_PRE_A::VER_PRE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Controls automatic post program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_PST_A {
    #[doc = "0: No post program verification"]
    VER_PST_0 = 0,
    #[doc = "1: Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VER_PST_1 = 1,
}
impl From<VER_PST_A> for bool {
    #[inline(always)]
    fn from(variant: VER_PST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VER_PST`"]
pub type VER_PST_R = crate::R<bool, VER_PST_A>;
impl VER_PST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_PST_A {
        match self.bits {
            false => VER_PST_A::VER_PST_0,
            true => VER_PST_A::VER_PST_1,
        }
    }
    #[doc = "Checks if the value of the field is `VER_PST_0`"]
    #[inline(always)]
    pub fn is_ver_pst_0(&self) -> bool {
        *self == VER_PST_A::VER_PST_0
    }
    #[doc = "Checks if the value of the field is `VER_PST_1`"]
    #[inline(always)]
    pub fn is_ver_pst_1(&self) -> bool {
        *self == VER_PST_A::VER_PST_1
    }
}
#[doc = "Write proxy for field `VER_PST`"]
pub struct VER_PST_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_PST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VER_PST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No post program verification"]
    #[inline(always)]
    pub fn ver_pst_0(self) -> &'a mut W {
        self.variant(VER_PST_A::VER_PST_0)
    }
    #[doc = "Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pst_1(self) -> &'a mut W {
        self.variant(VER_PST_A::VER_PST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Status of program operations in the Flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_A {
    #[doc = "0: Idle (no program operation currently active)"]
    STATUS_0 = 0,
    #[doc = "1: Single word program operation triggered, but pending"]
    STATUS_1 = 1,
    #[doc = "2: Single word program in progress"]
    STATUS_2 = 2,
}
impl From<STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u8, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATUS_A::STATUS_0),
            1 => Val(STATUS_A::STATUS_1),
            2 => Val(STATUS_A::STATUS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == STATUS_A::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == STATUS_A::STATUS_1
    }
    #[doc = "Checks if the value of the field is `STATUS_2`"]
    #[inline(always)]
    pub fn is_status_2(&self) -> bool {
        *self == STATUS_A::STATUS_2
    }
}
#[doc = "Bank active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK_ACT_A {
    #[doc = "0: Word in Bank0 being programmed"]
    BNK_ACT_0 = 0,
    #[doc = "1: Word in Bank1 being programmed"]
    BNK_ACT_1 = 1,
}
impl From<BNK_ACT_A> for bool {
    #[inline(always)]
    fn from(variant: BNK_ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK_ACT`"]
pub type BNK_ACT_R = crate::R<bool, BNK_ACT_A>;
impl BNK_ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK_ACT_A {
        match self.bits {
            false => BNK_ACT_A::BNK_ACT_0,
            true => BNK_ACT_A::BNK_ACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK_ACT_0`"]
    #[inline(always)]
    pub fn is_bnk_act_0(&self) -> bool {
        *self == BNK_ACT_A::BNK_ACT_0
    }
    #[doc = "Checks if the value of the field is `BNK_ACT_1`"]
    #[inline(always)]
    pub fn is_bnk_act_1(&self) -> bool {
        *self == BNK_ACT_A::BNK_ACT_1
    }
}
impl R {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&self) -> VER_PRE_R {
        VER_PRE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&self) -> VER_PST_R {
        VER_PST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Status of program operations in the Flash memory"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Bank active"]
    #[inline(always)]
    pub fn bnk_act(&self) -> BNK_ACT_R {
        BNK_ACT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&mut self) -> VER_PRE_W {
        VER_PRE_W { w: self }
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&mut self) -> VER_PST_W {
        VER_PST_W { w: self }
    }
}
