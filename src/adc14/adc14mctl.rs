#[doc = "Reader of register ADC14MCTL[%s]"]
pub type R = crate::R<u32, super::ADC14MCTL>;
#[doc = "Writer for register ADC14MCTL[%s]"]
pub type W = crate::W<u32, super::ADC14MCTL>;
#[doc = "Register ADC14MCTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14MCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14INCH_A {
    #[doc = "0: If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC14INCH_0 = 0,
    #[doc = "1: If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC14INCH_1 = 1,
    #[doc = "2: If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC14INCH_2 = 2,
    #[doc = "3: If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC14INCH_3 = 3,
    #[doc = "4: If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC14INCH_4 = 4,
    #[doc = "5: If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC14INCH_5 = 5,
    #[doc = "6: If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC14INCH_6 = 6,
    #[doc = "7: If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC14INCH_7 = 7,
    #[doc = "8: If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC14INCH_8 = 8,
    #[doc = "9: If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC14INCH_9 = 9,
    #[doc = "10: If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC14INCH_10 = 10,
    #[doc = "11: If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC14INCH_11 = 11,
    #[doc = "12: If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC14INCH_12 = 12,
    #[doc = "13: If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC14INCH_13 = 13,
    #[doc = "14: If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC14INCH_14 = 14,
    #[doc = "15: If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC14INCH_15 = 15,
    #[doc = "16: If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC14INCH_16 = 16,
    #[doc = "17: If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC14INCH_17 = 17,
    #[doc = "18: If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC14INCH_18 = 18,
    #[doc = "19: If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC14INCH_19 = 19,
    #[doc = "20: If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC14INCH_20 = 20,
    #[doc = "21: If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC14INCH_21 = 21,
    #[doc = "22: If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC14INCH_22 = 22,
    #[doc = "23: If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC14INCH_23 = 23,
    #[doc = "24: If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC14INCH_24 = 24,
    #[doc = "25: If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC14INCH_25 = 25,
    #[doc = "26: If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    ADC14INCH_26 = 26,
    #[doc = "27: If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    ADC14INCH_27 = 27,
    #[doc = "28: If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC14INCH_28 = 28,
    #[doc = "29: If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC14INCH_29 = 29,
    #[doc = "30: If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC14INCH_30 = 30,
    #[doc = "31: If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC14INCH_31 = 31,
}
impl From<ADC14INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14INCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC14INCH`"]
pub type ADC14INCH_R = crate::R<u8, ADC14INCH_A>;
impl ADC14INCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INCH_A {
        match self.bits {
            0 => ADC14INCH_A::ADC14INCH_0,
            1 => ADC14INCH_A::ADC14INCH_1,
            2 => ADC14INCH_A::ADC14INCH_2,
            3 => ADC14INCH_A::ADC14INCH_3,
            4 => ADC14INCH_A::ADC14INCH_4,
            5 => ADC14INCH_A::ADC14INCH_5,
            6 => ADC14INCH_A::ADC14INCH_6,
            7 => ADC14INCH_A::ADC14INCH_7,
            8 => ADC14INCH_A::ADC14INCH_8,
            9 => ADC14INCH_A::ADC14INCH_9,
            10 => ADC14INCH_A::ADC14INCH_10,
            11 => ADC14INCH_A::ADC14INCH_11,
            12 => ADC14INCH_A::ADC14INCH_12,
            13 => ADC14INCH_A::ADC14INCH_13,
            14 => ADC14INCH_A::ADC14INCH_14,
            15 => ADC14INCH_A::ADC14INCH_15,
            16 => ADC14INCH_A::ADC14INCH_16,
            17 => ADC14INCH_A::ADC14INCH_17,
            18 => ADC14INCH_A::ADC14INCH_18,
            19 => ADC14INCH_A::ADC14INCH_19,
            20 => ADC14INCH_A::ADC14INCH_20,
            21 => ADC14INCH_A::ADC14INCH_21,
            22 => ADC14INCH_A::ADC14INCH_22,
            23 => ADC14INCH_A::ADC14INCH_23,
            24 => ADC14INCH_A::ADC14INCH_24,
            25 => ADC14INCH_A::ADC14INCH_25,
            26 => ADC14INCH_A::ADC14INCH_26,
            27 => ADC14INCH_A::ADC14INCH_27,
            28 => ADC14INCH_A::ADC14INCH_28,
            29 => ADC14INCH_A::ADC14INCH_29,
            30 => ADC14INCH_A::ADC14INCH_30,
            31 => ADC14INCH_A::ADC14INCH_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_0`"]
    #[inline(always)]
    pub fn is_adc14inch_0(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_1`"]
    #[inline(always)]
    pub fn is_adc14inch_1(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_2`"]
    #[inline(always)]
    pub fn is_adc14inch_2(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_3`"]
    #[inline(always)]
    pub fn is_adc14inch_3(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_4`"]
    #[inline(always)]
    pub fn is_adc14inch_4(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_5`"]
    #[inline(always)]
    pub fn is_adc14inch_5(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_6`"]
    #[inline(always)]
    pub fn is_adc14inch_6(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_7`"]
    #[inline(always)]
    pub fn is_adc14inch_7(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_8`"]
    #[inline(always)]
    pub fn is_adc14inch_8(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_9`"]
    #[inline(always)]
    pub fn is_adc14inch_9(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_10`"]
    #[inline(always)]
    pub fn is_adc14inch_10(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_11`"]
    #[inline(always)]
    pub fn is_adc14inch_11(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_12`"]
    #[inline(always)]
    pub fn is_adc14inch_12(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_13`"]
    #[inline(always)]
    pub fn is_adc14inch_13(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_14`"]
    #[inline(always)]
    pub fn is_adc14inch_14(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_15`"]
    #[inline(always)]
    pub fn is_adc14inch_15(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_15
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_16`"]
    #[inline(always)]
    pub fn is_adc14inch_16(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_16
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_17`"]
    #[inline(always)]
    pub fn is_adc14inch_17(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_17
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_18`"]
    #[inline(always)]
    pub fn is_adc14inch_18(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_18
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_19`"]
    #[inline(always)]
    pub fn is_adc14inch_19(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_19
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_20`"]
    #[inline(always)]
    pub fn is_adc14inch_20(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_20
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_21`"]
    #[inline(always)]
    pub fn is_adc14inch_21(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_21
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_22`"]
    #[inline(always)]
    pub fn is_adc14inch_22(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_22
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_23`"]
    #[inline(always)]
    pub fn is_adc14inch_23(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_23
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_24`"]
    #[inline(always)]
    pub fn is_adc14inch_24(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_24
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_25`"]
    #[inline(always)]
    pub fn is_adc14inch_25(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_25
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_26`"]
    #[inline(always)]
    pub fn is_adc14inch_26(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_26
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_27`"]
    #[inline(always)]
    pub fn is_adc14inch_27(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_27
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_28`"]
    #[inline(always)]
    pub fn is_adc14inch_28(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_28
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_29`"]
    #[inline(always)]
    pub fn is_adc14inch_29(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_29
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_30`"]
    #[inline(always)]
    pub fn is_adc14inch_30(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_30
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_31`"]
    #[inline(always)]
    pub fn is_adc14inch_31(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_31
    }
}
#[doc = "Write proxy for field `ADC14INCH`"]
pub struct ADC14INCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14INCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14INCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_0(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_0)
    }
    #[doc = "If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_1(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_1)
    }
    #[doc = "If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_2(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_2)
    }
    #[doc = "If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_3(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_3)
    }
    #[doc = "If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_4(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_4)
    }
    #[doc = "If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_5(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_5)
    }
    #[doc = "If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_6(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_6)
    }
    #[doc = "If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_7(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_7)
    }
    #[doc = "If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_8(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_8)
    }
    #[doc = "If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_9(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_9)
    }
    #[doc = "If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_10(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_10)
    }
    #[doc = "If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_11(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_11)
    }
    #[doc = "If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_12(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_12)
    }
    #[doc = "If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_13(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_13)
    }
    #[doc = "If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_14(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_14)
    }
    #[doc = "If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_15(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_15)
    }
    #[doc = "If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_16(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_16)
    }
    #[doc = "If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_17(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_17)
    }
    #[doc = "If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_18(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_18)
    }
    #[doc = "If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_19(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_19)
    }
    #[doc = "If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_20(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_20)
    }
    #[doc = "If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_21(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_21)
    }
    #[doc = "If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_22(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_22)
    }
    #[doc = "If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_23(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_23)
    }
    #[doc = "If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_24(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_24)
    }
    #[doc = "If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_25(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_25)
    }
    #[doc = "If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_26(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_26)
    }
    #[doc = "If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_27(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_27)
    }
    #[doc = "If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_28(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_28)
    }
    #[doc = "If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_29(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_29)
    }
    #[doc = "If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_30(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_30)
    }
    #[doc = "If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_31(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "End of sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14EOS_A {
    #[doc = "0: Not end of sequence"]
    ADC14EOS_0 = 0,
    #[doc = "1: End of sequence"]
    ADC14EOS_1 = 1,
}
impl From<ADC14EOS_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14EOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14EOS`"]
pub type ADC14EOS_R = crate::R<bool, ADC14EOS_A>;
impl ADC14EOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14EOS_A {
        match self.bits {
            false => ADC14EOS_A::ADC14EOS_0,
            true => ADC14EOS_A::ADC14EOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14EOS_0`"]
    #[inline(always)]
    pub fn is_adc14eos_0(&self) -> bool {
        *self == ADC14EOS_A::ADC14EOS_0
    }
    #[doc = "Checks if the value of the field is `ADC14EOS_1`"]
    #[inline(always)]
    pub fn is_adc14eos_1(&self) -> bool {
        *self == ADC14EOS_A::ADC14EOS_1
    }
}
#[doc = "Write proxy for field `ADC14EOS`"]
pub struct ADC14EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14EOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14EOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn adc14eos_0(self) -> &'a mut W {
        self.variant(ADC14EOS_A::ADC14EOS_0)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn adc14eos_1(self) -> &'a mut W {
        self.variant(ADC14EOS_A::ADC14EOS_1)
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
#[doc = "Selects combinations of V(R+) and V(R-) sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14VRSEL_A {
    #[doc = "0: V(R+) = AVCC, V(R-) = AVSS"]
    ADC14VRSEL_0 = 0,
    #[doc = "1: V(R+) = VREF buffered, V(R-) = AVSS"]
    ADC14VRSEL_1 = 1,
    #[doc = "14: V(R+) = VeREF+, V(R-) = VeREF-"]
    ADC14VRSEL_14 = 14,
    #[doc = "15: V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    ADC14VRSEL_15 = 15,
}
impl From<ADC14VRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14VRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC14VRSEL`"]
pub type ADC14VRSEL_R = crate::R<u8, ADC14VRSEL_A>;
impl ADC14VRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC14VRSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC14VRSEL_A::ADC14VRSEL_0),
            1 => Val(ADC14VRSEL_A::ADC14VRSEL_1),
            14 => Val(ADC14VRSEL_A::ADC14VRSEL_14),
            15 => Val(ADC14VRSEL_A::ADC14VRSEL_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_0`"]
    #[inline(always)]
    pub fn is_adc14vrsel_0(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_1`"]
    #[inline(always)]
    pub fn is_adc14vrsel_1(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_14`"]
    #[inline(always)]
    pub fn is_adc14vrsel_14(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_14
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_15`"]
    #[inline(always)]
    pub fn is_adc14vrsel_15(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_15
    }
}
#[doc = "Write proxy for field `ADC14VRSEL`"]
pub struct ADC14VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14VRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "V(R+) = AVCC, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_0(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_0)
    }
    #[doc = "V(R+) = VREF buffered, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_1(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_1)
    }
    #[doc = "V(R+) = VeREF+, V(R-) = VeREF-"]
    #[inline(always)]
    pub fn adc14vrsel_14(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_14)
    }
    #[doc = "V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    #[inline(always)]
    pub fn adc14vrsel_15(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14DIF_A {
    #[doc = "0: Single-ended mode enabled"]
    ADC14DIF_0 = 0,
    #[doc = "1: Differential mode enabled"]
    ADC14DIF_1 = 1,
}
impl From<ADC14DIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14DIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14DIF`"]
pub type ADC14DIF_R = crate::R<bool, ADC14DIF_A>;
impl ADC14DIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14DIF_A {
        match self.bits {
            false => ADC14DIF_A::ADC14DIF_0,
            true => ADC14DIF_A::ADC14DIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14DIF_0`"]
    #[inline(always)]
    pub fn is_adc14dif_0(&self) -> bool {
        *self == ADC14DIF_A::ADC14DIF_0
    }
    #[doc = "Checks if the value of the field is `ADC14DIF_1`"]
    #[inline(always)]
    pub fn is_adc14dif_1(&self) -> bool {
        *self == ADC14DIF_A::ADC14DIF_1
    }
}
#[doc = "Write proxy for field `ADC14DIF`"]
pub struct ADC14DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14DIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn adc14dif_0(self) -> &'a mut W {
        self.variant(ADC14DIF_A::ADC14DIF_0)
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn adc14dif_1(self) -> &'a mut W {
        self.variant(ADC14DIF_A::ADC14DIF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Comparator window enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14WINC_A {
    #[doc = "0: Comparator window disabled"]
    ADC14WINC_0 = 0,
    #[doc = "1: Comparator window enabled"]
    ADC14WINC_1 = 1,
}
impl From<ADC14WINC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14WINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14WINC`"]
pub type ADC14WINC_R = crate::R<bool, ADC14WINC_A>;
impl ADC14WINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14WINC_A {
        match self.bits {
            false => ADC14WINC_A::ADC14WINC_0,
            true => ADC14WINC_A::ADC14WINC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14WINC_0`"]
    #[inline(always)]
    pub fn is_adc14winc_0(&self) -> bool {
        *self == ADC14WINC_A::ADC14WINC_0
    }
    #[doc = "Checks if the value of the field is `ADC14WINC_1`"]
    #[inline(always)]
    pub fn is_adc14winc_1(&self) -> bool {
        *self == ADC14WINC_A::ADC14WINC_1
    }
}
#[doc = "Write proxy for field `ADC14WINC`"]
pub struct ADC14WINC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14WINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14WINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn adc14winc_0(self) -> &'a mut W {
        self.variant(ADC14WINC_A::ADC14WINC_0)
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn adc14winc_1(self) -> &'a mut W {
        self.variant(ADC14WINC_A::ADC14WINC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Window comparator threshold register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14WINCTH_A {
    #[doc = "0: Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    ADC14WINCTH_0 = 0,
    #[doc = "1: Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    ADC14WINCTH_1 = 1,
}
impl From<ADC14WINCTH_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14WINCTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14WINCTH`"]
pub type ADC14WINCTH_R = crate::R<bool, ADC14WINCTH_A>;
impl ADC14WINCTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14WINCTH_A {
        match self.bits {
            false => ADC14WINCTH_A::ADC14WINCTH_0,
            true => ADC14WINCTH_A::ADC14WINCTH_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14WINCTH_0`"]
    #[inline(always)]
    pub fn is_adc14wincth_0(&self) -> bool {
        *self == ADC14WINCTH_A::ADC14WINCTH_0
    }
    #[doc = "Checks if the value of the field is `ADC14WINCTH_1`"]
    #[inline(always)]
    pub fn is_adc14wincth_1(&self) -> bool {
        *self == ADC14WINCTH_A::ADC14WINCTH_1
    }
}
#[doc = "Write proxy for field `ADC14WINCTH`"]
pub struct ADC14WINCTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14WINCTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14WINCTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    #[inline(always)]
    pub fn adc14wincth_0(self) -> &'a mut W {
        self.variant(ADC14WINCTH_A::ADC14WINCTH_0)
    }
    #[doc = "Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    #[inline(always)]
    pub fn adc14wincth_1(self) -> &'a mut W {
        self.variant(ADC14WINCTH_A::ADC14WINCTH_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&self) -> ADC14INCH_R {
        ADC14INCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&self) -> ADC14EOS_R {
        ADC14EOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&self) -> ADC14VRSEL_R {
        ADC14VRSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&self) -> ADC14DIF_R {
        ADC14DIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&self) -> ADC14WINC_R {
        ADC14WINC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&self) -> ADC14WINCTH_R {
        ADC14WINCTH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&mut self) -> ADC14INCH_W {
        ADC14INCH_W { w: self }
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&mut self) -> ADC14EOS_W {
        ADC14EOS_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&mut self) -> ADC14VRSEL_W {
        ADC14VRSEL_W { w: self }
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&mut self) -> ADC14DIF_W {
        ADC14DIF_W { w: self }
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&mut self) -> ADC14WINC_W {
        ADC14WINC_W { w: self }
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&mut self) -> ADC14WINCTH_W {
        ADC14WINCTH_W { w: self }
    }
}
