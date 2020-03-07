#[doc = "Reader of register ADC14IER0"]
pub type R = crate::R<u32, super::ADC14IER0>;
#[doc = "Writer for register ADC14IER0"]
pub type W = crate::W<u32, super::ADC14IER0>;
#[doc = "Register ADC14IER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14IER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE0_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE0_1 = 1,
}
impl From<ADC14IE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE0`"]
pub type ADC14IE0_R = crate::R<bool, ADC14IE0_A>;
impl ADC14IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE0_A {
        match self.bits {
            false => ADC14IE0_A::ADC14IE0_0,
            true => ADC14IE0_A::ADC14IE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE0_0`"]
    #[inline(always)]
    pub fn is_adc14ie0_0(&self) -> bool {
        *self == ADC14IE0_A::ADC14IE0_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE0_1`"]
    #[inline(always)]
    pub fn is_adc14ie0_1(&self) -> bool {
        *self == ADC14IE0_A::ADC14IE0_1
    }
}
#[doc = "Write proxy for field `ADC14IE0`"]
pub struct ADC14IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie0_0(self) -> &'a mut W {
        self.variant(ADC14IE0_A::ADC14IE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie0_1(self) -> &'a mut W {
        self.variant(ADC14IE0_A::ADC14IE0_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE1_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE1_1 = 1,
}
impl From<ADC14IE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE1`"]
pub type ADC14IE1_R = crate::R<bool, ADC14IE1_A>;
impl ADC14IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE1_A {
        match self.bits {
            false => ADC14IE1_A::ADC14IE1_0,
            true => ADC14IE1_A::ADC14IE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE1_0`"]
    #[inline(always)]
    pub fn is_adc14ie1_0(&self) -> bool {
        *self == ADC14IE1_A::ADC14IE1_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE1_1`"]
    #[inline(always)]
    pub fn is_adc14ie1_1(&self) -> bool {
        *self == ADC14IE1_A::ADC14IE1_1
    }
}
#[doc = "Write proxy for field `ADC14IE1`"]
pub struct ADC14IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie1_0(self) -> &'a mut W {
        self.variant(ADC14IE1_A::ADC14IE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie1_1(self) -> &'a mut W {
        self.variant(ADC14IE1_A::ADC14IE1_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE2_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE2_1 = 1,
}
impl From<ADC14IE2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE2`"]
pub type ADC14IE2_R = crate::R<bool, ADC14IE2_A>;
impl ADC14IE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE2_A {
        match self.bits {
            false => ADC14IE2_A::ADC14IE2_0,
            true => ADC14IE2_A::ADC14IE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE2_0`"]
    #[inline(always)]
    pub fn is_adc14ie2_0(&self) -> bool {
        *self == ADC14IE2_A::ADC14IE2_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE2_1`"]
    #[inline(always)]
    pub fn is_adc14ie2_1(&self) -> bool {
        *self == ADC14IE2_A::ADC14IE2_1
    }
}
#[doc = "Write proxy for field `ADC14IE2`"]
pub struct ADC14IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie2_0(self) -> &'a mut W {
        self.variant(ADC14IE2_A::ADC14IE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie2_1(self) -> &'a mut W {
        self.variant(ADC14IE2_A::ADC14IE2_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE3_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE3_1 = 1,
}
impl From<ADC14IE3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE3`"]
pub type ADC14IE3_R = crate::R<bool, ADC14IE3_A>;
impl ADC14IE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE3_A {
        match self.bits {
            false => ADC14IE3_A::ADC14IE3_0,
            true => ADC14IE3_A::ADC14IE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE3_0`"]
    #[inline(always)]
    pub fn is_adc14ie3_0(&self) -> bool {
        *self == ADC14IE3_A::ADC14IE3_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE3_1`"]
    #[inline(always)]
    pub fn is_adc14ie3_1(&self) -> bool {
        *self == ADC14IE3_A::ADC14IE3_1
    }
}
#[doc = "Write proxy for field `ADC14IE3`"]
pub struct ADC14IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie3_0(self) -> &'a mut W {
        self.variant(ADC14IE3_A::ADC14IE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie3_1(self) -> &'a mut W {
        self.variant(ADC14IE3_A::ADC14IE3_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE4_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE4_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE4_1 = 1,
}
impl From<ADC14IE4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE4`"]
pub type ADC14IE4_R = crate::R<bool, ADC14IE4_A>;
impl ADC14IE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE4_A {
        match self.bits {
            false => ADC14IE4_A::ADC14IE4_0,
            true => ADC14IE4_A::ADC14IE4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE4_0`"]
    #[inline(always)]
    pub fn is_adc14ie4_0(&self) -> bool {
        *self == ADC14IE4_A::ADC14IE4_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE4_1`"]
    #[inline(always)]
    pub fn is_adc14ie4_1(&self) -> bool {
        *self == ADC14IE4_A::ADC14IE4_1
    }
}
#[doc = "Write proxy for field `ADC14IE4`"]
pub struct ADC14IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie4_0(self) -> &'a mut W {
        self.variant(ADC14IE4_A::ADC14IE4_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie4_1(self) -> &'a mut W {
        self.variant(ADC14IE4_A::ADC14IE4_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE5_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE5_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE5_1 = 1,
}
impl From<ADC14IE5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE5`"]
pub type ADC14IE5_R = crate::R<bool, ADC14IE5_A>;
impl ADC14IE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE5_A {
        match self.bits {
            false => ADC14IE5_A::ADC14IE5_0,
            true => ADC14IE5_A::ADC14IE5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE5_0`"]
    #[inline(always)]
    pub fn is_adc14ie5_0(&self) -> bool {
        *self == ADC14IE5_A::ADC14IE5_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE5_1`"]
    #[inline(always)]
    pub fn is_adc14ie5_1(&self) -> bool {
        *self == ADC14IE5_A::ADC14IE5_1
    }
}
#[doc = "Write proxy for field `ADC14IE5`"]
pub struct ADC14IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie5_0(self) -> &'a mut W {
        self.variant(ADC14IE5_A::ADC14IE5_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie5_1(self) -> &'a mut W {
        self.variant(ADC14IE5_A::ADC14IE5_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE6_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE6_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE6_1 = 1,
}
impl From<ADC14IE6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE6`"]
pub type ADC14IE6_R = crate::R<bool, ADC14IE6_A>;
impl ADC14IE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE6_A {
        match self.bits {
            false => ADC14IE6_A::ADC14IE6_0,
            true => ADC14IE6_A::ADC14IE6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE6_0`"]
    #[inline(always)]
    pub fn is_adc14ie6_0(&self) -> bool {
        *self == ADC14IE6_A::ADC14IE6_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE6_1`"]
    #[inline(always)]
    pub fn is_adc14ie6_1(&self) -> bool {
        *self == ADC14IE6_A::ADC14IE6_1
    }
}
#[doc = "Write proxy for field `ADC14IE6`"]
pub struct ADC14IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie6_0(self) -> &'a mut W {
        self.variant(ADC14IE6_A::ADC14IE6_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie6_1(self) -> &'a mut W {
        self.variant(ADC14IE6_A::ADC14IE6_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE7_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE7_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE7_1 = 1,
}
impl From<ADC14IE7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE7`"]
pub type ADC14IE7_R = crate::R<bool, ADC14IE7_A>;
impl ADC14IE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE7_A {
        match self.bits {
            false => ADC14IE7_A::ADC14IE7_0,
            true => ADC14IE7_A::ADC14IE7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE7_0`"]
    #[inline(always)]
    pub fn is_adc14ie7_0(&self) -> bool {
        *self == ADC14IE7_A::ADC14IE7_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE7_1`"]
    #[inline(always)]
    pub fn is_adc14ie7_1(&self) -> bool {
        *self == ADC14IE7_A::ADC14IE7_1
    }
}
#[doc = "Write proxy for field `ADC14IE7`"]
pub struct ADC14IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie7_0(self) -> &'a mut W {
        self.variant(ADC14IE7_A::ADC14IE7_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie7_1(self) -> &'a mut W {
        self.variant(ADC14IE7_A::ADC14IE7_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE8_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE8_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE8_1 = 1,
}
impl From<ADC14IE8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE8`"]
pub type ADC14IE8_R = crate::R<bool, ADC14IE8_A>;
impl ADC14IE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE8_A {
        match self.bits {
            false => ADC14IE8_A::ADC14IE8_0,
            true => ADC14IE8_A::ADC14IE8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE8_0`"]
    #[inline(always)]
    pub fn is_adc14ie8_0(&self) -> bool {
        *self == ADC14IE8_A::ADC14IE8_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE8_1`"]
    #[inline(always)]
    pub fn is_adc14ie8_1(&self) -> bool {
        *self == ADC14IE8_A::ADC14IE8_1
    }
}
#[doc = "Write proxy for field `ADC14IE8`"]
pub struct ADC14IE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie8_0(self) -> &'a mut W {
        self.variant(ADC14IE8_A::ADC14IE8_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie8_1(self) -> &'a mut W {
        self.variant(ADC14IE8_A::ADC14IE8_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE9_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE9_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE9_1 = 1,
}
impl From<ADC14IE9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE9`"]
pub type ADC14IE9_R = crate::R<bool, ADC14IE9_A>;
impl ADC14IE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE9_A {
        match self.bits {
            false => ADC14IE9_A::ADC14IE9_0,
            true => ADC14IE9_A::ADC14IE9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE9_0`"]
    #[inline(always)]
    pub fn is_adc14ie9_0(&self) -> bool {
        *self == ADC14IE9_A::ADC14IE9_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE9_1`"]
    #[inline(always)]
    pub fn is_adc14ie9_1(&self) -> bool {
        *self == ADC14IE9_A::ADC14IE9_1
    }
}
#[doc = "Write proxy for field `ADC14IE9`"]
pub struct ADC14IE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie9_0(self) -> &'a mut W {
        self.variant(ADC14IE9_A::ADC14IE9_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie9_1(self) -> &'a mut W {
        self.variant(ADC14IE9_A::ADC14IE9_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE10_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE10_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE10_1 = 1,
}
impl From<ADC14IE10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE10`"]
pub type ADC14IE10_R = crate::R<bool, ADC14IE10_A>;
impl ADC14IE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE10_A {
        match self.bits {
            false => ADC14IE10_A::ADC14IE10_0,
            true => ADC14IE10_A::ADC14IE10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE10_0`"]
    #[inline(always)]
    pub fn is_adc14ie10_0(&self) -> bool {
        *self == ADC14IE10_A::ADC14IE10_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE10_1`"]
    #[inline(always)]
    pub fn is_adc14ie10_1(&self) -> bool {
        *self == ADC14IE10_A::ADC14IE10_1
    }
}
#[doc = "Write proxy for field `ADC14IE10`"]
pub struct ADC14IE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie10_0(self) -> &'a mut W {
        self.variant(ADC14IE10_A::ADC14IE10_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie10_1(self) -> &'a mut W {
        self.variant(ADC14IE10_A::ADC14IE10_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE11_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE11_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE11_1 = 1,
}
impl From<ADC14IE11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE11`"]
pub type ADC14IE11_R = crate::R<bool, ADC14IE11_A>;
impl ADC14IE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE11_A {
        match self.bits {
            false => ADC14IE11_A::ADC14IE11_0,
            true => ADC14IE11_A::ADC14IE11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE11_0`"]
    #[inline(always)]
    pub fn is_adc14ie11_0(&self) -> bool {
        *self == ADC14IE11_A::ADC14IE11_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE11_1`"]
    #[inline(always)]
    pub fn is_adc14ie11_1(&self) -> bool {
        *self == ADC14IE11_A::ADC14IE11_1
    }
}
#[doc = "Write proxy for field `ADC14IE11`"]
pub struct ADC14IE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie11_0(self) -> &'a mut W {
        self.variant(ADC14IE11_A::ADC14IE11_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie11_1(self) -> &'a mut W {
        self.variant(ADC14IE11_A::ADC14IE11_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE12_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE12_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE12_1 = 1,
}
impl From<ADC14IE12_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE12`"]
pub type ADC14IE12_R = crate::R<bool, ADC14IE12_A>;
impl ADC14IE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE12_A {
        match self.bits {
            false => ADC14IE12_A::ADC14IE12_0,
            true => ADC14IE12_A::ADC14IE12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE12_0`"]
    #[inline(always)]
    pub fn is_adc14ie12_0(&self) -> bool {
        *self == ADC14IE12_A::ADC14IE12_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE12_1`"]
    #[inline(always)]
    pub fn is_adc14ie12_1(&self) -> bool {
        *self == ADC14IE12_A::ADC14IE12_1
    }
}
#[doc = "Write proxy for field `ADC14IE12`"]
pub struct ADC14IE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie12_0(self) -> &'a mut W {
        self.variant(ADC14IE12_A::ADC14IE12_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie12_1(self) -> &'a mut W {
        self.variant(ADC14IE12_A::ADC14IE12_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE13_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE13_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE13_1 = 1,
}
impl From<ADC14IE13_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE13`"]
pub type ADC14IE13_R = crate::R<bool, ADC14IE13_A>;
impl ADC14IE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE13_A {
        match self.bits {
            false => ADC14IE13_A::ADC14IE13_0,
            true => ADC14IE13_A::ADC14IE13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE13_0`"]
    #[inline(always)]
    pub fn is_adc14ie13_0(&self) -> bool {
        *self == ADC14IE13_A::ADC14IE13_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE13_1`"]
    #[inline(always)]
    pub fn is_adc14ie13_1(&self) -> bool {
        *self == ADC14IE13_A::ADC14IE13_1
    }
}
#[doc = "Write proxy for field `ADC14IE13`"]
pub struct ADC14IE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie13_0(self) -> &'a mut W {
        self.variant(ADC14IE13_A::ADC14IE13_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie13_1(self) -> &'a mut W {
        self.variant(ADC14IE13_A::ADC14IE13_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE14_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE14_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE14_1 = 1,
}
impl From<ADC14IE14_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE14`"]
pub type ADC14IE14_R = crate::R<bool, ADC14IE14_A>;
impl ADC14IE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE14_A {
        match self.bits {
            false => ADC14IE14_A::ADC14IE14_0,
            true => ADC14IE14_A::ADC14IE14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE14_0`"]
    #[inline(always)]
    pub fn is_adc14ie14_0(&self) -> bool {
        *self == ADC14IE14_A::ADC14IE14_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE14_1`"]
    #[inline(always)]
    pub fn is_adc14ie14_1(&self) -> bool {
        *self == ADC14IE14_A::ADC14IE14_1
    }
}
#[doc = "Write proxy for field `ADC14IE14`"]
pub struct ADC14IE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie14_0(self) -> &'a mut W {
        self.variant(ADC14IE14_A::ADC14IE14_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie14_1(self) -> &'a mut W {
        self.variant(ADC14IE14_A::ADC14IE14_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE15_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE15_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE15_1 = 1,
}
impl From<ADC14IE15_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE15`"]
pub type ADC14IE15_R = crate::R<bool, ADC14IE15_A>;
impl ADC14IE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE15_A {
        match self.bits {
            false => ADC14IE15_A::ADC14IE15_0,
            true => ADC14IE15_A::ADC14IE15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE15_0`"]
    #[inline(always)]
    pub fn is_adc14ie15_0(&self) -> bool {
        *self == ADC14IE15_A::ADC14IE15_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE15_1`"]
    #[inline(always)]
    pub fn is_adc14ie15_1(&self) -> bool {
        *self == ADC14IE15_A::ADC14IE15_1
    }
}
#[doc = "Write proxy for field `ADC14IE15`"]
pub struct ADC14IE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie15_0(self) -> &'a mut W {
        self.variant(ADC14IE15_A::ADC14IE15_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie15_1(self) -> &'a mut W {
        self.variant(ADC14IE15_A::ADC14IE15_1)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE16_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE16_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE16_1 = 1,
}
impl From<ADC14IE16_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE16`"]
pub type ADC14IE16_R = crate::R<bool, ADC14IE16_A>;
impl ADC14IE16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE16_A {
        match self.bits {
            false => ADC14IE16_A::ADC14IE16_0,
            true => ADC14IE16_A::ADC14IE16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE16_0`"]
    #[inline(always)]
    pub fn is_adc14ie16_0(&self) -> bool {
        *self == ADC14IE16_A::ADC14IE16_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE16_1`"]
    #[inline(always)]
    pub fn is_adc14ie16_1(&self) -> bool {
        *self == ADC14IE16_A::ADC14IE16_1
    }
}
#[doc = "Write proxy for field `ADC14IE16`"]
pub struct ADC14IE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie16_0(self) -> &'a mut W {
        self.variant(ADC14IE16_A::ADC14IE16_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie16_1(self) -> &'a mut W {
        self.variant(ADC14IE16_A::ADC14IE16_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE17_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE17_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE17_1 = 1,
}
impl From<ADC14IE17_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE17`"]
pub type ADC14IE17_R = crate::R<bool, ADC14IE17_A>;
impl ADC14IE17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE17_A {
        match self.bits {
            false => ADC14IE17_A::ADC14IE17_0,
            true => ADC14IE17_A::ADC14IE17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE17_0`"]
    #[inline(always)]
    pub fn is_adc14ie17_0(&self) -> bool {
        *self == ADC14IE17_A::ADC14IE17_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE17_1`"]
    #[inline(always)]
    pub fn is_adc14ie17_1(&self) -> bool {
        *self == ADC14IE17_A::ADC14IE17_1
    }
}
#[doc = "Write proxy for field `ADC14IE17`"]
pub struct ADC14IE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie17_0(self) -> &'a mut W {
        self.variant(ADC14IE17_A::ADC14IE17_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie17_1(self) -> &'a mut W {
        self.variant(ADC14IE17_A::ADC14IE17_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE19_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE19_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE19_1 = 1,
}
impl From<ADC14IE19_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE19`"]
pub type ADC14IE19_R = crate::R<bool, ADC14IE19_A>;
impl ADC14IE19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE19_A {
        match self.bits {
            false => ADC14IE19_A::ADC14IE19_0,
            true => ADC14IE19_A::ADC14IE19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE19_0`"]
    #[inline(always)]
    pub fn is_adc14ie19_0(&self) -> bool {
        *self == ADC14IE19_A::ADC14IE19_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE19_1`"]
    #[inline(always)]
    pub fn is_adc14ie19_1(&self) -> bool {
        *self == ADC14IE19_A::ADC14IE19_1
    }
}
#[doc = "Write proxy for field `ADC14IE19`"]
pub struct ADC14IE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie19_0(self) -> &'a mut W {
        self.variant(ADC14IE19_A::ADC14IE19_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie19_1(self) -> &'a mut W {
        self.variant(ADC14IE19_A::ADC14IE19_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE18_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE18_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE18_1 = 1,
}
impl From<ADC14IE18_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE18`"]
pub type ADC14IE18_R = crate::R<bool, ADC14IE18_A>;
impl ADC14IE18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE18_A {
        match self.bits {
            false => ADC14IE18_A::ADC14IE18_0,
            true => ADC14IE18_A::ADC14IE18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE18_0`"]
    #[inline(always)]
    pub fn is_adc14ie18_0(&self) -> bool {
        *self == ADC14IE18_A::ADC14IE18_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE18_1`"]
    #[inline(always)]
    pub fn is_adc14ie18_1(&self) -> bool {
        *self == ADC14IE18_A::ADC14IE18_1
    }
}
#[doc = "Write proxy for field `ADC14IE18`"]
pub struct ADC14IE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie18_0(self) -> &'a mut W {
        self.variant(ADC14IE18_A::ADC14IE18_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie18_1(self) -> &'a mut W {
        self.variant(ADC14IE18_A::ADC14IE18_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE20_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE20_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE20_1 = 1,
}
impl From<ADC14IE20_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE20`"]
pub type ADC14IE20_R = crate::R<bool, ADC14IE20_A>;
impl ADC14IE20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE20_A {
        match self.bits {
            false => ADC14IE20_A::ADC14IE20_0,
            true => ADC14IE20_A::ADC14IE20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE20_0`"]
    #[inline(always)]
    pub fn is_adc14ie20_0(&self) -> bool {
        *self == ADC14IE20_A::ADC14IE20_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE20_1`"]
    #[inline(always)]
    pub fn is_adc14ie20_1(&self) -> bool {
        *self == ADC14IE20_A::ADC14IE20_1
    }
}
#[doc = "Write proxy for field `ADC14IE20`"]
pub struct ADC14IE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie20_0(self) -> &'a mut W {
        self.variant(ADC14IE20_A::ADC14IE20_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie20_1(self) -> &'a mut W {
        self.variant(ADC14IE20_A::ADC14IE20_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE21_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE21_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE21_1 = 1,
}
impl From<ADC14IE21_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE21`"]
pub type ADC14IE21_R = crate::R<bool, ADC14IE21_A>;
impl ADC14IE21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE21_A {
        match self.bits {
            false => ADC14IE21_A::ADC14IE21_0,
            true => ADC14IE21_A::ADC14IE21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE21_0`"]
    #[inline(always)]
    pub fn is_adc14ie21_0(&self) -> bool {
        *self == ADC14IE21_A::ADC14IE21_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE21_1`"]
    #[inline(always)]
    pub fn is_adc14ie21_1(&self) -> bool {
        *self == ADC14IE21_A::ADC14IE21_1
    }
}
#[doc = "Write proxy for field `ADC14IE21`"]
pub struct ADC14IE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie21_0(self) -> &'a mut W {
        self.variant(ADC14IE21_A::ADC14IE21_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie21_1(self) -> &'a mut W {
        self.variant(ADC14IE21_A::ADC14IE21_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE22_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE22_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE22_1 = 1,
}
impl From<ADC14IE22_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE22`"]
pub type ADC14IE22_R = crate::R<bool, ADC14IE22_A>;
impl ADC14IE22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE22_A {
        match self.bits {
            false => ADC14IE22_A::ADC14IE22_0,
            true => ADC14IE22_A::ADC14IE22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE22_0`"]
    #[inline(always)]
    pub fn is_adc14ie22_0(&self) -> bool {
        *self == ADC14IE22_A::ADC14IE22_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE22_1`"]
    #[inline(always)]
    pub fn is_adc14ie22_1(&self) -> bool {
        *self == ADC14IE22_A::ADC14IE22_1
    }
}
#[doc = "Write proxy for field `ADC14IE22`"]
pub struct ADC14IE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie22_0(self) -> &'a mut W {
        self.variant(ADC14IE22_A::ADC14IE22_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie22_1(self) -> &'a mut W {
        self.variant(ADC14IE22_A::ADC14IE22_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE23_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE23_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE23_1 = 1,
}
impl From<ADC14IE23_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE23`"]
pub type ADC14IE23_R = crate::R<bool, ADC14IE23_A>;
impl ADC14IE23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE23_A {
        match self.bits {
            false => ADC14IE23_A::ADC14IE23_0,
            true => ADC14IE23_A::ADC14IE23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE23_0`"]
    #[inline(always)]
    pub fn is_adc14ie23_0(&self) -> bool {
        *self == ADC14IE23_A::ADC14IE23_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE23_1`"]
    #[inline(always)]
    pub fn is_adc14ie23_1(&self) -> bool {
        *self == ADC14IE23_A::ADC14IE23_1
    }
}
#[doc = "Write proxy for field `ADC14IE23`"]
pub struct ADC14IE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie23_0(self) -> &'a mut W {
        self.variant(ADC14IE23_A::ADC14IE23_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie23_1(self) -> &'a mut W {
        self.variant(ADC14IE23_A::ADC14IE23_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE24_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE24_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE24_1 = 1,
}
impl From<ADC14IE24_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE24`"]
pub type ADC14IE24_R = crate::R<bool, ADC14IE24_A>;
impl ADC14IE24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE24_A {
        match self.bits {
            false => ADC14IE24_A::ADC14IE24_0,
            true => ADC14IE24_A::ADC14IE24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE24_0`"]
    #[inline(always)]
    pub fn is_adc14ie24_0(&self) -> bool {
        *self == ADC14IE24_A::ADC14IE24_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE24_1`"]
    #[inline(always)]
    pub fn is_adc14ie24_1(&self) -> bool {
        *self == ADC14IE24_A::ADC14IE24_1
    }
}
#[doc = "Write proxy for field `ADC14IE24`"]
pub struct ADC14IE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie24_0(self) -> &'a mut W {
        self.variant(ADC14IE24_A::ADC14IE24_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie24_1(self) -> &'a mut W {
        self.variant(ADC14IE24_A::ADC14IE24_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE25_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE25_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE25_1 = 1,
}
impl From<ADC14IE25_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE25`"]
pub type ADC14IE25_R = crate::R<bool, ADC14IE25_A>;
impl ADC14IE25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE25_A {
        match self.bits {
            false => ADC14IE25_A::ADC14IE25_0,
            true => ADC14IE25_A::ADC14IE25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE25_0`"]
    #[inline(always)]
    pub fn is_adc14ie25_0(&self) -> bool {
        *self == ADC14IE25_A::ADC14IE25_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE25_1`"]
    #[inline(always)]
    pub fn is_adc14ie25_1(&self) -> bool {
        *self == ADC14IE25_A::ADC14IE25_1
    }
}
#[doc = "Write proxy for field `ADC14IE25`"]
pub struct ADC14IE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie25_0(self) -> &'a mut W {
        self.variant(ADC14IE25_A::ADC14IE25_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie25_1(self) -> &'a mut W {
        self.variant(ADC14IE25_A::ADC14IE25_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE26_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE26_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE26_1 = 1,
}
impl From<ADC14IE26_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE26`"]
pub type ADC14IE26_R = crate::R<bool, ADC14IE26_A>;
impl ADC14IE26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE26_A {
        match self.bits {
            false => ADC14IE26_A::ADC14IE26_0,
            true => ADC14IE26_A::ADC14IE26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE26_0`"]
    #[inline(always)]
    pub fn is_adc14ie26_0(&self) -> bool {
        *self == ADC14IE26_A::ADC14IE26_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE26_1`"]
    #[inline(always)]
    pub fn is_adc14ie26_1(&self) -> bool {
        *self == ADC14IE26_A::ADC14IE26_1
    }
}
#[doc = "Write proxy for field `ADC14IE26`"]
pub struct ADC14IE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie26_0(self) -> &'a mut W {
        self.variant(ADC14IE26_A::ADC14IE26_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie26_1(self) -> &'a mut W {
        self.variant(ADC14IE26_A::ADC14IE26_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE27_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE27_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE27_1 = 1,
}
impl From<ADC14IE27_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE27`"]
pub type ADC14IE27_R = crate::R<bool, ADC14IE27_A>;
impl ADC14IE27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE27_A {
        match self.bits {
            false => ADC14IE27_A::ADC14IE27_0,
            true => ADC14IE27_A::ADC14IE27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE27_0`"]
    #[inline(always)]
    pub fn is_adc14ie27_0(&self) -> bool {
        *self == ADC14IE27_A::ADC14IE27_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE27_1`"]
    #[inline(always)]
    pub fn is_adc14ie27_1(&self) -> bool {
        *self == ADC14IE27_A::ADC14IE27_1
    }
}
#[doc = "Write proxy for field `ADC14IE27`"]
pub struct ADC14IE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie27_0(self) -> &'a mut W {
        self.variant(ADC14IE27_A::ADC14IE27_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie27_1(self) -> &'a mut W {
        self.variant(ADC14IE27_A::ADC14IE27_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE28_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE28_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE28_1 = 1,
}
impl From<ADC14IE28_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE28`"]
pub type ADC14IE28_R = crate::R<bool, ADC14IE28_A>;
impl ADC14IE28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE28_A {
        match self.bits {
            false => ADC14IE28_A::ADC14IE28_0,
            true => ADC14IE28_A::ADC14IE28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE28_0`"]
    #[inline(always)]
    pub fn is_adc14ie28_0(&self) -> bool {
        *self == ADC14IE28_A::ADC14IE28_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE28_1`"]
    #[inline(always)]
    pub fn is_adc14ie28_1(&self) -> bool {
        *self == ADC14IE28_A::ADC14IE28_1
    }
}
#[doc = "Write proxy for field `ADC14IE28`"]
pub struct ADC14IE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie28_0(self) -> &'a mut W {
        self.variant(ADC14IE28_A::ADC14IE28_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie28_1(self) -> &'a mut W {
        self.variant(ADC14IE28_A::ADC14IE28_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE29_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE29_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE29_1 = 1,
}
impl From<ADC14IE29_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE29`"]
pub type ADC14IE29_R = crate::R<bool, ADC14IE29_A>;
impl ADC14IE29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE29_A {
        match self.bits {
            false => ADC14IE29_A::ADC14IE29_0,
            true => ADC14IE29_A::ADC14IE29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE29_0`"]
    #[inline(always)]
    pub fn is_adc14ie29_0(&self) -> bool {
        *self == ADC14IE29_A::ADC14IE29_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE29_1`"]
    #[inline(always)]
    pub fn is_adc14ie29_1(&self) -> bool {
        *self == ADC14IE29_A::ADC14IE29_1
    }
}
#[doc = "Write proxy for field `ADC14IE29`"]
pub struct ADC14IE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie29_0(self) -> &'a mut W {
        self.variant(ADC14IE29_A::ADC14IE29_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie29_1(self) -> &'a mut W {
        self.variant(ADC14IE29_A::ADC14IE29_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE30_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE30_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE30_1 = 1,
}
impl From<ADC14IE30_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE30`"]
pub type ADC14IE30_R = crate::R<bool, ADC14IE30_A>;
impl ADC14IE30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE30_A {
        match self.bits {
            false => ADC14IE30_A::ADC14IE30_0,
            true => ADC14IE30_A::ADC14IE30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE30_0`"]
    #[inline(always)]
    pub fn is_adc14ie30_0(&self) -> bool {
        *self == ADC14IE30_A::ADC14IE30_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE30_1`"]
    #[inline(always)]
    pub fn is_adc14ie30_1(&self) -> bool {
        *self == ADC14IE30_A::ADC14IE30_1
    }
}
#[doc = "Write proxy for field `ADC14IE30`"]
pub struct ADC14IE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie30_0(self) -> &'a mut W {
        self.variant(ADC14IE30_A::ADC14IE30_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie30_1(self) -> &'a mut W {
        self.variant(ADC14IE30_A::ADC14IE30_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14IE31_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE31_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE31_1 = 1,
}
impl From<ADC14IE31_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC14IE31`"]
pub type ADC14IE31_R = crate::R<bool, ADC14IE31_A>;
impl ADC14IE31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE31_A {
        match self.bits {
            false => ADC14IE31_A::ADC14IE31_0,
            true => ADC14IE31_A::ADC14IE31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE31_0`"]
    #[inline(always)]
    pub fn is_adc14ie31_0(&self) -> bool {
        *self == ADC14IE31_A::ADC14IE31_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE31_1`"]
    #[inline(always)]
    pub fn is_adc14ie31_1(&self) -> bool {
        *self == ADC14IE31_A::ADC14IE31_1
    }
}
#[doc = "Write proxy for field `ADC14IE31`"]
pub struct ADC14IE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14IE31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14IE31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie31_0(self) -> &'a mut W {
        self.variant(ADC14IE31_A::ADC14IE31_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie31_1(self) -> &'a mut W {
        self.variant(ADC14IE31_A::ADC14IE31_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&self) -> ADC14IE0_R {
        ADC14IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&self) -> ADC14IE1_R {
        ADC14IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&self) -> ADC14IE2_R {
        ADC14IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&self) -> ADC14IE3_R {
        ADC14IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&self) -> ADC14IE4_R {
        ADC14IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&self) -> ADC14IE5_R {
        ADC14IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&self) -> ADC14IE6_R {
        ADC14IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&self) -> ADC14IE7_R {
        ADC14IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&self) -> ADC14IE8_R {
        ADC14IE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&self) -> ADC14IE9_R {
        ADC14IE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&self) -> ADC14IE10_R {
        ADC14IE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&self) -> ADC14IE11_R {
        ADC14IE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&self) -> ADC14IE12_R {
        ADC14IE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&self) -> ADC14IE13_R {
        ADC14IE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&self) -> ADC14IE14_R {
        ADC14IE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&self) -> ADC14IE15_R {
        ADC14IE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&self) -> ADC14IE16_R {
        ADC14IE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&self) -> ADC14IE17_R {
        ADC14IE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&self) -> ADC14IE19_R {
        ADC14IE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&self) -> ADC14IE18_R {
        ADC14IE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&self) -> ADC14IE20_R {
        ADC14IE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&self) -> ADC14IE21_R {
        ADC14IE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&self) -> ADC14IE22_R {
        ADC14IE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&self) -> ADC14IE23_R {
        ADC14IE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&self) -> ADC14IE24_R {
        ADC14IE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&self) -> ADC14IE25_R {
        ADC14IE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&self) -> ADC14IE26_R {
        ADC14IE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&self) -> ADC14IE27_R {
        ADC14IE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&self) -> ADC14IE28_R {
        ADC14IE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&self) -> ADC14IE29_R {
        ADC14IE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&self) -> ADC14IE30_R {
        ADC14IE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&self) -> ADC14IE31_R {
        ADC14IE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&mut self) -> ADC14IE0_W {
        ADC14IE0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&mut self) -> ADC14IE1_W {
        ADC14IE1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&mut self) -> ADC14IE2_W {
        ADC14IE2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&mut self) -> ADC14IE3_W {
        ADC14IE3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&mut self) -> ADC14IE4_W {
        ADC14IE4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&mut self) -> ADC14IE5_W {
        ADC14IE5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&mut self) -> ADC14IE6_W {
        ADC14IE6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&mut self) -> ADC14IE7_W {
        ADC14IE7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&mut self) -> ADC14IE8_W {
        ADC14IE8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&mut self) -> ADC14IE9_W {
        ADC14IE9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&mut self) -> ADC14IE10_W {
        ADC14IE10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&mut self) -> ADC14IE11_W {
        ADC14IE11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&mut self) -> ADC14IE12_W {
        ADC14IE12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&mut self) -> ADC14IE13_W {
        ADC14IE13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&mut self) -> ADC14IE14_W {
        ADC14IE14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&mut self) -> ADC14IE15_W {
        ADC14IE15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&mut self) -> ADC14IE16_W {
        ADC14IE16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&mut self) -> ADC14IE17_W {
        ADC14IE17_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&mut self) -> ADC14IE19_W {
        ADC14IE19_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&mut self) -> ADC14IE18_W {
        ADC14IE18_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&mut self) -> ADC14IE20_W {
        ADC14IE20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&mut self) -> ADC14IE21_W {
        ADC14IE21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&mut self) -> ADC14IE22_W {
        ADC14IE22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&mut self) -> ADC14IE23_W {
        ADC14IE23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&mut self) -> ADC14IE24_W {
        ADC14IE24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&mut self) -> ADC14IE25_W {
        ADC14IE25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&mut self) -> ADC14IE26_W {
        ADC14IE26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&mut self) -> ADC14IE27_W {
        ADC14IE27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&mut self) -> ADC14IE28_W {
        ADC14IE28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&mut self) -> ADC14IE29_W {
        ADC14IE29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&mut self) -> ADC14IE30_W {
        ADC14IE30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&mut self) -> ADC14IE31_W {
        ADC14IE31_W { w: self }
    }
}
