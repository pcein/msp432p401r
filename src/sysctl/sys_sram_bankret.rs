#[doc = "Reader of register SYS_SRAM_BANKRET"]
pub type R = crate::R<u32, super::SYS_SRAM_BANKRET>;
#[doc = "Writer for register SYS_SRAM_BANKRET"]
pub type W = crate::W<u32, super::SYS_SRAM_BANKRET>;
#[doc = "Register SYS_SRAM_BANKRET `reset()`'s with value 0xff"]
impl crate::ResetValue for super::SYS_SRAM_BANKRET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `BNK0_RET`"]
pub type BNK0_RET_R = crate::R<bool, bool>;
#[doc = "Bank1 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK1_RET_A {
    #[doc = "0: Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    BNK1_RET_0 = 0,
    #[doc = "1: Bank1 of the SRAM is retained in LPM3 and LPM4"]
    BNK1_RET_1 = 1,
}
impl From<BNK1_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK1_RET`"]
pub type BNK1_RET_R = crate::R<bool, BNK1_RET_A>;
impl BNK1_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_RET_A {
        match self.bits {
            false => BNK1_RET_A::BNK1_RET_0,
            true => BNK1_RET_A::BNK1_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_0`"]
    #[inline(always)]
    pub fn is_bnk1_ret_0(&self) -> bool {
        *self == BNK1_RET_A::BNK1_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_1`"]
    #[inline(always)]
    pub fn is_bnk1_ret_1(&self) -> bool {
        *self == BNK1_RET_A::BNK1_RET_1
    }
}
#[doc = "Write proxy for field `BNK1_RET`"]
pub struct BNK1_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK1_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK1_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_0(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_0)
    }
    #[doc = "Bank1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_1(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_1)
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
#[doc = "Bank2 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK2_RET_A {
    #[doc = "0: Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    BNK2_RET_0 = 0,
    #[doc = "1: Bank2 of the SRAM is retained in LPM3 and LPM4"]
    BNK2_RET_1 = 1,
}
impl From<BNK2_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK2_RET`"]
pub type BNK2_RET_R = crate::R<bool, BNK2_RET_A>;
impl BNK2_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_RET_A {
        match self.bits {
            false => BNK2_RET_A::BNK2_RET_0,
            true => BNK2_RET_A::BNK2_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_0`"]
    #[inline(always)]
    pub fn is_bnk2_ret_0(&self) -> bool {
        *self == BNK2_RET_A::BNK2_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_1`"]
    #[inline(always)]
    pub fn is_bnk2_ret_1(&self) -> bool {
        *self == BNK2_RET_A::BNK2_RET_1
    }
}
#[doc = "Write proxy for field `BNK2_RET`"]
pub struct BNK2_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK2_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK2_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_0(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_0)
    }
    #[doc = "Bank2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_1(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_1)
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
#[doc = "Bank3 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK3_RET_A {
    #[doc = "0: Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    BNK3_RET_0 = 0,
    #[doc = "1: Bank3 of the SRAM is retained in LPM3 and LPM4"]
    BNK3_RET_1 = 1,
}
impl From<BNK3_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK3_RET`"]
pub type BNK3_RET_R = crate::R<bool, BNK3_RET_A>;
impl BNK3_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_RET_A {
        match self.bits {
            false => BNK3_RET_A::BNK3_RET_0,
            true => BNK3_RET_A::BNK3_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_0`"]
    #[inline(always)]
    pub fn is_bnk3_ret_0(&self) -> bool {
        *self == BNK3_RET_A::BNK3_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_1`"]
    #[inline(always)]
    pub fn is_bnk3_ret_1(&self) -> bool {
        *self == BNK3_RET_A::BNK3_RET_1
    }
}
#[doc = "Write proxy for field `BNK3_RET`"]
pub struct BNK3_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK3_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK3_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_0(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_0)
    }
    #[doc = "Bank3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_1(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_1)
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
#[doc = "Bank4 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK4_RET_A {
    #[doc = "0: Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    BNK4_RET_0 = 0,
    #[doc = "1: Bank4 of the SRAM is retained in LPM3 and LPM4"]
    BNK4_RET_1 = 1,
}
impl From<BNK4_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK4_RET`"]
pub type BNK4_RET_R = crate::R<bool, BNK4_RET_A>;
impl BNK4_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_RET_A {
        match self.bits {
            false => BNK4_RET_A::BNK4_RET_0,
            true => BNK4_RET_A::BNK4_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_0`"]
    #[inline(always)]
    pub fn is_bnk4_ret_0(&self) -> bool {
        *self == BNK4_RET_A::BNK4_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_1`"]
    #[inline(always)]
    pub fn is_bnk4_ret_1(&self) -> bool {
        *self == BNK4_RET_A::BNK4_RET_1
    }
}
#[doc = "Write proxy for field `BNK4_RET`"]
pub struct BNK4_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK4_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK4_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_0(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_0)
    }
    #[doc = "Bank4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_1(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Bank5 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK5_RET_A {
    #[doc = "0: Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    BNK5_RET_0 = 0,
    #[doc = "1: Bank5 of the SRAM is retained in LPM3 and LPM4"]
    BNK5_RET_1 = 1,
}
impl From<BNK5_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK5_RET`"]
pub type BNK5_RET_R = crate::R<bool, BNK5_RET_A>;
impl BNK5_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_RET_A {
        match self.bits {
            false => BNK5_RET_A::BNK5_RET_0,
            true => BNK5_RET_A::BNK5_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_0`"]
    #[inline(always)]
    pub fn is_bnk5_ret_0(&self) -> bool {
        *self == BNK5_RET_A::BNK5_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_1`"]
    #[inline(always)]
    pub fn is_bnk5_ret_1(&self) -> bool {
        *self == BNK5_RET_A::BNK5_RET_1
    }
}
#[doc = "Write proxy for field `BNK5_RET`"]
pub struct BNK5_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK5_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK5_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_0(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_0)
    }
    #[doc = "Bank5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_1(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Bank6 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK6_RET_A {
    #[doc = "0: Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    BNK6_RET_0 = 0,
    #[doc = "1: Bank6 of the SRAM is retained in LPM3 and LPM4"]
    BNK6_RET_1 = 1,
}
impl From<BNK6_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK6_RET`"]
pub type BNK6_RET_R = crate::R<bool, BNK6_RET_A>;
impl BNK6_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_RET_A {
        match self.bits {
            false => BNK6_RET_A::BNK6_RET_0,
            true => BNK6_RET_A::BNK6_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_0`"]
    #[inline(always)]
    pub fn is_bnk6_ret_0(&self) -> bool {
        *self == BNK6_RET_A::BNK6_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_1`"]
    #[inline(always)]
    pub fn is_bnk6_ret_1(&self) -> bool {
        *self == BNK6_RET_A::BNK6_RET_1
    }
}
#[doc = "Write proxy for field `BNK6_RET`"]
pub struct BNK6_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK6_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK6_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_0(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_0)
    }
    #[doc = "Bank6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_1(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_1)
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
#[doc = "Bank7 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK7_RET_A {
    #[doc = "0: Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    BNK7_RET_0 = 0,
    #[doc = "1: Bank7 of the SRAM is retained in LPM3 and LPM4"]
    BNK7_RET_1 = 1,
}
impl From<BNK7_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK7_RET`"]
pub type BNK7_RET_R = crate::R<bool, BNK7_RET_A>;
impl BNK7_RET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_RET_A {
        match self.bits {
            false => BNK7_RET_A::BNK7_RET_0,
            true => BNK7_RET_A::BNK7_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_0`"]
    #[inline(always)]
    pub fn is_bnk7_ret_0(&self) -> bool {
        *self == BNK7_RET_A::BNK7_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_1`"]
    #[inline(always)]
    pub fn is_bnk7_ret_1(&self) -> bool {
        *self == BNK7_RET_A::BNK7_RET_1
    }
}
#[doc = "Write proxy for field `BNK7_RET`"]
pub struct BNK7_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK7_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK7_RET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_0(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_0)
    }
    #[doc = "Bank7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_1(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_1)
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
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RDY_A {
    #[doc = "0: SRAM banks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    SRAM_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled for retention according to values of bits 7:0 of this register"]
    SRAM_RDY_1 = 1,
}
impl From<SRAM_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM_RDY`"]
pub type SRAM_RDY_R = crate::R<bool, SRAM_RDY_A>;
impl SRAM_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_RDY_A {
        match self.bits {
            false => SRAM_RDY_A::SRAM_RDY_0,
            true => SRAM_RDY_A::SRAM_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_0`"]
    #[inline(always)]
    pub fn is_sram_rdy_0(&self) -> bool {
        *self == SRAM_RDY_A::SRAM_RDY_0
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_1`"]
    #[inline(always)]
    pub fn is_sram_rdy_1(&self) -> bool {
        *self == SRAM_RDY_A::SRAM_RDY_1
    }
}
impl R {
    #[doc = "Bit 0 - Bank0 retention"]
    #[inline(always)]
    pub fn bnk0_ret(&self) -> BNK0_RET_R {
        BNK0_RET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&self) -> BNK1_RET_R {
        BNK1_RET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&self) -> BNK2_RET_R {
        BNK2_RET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&self) -> BNK3_RET_R {
        BNK3_RET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&self) -> BNK4_RET_R {
        BNK4_RET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&self) -> BNK5_RET_R {
        BNK5_RET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&self) -> BNK6_RET_R {
        BNK6_RET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&self) -> BNK7_RET_R {
        BNK7_RET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SRAM_RDY_R {
        SRAM_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&mut self) -> BNK1_RET_W {
        BNK1_RET_W { w: self }
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&mut self) -> BNK2_RET_W {
        BNK2_RET_W { w: self }
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&mut self) -> BNK3_RET_W {
        BNK3_RET_W { w: self }
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&mut self) -> BNK4_RET_W {
        BNK4_RET_W { w: self }
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&mut self) -> BNK5_RET_W {
        BNK5_RET_W { w: self }
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&mut self) -> BNK6_RET_W {
        BNK6_RET_W { w: self }
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&mut self) -> BNK7_RET_W {
        BNK7_RET_W { w: self }
    }
}
