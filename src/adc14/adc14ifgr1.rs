#[doc = "Reader of register ADC14IFGR1"]
pub type R = crate::R<u32, super::ADC14IFGR1>;
#[doc = "Interrupt flag for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14INIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14INIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14INIFG_1 = 1,
}
impl From<ADC14INIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14INIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14INIFG`"]
pub type ADC14INIFG_R = crate::R<bool, ADC14INIFG_A>;
impl ADC14INIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INIFG_A {
        match self.bits {
            false => ADC14INIFG_A::ADC14INIFG_0,
            true => ADC14INIFG_A::ADC14INIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INIFG_0`"]
    #[inline(always)]
    pub fn is_adc14inifg_0(&self) -> bool {
        *self == ADC14INIFG_A::ADC14INIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14INIFG_1`"]
    #[inline(always)]
    pub fn is_adc14inifg_1(&self) -> bool {
        *self == ADC14INIFG_A::ADC14INIFG_1
    }
}
#[doc = "Interrupt flag for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14LOIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14LOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14LOIFG_1 = 1,
}
impl From<ADC14LOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14LOIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14LOIFG`"]
pub type ADC14LOIFG_R = crate::R<bool, ADC14LOIFG_A>;
impl ADC14LOIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14LOIFG_A {
        match self.bits {
            false => ADC14LOIFG_A::ADC14LOIFG_0,
            true => ADC14LOIFG_A::ADC14LOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14LOIFG_0`"]
    #[inline(always)]
    pub fn is_adc14loifg_0(&self) -> bool {
        *self == ADC14LOIFG_A::ADC14LOIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14LOIFG_1`"]
    #[inline(always)]
    pub fn is_adc14loifg_1(&self) -> bool {
        *self == ADC14LOIFG_A::ADC14LOIFG_1
    }
}
#[doc = "Interrupt flag for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14HIIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14HIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14HIIFG_1 = 1,
}
impl From<ADC14HIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14HIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14HIIFG`"]
pub type ADC14HIIFG_R = crate::R<bool, ADC14HIIFG_A>;
impl ADC14HIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14HIIFG_A {
        match self.bits {
            false => ADC14HIIFG_A::ADC14HIIFG_0,
            true => ADC14HIIFG_A::ADC14HIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14HIIFG_0`"]
    #[inline(always)]
    pub fn is_adc14hiifg_0(&self) -> bool {
        *self == ADC14HIIFG_A::ADC14HIIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14HIIFG_1`"]
    #[inline(always)]
    pub fn is_adc14hiifg_1(&self) -> bool {
        *self == ADC14HIIFG_A::ADC14HIIFG_1
    }
}
#[doc = "ADC14MEMx overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14OVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14OVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14OVIFG_1 = 1,
}
impl From<ADC14OVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14OVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14OVIFG`"]
pub type ADC14OVIFG_R = crate::R<bool, ADC14OVIFG_A>;
impl ADC14OVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14OVIFG_A {
        match self.bits {
            false => ADC14OVIFG_A::ADC14OVIFG_0,
            true => ADC14OVIFG_A::ADC14OVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14OVIFG_0`"]
    #[inline(always)]
    pub fn is_adc14ovifg_0(&self) -> bool {
        *self == ADC14OVIFG_A::ADC14OVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14OVIFG_1`"]
    #[inline(always)]
    pub fn is_adc14ovifg_1(&self) -> bool {
        *self == ADC14OVIFG_A::ADC14OVIFG_1
    }
}
#[doc = "ADC14 conversion time overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14TOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14TOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14TOVIFG_1 = 1,
}
impl From<ADC14TOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14TOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14TOVIFG`"]
pub type ADC14TOVIFG_R = crate::R<bool, ADC14TOVIFG_A>;
impl ADC14TOVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14TOVIFG_A {
        match self.bits {
            false => ADC14TOVIFG_A::ADC14TOVIFG_0,
            true => ADC14TOVIFG_A::ADC14TOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIFG_0`"]
    #[inline(always)]
    pub fn is_adc14tovifg_0(&self) -> bool {
        *self == ADC14TOVIFG_A::ADC14TOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIFG_1`"]
    #[inline(always)]
    pub fn is_adc14tovifg_1(&self) -> bool {
        *self == ADC14TOVIFG_A::ADC14TOVIFG_1
    }
}
#[doc = "ADC14 local buffered reference ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14RDYIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14RDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14RDYIFG_1 = 1,
}
impl From<ADC14RDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14RDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14RDYIFG`"]
pub type ADC14RDYIFG_R = crate::R<bool, ADC14RDYIFG_A>;
impl ADC14RDYIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14RDYIFG_A {
        match self.bits {
            false => ADC14RDYIFG_A::ADC14RDYIFG_0,
            true => ADC14RDYIFG_A::ADC14RDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIFG_0`"]
    #[inline(always)]
    pub fn is_adc14rdyifg_0(&self) -> bool {
        *self == ADC14RDYIFG_A::ADC14RDYIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIFG_1`"]
    #[inline(always)]
    pub fn is_adc14rdyifg_1(&self) -> bool {
        *self == ADC14RDYIFG_A::ADC14RDYIFG_1
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt flag for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inifg(&self) -> ADC14INIFG_R {
        ADC14INIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loifg(&self) -> ADC14LOIFG_R {
        ADC14LOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiifg(&self) -> ADC14HIIFG_R {
        ADC14HIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14ovifg(&self) -> ADC14OVIFG_R {
        ADC14OVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion time overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14tovifg(&self) -> ADC14TOVIFG_R {
        ADC14TOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt flag"]
    #[inline(always)]
    pub fn adc14rdyifg(&self) -> ADC14RDYIFG_R {
        ADC14RDYIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
