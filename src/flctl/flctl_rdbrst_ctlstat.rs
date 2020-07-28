#[doc = "Reader of register FLCTL_RDBRST_CTLSTAT"]
pub type R = crate::R<u32, super::FLCTL_RDBRST_CTLSTAT>;
#[doc = "Writer for register FLCTL_RDBRST_CTLSTAT"]
pub type W = crate::W<u32, super::FLCTL_RDBRST_CTLSTAT>;
#[doc = "Register FLCTL_RDBRST_CTLSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_RDBRST_CTLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Type of memory that burst is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_TYPE_A {
    #[doc = "0: Main Memory"]
    MEM_TYPE_0 = 0,
    #[doc = "1: Information Memory"]
    MEM_TYPE_1 = 1,
    #[doc = "3: Engineering Memory"]
    MEM_TYPE_3 = 3,
}
impl From<MEM_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEM_TYPE`"]
pub type MEM_TYPE_R = crate::R<u8, MEM_TYPE_A>;
impl MEM_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MEM_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MEM_TYPE_A::MEM_TYPE_0),
            1 => Val(MEM_TYPE_A::MEM_TYPE_1),
            3 => Val(MEM_TYPE_A::MEM_TYPE_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_0`"]
    #[inline(always)]
    pub fn is_mem_type_0(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_0
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_1`"]
    #[inline(always)]
    pub fn is_mem_type_1(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_1
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_3`"]
    #[inline(always)]
    pub fn is_mem_type_3(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_3
    }
}
#[doc = "Write proxy for field `MEM_TYPE`"]
pub struct MEM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn mem_type_0(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn mem_type_1(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn mem_type_3(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `STOP_FAIL`"]
pub type STOP_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_FAIL`"]
pub struct STOP_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_FAIL_W<'a> {
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
#[doc = "Data pattern used for comparison against memory read data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CMP_A {
    #[doc = "0: 0000_0000_0000_0000_0000_0000_0000_0000"]
    DATA_CMP_0 = 0,
    #[doc = "1: FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    DATA_CMP_1 = 1,
}
impl From<DATA_CMP_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_CMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_CMP`"]
pub type DATA_CMP_R = crate::R<bool, DATA_CMP_A>;
impl DATA_CMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_CMP_A {
        match self.bits {
            false => DATA_CMP_A::DATA_CMP_0,
            true => DATA_CMP_A::DATA_CMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_CMP_0`"]
    #[inline(always)]
    pub fn is_data_cmp_0(&self) -> bool {
        *self == DATA_CMP_A::DATA_CMP_0
    }
    #[doc = "Checks if the value of the field is `DATA_CMP_1`"]
    #[inline(always)]
    pub fn is_data_cmp_1(&self) -> bool {
        *self == DATA_CMP_A::DATA_CMP_1
    }
}
#[doc = "Write proxy for field `DATA_CMP`"]
pub struct DATA_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_CMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0000_0000_0000_0000_0000_0000_0000_0000"]
    #[inline(always)]
    pub fn data_cmp_0(self) -> &'a mut W {
        self.variant(DATA_CMP_A::DATA_CMP_0)
    }
    #[doc = "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    #[inline(always)]
    pub fn data_cmp_1(self) -> &'a mut W {
        self.variant(DATA_CMP_A::DATA_CMP_1)
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
#[doc = "Reader of field `TEST_EN`"]
pub type TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_EN`"]
pub struct TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_EN_W<'a> {
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
#[doc = "Status of Burst/Compare operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRST_STAT_A {
    #[doc = "0: Idle"]
    BRST_STAT_0 = 0,
    #[doc = "1: Burst/Compare START bit written, but operation pending"]
    BRST_STAT_1 = 1,
    #[doc = "2: Burst/Compare in progress"]
    BRST_STAT_2 = 2,
    #[doc = "3: Burst complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BRST_STAT_3 = 3,
}
impl From<BRST_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: BRST_STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BRST_STAT`"]
pub type BRST_STAT_R = crate::R<u8, BRST_STAT_A>;
impl BRST_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRST_STAT_A {
        match self.bits {
            0 => BRST_STAT_A::BRST_STAT_0,
            1 => BRST_STAT_A::BRST_STAT_1,
            2 => BRST_STAT_A::BRST_STAT_2,
            3 => BRST_STAT_A::BRST_STAT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_0`"]
    #[inline(always)]
    pub fn is_brst_stat_0(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_0
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_1`"]
    #[inline(always)]
    pub fn is_brst_stat_1(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_1
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_2`"]
    #[inline(always)]
    pub fn is_brst_stat_2(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_2
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_3`"]
    #[inline(always)]
    pub fn is_brst_stat_3(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_3
    }
}
#[doc = "Reader of field `CMP_ERR`"]
pub type CMP_ERR_R = crate::R<bool, bool>;
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
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&self) -> MEM_TYPE_R {
        MEM_TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&self) -> STOP_FAIL_R {
        STOP_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&self) -> DATA_CMP_R {
        DATA_CMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Status of Burst/Compare operation"]
    #[inline(always)]
    pub fn brst_stat(&self) -> BRST_STAT_R {
        BRST_STAT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Burst/Compare Operation encountered atleast one data"]
    #[inline(always)]
    pub fn cmp_err(&self) -> CMP_ERR_R {
        CMP_ERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Burst/Compare Operation was terminated due to access to"]
    #[inline(always)]
    pub fn addr_err(&self) -> ADDR_ERR_R {
        ADDR_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of burst/compare operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&mut self) -> MEM_TYPE_W {
        MEM_TYPE_W { w: self }
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&mut self) -> STOP_FAIL_W {
        STOP_FAIL_W { w: self }
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&mut self) -> DATA_CMP_W {
        DATA_CMP_W { w: self }
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W {
        TEST_EN_W { w: self }
    }
    #[doc = "Bit 23 - Clear status bits 19-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> CLR_STAT_W {
        CLR_STAT_W { w: self }
    }
}
