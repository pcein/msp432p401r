#[doc = "Reader of register TAxCCTL[%s]"]
pub type R = crate::R<u16, super::TAXCCTL>;
#[doc = "Writer for register TAxCCTL[%s]"]
pub type W = crate::W<u16, super::TAXCCTL>;
#[doc = "Register TAxCCTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TAXCCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIFG_A {
    #[doc = "0: No interrupt pending"]
    CCIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CCIFG_1 = 1,
}
impl From<CCIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CCIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIFG`"]
pub type CCIFG_R = crate::R<bool, CCIFG_A>;
impl CCIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIFG_A {
        match self.bits {
            false => CCIFG_A::CCIFG_0,
            true => CCIFG_A::CCIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIFG_0`"]
    #[inline(always)]
    pub fn is_ccifg_0(&self) -> bool {
        *self == CCIFG_A::CCIFG_0
    }
    #[doc = "Checks if the value of the field is `CCIFG_1`"]
    #[inline(always)]
    pub fn is_ccifg_1(&self) -> bool {
        *self == CCIFG_A::CCIFG_1
    }
}
#[doc = "Write proxy for field `CCIFG`"]
pub struct CCIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ccifg_0(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ccifg_1(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Capture overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COV_A {
    #[doc = "0: No capture overflow occurred"]
    COV_0 = 0,
    #[doc = "1: Capture overflow occurred"]
    COV_1 = 1,
}
impl From<COV_A> for bool {
    #[inline(always)]
    fn from(variant: COV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COV`"]
pub type COV_R = crate::R<bool, COV_A>;
impl COV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COV_A {
        match self.bits {
            false => COV_A::COV_0,
            true => COV_A::COV_1,
        }
    }
    #[doc = "Checks if the value of the field is `COV_0`"]
    #[inline(always)]
    pub fn is_cov_0(&self) -> bool {
        *self == COV_A::COV_0
    }
    #[doc = "Checks if the value of the field is `COV_1`"]
    #[inline(always)]
    pub fn is_cov_1(&self) -> bool {
        *self == COV_A::COV_1
    }
}
#[doc = "Write proxy for field `COV`"]
pub struct COV_W<'a> {
    w: &'a mut W,
}
impl<'a> COV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn cov_0(self) -> &'a mut W {
        self.variant(COV_A::COV_0)
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn cov_1(self) -> &'a mut W {
        self.variant(COV_A::COV_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_A {
    #[doc = "0: Output low"]
    OUT_0 = 0,
    #[doc = "1: Output high"]
    OUT_1 = 1,
}
impl From<OUT_A> for bool {
    #[inline(always)]
    fn from(variant: OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT`"]
pub type OUT_R = crate::R<bool, OUT_A>;
impl OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_A {
        match self.bits {
            false => OUT_A::OUT_0,
            true => OUT_A::OUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUT_0`"]
    #[inline(always)]
    pub fn is_out_0(&self) -> bool {
        *self == OUT_A::OUT_0
    }
    #[doc = "Checks if the value of the field is `OUT_1`"]
    #[inline(always)]
    pub fn is_out_1(&self) -> bool {
        *self == OUT_A::OUT_1
    }
}
#[doc = "Write proxy for field `OUT`"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output low"]
    #[inline(always)]
    pub fn out_0(self) -> &'a mut W {
        self.variant(OUT_A::OUT_0)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn out_1(self) -> &'a mut W {
        self.variant(OUT_A::OUT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CCI`"]
pub type CCI_R = crate::R<bool, bool>;
#[doc = "Capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Interrupt disabled"]
    CCIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CCIE_1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::CCIE_0,
            true => CCIE_A::CCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIE_0`"]
    #[inline(always)]
    pub fn is_ccie_0(&self) -> bool {
        *self == CCIE_A::CCIE_0
    }
    #[doc = "Checks if the value of the field is `CCIE_1`"]
    #[inline(always)]
    pub fn is_ccie_1(&self) -> bool {
        *self == CCIE_A::CCIE_1
    }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ccie_0(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ccie_1(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: OUT bit value"]
    OUTMOD_0 = 0,
    #[doc = "1: Set"]
    OUTMOD_1 = 1,
    #[doc = "2: Toggle/reset"]
    OUTMOD_2 = 2,
    #[doc = "3: Set/reset"]
    OUTMOD_3 = 3,
    #[doc = "4: Toggle"]
    OUTMOD_4 = 4,
    #[doc = "5: Reset"]
    OUTMOD_5 = 5,
    #[doc = "6: Toggle/set"]
    OUTMOD_6 = 6,
    #[doc = "7: Reset/set"]
    OUTMOD_7 = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTMOD`"]
pub type OUTMOD_R = crate::R<u8, OUTMOD_A>;
impl OUTMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::OUTMOD_0,
            1 => OUTMOD_A::OUTMOD_1,
            2 => OUTMOD_A::OUTMOD_2,
            3 => OUTMOD_A::OUTMOD_3,
            4 => OUTMOD_A::OUTMOD_4,
            5 => OUTMOD_A::OUTMOD_5,
            6 => OUTMOD_A::OUTMOD_6,
            7 => OUTMOD_A::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMOD_0`"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_0
    }
    #[doc = "Checks if the value of the field is `OUTMOD_1`"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_1
    }
    #[doc = "Checks if the value of the field is `OUTMOD_2`"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_2
    }
    #[doc = "Checks if the value of the field is `OUTMOD_3`"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_3
    }
    #[doc = "Checks if the value of the field is `OUTMOD_4`"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_4
    }
    #[doc = "Checks if the value of the field is `OUTMOD_5`"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_5
    }
    #[doc = "Checks if the value of the field is `OUTMOD_6`"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_6
    }
    #[doc = "Checks if the value of the field is `OUTMOD_7`"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_7
    }
}
#[doc = "Write proxy for field `OUTMOD`"]
pub struct OUTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_0)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_1)
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_2)
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_3)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_4)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_5)
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_6)
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP_A {
    #[doc = "0: Compare mode"]
    CAP_0 = 0,
    #[doc = "1: Capture mode"]
    CAP_1 = 1,
}
impl From<CAP_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<bool, CAP_A>;
impl CAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP_A {
        match self.bits {
            false => CAP_A::CAP_0,
            true => CAP_A::CAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAP_0`"]
    #[inline(always)]
    pub fn is_cap_0(&self) -> bool {
        *self == CAP_A::CAP_0
    }
    #[doc = "Checks if the value of the field is `CAP_1`"]
    #[inline(always)]
    pub fn is_cap_1(&self) -> bool {
        *self == CAP_A::CAP_1
    }
}
#[doc = "Write proxy for field `CAP`"]
pub struct CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn cap_0(self) -> &'a mut W {
        self.variant(CAP_A::CAP_0)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn cap_1(self) -> &'a mut W {
        self.variant(CAP_A::CAP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCCI`"]
pub type SCCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCCI`"]
pub struct SCCI_W<'a> {
    w: &'a mut W,
}
impl<'a> SCCI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Synchronize capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCS_A {
    #[doc = "0: Asynchronous capture"]
    SCS_0 = 0,
    #[doc = "1: Synchronous capture"]
    SCS_1 = 1,
}
impl From<SCS_A> for bool {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<bool, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            false => SCS_A::SCS_0,
            true => SCS_A::SCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCS_0`"]
    #[inline(always)]
    pub fn is_scs_0(&self) -> bool {
        *self == SCS_A::SCS_0
    }
    #[doc = "Checks if the value of the field is `SCS_1`"]
    #[inline(always)]
    pub fn is_scs_1(&self) -> bool {
        *self == SCS_A::SCS_1
    }
}
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn scs_0(self) -> &'a mut W {
        self.variant(SCS_A::SCS_0)
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn scs_1(self) -> &'a mut W {
        self.variant(SCS_A::SCS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Capture/compare input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCIS_A {
    #[doc = "0: CCIxA"]
    CCIS_0 = 0,
    #[doc = "1: CCIxB"]
    CCIS_1 = 1,
    #[doc = "2: GND"]
    CCIS_2 = 2,
    #[doc = "3: VCC"]
    CCIS_3 = 3,
}
impl From<CCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCIS`"]
pub type CCIS_R = crate::R<u8, CCIS_A>;
impl CCIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIS_A {
        match self.bits {
            0 => CCIS_A::CCIS_0,
            1 => CCIS_A::CCIS_1,
            2 => CCIS_A::CCIS_2,
            3 => CCIS_A::CCIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCIS_0`"]
    #[inline(always)]
    pub fn is_ccis_0(&self) -> bool {
        *self == CCIS_A::CCIS_0
    }
    #[doc = "Checks if the value of the field is `CCIS_1`"]
    #[inline(always)]
    pub fn is_ccis_1(&self) -> bool {
        *self == CCIS_A::CCIS_1
    }
    #[doc = "Checks if the value of the field is `CCIS_2`"]
    #[inline(always)]
    pub fn is_ccis_2(&self) -> bool {
        *self == CCIS_A::CCIS_2
    }
    #[doc = "Checks if the value of the field is `CCIS_3`"]
    #[inline(always)]
    pub fn is_ccis_3(&self) -> bool {
        *self == CCIS_A::CCIS_3
    }
}
#[doc = "Write proxy for field `CCIS`"]
pub struct CCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn ccis_0(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_0)
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn ccis_1(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_1)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn ccis_2(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_2)
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn ccis_3(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No capture"]
    CM_0 = 0,
    #[doc = "1: Capture on rising edge"]
    CM_1 = 1,
    #[doc = "2: Capture on falling edge"]
    CM_2 = 2,
    #[doc = "3: Capture on both rising and falling edges"]
    CM_3 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::CM_0,
            1 => CM_A::CM_1,
            2 => CM_A::CM_2,
            3 => CM_A::CM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_1`"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == CM_A::CM_1
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No capture"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut W {
        self.variant(CM_A::CM_1)
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CM_A::CM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/compare input"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    pub fn scci(&self) -> SCCI_R {
        SCCI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&self) -> CCIS_R {
        CCIS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CCIFG_W {
        CCIFG_W { w: self }
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&mut self) -> COV_W {
        COV_W { w: self }
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OUTMOD_W {
        OUTMOD_W { w: self }
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&mut self) -> CAP_W {
        CAP_W { w: self }
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    pub fn scci(&mut self) -> SCCI_W {
        SCCI_W { w: self }
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CCIS_W {
        CCIS_W { w: self }
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
