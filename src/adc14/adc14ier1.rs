#[doc = "Reader of register ADC14IER1"]
pub type R = crate::R<u32, super::ADC14IER1>;
#[doc = "Writer for register ADC14IER1"]
pub type W = crate::W<u32, super::ADC14IER1>;
#[doc = "Register ADC14IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt enable for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14INIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14INIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14INIE_1 = 1,
}
impl From<ADC14INIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14INIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14INIE`"]
pub type ADC14INIE_R = crate::R<bool, ADC14INIE_A>;
impl ADC14INIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INIE_A {
        match self.bits {
            false => ADC14INIE_A::ADC14INIE_0,
            true => ADC14INIE_A::ADC14INIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INIE_0`"]
    #[inline(always)]
    pub fn is_adc14inie_0(&self) -> bool {
        *self == ADC14INIE_A::ADC14INIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14INIE_1`"]
    #[inline(always)]
    pub fn is_adc14inie_1(&self) -> bool {
        *self == ADC14INIE_A::ADC14INIE_1
    }
}
#[doc = "Write proxy for field `ADC14INIE`"]
pub struct ADC14INIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14INIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14INIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14inie_0(self) -> &'a mut W {
        self.variant(ADC14INIE_A::ADC14INIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14inie_1(self) -> &'a mut W {
        self.variant(ADC14INIE_A::ADC14INIE_1)
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
#[doc = "Interrupt enable for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14LOIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14LOIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14LOIE_1 = 1,
}
impl From<ADC14LOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14LOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14LOIE`"]
pub type ADC14LOIE_R = crate::R<bool, ADC14LOIE_A>;
impl ADC14LOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14LOIE_A {
        match self.bits {
            false => ADC14LOIE_A::ADC14LOIE_0,
            true => ADC14LOIE_A::ADC14LOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14LOIE_0`"]
    #[inline(always)]
    pub fn is_adc14loie_0(&self) -> bool {
        *self == ADC14LOIE_A::ADC14LOIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14LOIE_1`"]
    #[inline(always)]
    pub fn is_adc14loie_1(&self) -> bool {
        *self == ADC14LOIE_A::ADC14LOIE_1
    }
}
#[doc = "Write proxy for field `ADC14LOIE`"]
pub struct ADC14LOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14LOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14LOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14loie_0(self) -> &'a mut W {
        self.variant(ADC14LOIE_A::ADC14LOIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14loie_1(self) -> &'a mut W {
        self.variant(ADC14LOIE_A::ADC14LOIE_1)
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
#[doc = "Interrupt enable for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14HIIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14HIIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14HIIE_1 = 1,
}
impl From<ADC14HIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14HIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14HIIE`"]
pub type ADC14HIIE_R = crate::R<bool, ADC14HIIE_A>;
impl ADC14HIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14HIIE_A {
        match self.bits {
            false => ADC14HIIE_A::ADC14HIIE_0,
            true => ADC14HIIE_A::ADC14HIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14HIIE_0`"]
    #[inline(always)]
    pub fn is_adc14hiie_0(&self) -> bool {
        *self == ADC14HIIE_A::ADC14HIIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14HIIE_1`"]
    #[inline(always)]
    pub fn is_adc14hiie_1(&self) -> bool {
        *self == ADC14HIIE_A::ADC14HIIE_1
    }
}
#[doc = "Write proxy for field `ADC14HIIE`"]
pub struct ADC14HIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14HIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14HIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14hiie_0(self) -> &'a mut W {
        self.variant(ADC14HIIE_A::ADC14HIIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14hiie_1(self) -> &'a mut W {
        self.variant(ADC14HIIE_A::ADC14HIIE_1)
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
#[doc = "ADC14MEMx overflow-interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14OVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14OVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14OVIE_1 = 1,
}
impl From<ADC14OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14OVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14OVIE`"]
pub type ADC14OVIE_R = crate::R<bool, ADC14OVIE_A>;
impl ADC14OVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14OVIE_A {
        match self.bits {
            false => ADC14OVIE_A::ADC14OVIE_0,
            true => ADC14OVIE_A::ADC14OVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14OVIE_0`"]
    #[inline(always)]
    pub fn is_adc14ovie_0(&self) -> bool {
        *self == ADC14OVIE_A::ADC14OVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14OVIE_1`"]
    #[inline(always)]
    pub fn is_adc14ovie_1(&self) -> bool {
        *self == ADC14OVIE_A::ADC14OVIE_1
    }
}
#[doc = "Write proxy for field `ADC14OVIE`"]
pub struct ADC14OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14OVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14OVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ovie_0(self) -> &'a mut W {
        self.variant(ADC14OVIE_A::ADC14OVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ovie_1(self) -> &'a mut W {
        self.variant(ADC14OVIE_A::ADC14OVIE_1)
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
#[doc = "ADC14 conversion-time-overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14TOVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14TOVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14TOVIE_1 = 1,
}
impl From<ADC14TOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14TOVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14TOVIE`"]
pub type ADC14TOVIE_R = crate::R<bool, ADC14TOVIE_A>;
impl ADC14TOVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14TOVIE_A {
        match self.bits {
            false => ADC14TOVIE_A::ADC14TOVIE_0,
            true => ADC14TOVIE_A::ADC14TOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIE_0`"]
    #[inline(always)]
    pub fn is_adc14tovie_0(&self) -> bool {
        *self == ADC14TOVIE_A::ADC14TOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIE_1`"]
    #[inline(always)]
    pub fn is_adc14tovie_1(&self) -> bool {
        *self == ADC14TOVIE_A::ADC14TOVIE_1
    }
}
#[doc = "Write proxy for field `ADC14TOVIE`"]
pub struct ADC14TOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14TOVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14TOVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14tovie_0(self) -> &'a mut W {
        self.variant(ADC14TOVIE_A::ADC14TOVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14tovie_1(self) -> &'a mut W {
        self.variant(ADC14TOVIE_A::ADC14TOVIE_1)
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
#[doc = "ADC14 local buffered reference ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14RDYIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14RDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14RDYIE_1 = 1,
}
impl From<ADC14RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14RDYIE`"]
pub type ADC14RDYIE_R = crate::R<bool, ADC14RDYIE_A>;
impl ADC14RDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14RDYIE_A {
        match self.bits {
            false => ADC14RDYIE_A::ADC14RDYIE_0,
            true => ADC14RDYIE_A::ADC14RDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIE_0`"]
    #[inline(always)]
    pub fn is_adc14rdyie_0(&self) -> bool {
        *self == ADC14RDYIE_A::ADC14RDYIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIE_1`"]
    #[inline(always)]
    pub fn is_adc14rdyie_1(&self) -> bool {
        *self == ADC14RDYIE_A::ADC14RDYIE_1
    }
}
#[doc = "Write proxy for field `ADC14RDYIE`"]
pub struct ADC14RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14RDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14RDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14rdyie_0(self) -> &'a mut W {
        self.variant(ADC14RDYIE_A::ADC14RDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14rdyie_1(self) -> &'a mut W {
        self.variant(ADC14RDYIE_A::ADC14RDYIE_1)
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
impl R {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&self) -> ADC14INIE_R {
        ADC14INIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&self) -> ADC14LOIE_R {
        ADC14LOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&self) -> ADC14HIIE_R {
        ADC14HIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&self) -> ADC14OVIE_R {
        ADC14OVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&self) -> ADC14TOVIE_R {
        ADC14TOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&self) -> ADC14RDYIE_R {
        ADC14RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&mut self) -> ADC14INIE_W {
        ADC14INIE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&mut self) -> ADC14LOIE_W {
        ADC14LOIE_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&mut self) -> ADC14HIIE_W {
        ADC14HIIE_W { w: self }
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&mut self) -> ADC14OVIE_W {
        ADC14OVIE_W { w: self }
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&mut self) -> ADC14TOVIE_W {
        ADC14TOVIE_W { w: self }
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&mut self) -> ADC14RDYIE_W {
        ADC14RDYIE_W { w: self }
    }
}
