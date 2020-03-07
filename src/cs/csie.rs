#[doc = "Reader of register CSIE"]
pub type R = crate::R<u32, super::CSIE>;
#[doc = "Writer for register CSIE"]
pub type W = crate::W<u32, super::CSIE>;
#[doc = "Register CSIE `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTIE_A {
    #[doc = "0: Interrupt disabled"]
    LFXTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LFXTIE_1 = 1,
}
impl From<LFXTIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTIE`"]
pub type LFXTIE_R = crate::R<bool, LFXTIE_A>;
impl LFXTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTIE_A {
        match self.bits {
            false => LFXTIE_A::LFXTIE_0,
            true => LFXTIE_A::LFXTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTIE_0`"]
    #[inline(always)]
    pub fn is_lfxtie_0(&self) -> bool {
        *self == LFXTIE_A::LFXTIE_0
    }
    #[doc = "Checks if the value of the field is `LFXTIE_1`"]
    #[inline(always)]
    pub fn is_lfxtie_1(&self) -> bool {
        *self == LFXTIE_A::LFXTIE_1
    }
}
#[doc = "Write proxy for field `LFXTIE`"]
pub struct LFXTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lfxtie_0(self) -> &'a mut W {
        self.variant(LFXTIE_A::LFXTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lfxtie_1(self) -> &'a mut W {
        self.variant(LFXTIE_A::LFXTIE_1)
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
#[doc = "HFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTIE_A {
    #[doc = "0: Interrupt disabled"]
    HFXTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    HFXTIE_1 = 1,
}
impl From<HFXTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTIE`"]
pub type HFXTIE_R = crate::R<bool, HFXTIE_A>;
impl HFXTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTIE_A {
        match self.bits {
            false => HFXTIE_A::HFXTIE_0,
            true => HFXTIE_A::HFXTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTIE_0`"]
    #[inline(always)]
    pub fn is_hfxtie_0(&self) -> bool {
        *self == HFXTIE_A::HFXTIE_0
    }
    #[doc = "Checks if the value of the field is `HFXTIE_1`"]
    #[inline(always)]
    pub fn is_hfxtie_1(&self) -> bool {
        *self == HFXTIE_A::HFXTIE_1
    }
}
#[doc = "Write proxy for field `HFXTIE`"]
pub struct HFXTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxtie_0(self) -> &'a mut W {
        self.variant(HFXTIE_A::HFXTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxtie_1(self) -> &'a mut W {
        self.variant(HFXTIE_A::HFXTIE_1)
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
#[doc = "HFXT2 oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXT2IE_A {
    #[doc = "0: Interrupt disabled"]
    HFXT2IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    HFXT2IE_1 = 1,
}
impl From<HFXT2IE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXT2IE`"]
pub type HFXT2IE_R = crate::R<bool, HFXT2IE_A>;
impl HFXT2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT2IE_A {
        match self.bits {
            false => HFXT2IE_A::HFXT2IE_0,
            true => HFXT2IE_A::HFXT2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT2IE_0`"]
    #[inline(always)]
    pub fn is_hfxt2ie_0(&self) -> bool {
        *self == HFXT2IE_A::HFXT2IE_0
    }
    #[doc = "Checks if the value of the field is `HFXT2IE_1`"]
    #[inline(always)]
    pub fn is_hfxt2ie_1(&self) -> bool {
        *self == HFXT2IE_A::HFXT2IE_1
    }
}
#[doc = "Write proxy for field `HFXT2IE`"]
pub struct HFXT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXT2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXT2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxt2ie_0(self) -> &'a mut W {
        self.variant(HFXT2IE_A::HFXT2IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxt2ie_1(self) -> &'a mut W {
        self.variant(HFXT2IE_A::HFXT2IE_1)
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
#[doc = "DCO external resistor open circuit fault flag interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOR_OPNIE_A {
    #[doc = "0: Interrupt disabled"]
    DCOR_OPNIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    DCOR_OPNIE_1 = 1,
}
impl From<DCOR_OPNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_OPNIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCOR_OPNIE`"]
pub type DCOR_OPNIE_R = crate::R<bool, DCOR_OPNIE_A>;
impl DCOR_OPNIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_OPNIE_A {
        match self.bits {
            false => DCOR_OPNIE_A::DCOR_OPNIE_0,
            true => DCOR_OPNIE_A::DCOR_OPNIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIE_0`"]
    #[inline(always)]
    pub fn is_dcor_opnie_0(&self) -> bool {
        *self == DCOR_OPNIE_A::DCOR_OPNIE_0
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIE_1`"]
    #[inline(always)]
    pub fn is_dcor_opnie_1(&self) -> bool {
        *self == DCOR_OPNIE_A::DCOR_OPNIE_1
    }
}
#[doc = "Write proxy for field `DCOR_OPNIE`"]
pub struct DCOR_OPNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOR_OPNIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOR_OPNIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn dcor_opnie_0(self) -> &'a mut W {
        self.variant(DCOR_OPNIE_A::DCOR_OPNIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn dcor_opnie_1(self) -> &'a mut W {
        self.variant(DCOR_OPNIE_A::DCOR_OPNIE_1)
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
#[doc = "Start fault counter interrupt enable LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTLFIE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTLFIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTLFIE_1 = 1,
}
impl From<FCNTLFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTLFIE`"]
pub type FCNTLFIE_R = crate::R<bool, FCNTLFIE_A>;
impl FCNTLFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLFIE_A {
        match self.bits {
            false => FCNTLFIE_A::FCNTLFIE_0,
            true => FCNTLFIE_A::FCNTLFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLFIE_0`"]
    #[inline(always)]
    pub fn is_fcntlfie_0(&self) -> bool {
        *self == FCNTLFIE_A::FCNTLFIE_0
    }
    #[doc = "Checks if the value of the field is `FCNTLFIE_1`"]
    #[inline(always)]
    pub fn is_fcntlfie_1(&self) -> bool {
        *self == FCNTLFIE_A::FCNTLFIE_1
    }
}
#[doc = "Write proxy for field `FCNTLFIE`"]
pub struct FCNTLFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTLFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTLFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcntlfie_0(self) -> &'a mut W {
        self.variant(FCNTLFIE_A::FCNTLFIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcntlfie_1(self) -> &'a mut W {
        self.variant(FCNTLFIE_A::FCNTLFIE_1)
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
#[doc = "Start fault counter interrupt enable HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHFIE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTHFIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTHFIE_1 = 1,
}
impl From<FCNTHFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHFIE`"]
pub type FCNTHFIE_R = crate::R<bool, FCNTHFIE_A>;
impl FCNTHFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHFIE_A {
        match self.bits {
            false => FCNTHFIE_A::FCNTHFIE_0,
            true => FCNTHFIE_A::FCNTHFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHFIE_0`"]
    #[inline(always)]
    pub fn is_fcnthfie_0(&self) -> bool {
        *self == FCNTHFIE_A::FCNTHFIE_0
    }
    #[doc = "Checks if the value of the field is `FCNTHFIE_1`"]
    #[inline(always)]
    pub fn is_fcnthfie_1(&self) -> bool {
        *self == FCNTHFIE_A::FCNTHFIE_1
    }
}
#[doc = "Write proxy for field `FCNTHFIE`"]
pub struct FCNTHFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthfie_0(self) -> &'a mut W {
        self.variant(FCNTHFIE_A::FCNTHFIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthfie_1(self) -> &'a mut W {
        self.variant(FCNTHFIE_A::FCNTHFIE_1)
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
#[doc = "Start fault counter interrupt enable HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHF2IE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTHF2IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTHF2IE_1 = 1,
}
impl From<FCNTHF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHF2IE`"]
pub type FCNTHF2IE_R = crate::R<bool, FCNTHF2IE_A>;
impl FCNTHF2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2IE_A {
        match self.bits {
            false => FCNTHF2IE_A::FCNTHF2IE_0,
            true => FCNTHF2IE_A::FCNTHF2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IE_0`"]
    #[inline(always)]
    pub fn is_fcnthf2ie_0(&self) -> bool {
        *self == FCNTHF2IE_A::FCNTHF2IE_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IE_1`"]
    #[inline(always)]
    pub fn is_fcnthf2ie_1(&self) -> bool {
        *self == FCNTHF2IE_A::FCNTHF2IE_1
    }
}
#[doc = "Write proxy for field `FCNTHF2IE`"]
pub struct FCNTHF2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHF2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHF2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthf2ie_0(self) -> &'a mut W {
        self.variant(FCNTHF2IE_A::FCNTHF2IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthf2ie_1(self) -> &'a mut W {
        self.variant(FCNTHF2IE_A::FCNTHF2IE_1)
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
#[doc = "PLL out-of-lock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLOOLIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLOOLIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLOOLIE_1 = 1,
}
impl From<PLLOOLIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOOLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLOOLIE`"]
pub type PLLOOLIE_R = crate::R<bool, PLLOOLIE_A>;
impl PLLOOLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOOLIE_A {
        match self.bits {
            false => PLLOOLIE_A::PLLOOLIE_0,
            true => PLLOOLIE_A::PLLOOLIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOOLIE_0`"]
    #[inline(always)]
    pub fn is_plloolie_0(&self) -> bool {
        *self == PLLOOLIE_A::PLLOOLIE_0
    }
    #[doc = "Checks if the value of the field is `PLLOOLIE_1`"]
    #[inline(always)]
    pub fn is_plloolie_1(&self) -> bool {
        *self == PLLOOLIE_A::PLLOOLIE_1
    }
}
#[doc = "Write proxy for field `PLLOOLIE`"]
pub struct PLLOOLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOOLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLOOLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloolie_0(self) -> &'a mut W {
        self.variant(PLLOOLIE_A::PLLOOLIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloolie_1(self) -> &'a mut W {
        self.variant(PLLOOLIE_A::PLLOOLIE_1)
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
#[doc = "PLL loss-of-signal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLOSIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLLOSIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLLOSIE_1 = 1,
}
impl From<PLLLOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLLOSIE`"]
pub type PLLLOSIE_R = crate::R<bool, PLLLOSIE_A>;
impl PLLLOSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOSIE_A {
        match self.bits {
            false => PLLLOSIE_A::PLLLOSIE_0,
            true => PLLLOSIE_A::PLLLOSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLLOSIE_0`"]
    #[inline(always)]
    pub fn is_plllosie_0(&self) -> bool {
        *self == PLLLOSIE_A::PLLLOSIE_0
    }
    #[doc = "Checks if the value of the field is `PLLLOSIE_1`"]
    #[inline(always)]
    pub fn is_plllosie_1(&self) -> bool {
        *self == PLLLOSIE_A::PLLLOSIE_1
    }
}
#[doc = "Write proxy for field `PLLLOSIE`"]
pub struct PLLLOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLOSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLLOSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plllosie_0(self) -> &'a mut W {
        self.variant(PLLLOSIE_A::PLLLOSIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plllosie_1(self) -> &'a mut W {
        self.variant(PLLLOSIE_A::PLLLOSIE_1)
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
#[doc = "PLL out-of-range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLOORIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLOORIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLOORIE_1 = 1,
}
impl From<PLLOORIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOORIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLOORIE`"]
pub type PLLOORIE_R = crate::R<bool, PLLOORIE_A>;
impl PLLOORIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOORIE_A {
        match self.bits {
            false => PLLOORIE_A::PLLOORIE_0,
            true => PLLOORIE_A::PLLOORIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOORIE_0`"]
    #[inline(always)]
    pub fn is_plloorie_0(&self) -> bool {
        *self == PLLOORIE_A::PLLOORIE_0
    }
    #[doc = "Checks if the value of the field is `PLLOORIE_1`"]
    #[inline(always)]
    pub fn is_plloorie_1(&self) -> bool {
        *self == PLLOORIE_A::PLLOORIE_1
    }
}
#[doc = "Write proxy for field `PLLOORIE`"]
pub struct PLLOORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOORIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLOORIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloorie_0(self) -> &'a mut W {
        self.variant(PLLOORIE_A::PLLOORIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloorie_1(self) -> &'a mut W {
        self.variant(PLLOORIE_A::PLLOORIE_1)
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
#[doc = "REFCNT period counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIE_A {
    #[doc = "0: Interrupt disabled"]
    CALIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CALIE_1 = 1,
}
impl From<CALIE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALIE`"]
pub type CALIE_R = crate::R<bool, CALIE_A>;
impl CALIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIE_A {
        match self.bits {
            false => CALIE_A::CALIE_0,
            true => CALIE_A::CALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALIE_0`"]
    #[inline(always)]
    pub fn is_calie_0(&self) -> bool {
        *self == CALIE_A::CALIE_0
    }
    #[doc = "Checks if the value of the field is `CALIE_1`"]
    #[inline(always)]
    pub fn is_calie_1(&self) -> bool {
        *self == CALIE_A::CALIE_1
    }
}
#[doc = "Write proxy for field `CALIE`"]
pub struct CALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn calie_0(self) -> &'a mut W {
        self.variant(CALIE_A::CALIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn calie_1(self) -> &'a mut W {
        self.variant(CALIE_A::CALIE_1)
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
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&self) -> LFXTIE_R {
        LFXTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&self) -> HFXTIE_R {
        HFXTIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&self) -> HFXT2IE_R {
        HFXT2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&self) -> DCOR_OPNIE_R {
        DCOR_OPNIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&self) -> FCNTLFIE_R {
        FCNTLFIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&self) -> FCNTHFIE_R {
        FCNTHFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&self) -> FCNTHF2IE_R {
        FCNTHF2IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&self) -> PLLOOLIE_R {
        PLLOOLIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&self) -> PLLLOSIE_R {
        PLLLOSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&self) -> PLLOORIE_R {
        PLLOORIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&self) -> CALIE_R {
        CALIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&mut self) -> LFXTIE_W {
        LFXTIE_W { w: self }
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&mut self) -> HFXTIE_W {
        HFXTIE_W { w: self }
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&mut self) -> HFXT2IE_W {
        HFXT2IE_W { w: self }
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&mut self) -> DCOR_OPNIE_W {
        DCOR_OPNIE_W { w: self }
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&mut self) -> FCNTLFIE_W {
        FCNTLFIE_W { w: self }
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&mut self) -> FCNTHFIE_W {
        FCNTHFIE_W { w: self }
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&mut self) -> FCNTHF2IE_W {
        FCNTHF2IE_W { w: self }
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&mut self) -> PLLOOLIE_W {
        PLLOOLIE_W { w: self }
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&mut self) -> PLLLOSIE_W {
        PLLLOSIE_W { w: self }
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&mut self) -> PLLOORIE_W {
        PLLOORIE_W { w: self }
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&mut self) -> CALIE_W {
        CALIE_W { w: self }
    }
}
