#[doc = "Reader of register RTCCTL0"]
pub type R = crate::R<u16, super::RTCCTL0>;
#[doc = "Writer for register RTCCTL0"]
pub type W = crate::W<u16, super::RTCCTL0>;
#[doc = "Register RTCCTL0 `reset()`'s with value 0x9608"]
impl crate::ResetValue for super::RTCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9608
    }
}
#[doc = "Real-time clock ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCRDYIFG_A {
    #[doc = "0: RTC cannot be read safely"]
    RTCRDYIFG_0 = 0,
    #[doc = "1: RTC can be read safely"]
    RTCRDYIFG_1 = 1,
}
impl From<RTCRDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCRDYIFG`"]
pub type RTCRDYIFG_R = crate::R<bool, RTCRDYIFG_A>;
impl RTCRDYIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDYIFG_A {
        match self.bits {
            false => RTCRDYIFG_A::RTCRDYIFG_0,
            true => RTCRDYIFG_A::RTCRDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDYIFG_0`"]
    #[inline(always)]
    pub fn is_rtcrdyifg_0(&self) -> bool {
        *self == RTCRDYIFG_A::RTCRDYIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCRDYIFG_1`"]
    #[inline(always)]
    pub fn is_rtcrdyifg_1(&self) -> bool {
        *self == RTCRDYIFG_A::RTCRDYIFG_1
    }
}
#[doc = "Write proxy for field `RTCRDYIFG`"]
pub struct RTCRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCRDYIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC cannot be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_0(self) -> &'a mut W {
        self.variant(RTCRDYIFG_A::RTCRDYIFG_0)
    }
    #[doc = "RTC can be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_1(self) -> &'a mut W {
        self.variant(RTCRDYIFG_A::RTCRDYIFG_1)
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
#[doc = "Real-time clock alarm interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCAIFG_A {
    #[doc = "0: No time event occurred"]
    RTCAIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RTCAIFG_1 = 1,
}
impl From<RTCAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCAIFG`"]
pub type RTCAIFG_R = crate::R<bool, RTCAIFG_A>;
impl RTCAIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAIFG_A {
        match self.bits {
            false => RTCAIFG_A::RTCAIFG_0,
            true => RTCAIFG_A::RTCAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCAIFG_0`"]
    #[inline(always)]
    pub fn is_rtcaifg_0(&self) -> bool {
        *self == RTCAIFG_A::RTCAIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCAIFG_1`"]
    #[inline(always)]
    pub fn is_rtcaifg_1(&self) -> bool {
        *self == RTCAIFG_A::RTCAIFG_1
    }
}
#[doc = "Write proxy for field `RTCAIFG`"]
pub struct RTCAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_0(self) -> &'a mut W {
        self.variant(RTCAIFG_A::RTCAIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_1(self) -> &'a mut W {
        self.variant(RTCAIFG_A::RTCAIFG_1)
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
#[doc = "Real-time clock time event interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTEVIFG_A {
    #[doc = "0: No time event occurred"]
    RTCTEVIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RTCTEVIFG_1 = 1,
}
impl From<RTCTEVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTEVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCTEVIFG`"]
pub type RTCTEVIFG_R = crate::R<bool, RTCTEVIFG_A>;
impl RTCTEVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEVIFG_A {
        match self.bits {
            false => RTCTEVIFG_A::RTCTEVIFG_0,
            true => RTCTEVIFG_A::RTCTEVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEVIFG_0`"]
    #[inline(always)]
    pub fn is_rtctevifg_0(&self) -> bool {
        *self == RTCTEVIFG_A::RTCTEVIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCTEVIFG_1`"]
    #[inline(always)]
    pub fn is_rtctevifg_1(&self) -> bool {
        *self == RTCTEVIFG_A::RTCTEVIFG_1
    }
}
#[doc = "Write proxy for field `RTCTEVIFG`"]
pub struct RTCTEVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTEVIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_0(self) -> &'a mut W {
        self.variant(RTCTEVIFG_A::RTCTEVIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_1(self) -> &'a mut W {
        self.variant(RTCTEVIFG_A::RTCTEVIFG_1)
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
#[doc = "32-kHz crystal oscillator fault interrupt flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOFIFG_A {
    #[doc = "0: No interrupt pending"]
    RTCOFIFG_0 = 0,
    #[doc = "1: Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    RTCOFIFG_1 = 1,
}
impl From<RTCOFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOFIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCOFIFG`"]
pub type RTCOFIFG_R = crate::R<bool, RTCOFIFG_A>;
impl RTCOFIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOFIFG_A {
        match self.bits {
            false => RTCOFIFG_A::RTCOFIFG_0,
            true => RTCOFIFG_A::RTCOFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOFIFG_0`"]
    #[inline(always)]
    pub fn is_rtcofifg_0(&self) -> bool {
        *self == RTCOFIFG_A::RTCOFIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCOFIFG_1`"]
    #[inline(always)]
    pub fn is_rtcofifg_1(&self) -> bool {
        *self == RTCOFIFG_A::RTCOFIFG_1
    }
}
#[doc = "Write proxy for field `RTCOFIFG`"]
pub struct RTCOFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCOFIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn rtcofifg_0(self) -> &'a mut W {
        self.variant(RTCOFIFG_A::RTCOFIFG_0)
    }
    #[doc = "Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    #[inline(always)]
    pub fn rtcofifg_1(self) -> &'a mut W {
        self.variant(RTCOFIFG_A::RTCOFIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Real-time clock ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCRDYIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCRDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    RTCRDYIE_1 = 1,
}
impl From<RTCRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCRDYIE`"]
pub type RTCRDYIE_R = crate::R<bool, RTCRDYIE_A>;
impl RTCRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDYIE_A {
        match self.bits {
            false => RTCRDYIE_A::RTCRDYIE_0,
            true => RTCRDYIE_A::RTCRDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDYIE_0`"]
    #[inline(always)]
    pub fn is_rtcrdyie_0(&self) -> bool {
        *self == RTCRDYIE_A::RTCRDYIE_0
    }
    #[doc = "Checks if the value of the field is `RTCRDYIE_1`"]
    #[inline(always)]
    pub fn is_rtcrdyie_1(&self) -> bool {
        *self == RTCRDYIE_A::RTCRDYIE_1
    }
}
#[doc = "Write proxy for field `RTCRDYIE`"]
pub struct RTCRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcrdyie_0(self) -> &'a mut W {
        self.variant(RTCRDYIE_A::RTCRDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn rtcrdyie_1(self) -> &'a mut W {
        self.variant(RTCRDYIE_A::RTCRDYIE_1)
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
#[doc = "Real-time clock alarm interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCAIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCAIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCAIE_1 = 1,
}
impl From<RTCAIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCAIE`"]
pub type RTCAIE_R = crate::R<bool, RTCAIE_A>;
impl RTCAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAIE_A {
        match self.bits {
            false => RTCAIE_A::RTCAIE_0,
            true => RTCAIE_A::RTCAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCAIE_0`"]
    #[inline(always)]
    pub fn is_rtcaie_0(&self) -> bool {
        *self == RTCAIE_A::RTCAIE_0
    }
    #[doc = "Checks if the value of the field is `RTCAIE_1`"]
    #[inline(always)]
    pub fn is_rtcaie_1(&self) -> bool {
        *self == RTCAIE_A::RTCAIE_1
    }
}
#[doc = "Write proxy for field `RTCAIE`"]
pub struct RTCAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcaie_0(self) -> &'a mut W {
        self.variant(RTCAIE_A::RTCAIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcaie_1(self) -> &'a mut W {
        self.variant(RTCAIE_A::RTCAIE_1)
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
#[doc = "Real-time clock time event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTEVIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCTEVIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCTEVIE_1 = 1,
}
impl From<RTCTEVIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTEVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCTEVIE`"]
pub type RTCTEVIE_R = crate::R<bool, RTCTEVIE_A>;
impl RTCTEVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEVIE_A {
        match self.bits {
            false => RTCTEVIE_A::RTCTEVIE_0,
            true => RTCTEVIE_A::RTCTEVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEVIE_0`"]
    #[inline(always)]
    pub fn is_rtctevie_0(&self) -> bool {
        *self == RTCTEVIE_A::RTCTEVIE_0
    }
    #[doc = "Checks if the value of the field is `RTCTEVIE_1`"]
    #[inline(always)]
    pub fn is_rtctevie_1(&self) -> bool {
        *self == RTCTEVIE_A::RTCTEVIE_1
    }
}
#[doc = "Write proxy for field `RTCTEVIE`"]
pub struct RTCTEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTEVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtctevie_0(self) -> &'a mut W {
        self.variant(RTCTEVIE_A::RTCTEVIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtctevie_1(self) -> &'a mut W {
        self.variant(RTCTEVIE_A::RTCTEVIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "32-kHz crystal oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOFIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCOFIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCOFIE_1 = 1,
}
impl From<RTCOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCOFIE`"]
pub type RTCOFIE_R = crate::R<bool, RTCOFIE_A>;
impl RTCOFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOFIE_A {
        match self.bits {
            false => RTCOFIE_A::RTCOFIE_0,
            true => RTCOFIE_A::RTCOFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOFIE_0`"]
    #[inline(always)]
    pub fn is_rtcofie_0(&self) -> bool {
        *self == RTCOFIE_A::RTCOFIE_0
    }
    #[doc = "Checks if the value of the field is `RTCOFIE_1`"]
    #[inline(always)]
    pub fn is_rtcofie_1(&self) -> bool {
        *self == RTCOFIE_A::RTCOFIE_1
    }
}
#[doc = "Write proxy for field `RTCOFIE`"]
pub struct RTCOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCOFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcofie_0(self) -> &'a mut W {
        self.variant(RTCOFIE_A::RTCOFIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcofie_1(self) -> &'a mut W {
        self.variant(RTCOFIE_A::RTCOFIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTCKEY`"]
pub type RTCKEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCKEY`"]
pub struct RTCKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RTCOFIFG_R {
        RTCOFIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RTCOFIE_R {
        RTCOFIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&self) -> RTCKEY_R {
        RTCKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W {
        RTCRDYIFG_W { w: self }
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W {
        RTCAIFG_W { w: self }
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W {
        RTCTEVIFG_W { w: self }
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RTCOFIFG_W {
        RTCOFIFG_W { w: self }
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W {
        RTCRDYIE_W { w: self }
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RTCAIE_W {
        RTCAIE_W { w: self }
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W {
        RTCTEVIE_W { w: self }
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RTCOFIE_W {
        RTCOFIE_W { w: self }
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&mut self) -> RTCKEY_W {
        RTCKEY_W { w: self }
    }
}
