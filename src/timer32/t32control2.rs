#[doc = "Reader of register T32CONTROL2"]
pub type R = crate::R<u32, super::T32CONTROL2>;
#[doc = "Writer for register T32CONTROL2"]
pub type W = crate::W<u32, super::T32CONTROL2>;
#[doc = "Register T32CONTROL2 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::T32CONTROL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Selects one-shot or wrapping counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOT_A {
    #[doc = "0: wrapping mode"]
    ONESHOT_0 = 0,
    #[doc = "1: one-shot mode"]
    ONESHOT_1 = 1,
}
impl From<ONESHOT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONESHOT`"]
pub type ONESHOT_R = crate::R<bool, ONESHOT_A>;
impl ONESHOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOT_A {
        match self.bits {
            false => ONESHOT_A::ONESHOT_0,
            true => ONESHOT_A::ONESHOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOT_0`"]
    #[inline(always)]
    pub fn is_oneshot_0(&self) -> bool {
        *self == ONESHOT_A::ONESHOT_0
    }
    #[doc = "Checks if the value of the field is `ONESHOT_1`"]
    #[inline(always)]
    pub fn is_oneshot_1(&self) -> bool {
        *self == ONESHOT_A::ONESHOT_1
    }
}
#[doc = "Write proxy for field `ONESHOT`"]
pub struct ONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "wrapping mode"]
    #[inline(always)]
    pub fn oneshot_0(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_0)
    }
    #[doc = "one-shot mode"]
    #[inline(always)]
    pub fn oneshot_1(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_1)
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
#[doc = "Selects 16 or 32 bit counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: 16-bit counter"]
    SIZE_0 = 0,
    #[doc = "1: 32-bit counter"]
    SIZE_1 = 1,
}
impl From<SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<bool, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            false => SIZE_A::SIZE_0,
            true => SIZE_A::SIZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_0`"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        *self == SIZE_A::SIZE_0
    }
    #[doc = "Checks if the value of the field is `SIZE_1`"]
    #[inline(always)]
    pub fn is_size_1(&self) -> bool {
        *self == SIZE_A::SIZE_1
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16-bit counter"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_0)
    }
    #[doc = "32-bit counter"]
    #[inline(always)]
    pub fn size_1(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_1)
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
#[doc = "Prescale bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: 0 stages of prescale, clock is divided by 1"]
    PRESCALE_0 = 0,
    #[doc = "1: 4 stages of prescale, clock is divided by 16"]
    PRESCALE_1 = 1,
    #[doc = "2: 8 stages of prescale, clock is divided by 256"]
    PRESCALE_2 = 2,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALE_A::PRESCALE_0),
            1 => Val(PRESCALE_A::PRESCALE_1),
            2 => Val(PRESCALE_A::PRESCALE_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_2
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 stages of prescale, clock is divided by 1"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "4 stages of prescale, clock is divided by 16"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "8 stages of prescale, clock is divided by 256"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Interrupt enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Timer interrupt disabled"]
    IE_0 = 0,
    #[doc = "1: Timer interrupt enabled"]
    IE_1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, IE_A>;
impl IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::IE_0,
            true => IE_A::IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IE_0`"]
    #[inline(always)]
    pub fn is_ie_0(&self) -> bool {
        *self == IE_A::IE_0
    }
    #[doc = "Checks if the value of the field is `IE_1`"]
    #[inline(always)]
    pub fn is_ie_1(&self) -> bool {
        *self == IE_A::IE_1
    }
}
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer interrupt disabled"]
    #[inline(always)]
    pub fn ie_0(self) -> &'a mut W {
        self.variant(IE_A::IE_0)
    }
    #[doc = "Timer interrupt enabled"]
    #[inline(always)]
    pub fn ie_1(self) -> &'a mut W {
        self.variant(IE_A::IE_1)
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
#[doc = "Mode bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Timer is in free-running mode"]
    MODE_0 = 0,
    #[doc = "1: Timer is in periodic mode"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer is in free-running mode"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Timer is in periodic mode"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
#[doc = "Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Timer disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Timer enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLE_A::ENABLE_1
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Timer enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
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
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W { w: self }
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
