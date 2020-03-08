#[doc = "Reader of register SYS_SRAM_BANKEN"]
pub type R = crate::R<u32, super::SYS_SRAM_BANKEN>;
#[doc = "Writer for register SYS_SRAM_BANKEN"]
pub type W = crate::W<u32, super::SYS_SRAM_BANKEN>;
#[doc = "Register SYS_SRAM_BANKEN `reset()`'s with value 0xff"]
impl crate::ResetValue for super::SYS_SRAM_BANKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `BNK0_EN`"]
pub type BNK0_EN_R = crate::R<bool, bool>;
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK1_EN_A {
    #[doc = "0: Disables Bank1 of the SRAM"]
    BNK1_EN_0 = 0,
    #[doc = "1: Enables Bank1 of the SRAM"]
    BNK1_EN_1 = 1,
}
impl From<BNK1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK1_EN`"]
pub type BNK1_EN_R = crate::R<bool, BNK1_EN_A>;
impl BNK1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_EN_A {
        match self.bits {
            false => BNK1_EN_A::BNK1_EN_0,
            true => BNK1_EN_A::BNK1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_0`"]
    #[inline(always)]
    pub fn is_bnk1_en_0(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_1`"]
    #[inline(always)]
    pub fn is_bnk1_en_1(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_1
    }
}
#[doc = "Write proxy for field `BNK1_EN`"]
pub struct BNK1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_0(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_0)
    }
    #[doc = "Enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_1(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK2_EN_A {
    #[doc = "0: Disables Bank2 of the SRAM"]
    BNK2_EN_0 = 0,
    #[doc = "1: Enables Bank2 of the SRAM"]
    BNK2_EN_1 = 1,
}
impl From<BNK2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK2_EN`"]
pub type BNK2_EN_R = crate::R<bool, BNK2_EN_A>;
impl BNK2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_EN_A {
        match self.bits {
            false => BNK2_EN_A::BNK2_EN_0,
            true => BNK2_EN_A::BNK2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_0`"]
    #[inline(always)]
    pub fn is_bnk2_en_0(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_1`"]
    #[inline(always)]
    pub fn is_bnk2_en_1(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_1
    }
}
#[doc = "Write proxy for field `BNK2_EN`"]
pub struct BNK2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_0(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_0)
    }
    #[doc = "Enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_1(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK3_EN_A {
    #[doc = "0: Disables Bank3 of the SRAM"]
    BNK3_EN_0 = 0,
    #[doc = "1: Enables Bank3 of the SRAM"]
    BNK3_EN_1 = 1,
}
impl From<BNK3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK3_EN`"]
pub type BNK3_EN_R = crate::R<bool, BNK3_EN_A>;
impl BNK3_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_EN_A {
        match self.bits {
            false => BNK3_EN_A::BNK3_EN_0,
            true => BNK3_EN_A::BNK3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_0`"]
    #[inline(always)]
    pub fn is_bnk3_en_0(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_1`"]
    #[inline(always)]
    pub fn is_bnk3_en_1(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_1
    }
}
#[doc = "Write proxy for field `BNK3_EN`"]
pub struct BNK3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK3_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK3_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_0(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_0)
    }
    #[doc = "Enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_1(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK4_EN_A {
    #[doc = "0: Disables Bank4 of the SRAM"]
    BNK4_EN_0 = 0,
    #[doc = "1: Enables Bank4 of the SRAM"]
    BNK4_EN_1 = 1,
}
impl From<BNK4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK4_EN`"]
pub type BNK4_EN_R = crate::R<bool, BNK4_EN_A>;
impl BNK4_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_EN_A {
        match self.bits {
            false => BNK4_EN_A::BNK4_EN_0,
            true => BNK4_EN_A::BNK4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_0`"]
    #[inline(always)]
    pub fn is_bnk4_en_0(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_1`"]
    #[inline(always)]
    pub fn is_bnk4_en_1(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_1
    }
}
#[doc = "Write proxy for field `BNK4_EN`"]
pub struct BNK4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK4_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK4_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_0(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_0)
    }
    #[doc = "Enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_1(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK5_EN_A {
    #[doc = "0: Disables Bank5 of the SRAM"]
    BNK5_EN_0 = 0,
    #[doc = "1: Enables Bank5 of the SRAM"]
    BNK5_EN_1 = 1,
}
impl From<BNK5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK5_EN`"]
pub type BNK5_EN_R = crate::R<bool, BNK5_EN_A>;
impl BNK5_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_EN_A {
        match self.bits {
            false => BNK5_EN_A::BNK5_EN_0,
            true => BNK5_EN_A::BNK5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_0`"]
    #[inline(always)]
    pub fn is_bnk5_en_0(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_1`"]
    #[inline(always)]
    pub fn is_bnk5_en_1(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_1
    }
}
#[doc = "Write proxy for field `BNK5_EN`"]
pub struct BNK5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK5_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK5_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_0(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_0)
    }
    #[doc = "Enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_1(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK6_EN_A {
    #[doc = "0: Disables Bank6 of the SRAM"]
    BNK6_EN_0 = 0,
    #[doc = "1: Enables Bank6 of the SRAM"]
    BNK6_EN_1 = 1,
}
impl From<BNK6_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK6_EN`"]
pub type BNK6_EN_R = crate::R<bool, BNK6_EN_A>;
impl BNK6_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_EN_A {
        match self.bits {
            false => BNK6_EN_A::BNK6_EN_0,
            true => BNK6_EN_A::BNK6_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_0`"]
    #[inline(always)]
    pub fn is_bnk6_en_0(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_1`"]
    #[inline(always)]
    pub fn is_bnk6_en_1(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_1
    }
}
#[doc = "Write proxy for field `BNK6_EN`"]
pub struct BNK6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK6_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK6_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_0(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_0)
    }
    #[doc = "Enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_1(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_1)
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
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK7_EN_A {
    #[doc = "0: Disables Bank7 of the SRAM"]
    BNK7_EN_0 = 0,
    #[doc = "1: Enables Bank7 of the SRAM"]
    BNK7_EN_1 = 1,
}
impl From<BNK7_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BNK7_EN`"]
pub type BNK7_EN_R = crate::R<bool, BNK7_EN_A>;
impl BNK7_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_EN_A {
        match self.bits {
            false => BNK7_EN_A::BNK7_EN_0,
            true => BNK7_EN_A::BNK7_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_0`"]
    #[inline(always)]
    pub fn is_bnk7_en_0(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_1`"]
    #[inline(always)]
    pub fn is_bnk7_en_1(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_1
    }
}
#[doc = "Write proxy for field `BNK7_EN`"]
pub struct BNK7_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK7_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK7_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_0(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_0)
    }
    #[doc = "Enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_1(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_1)
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
    #[doc = "0: SRAM is not ready for accesses. Banks are undergoing an enable or disable sequence, and reads or writes to SRAM are stalled until the banks are ready"]
    SRAM_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled according to values of bits 7:0 of this register"]
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
    #[doc = "Bit 0 - SRAM Bank0 enable"]
    #[inline(always)]
    pub fn bnk0_en(&self) -> BNK0_EN_R {
        BNK0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&self) -> BNK1_EN_R {
        BNK1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&self) -> BNK2_EN_R {
        BNK2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&self) -> BNK3_EN_R {
        BNK3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&self) -> BNK4_EN_R {
        BNK4_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&self) -> BNK5_EN_R {
        BNK5_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&self) -> BNK6_EN_R {
        BNK6_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&self) -> BNK7_EN_R {
        BNK7_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SRAM_RDY_R {
        SRAM_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&mut self) -> BNK1_EN_W {
        BNK1_EN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&mut self) -> BNK2_EN_W {
        BNK2_EN_W { w: self }
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&mut self) -> BNK3_EN_W {
        BNK3_EN_W { w: self }
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&mut self) -> BNK4_EN_W {
        BNK4_EN_W { w: self }
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&mut self) -> BNK5_EN_W {
        BNK5_EN_W { w: self }
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&mut self) -> BNK6_EN_W {
        BNK6_EN_W { w: self }
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&mut self) -> BNK7_EN_W {
        BNK7_EN_W { w: self }
    }
}
