#[doc = "Reader of register CExCTL2"]
pub type R = crate::R<u16, super::CEXCTL2>;
#[doc = "Writer for register CExCTL2"]
pub type W = crate::W<u16, super::CEXCTL2>;
#[doc = "Register CExCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CEXCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reference resistor tap 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREF0_A {
    #[doc = "0: Reference resistor tap for setting 0."]
    CEREF0_0 = 0,
    #[doc = "1: Reference resistor tap for setting 1."]
    CEREF0_1 = 1,
    #[doc = "2: Reference resistor tap for setting 2."]
    CEREF0_2 = 2,
    #[doc = "3: Reference resistor tap for setting 3."]
    CEREF0_3 = 3,
    #[doc = "4: Reference resistor tap for setting 4."]
    CEREF0_4 = 4,
    #[doc = "5: Reference resistor tap for setting 5."]
    CEREF0_5 = 5,
    #[doc = "6: Reference resistor tap for setting 6."]
    CEREF0_6 = 6,
    #[doc = "7: Reference resistor tap for setting 7."]
    CEREF0_7 = 7,
    #[doc = "8: Reference resistor tap for setting 8."]
    CEREF0_8 = 8,
    #[doc = "9: Reference resistor tap for setting 9."]
    CEREF0_9 = 9,
    #[doc = "10: Reference resistor tap for setting 10."]
    CEREF0_10 = 10,
    #[doc = "11: Reference resistor tap for setting 11."]
    CEREF0_11 = 11,
    #[doc = "12: Reference resistor tap for setting 12."]
    CEREF0_12 = 12,
    #[doc = "13: Reference resistor tap for setting 13."]
    CEREF0_13 = 13,
    #[doc = "14: Reference resistor tap for setting 14."]
    CEREF0_14 = 14,
    #[doc = "15: Reference resistor tap for setting 15."]
    CEREF0_15 = 15,
    #[doc = "16: Reference resistor tap for setting 16."]
    CEREF0_16 = 16,
    #[doc = "17: Reference resistor tap for setting 17."]
    CEREF0_17 = 17,
    #[doc = "18: Reference resistor tap for setting 18."]
    CEREF0_18 = 18,
    #[doc = "19: Reference resistor tap for setting 19."]
    CEREF0_19 = 19,
    #[doc = "20: Reference resistor tap for setting 20."]
    CEREF0_20 = 20,
    #[doc = "21: Reference resistor tap for setting 21."]
    CEREF0_21 = 21,
    #[doc = "22: Reference resistor tap for setting 22."]
    CEREF0_22 = 22,
    #[doc = "23: Reference resistor tap for setting 23."]
    CEREF0_23 = 23,
    #[doc = "24: Reference resistor tap for setting 24."]
    CEREF0_24 = 24,
    #[doc = "25: Reference resistor tap for setting 25."]
    CEREF0_25 = 25,
    #[doc = "26: Reference resistor tap for setting 26."]
    CEREF0_26 = 26,
    #[doc = "27: Reference resistor tap for setting 27."]
    CEREF0_27 = 27,
    #[doc = "28: Reference resistor tap for setting 28."]
    CEREF0_28 = 28,
    #[doc = "29: Reference resistor tap for setting 29."]
    CEREF0_29 = 29,
    #[doc = "30: Reference resistor tap for setting 30."]
    CEREF0_30 = 30,
    #[doc = "31: Reference resistor tap for setting 31."]
    CEREF0_31 = 31,
}
impl From<CEREF0_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREF0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREF0`"]
pub type CEREF0_R = crate::R<u8, CEREF0_A>;
impl CEREF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREF0_A {
        match self.bits {
            0 => CEREF0_A::CEREF0_0,
            1 => CEREF0_A::CEREF0_1,
            2 => CEREF0_A::CEREF0_2,
            3 => CEREF0_A::CEREF0_3,
            4 => CEREF0_A::CEREF0_4,
            5 => CEREF0_A::CEREF0_5,
            6 => CEREF0_A::CEREF0_6,
            7 => CEREF0_A::CEREF0_7,
            8 => CEREF0_A::CEREF0_8,
            9 => CEREF0_A::CEREF0_9,
            10 => CEREF0_A::CEREF0_10,
            11 => CEREF0_A::CEREF0_11,
            12 => CEREF0_A::CEREF0_12,
            13 => CEREF0_A::CEREF0_13,
            14 => CEREF0_A::CEREF0_14,
            15 => CEREF0_A::CEREF0_15,
            16 => CEREF0_A::CEREF0_16,
            17 => CEREF0_A::CEREF0_17,
            18 => CEREF0_A::CEREF0_18,
            19 => CEREF0_A::CEREF0_19,
            20 => CEREF0_A::CEREF0_20,
            21 => CEREF0_A::CEREF0_21,
            22 => CEREF0_A::CEREF0_22,
            23 => CEREF0_A::CEREF0_23,
            24 => CEREF0_A::CEREF0_24,
            25 => CEREF0_A::CEREF0_25,
            26 => CEREF0_A::CEREF0_26,
            27 => CEREF0_A::CEREF0_27,
            28 => CEREF0_A::CEREF0_28,
            29 => CEREF0_A::CEREF0_29,
            30 => CEREF0_A::CEREF0_30,
            31 => CEREF0_A::CEREF0_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREF0_0`"]
    #[inline(always)]
    pub fn is_ceref0_0(&self) -> bool {
        *self == CEREF0_A::CEREF0_0
    }
    #[doc = "Checks if the value of the field is `CEREF0_1`"]
    #[inline(always)]
    pub fn is_ceref0_1(&self) -> bool {
        *self == CEREF0_A::CEREF0_1
    }
    #[doc = "Checks if the value of the field is `CEREF0_2`"]
    #[inline(always)]
    pub fn is_ceref0_2(&self) -> bool {
        *self == CEREF0_A::CEREF0_2
    }
    #[doc = "Checks if the value of the field is `CEREF0_3`"]
    #[inline(always)]
    pub fn is_ceref0_3(&self) -> bool {
        *self == CEREF0_A::CEREF0_3
    }
    #[doc = "Checks if the value of the field is `CEREF0_4`"]
    #[inline(always)]
    pub fn is_ceref0_4(&self) -> bool {
        *self == CEREF0_A::CEREF0_4
    }
    #[doc = "Checks if the value of the field is `CEREF0_5`"]
    #[inline(always)]
    pub fn is_ceref0_5(&self) -> bool {
        *self == CEREF0_A::CEREF0_5
    }
    #[doc = "Checks if the value of the field is `CEREF0_6`"]
    #[inline(always)]
    pub fn is_ceref0_6(&self) -> bool {
        *self == CEREF0_A::CEREF0_6
    }
    #[doc = "Checks if the value of the field is `CEREF0_7`"]
    #[inline(always)]
    pub fn is_ceref0_7(&self) -> bool {
        *self == CEREF0_A::CEREF0_7
    }
    #[doc = "Checks if the value of the field is `CEREF0_8`"]
    #[inline(always)]
    pub fn is_ceref0_8(&self) -> bool {
        *self == CEREF0_A::CEREF0_8
    }
    #[doc = "Checks if the value of the field is `CEREF0_9`"]
    #[inline(always)]
    pub fn is_ceref0_9(&self) -> bool {
        *self == CEREF0_A::CEREF0_9
    }
    #[doc = "Checks if the value of the field is `CEREF0_10`"]
    #[inline(always)]
    pub fn is_ceref0_10(&self) -> bool {
        *self == CEREF0_A::CEREF0_10
    }
    #[doc = "Checks if the value of the field is `CEREF0_11`"]
    #[inline(always)]
    pub fn is_ceref0_11(&self) -> bool {
        *self == CEREF0_A::CEREF0_11
    }
    #[doc = "Checks if the value of the field is `CEREF0_12`"]
    #[inline(always)]
    pub fn is_ceref0_12(&self) -> bool {
        *self == CEREF0_A::CEREF0_12
    }
    #[doc = "Checks if the value of the field is `CEREF0_13`"]
    #[inline(always)]
    pub fn is_ceref0_13(&self) -> bool {
        *self == CEREF0_A::CEREF0_13
    }
    #[doc = "Checks if the value of the field is `CEREF0_14`"]
    #[inline(always)]
    pub fn is_ceref0_14(&self) -> bool {
        *self == CEREF0_A::CEREF0_14
    }
    #[doc = "Checks if the value of the field is `CEREF0_15`"]
    #[inline(always)]
    pub fn is_ceref0_15(&self) -> bool {
        *self == CEREF0_A::CEREF0_15
    }
    #[doc = "Checks if the value of the field is `CEREF0_16`"]
    #[inline(always)]
    pub fn is_ceref0_16(&self) -> bool {
        *self == CEREF0_A::CEREF0_16
    }
    #[doc = "Checks if the value of the field is `CEREF0_17`"]
    #[inline(always)]
    pub fn is_ceref0_17(&self) -> bool {
        *self == CEREF0_A::CEREF0_17
    }
    #[doc = "Checks if the value of the field is `CEREF0_18`"]
    #[inline(always)]
    pub fn is_ceref0_18(&self) -> bool {
        *self == CEREF0_A::CEREF0_18
    }
    #[doc = "Checks if the value of the field is `CEREF0_19`"]
    #[inline(always)]
    pub fn is_ceref0_19(&self) -> bool {
        *self == CEREF0_A::CEREF0_19
    }
    #[doc = "Checks if the value of the field is `CEREF0_20`"]
    #[inline(always)]
    pub fn is_ceref0_20(&self) -> bool {
        *self == CEREF0_A::CEREF0_20
    }
    #[doc = "Checks if the value of the field is `CEREF0_21`"]
    #[inline(always)]
    pub fn is_ceref0_21(&self) -> bool {
        *self == CEREF0_A::CEREF0_21
    }
    #[doc = "Checks if the value of the field is `CEREF0_22`"]
    #[inline(always)]
    pub fn is_ceref0_22(&self) -> bool {
        *self == CEREF0_A::CEREF0_22
    }
    #[doc = "Checks if the value of the field is `CEREF0_23`"]
    #[inline(always)]
    pub fn is_ceref0_23(&self) -> bool {
        *self == CEREF0_A::CEREF0_23
    }
    #[doc = "Checks if the value of the field is `CEREF0_24`"]
    #[inline(always)]
    pub fn is_ceref0_24(&self) -> bool {
        *self == CEREF0_A::CEREF0_24
    }
    #[doc = "Checks if the value of the field is `CEREF0_25`"]
    #[inline(always)]
    pub fn is_ceref0_25(&self) -> bool {
        *self == CEREF0_A::CEREF0_25
    }
    #[doc = "Checks if the value of the field is `CEREF0_26`"]
    #[inline(always)]
    pub fn is_ceref0_26(&self) -> bool {
        *self == CEREF0_A::CEREF0_26
    }
    #[doc = "Checks if the value of the field is `CEREF0_27`"]
    #[inline(always)]
    pub fn is_ceref0_27(&self) -> bool {
        *self == CEREF0_A::CEREF0_27
    }
    #[doc = "Checks if the value of the field is `CEREF0_28`"]
    #[inline(always)]
    pub fn is_ceref0_28(&self) -> bool {
        *self == CEREF0_A::CEREF0_28
    }
    #[doc = "Checks if the value of the field is `CEREF0_29`"]
    #[inline(always)]
    pub fn is_ceref0_29(&self) -> bool {
        *self == CEREF0_A::CEREF0_29
    }
    #[doc = "Checks if the value of the field is `CEREF0_30`"]
    #[inline(always)]
    pub fn is_ceref0_30(&self) -> bool {
        *self == CEREF0_A::CEREF0_30
    }
    #[doc = "Checks if the value of the field is `CEREF0_31`"]
    #[inline(always)]
    pub fn is_ceref0_31(&self) -> bool {
        *self == CEREF0_A::CEREF0_31
    }
}
#[doc = "Write proxy for field `CEREF0`"]
pub struct CEREF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREF0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn ceref0_0(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_0)
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn ceref0_1(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_1)
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn ceref0_2(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_2)
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn ceref0_3(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_3)
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn ceref0_4(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_4)
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn ceref0_5(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_5)
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn ceref0_6(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_6)
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn ceref0_7(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_7)
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn ceref0_8(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_8)
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn ceref0_9(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_9)
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn ceref0_10(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_10)
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn ceref0_11(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_11)
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn ceref0_12(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_12)
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn ceref0_13(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_13)
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn ceref0_14(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_14)
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn ceref0_15(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_15)
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn ceref0_16(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_16)
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn ceref0_17(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_17)
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn ceref0_18(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_18)
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn ceref0_19(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_19)
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn ceref0_20(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_20)
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn ceref0_21(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_21)
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn ceref0_22(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_22)
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn ceref0_23(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_23)
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn ceref0_24(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_24)
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn ceref0_25(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_25)
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn ceref0_26(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_26)
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn ceref0_27(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_27)
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn ceref0_28(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_28)
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn ceref0_29(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_29)
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn ceref0_30(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_30)
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn ceref0_31(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERSEL_A {
    #[doc = "0: When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    CERSEL_0 = 0,
    #[doc = "1: When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    CERSEL_1 = 1,
}
impl From<CERSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CERSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CERSEL`"]
pub type CERSEL_R = crate::R<bool, CERSEL_A>;
impl CERSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERSEL_A {
        match self.bits {
            false => CERSEL_A::CERSEL_0,
            true => CERSEL_A::CERSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CERSEL_0`"]
    #[inline(always)]
    pub fn is_cersel_0(&self) -> bool {
        *self == CERSEL_A::CERSEL_0
    }
    #[doc = "Checks if the value of the field is `CERSEL_1`"]
    #[inline(always)]
    pub fn is_cersel_1(&self) -> bool {
        *self == CERSEL_A::CERSEL_1
    }
}
#[doc = "Write proxy for field `CERSEL`"]
pub struct CERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CERSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn cersel_0(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_0)
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn cersel_1(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reference source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CERS_A {
    #[doc = "0: No current is drawn by the reference circuitry"]
    CERS_0 = 0,
    #[doc = "1: VCC applied to the resistor ladder"]
    CERS_1 = 1,
    #[doc = "2: Shared reference voltage applied to the resistor ladder"]
    CERS_2 = 2,
    #[doc = "3: Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    CERS_3 = 3,
}
impl From<CERS_A> for u8 {
    #[inline(always)]
    fn from(variant: CERS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CERS`"]
pub type CERS_R = crate::R<u8, CERS_A>;
impl CERS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERS_A {
        match self.bits {
            0 => CERS_A::CERS_0,
            1 => CERS_A::CERS_1,
            2 => CERS_A::CERS_2,
            3 => CERS_A::CERS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CERS_0`"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == CERS_A::CERS_0
    }
    #[doc = "Checks if the value of the field is `CERS_1`"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == CERS_A::CERS_1
    }
    #[doc = "Checks if the value of the field is `CERS_2`"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == CERS_A::CERS_2
    }
    #[doc = "Checks if the value of the field is `CERS_3`"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == CERS_A::CERS_3
    }
}
#[doc = "Write proxy for field `CERS`"]
pub struct CERS_W<'a> {
    w: &'a mut W,
}
impl<'a> CERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut W {
        self.variant(CERS_A::CERS_0)
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut W {
        self.variant(CERS_A::CERS_1)
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut W {
        self.variant(CERS_A::CERS_2)
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut W {
        self.variant(CERS_A::CERS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reference resistor tap 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREF1_A {
    #[doc = "0: Reference resistor tap for setting 0."]
    CEREF1_0 = 0,
    #[doc = "1: Reference resistor tap for setting 1."]
    CEREF1_1 = 1,
    #[doc = "2: Reference resistor tap for setting 2."]
    CEREF1_2 = 2,
    #[doc = "3: Reference resistor tap for setting 3."]
    CEREF1_3 = 3,
    #[doc = "4: Reference resistor tap for setting 4."]
    CEREF1_4 = 4,
    #[doc = "5: Reference resistor tap for setting 5."]
    CEREF1_5 = 5,
    #[doc = "6: Reference resistor tap for setting 6."]
    CEREF1_6 = 6,
    #[doc = "7: Reference resistor tap for setting 7."]
    CEREF1_7 = 7,
    #[doc = "8: Reference resistor tap for setting 8."]
    CEREF1_8 = 8,
    #[doc = "9: Reference resistor tap for setting 9."]
    CEREF1_9 = 9,
    #[doc = "10: Reference resistor tap for setting 10."]
    CEREF1_10 = 10,
    #[doc = "11: Reference resistor tap for setting 11."]
    CEREF1_11 = 11,
    #[doc = "12: Reference resistor tap for setting 12."]
    CEREF1_12 = 12,
    #[doc = "13: Reference resistor tap for setting 13."]
    CEREF1_13 = 13,
    #[doc = "14: Reference resistor tap for setting 14."]
    CEREF1_14 = 14,
    #[doc = "15: Reference resistor tap for setting 15."]
    CEREF1_15 = 15,
    #[doc = "16: Reference resistor tap for setting 16."]
    CEREF1_16 = 16,
    #[doc = "17: Reference resistor tap for setting 17."]
    CEREF1_17 = 17,
    #[doc = "18: Reference resistor tap for setting 18."]
    CEREF1_18 = 18,
    #[doc = "19: Reference resistor tap for setting 19."]
    CEREF1_19 = 19,
    #[doc = "20: Reference resistor tap for setting 20."]
    CEREF1_20 = 20,
    #[doc = "21: Reference resistor tap for setting 21."]
    CEREF1_21 = 21,
    #[doc = "22: Reference resistor tap for setting 22."]
    CEREF1_22 = 22,
    #[doc = "23: Reference resistor tap for setting 23."]
    CEREF1_23 = 23,
    #[doc = "24: Reference resistor tap for setting 24."]
    CEREF1_24 = 24,
    #[doc = "25: Reference resistor tap for setting 25."]
    CEREF1_25 = 25,
    #[doc = "26: Reference resistor tap for setting 26."]
    CEREF1_26 = 26,
    #[doc = "27: Reference resistor tap for setting 27."]
    CEREF1_27 = 27,
    #[doc = "28: Reference resistor tap for setting 28."]
    CEREF1_28 = 28,
    #[doc = "29: Reference resistor tap for setting 29."]
    CEREF1_29 = 29,
    #[doc = "30: Reference resistor tap for setting 30."]
    CEREF1_30 = 30,
    #[doc = "31: Reference resistor tap for setting 31."]
    CEREF1_31 = 31,
}
impl From<CEREF1_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREF1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREF1`"]
pub type CEREF1_R = crate::R<u8, CEREF1_A>;
impl CEREF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREF1_A {
        match self.bits {
            0 => CEREF1_A::CEREF1_0,
            1 => CEREF1_A::CEREF1_1,
            2 => CEREF1_A::CEREF1_2,
            3 => CEREF1_A::CEREF1_3,
            4 => CEREF1_A::CEREF1_4,
            5 => CEREF1_A::CEREF1_5,
            6 => CEREF1_A::CEREF1_6,
            7 => CEREF1_A::CEREF1_7,
            8 => CEREF1_A::CEREF1_8,
            9 => CEREF1_A::CEREF1_9,
            10 => CEREF1_A::CEREF1_10,
            11 => CEREF1_A::CEREF1_11,
            12 => CEREF1_A::CEREF1_12,
            13 => CEREF1_A::CEREF1_13,
            14 => CEREF1_A::CEREF1_14,
            15 => CEREF1_A::CEREF1_15,
            16 => CEREF1_A::CEREF1_16,
            17 => CEREF1_A::CEREF1_17,
            18 => CEREF1_A::CEREF1_18,
            19 => CEREF1_A::CEREF1_19,
            20 => CEREF1_A::CEREF1_20,
            21 => CEREF1_A::CEREF1_21,
            22 => CEREF1_A::CEREF1_22,
            23 => CEREF1_A::CEREF1_23,
            24 => CEREF1_A::CEREF1_24,
            25 => CEREF1_A::CEREF1_25,
            26 => CEREF1_A::CEREF1_26,
            27 => CEREF1_A::CEREF1_27,
            28 => CEREF1_A::CEREF1_28,
            29 => CEREF1_A::CEREF1_29,
            30 => CEREF1_A::CEREF1_30,
            31 => CEREF1_A::CEREF1_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREF1_0`"]
    #[inline(always)]
    pub fn is_ceref1_0(&self) -> bool {
        *self == CEREF1_A::CEREF1_0
    }
    #[doc = "Checks if the value of the field is `CEREF1_1`"]
    #[inline(always)]
    pub fn is_ceref1_1(&self) -> bool {
        *self == CEREF1_A::CEREF1_1
    }
    #[doc = "Checks if the value of the field is `CEREF1_2`"]
    #[inline(always)]
    pub fn is_ceref1_2(&self) -> bool {
        *self == CEREF1_A::CEREF1_2
    }
    #[doc = "Checks if the value of the field is `CEREF1_3`"]
    #[inline(always)]
    pub fn is_ceref1_3(&self) -> bool {
        *self == CEREF1_A::CEREF1_3
    }
    #[doc = "Checks if the value of the field is `CEREF1_4`"]
    #[inline(always)]
    pub fn is_ceref1_4(&self) -> bool {
        *self == CEREF1_A::CEREF1_4
    }
    #[doc = "Checks if the value of the field is `CEREF1_5`"]
    #[inline(always)]
    pub fn is_ceref1_5(&self) -> bool {
        *self == CEREF1_A::CEREF1_5
    }
    #[doc = "Checks if the value of the field is `CEREF1_6`"]
    #[inline(always)]
    pub fn is_ceref1_6(&self) -> bool {
        *self == CEREF1_A::CEREF1_6
    }
    #[doc = "Checks if the value of the field is `CEREF1_7`"]
    #[inline(always)]
    pub fn is_ceref1_7(&self) -> bool {
        *self == CEREF1_A::CEREF1_7
    }
    #[doc = "Checks if the value of the field is `CEREF1_8`"]
    #[inline(always)]
    pub fn is_ceref1_8(&self) -> bool {
        *self == CEREF1_A::CEREF1_8
    }
    #[doc = "Checks if the value of the field is `CEREF1_9`"]
    #[inline(always)]
    pub fn is_ceref1_9(&self) -> bool {
        *self == CEREF1_A::CEREF1_9
    }
    #[doc = "Checks if the value of the field is `CEREF1_10`"]
    #[inline(always)]
    pub fn is_ceref1_10(&self) -> bool {
        *self == CEREF1_A::CEREF1_10
    }
    #[doc = "Checks if the value of the field is `CEREF1_11`"]
    #[inline(always)]
    pub fn is_ceref1_11(&self) -> bool {
        *self == CEREF1_A::CEREF1_11
    }
    #[doc = "Checks if the value of the field is `CEREF1_12`"]
    #[inline(always)]
    pub fn is_ceref1_12(&self) -> bool {
        *self == CEREF1_A::CEREF1_12
    }
    #[doc = "Checks if the value of the field is `CEREF1_13`"]
    #[inline(always)]
    pub fn is_ceref1_13(&self) -> bool {
        *self == CEREF1_A::CEREF1_13
    }
    #[doc = "Checks if the value of the field is `CEREF1_14`"]
    #[inline(always)]
    pub fn is_ceref1_14(&self) -> bool {
        *self == CEREF1_A::CEREF1_14
    }
    #[doc = "Checks if the value of the field is `CEREF1_15`"]
    #[inline(always)]
    pub fn is_ceref1_15(&self) -> bool {
        *self == CEREF1_A::CEREF1_15
    }
    #[doc = "Checks if the value of the field is `CEREF1_16`"]
    #[inline(always)]
    pub fn is_ceref1_16(&self) -> bool {
        *self == CEREF1_A::CEREF1_16
    }
    #[doc = "Checks if the value of the field is `CEREF1_17`"]
    #[inline(always)]
    pub fn is_ceref1_17(&self) -> bool {
        *self == CEREF1_A::CEREF1_17
    }
    #[doc = "Checks if the value of the field is `CEREF1_18`"]
    #[inline(always)]
    pub fn is_ceref1_18(&self) -> bool {
        *self == CEREF1_A::CEREF1_18
    }
    #[doc = "Checks if the value of the field is `CEREF1_19`"]
    #[inline(always)]
    pub fn is_ceref1_19(&self) -> bool {
        *self == CEREF1_A::CEREF1_19
    }
    #[doc = "Checks if the value of the field is `CEREF1_20`"]
    #[inline(always)]
    pub fn is_ceref1_20(&self) -> bool {
        *self == CEREF1_A::CEREF1_20
    }
    #[doc = "Checks if the value of the field is `CEREF1_21`"]
    #[inline(always)]
    pub fn is_ceref1_21(&self) -> bool {
        *self == CEREF1_A::CEREF1_21
    }
    #[doc = "Checks if the value of the field is `CEREF1_22`"]
    #[inline(always)]
    pub fn is_ceref1_22(&self) -> bool {
        *self == CEREF1_A::CEREF1_22
    }
    #[doc = "Checks if the value of the field is `CEREF1_23`"]
    #[inline(always)]
    pub fn is_ceref1_23(&self) -> bool {
        *self == CEREF1_A::CEREF1_23
    }
    #[doc = "Checks if the value of the field is `CEREF1_24`"]
    #[inline(always)]
    pub fn is_ceref1_24(&self) -> bool {
        *self == CEREF1_A::CEREF1_24
    }
    #[doc = "Checks if the value of the field is `CEREF1_25`"]
    #[inline(always)]
    pub fn is_ceref1_25(&self) -> bool {
        *self == CEREF1_A::CEREF1_25
    }
    #[doc = "Checks if the value of the field is `CEREF1_26`"]
    #[inline(always)]
    pub fn is_ceref1_26(&self) -> bool {
        *self == CEREF1_A::CEREF1_26
    }
    #[doc = "Checks if the value of the field is `CEREF1_27`"]
    #[inline(always)]
    pub fn is_ceref1_27(&self) -> bool {
        *self == CEREF1_A::CEREF1_27
    }
    #[doc = "Checks if the value of the field is `CEREF1_28`"]
    #[inline(always)]
    pub fn is_ceref1_28(&self) -> bool {
        *self == CEREF1_A::CEREF1_28
    }
    #[doc = "Checks if the value of the field is `CEREF1_29`"]
    #[inline(always)]
    pub fn is_ceref1_29(&self) -> bool {
        *self == CEREF1_A::CEREF1_29
    }
    #[doc = "Checks if the value of the field is `CEREF1_30`"]
    #[inline(always)]
    pub fn is_ceref1_30(&self) -> bool {
        *self == CEREF1_A::CEREF1_30
    }
    #[doc = "Checks if the value of the field is `CEREF1_31`"]
    #[inline(always)]
    pub fn is_ceref1_31(&self) -> bool {
        *self == CEREF1_A::CEREF1_31
    }
}
#[doc = "Write proxy for field `CEREF1`"]
pub struct CEREF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREF1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn ceref1_0(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_0)
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn ceref1_1(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_1)
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn ceref1_2(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_2)
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn ceref1_3(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_3)
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn ceref1_4(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_4)
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn ceref1_5(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_5)
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn ceref1_6(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_6)
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn ceref1_7(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_7)
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn ceref1_8(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_8)
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn ceref1_9(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_9)
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn ceref1_10(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_10)
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn ceref1_11(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_11)
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn ceref1_12(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_12)
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn ceref1_13(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_13)
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn ceref1_14(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_14)
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn ceref1_15(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_15)
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn ceref1_16(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_16)
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn ceref1_17(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_17)
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn ceref1_18(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_18)
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn ceref1_19(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_19)
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn ceref1_20(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_20)
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn ceref1_21(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_21)
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn ceref1_22(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_22)
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn ceref1_23(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_23)
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn ceref1_24(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_24)
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn ceref1_25(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_25)
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn ceref1_26(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_26)
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn ceref1_27(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_27)
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn ceref1_28(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_28)
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn ceref1_29(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_29)
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn ceref1_30(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_30)
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn ceref1_31(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reference voltage level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREFL_A {
    #[doc = "0: Reference amplifier is disabled. No reference voltage is requested"]
    CEREFL_0 = 0,
    #[doc = "1: 1.2 V is selected as shared reference voltage input"]
    CEREFL_1 = 1,
    #[doc = "2: 2.0 V is selected as shared reference voltage input"]
    CEREFL_2 = 2,
    #[doc = "3: 2.5 V is selected as shared reference voltage input"]
    CEREFL_3 = 3,
}
impl From<CEREFL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREFL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREFL`"]
pub type CEREFL_R = crate::R<u8, CEREFL_A>;
impl CEREFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFL_A {
        match self.bits {
            0 => CEREFL_A::CEREFL_0,
            1 => CEREFL_A::CEREFL_1,
            2 => CEREFL_A::CEREFL_2,
            3 => CEREFL_A::CEREFL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREFL_0`"]
    #[inline(always)]
    pub fn is_cerefl_0(&self) -> bool {
        *self == CEREFL_A::CEREFL_0
    }
    #[doc = "Checks if the value of the field is `CEREFL_1`"]
    #[inline(always)]
    pub fn is_cerefl_1(&self) -> bool {
        *self == CEREFL_A::CEREFL_1
    }
    #[doc = "Checks if the value of the field is `CEREFL_2`"]
    #[inline(always)]
    pub fn is_cerefl_2(&self) -> bool {
        *self == CEREFL_A::CEREFL_2
    }
    #[doc = "Checks if the value of the field is `CEREFL_3`"]
    #[inline(always)]
    pub fn is_cerefl_3(&self) -> bool {
        *self == CEREFL_A::CEREFL_3
    }
}
#[doc = "Write proxy for field `CEREFL`"]
pub struct CEREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREFL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn cerefl_0(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_0)
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_1(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_1)
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_2(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_2)
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_3(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reference accuracy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEREFACC_A {
    #[doc = "0: Static mode"]
    CEREFACC_0 = 0,
    #[doc = "1: Clocked (low power, low accuracy) mode"]
    CEREFACC_1 = 1,
}
impl From<CEREFACC_A> for bool {
    #[inline(always)]
    fn from(variant: CEREFACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEREFACC`"]
pub type CEREFACC_R = crate::R<bool, CEREFACC_A>;
impl CEREFACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFACC_A {
        match self.bits {
            false => CEREFACC_A::CEREFACC_0,
            true => CEREFACC_A::CEREFACC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEREFACC_0`"]
    #[inline(always)]
    pub fn is_cerefacc_0(&self) -> bool {
        *self == CEREFACC_A::CEREFACC_0
    }
    #[doc = "Checks if the value of the field is `CEREFACC_1`"]
    #[inline(always)]
    pub fn is_cerefacc_1(&self) -> bool {
        *self == CEREFACC_A::CEREFACC_1
    }
}
#[doc = "Write proxy for field `CEREFACC`"]
pub struct CEREFACC_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREFACC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn cerefacc_0(self) -> &'a mut W {
        self.variant(CEREFACC_A::CEREFACC_0)
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn cerefacc_1(self) -> &'a mut W {
        self.variant(CEREFACC_A::CEREFACC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> CEREF0_R {
        CEREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CERSEL_R {
        CERSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&self) -> CERS_R {
        CERS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&self) -> CEREF1_R {
        CEREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&self) -> CEREFL_R {
        CEREFL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CEREFACC_R {
        CEREFACC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> CEREF0_W {
        CEREF0_W { w: self }
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CERSEL_W {
        CERSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&mut self) -> CERS_W {
        CERS_W { w: self }
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> CEREF1_W {
        CEREF1_W { w: self }
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CEREFL_W {
        CEREFL_W { w: self }
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CEREFACC_W {
        CEREFACC_W { w: self }
    }
}
