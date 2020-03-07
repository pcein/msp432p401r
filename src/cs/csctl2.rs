#[doc = "Reader of register CSCTL2"]
pub type R = crate::R<u32, super::CSCTL2>;
#[doc = "Writer for register CSCTL2"]
pub type W = crate::W<u32, super::CSCTL2>;
#[doc = "Register CSCTL2 `reset()`'s with value 0x0001_0003"]
impl crate::ResetValue for super::CSCTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0003
    }
}
#[doc = "LFXT oscillator current can be adjusted to its drive needs\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFXTDRIVE_A {
    #[doc = "0: Lowest drive strength and current consumption LFXT oscillator."]
    LFXTDRIVE_0 = 0,
    #[doc = "1: Increased drive strength LFXT oscillator."]
    LFXTDRIVE_1 = 1,
    #[doc = "2: Increased drive strength LFXT oscillator."]
    LFXTDRIVE_2 = 2,
    #[doc = "3: Maximum drive strength and maximum current consumption LFXT oscillator."]
    LFXTDRIVE_3 = 3,
}
impl From<LFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFXTDRIVE`"]
pub type LFXTDRIVE_R = crate::R<u8, LFXTDRIVE_A>;
impl LFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTDRIVE_A {
        match self.bits {
            0 => LFXTDRIVE_A::LFXTDRIVE_0,
            1 => LFXTDRIVE_A::LFXTDRIVE_1,
            2 => LFXTDRIVE_A::LFXTDRIVE_2,
            3 => LFXTDRIVE_A::LFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_3
    }
}
#[doc = "Write proxy for field `LFXTDRIVE`"]
pub struct LFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTDRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lowest drive strength and current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_0)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_1)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_2)
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Disables the automatic gain control of the LFXT crystal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTAGCOFF_A {
    #[doc = "0: AGC enabled."]
    LFXTAGCOFF_0 = 0,
    #[doc = "1: AGC disabled."]
    LFXTAGCOFF_1 = 1,
}
impl From<LFXTAGCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTAGCOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTAGCOFF`"]
pub type LFXTAGCOFF_R = crate::R<bool, LFXTAGCOFF_A>;
impl LFXTAGCOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTAGCOFF_A {
        match self.bits {
            false => LFXTAGCOFF_A::LFXTAGCOFF_0,
            true => LFXTAGCOFF_A::LFXTAGCOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTAGCOFF_0`"]
    #[inline(always)]
    pub fn is_lfxtagcoff_0(&self) -> bool {
        *self == LFXTAGCOFF_A::LFXTAGCOFF_0
    }
    #[doc = "Checks if the value of the field is `LFXTAGCOFF_1`"]
    #[inline(always)]
    pub fn is_lfxtagcoff_1(&self) -> bool {
        *self == LFXTAGCOFF_A::LFXTAGCOFF_1
    }
}
#[doc = "Write proxy for field `LFXTAGCOFF`"]
pub struct LFXTAGCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTAGCOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTAGCOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AGC enabled."]
    #[inline(always)]
    pub fn lfxtagcoff_0(self) -> &'a mut W {
        self.variant(LFXTAGCOFF_A::LFXTAGCOFF_0)
    }
    #[doc = "AGC disabled."]
    #[inline(always)]
    pub fn lfxtagcoff_1(self) -> &'a mut W {
        self.variant(LFXTAGCOFF_A::LFXTAGCOFF_1)
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
#[doc = "Turns on the LFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXT_EN_A {
    #[doc = "0: LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK\r\r\nand is selected via the port selection and not in bypass mode of operation."]
    LFXT_EN_0 = 0,
    #[doc = "1: LFXT is on if LFXT is selected via the port selection and LFXT is not in\r\r\nbypass mode of operation."]
    LFXT_EN_1 = 1,
}
impl From<LFXT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LFXT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXT_EN`"]
pub type LFXT_EN_R = crate::R<bool, LFXT_EN_A>;
impl LFXT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXT_EN_A {
        match self.bits {
            false => LFXT_EN_A::LFXT_EN_0,
            true => LFXT_EN_A::LFXT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXT_EN_0`"]
    #[inline(always)]
    pub fn is_lfxt_en_0(&self) -> bool {
        *self == LFXT_EN_A::LFXT_EN_0
    }
    #[doc = "Checks if the value of the field is `LFXT_EN_1`"]
    #[inline(always)]
    pub fn is_lfxt_en_1(&self) -> bool {
        *self == LFXT_EN_A::LFXT_EN_1
    }
}
#[doc = "Write proxy for field `LFXT_EN`"]
pub struct LFXT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_0(self) -> &'a mut W {
        self.variant(LFXT_EN_A::LFXT_EN_0)
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_1(self) -> &'a mut W {
        self.variant(LFXT_EN_A::LFXT_EN_1)
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
#[doc = "LFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTBYPASS_A {
    #[doc = "0: LFXT sourced by external crystal."]
    LFXTBYPASS_0 = 0,
    #[doc = "1: LFXT sourced by external square wave."]
    LFXTBYPASS_1 = 1,
}
impl From<LFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTBYPASS`"]
pub type LFXTBYPASS_R = crate::R<bool, LFXTBYPASS_A>;
impl LFXTBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTBYPASS_A {
        match self.bits {
            false => LFXTBYPASS_A::LFXTBYPASS_0,
            true => LFXTBYPASS_A::LFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_lfxtbypass_0(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_lfxtbypass_1(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_1
    }
}
#[doc = "Write proxy for field `LFXTBYPASS`"]
pub struct LFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LFXT sourced by external crystal."]
    #[inline(always)]
    pub fn lfxtbypass_0(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_0)
    }
    #[doc = "LFXT sourced by external square wave."]
    #[inline(always)]
    pub fn lfxtbypass_1(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_1)
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
#[doc = "HFXT oscillator drive selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTDRIVE_A {
    #[doc = "0: To be used for HFXTFREQ setting 000b"]
    HFXTDRIVE_0 = 0,
    #[doc = "1: To be used for HFXTFREQ settings 001b to 110b"]
    HFXTDRIVE_1 = 1,
}
impl From<HFXTDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTDRIVE`"]
pub type HFXTDRIVE_R = crate::R<bool, HFXTDRIVE_A>;
impl HFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTDRIVE_A {
        match self.bits {
            false => HFXTDRIVE_A::HFXTDRIVE_0,
            true => HFXTDRIVE_A::HFXTDRIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_1
    }
}
#[doc = "Write proxy for field `HFXTDRIVE`"]
pub struct HFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTDRIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "To be used for HFXTFREQ setting 000b"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_0)
    }
    #[doc = "To be used for HFXTFREQ settings 001b to 110b"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_1)
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
#[doc = "HFXT frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXTFREQ_A {
    #[doc = "0: 1 MHz to 4 MHz"]
    HFXTFREQ_0 = 0,
    #[doc = "1: >4 MHz to 8 MHz"]
    HFXTFREQ_1 = 1,
    #[doc = "2: >8 MHz to 16 MHz"]
    HFXTFREQ_2 = 2,
    #[doc = "3: >16 MHz to 24 MHz"]
    HFXTFREQ_3 = 3,
    #[doc = "4: >24 MHz to 32 MHz"]
    HFXTFREQ_4 = 4,
    #[doc = "5: >32 MHz to 40 MHz"]
    HFXTFREQ_5 = 5,
    #[doc = "6: >40 MHz to 48 MHz"]
    HFXTFREQ_6 = 6,
}
impl From<HFXTFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXTFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFXTFREQ`"]
pub type HFXTFREQ_R = crate::R<u8, HFXTFREQ_A>;
impl HFXTFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFXTFREQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HFXTFREQ_A::HFXTFREQ_0),
            1 => Val(HFXTFREQ_A::HFXTFREQ_1),
            2 => Val(HFXTFREQ_A::HFXTFREQ_2),
            3 => Val(HFXTFREQ_A::HFXTFREQ_3),
            4 => Val(HFXTFREQ_A::HFXTFREQ_4),
            5 => Val(HFXTFREQ_A::HFXTFREQ_5),
            6 => Val(HFXTFREQ_A::HFXTFREQ_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_0`"]
    #[inline(always)]
    pub fn is_hfxtfreq_0(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_0
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_1`"]
    #[inline(always)]
    pub fn is_hfxtfreq_1(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_1
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_2`"]
    #[inline(always)]
    pub fn is_hfxtfreq_2(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_2
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_3`"]
    #[inline(always)]
    pub fn is_hfxtfreq_3(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_3
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_4`"]
    #[inline(always)]
    pub fn is_hfxtfreq_4(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_4
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_5`"]
    #[inline(always)]
    pub fn is_hfxtfreq_5(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_5
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_6`"]
    #[inline(always)]
    pub fn is_hfxtfreq_6(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_6
    }
}
#[doc = "Write proxy for field `HFXTFREQ`"]
pub struct HFXTFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTFREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 MHz to 4 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_0(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_0)
    }
    #[doc = ">4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_1(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_1)
    }
    #[doc = ">8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_2(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_2)
    }
    #[doc = ">16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_3(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_3)
    }
    #[doc = ">24 MHz to 32 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_4(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_4)
    }
    #[doc = ">32 MHz to 40 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_5(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_5)
    }
    #[doc = ">40 MHz to 48 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_6(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Turns on the HFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXT_EN_A {
    #[doc = "0: HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    HFXT_EN_0 = 0,
    #[doc = "1: HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    HFXT_EN_1 = 1,
}
impl From<HFXT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXT_EN`"]
pub type HFXT_EN_R = crate::R<bool, HFXT_EN_A>;
impl HFXT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT_EN_A {
        match self.bits {
            false => HFXT_EN_A::HFXT_EN_0,
            true => HFXT_EN_A::HFXT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT_EN_0`"]
    #[inline(always)]
    pub fn is_hfxt_en_0(&self) -> bool {
        *self == HFXT_EN_A::HFXT_EN_0
    }
    #[doc = "Checks if the value of the field is `HFXT_EN_1`"]
    #[inline(always)]
    pub fn is_hfxt_en_1(&self) -> bool {
        *self == HFXT_EN_A::HFXT_EN_1
    }
}
#[doc = "Write proxy for field `HFXT_EN`"]
pub struct HFXT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_0(self) -> &'a mut W {
        self.variant(HFXT_EN_A::HFXT_EN_0)
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_1(self) -> &'a mut W {
        self.variant(HFXT_EN_A::HFXT_EN_1)
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
#[doc = "HFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTBYPASS_A {
    #[doc = "0: HFXT sourced by external crystal."]
    HFXTBYPASS_0 = 0,
    #[doc = "1: HFXT sourced by external square wave."]
    HFXTBYPASS_1 = 1,
}
impl From<HFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTBYPASS`"]
pub type HFXTBYPASS_R = crate::R<bool, HFXTBYPASS_A>;
impl HFXTBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTBYPASS_A {
        match self.bits {
            false => HFXTBYPASS_A::HFXTBYPASS_0,
            true => HFXTBYPASS_A::HFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_hfxtbypass_0(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_hfxtbypass_1(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_1
    }
}
#[doc = "Write proxy for field `HFXTBYPASS`"]
pub struct HFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFXT sourced by external crystal."]
    #[inline(always)]
    pub fn hfxtbypass_0(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_0)
    }
    #[doc = "HFXT sourced by external square wave."]
    #[inline(always)]
    pub fn hfxtbypass_1(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_1)
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
impl R {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LFXTDRIVE_R {
        LFXTDRIVE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&self) -> LFXTAGCOFF_R {
        LFXTAGCOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&self) -> LFXT_EN_R {
        LFXT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LFXTBYPASS_R {
        LFXTBYPASS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HFXTDRIVE_R {
        HFXTDRIVE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&self) -> HFXTFREQ_R {
        HFXTFREQ_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&self) -> HFXT_EN_R {
        HFXT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HFXTBYPASS_R {
        HFXTBYPASS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LFXTDRIVE_W {
        LFXTDRIVE_W { w: self }
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&mut self) -> LFXTAGCOFF_W {
        LFXTAGCOFF_W { w: self }
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&mut self) -> LFXT_EN_W {
        LFXT_EN_W { w: self }
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LFXTBYPASS_W {
        LFXTBYPASS_W { w: self }
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HFXTDRIVE_W {
        HFXTDRIVE_W { w: self }
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&mut self) -> HFXTFREQ_W {
        HFXTFREQ_W { w: self }
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&mut self) -> HFXT_EN_W {
        HFXT_EN_W { w: self }
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HFXTBYPASS_W {
        HFXTBYPASS_W { w: self }
    }
}
