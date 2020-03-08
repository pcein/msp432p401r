#[doc = "Reader of register ADC14IV"]
pub type R = crate::R<u32, super::ADC14IV>;
#[doc = "Writer for register ADC14IV"]
pub type W = crate::W<u32, super::ADC14IV>;
#[doc = "Register ADC14IV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14IV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC14 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ADC14IV_A {
    #[doc = "0: No interrupt pending"]
    ADC14IV_0 = 0,
    #[doc = "2: Interrupt Source: ADC14MEMx overflow; Interrupt Flag: ADC14OVIFG; Interrupt Priority: Highest"]
    ADC14IV_2 = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADC14TOVIFG"]
    ADC14IV_4 = 4,
    #[doc = "6: Interrupt Source: ADC14 window high interrupt flag; Interrupt Flag: ADC14HIIFG"]
    ADC14IV_6 = 6,
    #[doc = "8: Interrupt Source: ADC14 window low interrupt flag; Interrupt Flag: ADC14LOIFG"]
    ADC14IV_8 = 8,
    #[doc = "10: Interrupt Source: ADC14 in-window interrupt flag; Interrupt Flag: ADC14INIFG"]
    ADC14IV_10 = 10,
    #[doc = "12: Interrupt Source: ADC14MEM0 interrupt flag; Interrupt Flag: ADC14IFG0"]
    ADC14IV_12 = 12,
    #[doc = "14: Interrupt Source: ADC14MEM1 interrupt flag; Interrupt Flag: ADC14IFG1"]
    ADC14IV_14 = 14,
    #[doc = "16: Interrupt Source: ADC14MEM2 interrupt flag; Interrupt Flag: ADC14IFG2"]
    ADC14IV_16 = 16,
    #[doc = "18: Interrupt Source: ADC14MEM3 interrupt flag; Interrupt Flag: ADC14IFG3"]
    ADC14IV_18 = 18,
    #[doc = "20: Interrupt Source: ADC14MEM4 interrupt flag; Interrupt Flag: ADC14IFG4"]
    ADC14IV_20 = 20,
    #[doc = "22: Interrupt Source: ADC14MEM5 interrupt flag; Interrupt Flag: ADC14IFG5"]
    ADC14IV_22 = 22,
    #[doc = "24: Interrupt Source: ADC14MEM6 interrupt flag; Interrupt Flag: ADC14IFG6"]
    ADC14IV_24 = 24,
    #[doc = "26: Interrupt Source: ADC14MEM7 interrupt flag; Interrupt Flag: ADC14IFG7"]
    ADC14IV_26 = 26,
    #[doc = "28: Interrupt Source: ADC14MEM8 interrupt flag; Interrupt Flag: ADC14IFG8"]
    ADC14IV_28 = 28,
    #[doc = "30: Interrupt Source: ADC14MEM9 interrupt flag; Interrupt Flag: ADC14IFG9"]
    ADC14IV_30 = 30,
    #[doc = "32: Interrupt Source: ADC14MEM10 interrupt flag; Interrupt Flag: ADC14IFG10"]
    ADC14IV_32 = 32,
    #[doc = "34: Interrupt Source: ADC14MEM11 interrupt flag; Interrupt Flag: ADC14IFG11"]
    ADC14IV_34 = 34,
    #[doc = "36: Interrupt Source: ADC14MEM12 interrupt flag; Interrupt Flag: ADC14IFG12"]
    ADC14IV_36 = 36,
    #[doc = "38: Interrupt Source: ADC14MEM13 interrupt flag; Interrupt Flag: ADC14IFG13"]
    ADC14IV_38 = 38,
    #[doc = "40: Interrupt Source: ADC14MEM14 interrupt flag; Interrupt Flag: ADC14IFG14"]
    ADC14IV_40 = 40,
    #[doc = "42: Interrupt Source: ADC14MEM15 interrupt flag; Interrupt Flag: ADC14IFG15"]
    ADC14IV_42 = 42,
    #[doc = "44: Interrupt Source: ADC14MEM16 interrupt flag; Interrupt Flag: ADC14IFG16"]
    ADC14IV_44 = 44,
    #[doc = "46: Interrupt Source: ADC14MEM17 interrupt flag; Interrupt Flag: ADC14IFG17"]
    ADC14IV_46 = 46,
    #[doc = "48: Interrupt Source: ADC14MEM18 interrupt flag; Interrupt Flag: ADC14IFG18"]
    ADC14IV_48 = 48,
    #[doc = "50: Interrupt Source: ADC14MEM19 interrupt flag; Interrupt Flag: ADC14IFG19"]
    ADC14IV_50 = 50,
    #[doc = "52: Interrupt Source: ADC14MEM20 interrupt flag; Interrupt Flag: ADC14IFG20"]
    ADC14IV_52 = 52,
    #[doc = "54: Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    ADC14IV_54 = 54,
    #[doc = "56: Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    ADC14IV_56 = 56,
    #[doc = "58: Interrupt Source: ADC14MEM23 interrupt flag; Interrupt Flag: ADC14IFG23"]
    ADC14IV_58 = 58,
    #[doc = "60: Interrupt Source: ADC14MEM24 interrupt flag; Interrupt Flag: ADC14IFG24"]
    ADC14IV_60 = 60,
    #[doc = "62: Interrupt Source: ADC14MEM25 interrupt flag; Interrupt Flag: ADC14IFG25"]
    ADC14IV_62 = 62,
    #[doc = "64: Interrupt Source: ADC14MEM26 interrupt flag; Interrupt Flag: ADC14IFG26"]
    ADC14IV_64 = 64,
    #[doc = "66: Interrupt Source: ADC14MEM27 interrupt flag; Interrupt Flag: ADC14IFG27"]
    ADC14IV_66 = 66,
    #[doc = "68: Interrupt Source: ADC14MEM28 interrupt flag; Interrupt Flag: ADC14IFG28"]
    ADC14IV_68 = 68,
    #[doc = "70: Interrupt Source: ADC14MEM29 interrupt flag; Interrupt Flag: ADC14IFG29"]
    ADC14IV_70 = 70,
    #[doc = "72: Interrupt Source: ADC14MEM30 interrupt flag; Interrupt Flag: ADC14IFG30"]
    ADC14IV_72 = 72,
    #[doc = "74: Interrupt Source: ADC14MEM31 interrupt flag; Interrupt Flag: ADC14IFG31"]
    ADC14IV_74 = 74,
    #[doc = "76: Interrupt Source: ADC14RDYIFG interrupt flag; Interrupt Flag: ADC14RDYIFG; Interrupt Priority: Lowest"]
    ADC14IV_76 = 76,
}
impl From<ADC14IV_A> for u32 {
    #[inline(always)]
    fn from(variant: ADC14IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC14IV`"]
pub type ADC14IV_R = crate::R<u32, ADC14IV_A>;
impl ADC14IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, ADC14IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC14IV_A::ADC14IV_0),
            2 => Val(ADC14IV_A::ADC14IV_2),
            4 => Val(ADC14IV_A::ADC14IV_4),
            6 => Val(ADC14IV_A::ADC14IV_6),
            8 => Val(ADC14IV_A::ADC14IV_8),
            10 => Val(ADC14IV_A::ADC14IV_10),
            12 => Val(ADC14IV_A::ADC14IV_12),
            14 => Val(ADC14IV_A::ADC14IV_14),
            16 => Val(ADC14IV_A::ADC14IV_16),
            18 => Val(ADC14IV_A::ADC14IV_18),
            20 => Val(ADC14IV_A::ADC14IV_20),
            22 => Val(ADC14IV_A::ADC14IV_22),
            24 => Val(ADC14IV_A::ADC14IV_24),
            26 => Val(ADC14IV_A::ADC14IV_26),
            28 => Val(ADC14IV_A::ADC14IV_28),
            30 => Val(ADC14IV_A::ADC14IV_30),
            32 => Val(ADC14IV_A::ADC14IV_32),
            34 => Val(ADC14IV_A::ADC14IV_34),
            36 => Val(ADC14IV_A::ADC14IV_36),
            38 => Val(ADC14IV_A::ADC14IV_38),
            40 => Val(ADC14IV_A::ADC14IV_40),
            42 => Val(ADC14IV_A::ADC14IV_42),
            44 => Val(ADC14IV_A::ADC14IV_44),
            46 => Val(ADC14IV_A::ADC14IV_46),
            48 => Val(ADC14IV_A::ADC14IV_48),
            50 => Val(ADC14IV_A::ADC14IV_50),
            52 => Val(ADC14IV_A::ADC14IV_52),
            54 => Val(ADC14IV_A::ADC14IV_54),
            56 => Val(ADC14IV_A::ADC14IV_56),
            58 => Val(ADC14IV_A::ADC14IV_58),
            60 => Val(ADC14IV_A::ADC14IV_60),
            62 => Val(ADC14IV_A::ADC14IV_62),
            64 => Val(ADC14IV_A::ADC14IV_64),
            66 => Val(ADC14IV_A::ADC14IV_66),
            68 => Val(ADC14IV_A::ADC14IV_68),
            70 => Val(ADC14IV_A::ADC14IV_70),
            72 => Val(ADC14IV_A::ADC14IV_72),
            74 => Val(ADC14IV_A::ADC14IV_74),
            76 => Val(ADC14IV_A::ADC14IV_76),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IV_0`"]
    #[inline(always)]
    pub fn is_adc14iv_0(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_0
    }
    #[doc = "Checks if the value of the field is `ADC14IV_2`"]
    #[inline(always)]
    pub fn is_adc14iv_2(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_2
    }
    #[doc = "Checks if the value of the field is `ADC14IV_4`"]
    #[inline(always)]
    pub fn is_adc14iv_4(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_4
    }
    #[doc = "Checks if the value of the field is `ADC14IV_6`"]
    #[inline(always)]
    pub fn is_adc14iv_6(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_6
    }
    #[doc = "Checks if the value of the field is `ADC14IV_8`"]
    #[inline(always)]
    pub fn is_adc14iv_8(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_8
    }
    #[doc = "Checks if the value of the field is `ADC14IV_10`"]
    #[inline(always)]
    pub fn is_adc14iv_10(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_10
    }
    #[doc = "Checks if the value of the field is `ADC14IV_12`"]
    #[inline(always)]
    pub fn is_adc14iv_12(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_12
    }
    #[doc = "Checks if the value of the field is `ADC14IV_14`"]
    #[inline(always)]
    pub fn is_adc14iv_14(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_14
    }
    #[doc = "Checks if the value of the field is `ADC14IV_16`"]
    #[inline(always)]
    pub fn is_adc14iv_16(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_16
    }
    #[doc = "Checks if the value of the field is `ADC14IV_18`"]
    #[inline(always)]
    pub fn is_adc14iv_18(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_18
    }
    #[doc = "Checks if the value of the field is `ADC14IV_20`"]
    #[inline(always)]
    pub fn is_adc14iv_20(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_20
    }
    #[doc = "Checks if the value of the field is `ADC14IV_22`"]
    #[inline(always)]
    pub fn is_adc14iv_22(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_22
    }
    #[doc = "Checks if the value of the field is `ADC14IV_24`"]
    #[inline(always)]
    pub fn is_adc14iv_24(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_24
    }
    #[doc = "Checks if the value of the field is `ADC14IV_26`"]
    #[inline(always)]
    pub fn is_adc14iv_26(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_26
    }
    #[doc = "Checks if the value of the field is `ADC14IV_28`"]
    #[inline(always)]
    pub fn is_adc14iv_28(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_28
    }
    #[doc = "Checks if the value of the field is `ADC14IV_30`"]
    #[inline(always)]
    pub fn is_adc14iv_30(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_30
    }
    #[doc = "Checks if the value of the field is `ADC14IV_32`"]
    #[inline(always)]
    pub fn is_adc14iv_32(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_32
    }
    #[doc = "Checks if the value of the field is `ADC14IV_34`"]
    #[inline(always)]
    pub fn is_adc14iv_34(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_34
    }
    #[doc = "Checks if the value of the field is `ADC14IV_36`"]
    #[inline(always)]
    pub fn is_adc14iv_36(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_36
    }
    #[doc = "Checks if the value of the field is `ADC14IV_38`"]
    #[inline(always)]
    pub fn is_adc14iv_38(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_38
    }
    #[doc = "Checks if the value of the field is `ADC14IV_40`"]
    #[inline(always)]
    pub fn is_adc14iv_40(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_40
    }
    #[doc = "Checks if the value of the field is `ADC14IV_42`"]
    #[inline(always)]
    pub fn is_adc14iv_42(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_42
    }
    #[doc = "Checks if the value of the field is `ADC14IV_44`"]
    #[inline(always)]
    pub fn is_adc14iv_44(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_44
    }
    #[doc = "Checks if the value of the field is `ADC14IV_46`"]
    #[inline(always)]
    pub fn is_adc14iv_46(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_46
    }
    #[doc = "Checks if the value of the field is `ADC14IV_48`"]
    #[inline(always)]
    pub fn is_adc14iv_48(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_48
    }
    #[doc = "Checks if the value of the field is `ADC14IV_50`"]
    #[inline(always)]
    pub fn is_adc14iv_50(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_50
    }
    #[doc = "Checks if the value of the field is `ADC14IV_52`"]
    #[inline(always)]
    pub fn is_adc14iv_52(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_52
    }
    #[doc = "Checks if the value of the field is `ADC14IV_54`"]
    #[inline(always)]
    pub fn is_adc14iv_54(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_54
    }
    #[doc = "Checks if the value of the field is `ADC14IV_56`"]
    #[inline(always)]
    pub fn is_adc14iv_56(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_56
    }
    #[doc = "Checks if the value of the field is `ADC14IV_58`"]
    #[inline(always)]
    pub fn is_adc14iv_58(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_58
    }
    #[doc = "Checks if the value of the field is `ADC14IV_60`"]
    #[inline(always)]
    pub fn is_adc14iv_60(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_60
    }
    #[doc = "Checks if the value of the field is `ADC14IV_62`"]
    #[inline(always)]
    pub fn is_adc14iv_62(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_62
    }
    #[doc = "Checks if the value of the field is `ADC14IV_64`"]
    #[inline(always)]
    pub fn is_adc14iv_64(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_64
    }
    #[doc = "Checks if the value of the field is `ADC14IV_66`"]
    #[inline(always)]
    pub fn is_adc14iv_66(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_66
    }
    #[doc = "Checks if the value of the field is `ADC14IV_68`"]
    #[inline(always)]
    pub fn is_adc14iv_68(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_68
    }
    #[doc = "Checks if the value of the field is `ADC14IV_70`"]
    #[inline(always)]
    pub fn is_adc14iv_70(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_70
    }
    #[doc = "Checks if the value of the field is `ADC14IV_72`"]
    #[inline(always)]
    pub fn is_adc14iv_72(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_72
    }
    #[doc = "Checks if the value of the field is `ADC14IV_74`"]
    #[inline(always)]
    pub fn is_adc14iv_74(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_74
    }
    #[doc = "Checks if the value of the field is `ADC14IV_76`"]
    #[inline(always)]
    pub fn is_adc14iv_76(&self) -> bool {
        *self == ADC14IV_A::ADC14IV_76
    }
}
#[doc = "Write proxy for field `ADC14IV`"]
pub struct ADC14IV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc14iv_0(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_0)
    }
    #[doc = "Interrupt Source: ADC14MEMx overflow; Interrupt Flag: ADC14OVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adc14iv_2(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_2)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADC14TOVIFG"]
    #[inline(always)]
    pub fn adc14iv_4(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_4)
    }
    #[doc = "Interrupt Source: ADC14 window high interrupt flag; Interrupt Flag: ADC14HIIFG"]
    #[inline(always)]
    pub fn adc14iv_6(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_6)
    }
    #[doc = "Interrupt Source: ADC14 window low interrupt flag; Interrupt Flag: ADC14LOIFG"]
    #[inline(always)]
    pub fn adc14iv_8(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_8)
    }
    #[doc = "Interrupt Source: ADC14 in-window interrupt flag; Interrupt Flag: ADC14INIFG"]
    #[inline(always)]
    pub fn adc14iv_10(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_10)
    }
    #[doc = "Interrupt Source: ADC14MEM0 interrupt flag; Interrupt Flag: ADC14IFG0"]
    #[inline(always)]
    pub fn adc14iv_12(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_12)
    }
    #[doc = "Interrupt Source: ADC14MEM1 interrupt flag; Interrupt Flag: ADC14IFG1"]
    #[inline(always)]
    pub fn adc14iv_14(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_14)
    }
    #[doc = "Interrupt Source: ADC14MEM2 interrupt flag; Interrupt Flag: ADC14IFG2"]
    #[inline(always)]
    pub fn adc14iv_16(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_16)
    }
    #[doc = "Interrupt Source: ADC14MEM3 interrupt flag; Interrupt Flag: ADC14IFG3"]
    #[inline(always)]
    pub fn adc14iv_18(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_18)
    }
    #[doc = "Interrupt Source: ADC14MEM4 interrupt flag; Interrupt Flag: ADC14IFG4"]
    #[inline(always)]
    pub fn adc14iv_20(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_20)
    }
    #[doc = "Interrupt Source: ADC14MEM5 interrupt flag; Interrupt Flag: ADC14IFG5"]
    #[inline(always)]
    pub fn adc14iv_22(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_22)
    }
    #[doc = "Interrupt Source: ADC14MEM6 interrupt flag; Interrupt Flag: ADC14IFG6"]
    #[inline(always)]
    pub fn adc14iv_24(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_24)
    }
    #[doc = "Interrupt Source: ADC14MEM7 interrupt flag; Interrupt Flag: ADC14IFG7"]
    #[inline(always)]
    pub fn adc14iv_26(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_26)
    }
    #[doc = "Interrupt Source: ADC14MEM8 interrupt flag; Interrupt Flag: ADC14IFG8"]
    #[inline(always)]
    pub fn adc14iv_28(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_28)
    }
    #[doc = "Interrupt Source: ADC14MEM9 interrupt flag; Interrupt Flag: ADC14IFG9"]
    #[inline(always)]
    pub fn adc14iv_30(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_30)
    }
    #[doc = "Interrupt Source: ADC14MEM10 interrupt flag; Interrupt Flag: ADC14IFG10"]
    #[inline(always)]
    pub fn adc14iv_32(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_32)
    }
    #[doc = "Interrupt Source: ADC14MEM11 interrupt flag; Interrupt Flag: ADC14IFG11"]
    #[inline(always)]
    pub fn adc14iv_34(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_34)
    }
    #[doc = "Interrupt Source: ADC14MEM12 interrupt flag; Interrupt Flag: ADC14IFG12"]
    #[inline(always)]
    pub fn adc14iv_36(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_36)
    }
    #[doc = "Interrupt Source: ADC14MEM13 interrupt flag; Interrupt Flag: ADC14IFG13"]
    #[inline(always)]
    pub fn adc14iv_38(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_38)
    }
    #[doc = "Interrupt Source: ADC14MEM14 interrupt flag; Interrupt Flag: ADC14IFG14"]
    #[inline(always)]
    pub fn adc14iv_40(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_40)
    }
    #[doc = "Interrupt Source: ADC14MEM15 interrupt flag; Interrupt Flag: ADC14IFG15"]
    #[inline(always)]
    pub fn adc14iv_42(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_42)
    }
    #[doc = "Interrupt Source: ADC14MEM16 interrupt flag; Interrupt Flag: ADC14IFG16"]
    #[inline(always)]
    pub fn adc14iv_44(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_44)
    }
    #[doc = "Interrupt Source: ADC14MEM17 interrupt flag; Interrupt Flag: ADC14IFG17"]
    #[inline(always)]
    pub fn adc14iv_46(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_46)
    }
    #[doc = "Interrupt Source: ADC14MEM18 interrupt flag; Interrupt Flag: ADC14IFG18"]
    #[inline(always)]
    pub fn adc14iv_48(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_48)
    }
    #[doc = "Interrupt Source: ADC14MEM19 interrupt flag; Interrupt Flag: ADC14IFG19"]
    #[inline(always)]
    pub fn adc14iv_50(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_50)
    }
    #[doc = "Interrupt Source: ADC14MEM20 interrupt flag; Interrupt Flag: ADC14IFG20"]
    #[inline(always)]
    pub fn adc14iv_52(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_52)
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn adc14iv_54(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_54)
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn adc14iv_56(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_56)
    }
    #[doc = "Interrupt Source: ADC14MEM23 interrupt flag; Interrupt Flag: ADC14IFG23"]
    #[inline(always)]
    pub fn adc14iv_58(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_58)
    }
    #[doc = "Interrupt Source: ADC14MEM24 interrupt flag; Interrupt Flag: ADC14IFG24"]
    #[inline(always)]
    pub fn adc14iv_60(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_60)
    }
    #[doc = "Interrupt Source: ADC14MEM25 interrupt flag; Interrupt Flag: ADC14IFG25"]
    #[inline(always)]
    pub fn adc14iv_62(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_62)
    }
    #[doc = "Interrupt Source: ADC14MEM26 interrupt flag; Interrupt Flag: ADC14IFG26"]
    #[inline(always)]
    pub fn adc14iv_64(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_64)
    }
    #[doc = "Interrupt Source: ADC14MEM27 interrupt flag; Interrupt Flag: ADC14IFG27"]
    #[inline(always)]
    pub fn adc14iv_66(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_66)
    }
    #[doc = "Interrupt Source: ADC14MEM28 interrupt flag; Interrupt Flag: ADC14IFG28"]
    #[inline(always)]
    pub fn adc14iv_68(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_68)
    }
    #[doc = "Interrupt Source: ADC14MEM29 interrupt flag; Interrupt Flag: ADC14IFG29"]
    #[inline(always)]
    pub fn adc14iv_70(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_70)
    }
    #[doc = "Interrupt Source: ADC14MEM30 interrupt flag; Interrupt Flag: ADC14IFG30"]
    #[inline(always)]
    pub fn adc14iv_72(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_72)
    }
    #[doc = "Interrupt Source: ADC14MEM31 interrupt flag; Interrupt Flag: ADC14IFG31"]
    #[inline(always)]
    pub fn adc14iv_74(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_74)
    }
    #[doc = "Interrupt Source: ADC14RDYIFG interrupt flag; Interrupt Flag: ADC14RDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn adc14iv_76(self) -> &'a mut W {
        self.variant(ADC14IV_A::ADC14IV_76)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ADC14 interrupt vector value"]
    #[inline(always)]
    pub fn adc14iv(&self) -> ADC14IV_R {
        ADC14IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADC14 interrupt vector value"]
    #[inline(always)]
    pub fn adc14iv(&mut self) -> ADC14IV_W {
        ADC14IV_W { w: self }
    }
}
