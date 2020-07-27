#[doc = "Reader of register ADC14IFGR0"]
pub type R = crate::R<u32, super::ADC14IFGR0>;
#[doc = "ADC14MEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG0_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG0_1 = 1,
}
impl From<ADC14IFG0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG0`"]
pub type ADC14IFG0_R = crate::R<bool, ADC14IFG0_A>;
impl ADC14IFG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG0_A {
        match self.bits {
            false => ADC14IFG0_A::ADC14IFG0_0,
            true => ADC14IFG0_A::ADC14IFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG0_0`"]
    #[inline(always)]
    pub fn is_adc14ifg0_0(&self) -> bool {
        *self == ADC14IFG0_A::ADC14IFG0_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG0_1`"]
    #[inline(always)]
    pub fn is_adc14ifg0_1(&self) -> bool {
        *self == ADC14IFG0_A::ADC14IFG0_1
    }
}
#[doc = "ADC14MEM1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG1_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG1_1 = 1,
}
impl From<ADC14IFG1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG1`"]
pub type ADC14IFG1_R = crate::R<bool, ADC14IFG1_A>;
impl ADC14IFG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG1_A {
        match self.bits {
            false => ADC14IFG1_A::ADC14IFG1_0,
            true => ADC14IFG1_A::ADC14IFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG1_0`"]
    #[inline(always)]
    pub fn is_adc14ifg1_0(&self) -> bool {
        *self == ADC14IFG1_A::ADC14IFG1_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG1_1`"]
    #[inline(always)]
    pub fn is_adc14ifg1_1(&self) -> bool {
        *self == ADC14IFG1_A::ADC14IFG1_1
    }
}
#[doc = "ADC14MEM2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG2_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG2_1 = 1,
}
impl From<ADC14IFG2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG2`"]
pub type ADC14IFG2_R = crate::R<bool, ADC14IFG2_A>;
impl ADC14IFG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG2_A {
        match self.bits {
            false => ADC14IFG2_A::ADC14IFG2_0,
            true => ADC14IFG2_A::ADC14IFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG2_0`"]
    #[inline(always)]
    pub fn is_adc14ifg2_0(&self) -> bool {
        *self == ADC14IFG2_A::ADC14IFG2_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG2_1`"]
    #[inline(always)]
    pub fn is_adc14ifg2_1(&self) -> bool {
        *self == ADC14IFG2_A::ADC14IFG2_1
    }
}
#[doc = "ADC14MEM3 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG3_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG3_1 = 1,
}
impl From<ADC14IFG3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG3`"]
pub type ADC14IFG3_R = crate::R<bool, ADC14IFG3_A>;
impl ADC14IFG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG3_A {
        match self.bits {
            false => ADC14IFG3_A::ADC14IFG3_0,
            true => ADC14IFG3_A::ADC14IFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG3_0`"]
    #[inline(always)]
    pub fn is_adc14ifg3_0(&self) -> bool {
        *self == ADC14IFG3_A::ADC14IFG3_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG3_1`"]
    #[inline(always)]
    pub fn is_adc14ifg3_1(&self) -> bool {
        *self == ADC14IFG3_A::ADC14IFG3_1
    }
}
#[doc = "ADC14MEM4 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG4_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG4_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG4_1 = 1,
}
impl From<ADC14IFG4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG4`"]
pub type ADC14IFG4_R = crate::R<bool, ADC14IFG4_A>;
impl ADC14IFG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG4_A {
        match self.bits {
            false => ADC14IFG4_A::ADC14IFG4_0,
            true => ADC14IFG4_A::ADC14IFG4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG4_0`"]
    #[inline(always)]
    pub fn is_adc14ifg4_0(&self) -> bool {
        *self == ADC14IFG4_A::ADC14IFG4_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG4_1`"]
    #[inline(always)]
    pub fn is_adc14ifg4_1(&self) -> bool {
        *self == ADC14IFG4_A::ADC14IFG4_1
    }
}
#[doc = "ADC14MEM5 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG5_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG5_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG5_1 = 1,
}
impl From<ADC14IFG5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG5`"]
pub type ADC14IFG5_R = crate::R<bool, ADC14IFG5_A>;
impl ADC14IFG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG5_A {
        match self.bits {
            false => ADC14IFG5_A::ADC14IFG5_0,
            true => ADC14IFG5_A::ADC14IFG5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG5_0`"]
    #[inline(always)]
    pub fn is_adc14ifg5_0(&self) -> bool {
        *self == ADC14IFG5_A::ADC14IFG5_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG5_1`"]
    #[inline(always)]
    pub fn is_adc14ifg5_1(&self) -> bool {
        *self == ADC14IFG5_A::ADC14IFG5_1
    }
}
#[doc = "ADC14MEM6 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG6_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG6_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG6_1 = 1,
}
impl From<ADC14IFG6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG6`"]
pub type ADC14IFG6_R = crate::R<bool, ADC14IFG6_A>;
impl ADC14IFG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG6_A {
        match self.bits {
            false => ADC14IFG6_A::ADC14IFG6_0,
            true => ADC14IFG6_A::ADC14IFG6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG6_0`"]
    #[inline(always)]
    pub fn is_adc14ifg6_0(&self) -> bool {
        *self == ADC14IFG6_A::ADC14IFG6_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG6_1`"]
    #[inline(always)]
    pub fn is_adc14ifg6_1(&self) -> bool {
        *self == ADC14IFG6_A::ADC14IFG6_1
    }
}
#[doc = "ADC14MEM7 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG7_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG7_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG7_1 = 1,
}
impl From<ADC14IFG7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG7`"]
pub type ADC14IFG7_R = crate::R<bool, ADC14IFG7_A>;
impl ADC14IFG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG7_A {
        match self.bits {
            false => ADC14IFG7_A::ADC14IFG7_0,
            true => ADC14IFG7_A::ADC14IFG7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG7_0`"]
    #[inline(always)]
    pub fn is_adc14ifg7_0(&self) -> bool {
        *self == ADC14IFG7_A::ADC14IFG7_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG7_1`"]
    #[inline(always)]
    pub fn is_adc14ifg7_1(&self) -> bool {
        *self == ADC14IFG7_A::ADC14IFG7_1
    }
}
#[doc = "ADC14MEM8 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG8_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG8_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG8_1 = 1,
}
impl From<ADC14IFG8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG8`"]
pub type ADC14IFG8_R = crate::R<bool, ADC14IFG8_A>;
impl ADC14IFG8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG8_A {
        match self.bits {
            false => ADC14IFG8_A::ADC14IFG8_0,
            true => ADC14IFG8_A::ADC14IFG8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG8_0`"]
    #[inline(always)]
    pub fn is_adc14ifg8_0(&self) -> bool {
        *self == ADC14IFG8_A::ADC14IFG8_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG8_1`"]
    #[inline(always)]
    pub fn is_adc14ifg8_1(&self) -> bool {
        *self == ADC14IFG8_A::ADC14IFG8_1
    }
}
#[doc = "ADC14MEM9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG9_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG9_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG9_1 = 1,
}
impl From<ADC14IFG9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG9`"]
pub type ADC14IFG9_R = crate::R<bool, ADC14IFG9_A>;
impl ADC14IFG9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG9_A {
        match self.bits {
            false => ADC14IFG9_A::ADC14IFG9_0,
            true => ADC14IFG9_A::ADC14IFG9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG9_0`"]
    #[inline(always)]
    pub fn is_adc14ifg9_0(&self) -> bool {
        *self == ADC14IFG9_A::ADC14IFG9_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG9_1`"]
    #[inline(always)]
    pub fn is_adc14ifg9_1(&self) -> bool {
        *self == ADC14IFG9_A::ADC14IFG9_1
    }
}
#[doc = "ADC14MEM10 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG10_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG10_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG10_1 = 1,
}
impl From<ADC14IFG10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG10`"]
pub type ADC14IFG10_R = crate::R<bool, ADC14IFG10_A>;
impl ADC14IFG10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG10_A {
        match self.bits {
            false => ADC14IFG10_A::ADC14IFG10_0,
            true => ADC14IFG10_A::ADC14IFG10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG10_0`"]
    #[inline(always)]
    pub fn is_adc14ifg10_0(&self) -> bool {
        *self == ADC14IFG10_A::ADC14IFG10_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG10_1`"]
    #[inline(always)]
    pub fn is_adc14ifg10_1(&self) -> bool {
        *self == ADC14IFG10_A::ADC14IFG10_1
    }
}
#[doc = "ADC14MEM11 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG11_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG11_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG11_1 = 1,
}
impl From<ADC14IFG11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG11`"]
pub type ADC14IFG11_R = crate::R<bool, ADC14IFG11_A>;
impl ADC14IFG11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG11_A {
        match self.bits {
            false => ADC14IFG11_A::ADC14IFG11_0,
            true => ADC14IFG11_A::ADC14IFG11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG11_0`"]
    #[inline(always)]
    pub fn is_adc14ifg11_0(&self) -> bool {
        *self == ADC14IFG11_A::ADC14IFG11_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG11_1`"]
    #[inline(always)]
    pub fn is_adc14ifg11_1(&self) -> bool {
        *self == ADC14IFG11_A::ADC14IFG11_1
    }
}
#[doc = "ADC14MEM12 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG12_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG12_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG12_1 = 1,
}
impl From<ADC14IFG12_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG12`"]
pub type ADC14IFG12_R = crate::R<bool, ADC14IFG12_A>;
impl ADC14IFG12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG12_A {
        match self.bits {
            false => ADC14IFG12_A::ADC14IFG12_0,
            true => ADC14IFG12_A::ADC14IFG12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG12_0`"]
    #[inline(always)]
    pub fn is_adc14ifg12_0(&self) -> bool {
        *self == ADC14IFG12_A::ADC14IFG12_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG12_1`"]
    #[inline(always)]
    pub fn is_adc14ifg12_1(&self) -> bool {
        *self == ADC14IFG12_A::ADC14IFG12_1
    }
}
#[doc = "ADC14MEM13 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG13_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG13_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG13_1 = 1,
}
impl From<ADC14IFG13_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG13`"]
pub type ADC14IFG13_R = crate::R<bool, ADC14IFG13_A>;
impl ADC14IFG13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG13_A {
        match self.bits {
            false => ADC14IFG13_A::ADC14IFG13_0,
            true => ADC14IFG13_A::ADC14IFG13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG13_0`"]
    #[inline(always)]
    pub fn is_adc14ifg13_0(&self) -> bool {
        *self == ADC14IFG13_A::ADC14IFG13_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG13_1`"]
    #[inline(always)]
    pub fn is_adc14ifg13_1(&self) -> bool {
        *self == ADC14IFG13_A::ADC14IFG13_1
    }
}
#[doc = "ADC14MEM14 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG14_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG14_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG14_1 = 1,
}
impl From<ADC14IFG14_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG14`"]
pub type ADC14IFG14_R = crate::R<bool, ADC14IFG14_A>;
impl ADC14IFG14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG14_A {
        match self.bits {
            false => ADC14IFG14_A::ADC14IFG14_0,
            true => ADC14IFG14_A::ADC14IFG14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG14_0`"]
    #[inline(always)]
    pub fn is_adc14ifg14_0(&self) -> bool {
        *self == ADC14IFG14_A::ADC14IFG14_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG14_1`"]
    #[inline(always)]
    pub fn is_adc14ifg14_1(&self) -> bool {
        *self == ADC14IFG14_A::ADC14IFG14_1
    }
}
#[doc = "ADC14MEM15 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG15_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG15_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG15_1 = 1,
}
impl From<ADC14IFG15_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG15`"]
pub type ADC14IFG15_R = crate::R<bool, ADC14IFG15_A>;
impl ADC14IFG15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG15_A {
        match self.bits {
            false => ADC14IFG15_A::ADC14IFG15_0,
            true => ADC14IFG15_A::ADC14IFG15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG15_0`"]
    #[inline(always)]
    pub fn is_adc14ifg15_0(&self) -> bool {
        *self == ADC14IFG15_A::ADC14IFG15_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG15_1`"]
    #[inline(always)]
    pub fn is_adc14ifg15_1(&self) -> bool {
        *self == ADC14IFG15_A::ADC14IFG15_1
    }
}
#[doc = "ADC14MEM16 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG16_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG16_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG16_1 = 1,
}
impl From<ADC14IFG16_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG16`"]
pub type ADC14IFG16_R = crate::R<bool, ADC14IFG16_A>;
impl ADC14IFG16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG16_A {
        match self.bits {
            false => ADC14IFG16_A::ADC14IFG16_0,
            true => ADC14IFG16_A::ADC14IFG16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG16_0`"]
    #[inline(always)]
    pub fn is_adc14ifg16_0(&self) -> bool {
        *self == ADC14IFG16_A::ADC14IFG16_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG16_1`"]
    #[inline(always)]
    pub fn is_adc14ifg16_1(&self) -> bool {
        *self == ADC14IFG16_A::ADC14IFG16_1
    }
}
#[doc = "ADC14MEM17 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG17_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG17_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG17_1 = 1,
}
impl From<ADC14IFG17_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG17`"]
pub type ADC14IFG17_R = crate::R<bool, ADC14IFG17_A>;
impl ADC14IFG17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG17_A {
        match self.bits {
            false => ADC14IFG17_A::ADC14IFG17_0,
            true => ADC14IFG17_A::ADC14IFG17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG17_0`"]
    #[inline(always)]
    pub fn is_adc14ifg17_0(&self) -> bool {
        *self == ADC14IFG17_A::ADC14IFG17_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG17_1`"]
    #[inline(always)]
    pub fn is_adc14ifg17_1(&self) -> bool {
        *self == ADC14IFG17_A::ADC14IFG17_1
    }
}
#[doc = "ADC14MEM18 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG18_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG18_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG18_1 = 1,
}
impl From<ADC14IFG18_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG18`"]
pub type ADC14IFG18_R = crate::R<bool, ADC14IFG18_A>;
impl ADC14IFG18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG18_A {
        match self.bits {
            false => ADC14IFG18_A::ADC14IFG18_0,
            true => ADC14IFG18_A::ADC14IFG18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG18_0`"]
    #[inline(always)]
    pub fn is_adc14ifg18_0(&self) -> bool {
        *self == ADC14IFG18_A::ADC14IFG18_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG18_1`"]
    #[inline(always)]
    pub fn is_adc14ifg18_1(&self) -> bool {
        *self == ADC14IFG18_A::ADC14IFG18_1
    }
}
#[doc = "ADC14MEM19 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG19_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG19_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG19_1 = 1,
}
impl From<ADC14IFG19_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG19`"]
pub type ADC14IFG19_R = crate::R<bool, ADC14IFG19_A>;
impl ADC14IFG19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG19_A {
        match self.bits {
            false => ADC14IFG19_A::ADC14IFG19_0,
            true => ADC14IFG19_A::ADC14IFG19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG19_0`"]
    #[inline(always)]
    pub fn is_adc14ifg19_0(&self) -> bool {
        *self == ADC14IFG19_A::ADC14IFG19_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG19_1`"]
    #[inline(always)]
    pub fn is_adc14ifg19_1(&self) -> bool {
        *self == ADC14IFG19_A::ADC14IFG19_1
    }
}
#[doc = "ADC14MEM20 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG20_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG20_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG20_1 = 1,
}
impl From<ADC14IFG20_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG20`"]
pub type ADC14IFG20_R = crate::R<bool, ADC14IFG20_A>;
impl ADC14IFG20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG20_A {
        match self.bits {
            false => ADC14IFG20_A::ADC14IFG20_0,
            true => ADC14IFG20_A::ADC14IFG20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG20_0`"]
    #[inline(always)]
    pub fn is_adc14ifg20_0(&self) -> bool {
        *self == ADC14IFG20_A::ADC14IFG20_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG20_1`"]
    #[inline(always)]
    pub fn is_adc14ifg20_1(&self) -> bool {
        *self == ADC14IFG20_A::ADC14IFG20_1
    }
}
#[doc = "ADC14MEM21 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG21_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG21_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG21_1 = 1,
}
impl From<ADC14IFG21_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG21`"]
pub type ADC14IFG21_R = crate::R<bool, ADC14IFG21_A>;
impl ADC14IFG21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG21_A {
        match self.bits {
            false => ADC14IFG21_A::ADC14IFG21_0,
            true => ADC14IFG21_A::ADC14IFG21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG21_0`"]
    #[inline(always)]
    pub fn is_adc14ifg21_0(&self) -> bool {
        *self == ADC14IFG21_A::ADC14IFG21_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG21_1`"]
    #[inline(always)]
    pub fn is_adc14ifg21_1(&self) -> bool {
        *self == ADC14IFG21_A::ADC14IFG21_1
    }
}
#[doc = "ADC14MEM22 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG22_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG22_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG22_1 = 1,
}
impl From<ADC14IFG22_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG22`"]
pub type ADC14IFG22_R = crate::R<bool, ADC14IFG22_A>;
impl ADC14IFG22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG22_A {
        match self.bits {
            false => ADC14IFG22_A::ADC14IFG22_0,
            true => ADC14IFG22_A::ADC14IFG22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG22_0`"]
    #[inline(always)]
    pub fn is_adc14ifg22_0(&self) -> bool {
        *self == ADC14IFG22_A::ADC14IFG22_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG22_1`"]
    #[inline(always)]
    pub fn is_adc14ifg22_1(&self) -> bool {
        *self == ADC14IFG22_A::ADC14IFG22_1
    }
}
#[doc = "ADC14MEM23 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG23_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG23_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG23_1 = 1,
}
impl From<ADC14IFG23_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG23`"]
pub type ADC14IFG23_R = crate::R<bool, ADC14IFG23_A>;
impl ADC14IFG23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG23_A {
        match self.bits {
            false => ADC14IFG23_A::ADC14IFG23_0,
            true => ADC14IFG23_A::ADC14IFG23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG23_0`"]
    #[inline(always)]
    pub fn is_adc14ifg23_0(&self) -> bool {
        *self == ADC14IFG23_A::ADC14IFG23_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG23_1`"]
    #[inline(always)]
    pub fn is_adc14ifg23_1(&self) -> bool {
        *self == ADC14IFG23_A::ADC14IFG23_1
    }
}
#[doc = "ADC14MEM24 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG24_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG24_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG24_1 = 1,
}
impl From<ADC14IFG24_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG24`"]
pub type ADC14IFG24_R = crate::R<bool, ADC14IFG24_A>;
impl ADC14IFG24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG24_A {
        match self.bits {
            false => ADC14IFG24_A::ADC14IFG24_0,
            true => ADC14IFG24_A::ADC14IFG24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG24_0`"]
    #[inline(always)]
    pub fn is_adc14ifg24_0(&self) -> bool {
        *self == ADC14IFG24_A::ADC14IFG24_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG24_1`"]
    #[inline(always)]
    pub fn is_adc14ifg24_1(&self) -> bool {
        *self == ADC14IFG24_A::ADC14IFG24_1
    }
}
#[doc = "ADC14MEM25 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG25_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG25_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG25_1 = 1,
}
impl From<ADC14IFG25_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG25`"]
pub type ADC14IFG25_R = crate::R<bool, ADC14IFG25_A>;
impl ADC14IFG25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG25_A {
        match self.bits {
            false => ADC14IFG25_A::ADC14IFG25_0,
            true => ADC14IFG25_A::ADC14IFG25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG25_0`"]
    #[inline(always)]
    pub fn is_adc14ifg25_0(&self) -> bool {
        *self == ADC14IFG25_A::ADC14IFG25_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG25_1`"]
    #[inline(always)]
    pub fn is_adc14ifg25_1(&self) -> bool {
        *self == ADC14IFG25_A::ADC14IFG25_1
    }
}
#[doc = "ADC14MEM26 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG26_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG26_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG26_1 = 1,
}
impl From<ADC14IFG26_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG26`"]
pub type ADC14IFG26_R = crate::R<bool, ADC14IFG26_A>;
impl ADC14IFG26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG26_A {
        match self.bits {
            false => ADC14IFG26_A::ADC14IFG26_0,
            true => ADC14IFG26_A::ADC14IFG26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG26_0`"]
    #[inline(always)]
    pub fn is_adc14ifg26_0(&self) -> bool {
        *self == ADC14IFG26_A::ADC14IFG26_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG26_1`"]
    #[inline(always)]
    pub fn is_adc14ifg26_1(&self) -> bool {
        *self == ADC14IFG26_A::ADC14IFG26_1
    }
}
#[doc = "ADC14MEM27 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG27_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG27_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG27_1 = 1,
}
impl From<ADC14IFG27_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG27`"]
pub type ADC14IFG27_R = crate::R<bool, ADC14IFG27_A>;
impl ADC14IFG27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG27_A {
        match self.bits {
            false => ADC14IFG27_A::ADC14IFG27_0,
            true => ADC14IFG27_A::ADC14IFG27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG27_0`"]
    #[inline(always)]
    pub fn is_adc14ifg27_0(&self) -> bool {
        *self == ADC14IFG27_A::ADC14IFG27_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG27_1`"]
    #[inline(always)]
    pub fn is_adc14ifg27_1(&self) -> bool {
        *self == ADC14IFG27_A::ADC14IFG27_1
    }
}
#[doc = "ADC14MEM28 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG28_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG28_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG28_1 = 1,
}
impl From<ADC14IFG28_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG28`"]
pub type ADC14IFG28_R = crate::R<bool, ADC14IFG28_A>;
impl ADC14IFG28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG28_A {
        match self.bits {
            false => ADC14IFG28_A::ADC14IFG28_0,
            true => ADC14IFG28_A::ADC14IFG28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG28_0`"]
    #[inline(always)]
    pub fn is_adc14ifg28_0(&self) -> bool {
        *self == ADC14IFG28_A::ADC14IFG28_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG28_1`"]
    #[inline(always)]
    pub fn is_adc14ifg28_1(&self) -> bool {
        *self == ADC14IFG28_A::ADC14IFG28_1
    }
}
#[doc = "ADC14MEM29 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG29_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG29_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG29_1 = 1,
}
impl From<ADC14IFG29_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG29`"]
pub type ADC14IFG29_R = crate::R<bool, ADC14IFG29_A>;
impl ADC14IFG29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG29_A {
        match self.bits {
            false => ADC14IFG29_A::ADC14IFG29_0,
            true => ADC14IFG29_A::ADC14IFG29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG29_0`"]
    #[inline(always)]
    pub fn is_adc14ifg29_0(&self) -> bool {
        *self == ADC14IFG29_A::ADC14IFG29_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG29_1`"]
    #[inline(always)]
    pub fn is_adc14ifg29_1(&self) -> bool {
        *self == ADC14IFG29_A::ADC14IFG29_1
    }
}
#[doc = "ADC14MEM30 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG30_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG30_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG30_1 = 1,
}
impl From<ADC14IFG30_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG30`"]
pub type ADC14IFG30_R = crate::R<bool, ADC14IFG30_A>;
impl ADC14IFG30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG30_A {
        match self.bits {
            false => ADC14IFG30_A::ADC14IFG30_0,
            true => ADC14IFG30_A::ADC14IFG30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG30_0`"]
    #[inline(always)]
    pub fn is_adc14ifg30_0(&self) -> bool {
        *self == ADC14IFG30_A::ADC14IFG30_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG30_1`"]
    #[inline(always)]
    pub fn is_adc14ifg30_1(&self) -> bool {
        *self == ADC14IFG30_A::ADC14IFG30_1
    }
}
#[doc = "ADC14MEM31 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IFG31_A {
    #[doc = "0: No interrupt pending"]
    ADC14IFG31_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14IFG31_1 = 1,
}
impl From<ADC14IFG31_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IFG31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IFG31`"]
pub type ADC14IFG31_R = crate::R<bool, ADC14IFG31_A>;
impl ADC14IFG31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IFG31_A {
        match self.bits {
            false => ADC14IFG31_A::ADC14IFG31_0,
            true => ADC14IFG31_A::ADC14IFG31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IFG31_0`"]
    #[inline(always)]
    pub fn is_adc14ifg31_0(&self) -> bool {
        *self == ADC14IFG31_A::ADC14IFG31_0
    }
    #[doc = "Checks if the value of the field is `ADC14IFG31_1`"]
    #[inline(always)]
    pub fn is_adc14ifg31_1(&self) -> bool {
        *self == ADC14IFG31_A::ADC14IFG31_1
    }
}
impl R {
    #[doc = "Bit 0 - ADC14MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg0(&self) -> ADC14IFG0_R {
        ADC14IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC14MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg1(&self) -> ADC14IFG1_R {
        ADC14IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC14MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg2(&self) -> ADC14IFG2_R {
        ADC14IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC14MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg3(&self) -> ADC14IFG3_R {
        ADC14IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC14MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg4(&self) -> ADC14IFG4_R {
        ADC14IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC14MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg5(&self) -> ADC14IFG5_R {
        ADC14IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC14MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg6(&self) -> ADC14IFG6_R {
        ADC14IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC14MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg7(&self) -> ADC14IFG7_R {
        ADC14IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC14MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg8(&self) -> ADC14IFG8_R {
        ADC14IFG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC14MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg9(&self) -> ADC14IFG9_R {
        ADC14IFG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC14MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg10(&self) -> ADC14IFG10_R {
        ADC14IFG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC14MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg11(&self) -> ADC14IFG11_R {
        ADC14IFG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC14MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg12(&self) -> ADC14IFG12_R {
        ADC14IFG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC14MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg13(&self) -> ADC14IFG13_R {
        ADC14IFG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC14MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg14(&self) -> ADC14IFG14_R {
        ADC14IFG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC14MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg15(&self) -> ADC14IFG15_R {
        ADC14IFG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC14MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg16(&self) -> ADC14IFG16_R {
        ADC14IFG16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC14MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg17(&self) -> ADC14IFG17_R {
        ADC14IFG17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC14MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg18(&self) -> ADC14IFG18_R {
        ADC14IFG18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC14MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg19(&self) -> ADC14IFG19_R {
        ADC14IFG19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC14MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg20(&self) -> ADC14IFG20_R {
        ADC14IFG20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC14MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg21(&self) -> ADC14IFG21_R {
        ADC14IFG21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC14MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg22(&self) -> ADC14IFG22_R {
        ADC14IFG22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC14MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg23(&self) -> ADC14IFG23_R {
        ADC14IFG23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC14MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg24(&self) -> ADC14IFG24_R {
        ADC14IFG24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC14MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg25(&self) -> ADC14IFG25_R {
        ADC14IFG25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC14MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg26(&self) -> ADC14IFG26_R {
        ADC14IFG26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC14MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg27(&self) -> ADC14IFG27_R {
        ADC14IFG27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC14MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg28(&self) -> ADC14IFG28_R {
        ADC14IFG28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC14MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg29(&self) -> ADC14IFG29_R {
        ADC14IFG29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ADC14MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg30(&self) -> ADC14IFG30_R {
        ADC14IFG30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ADC14MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg31(&self) -> ADC14IFG31_R {
        ADC14IFG31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
