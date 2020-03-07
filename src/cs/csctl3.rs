#[doc = "Reader of register CSCTL3"]
pub type R = crate::R<u32, super::CSCTL3>;
#[doc = "Writer for register CSCTL3"]
pub type W = crate::W<u32, super::CSCTL3>;
#[doc = "Register CSCTL3 `reset()`'s with value 0x0bbb"]
impl crate::ResetValue for super::CSCTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0bbb
    }
}
#[doc = "Start flag counter for LFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCNTLF_A {
    #[doc = "0: 4096 cycles"]
    FCNTLF_0 = 0,
    #[doc = "1: 8192 cycles"]
    FCNTLF_1 = 1,
    #[doc = "2: 16384 cycles"]
    FCNTLF_2 = 2,
    #[doc = "3: 32768 cycles"]
    FCNTLF_3 = 3,
}
impl From<FCNTLF_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTLF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCNTLF`"]
pub type FCNTLF_R = crate::R<u8, FCNTLF_A>;
impl FCNTLF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLF_A {
        match self.bits {
            0 => FCNTLF_A::FCNTLF_0,
            1 => FCNTLF_A::FCNTLF_1,
            2 => FCNTLF_A::FCNTLF_2,
            3 => FCNTLF_A::FCNTLF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLF_0`"]
    #[inline(always)]
    pub fn is_fcntlf_0(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_0
    }
    #[doc = "Checks if the value of the field is `FCNTLF_1`"]
    #[inline(always)]
    pub fn is_fcntlf_1(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_1
    }
    #[doc = "Checks if the value of the field is `FCNTLF_2`"]
    #[inline(always)]
    pub fn is_fcntlf_2(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_2
    }
    #[doc = "Checks if the value of the field is `FCNTLF_3`"]
    #[inline(always)]
    pub fn is_fcntlf_3(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_3
    }
}
#[doc = "Write proxy for field `FCNTLF`"]
pub struct FCNTLF_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTLF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTLF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcntlf_0(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_0)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcntlf_1(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_1)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcntlf_2(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_2)
    }
    #[doc = "32768 cycles"]
    #[inline(always)]
    pub fn fcntlf_3(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reset start fault counter for LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCNTLF_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTLF_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTLF_1 = 1,
}
impl From<RFCNTLF_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTLF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RFCNTLF`"]
pub struct RFCNTLF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCNTLF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCNTLF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcntlf_0(self) -> &'a mut W {
        self.variant(RFCNTLF_AW::RFCNTLF_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcntlf_1(self) -> &'a mut W {
        self.variant(RFCNTLF_AW::RFCNTLF_1)
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
#[doc = "Enable start fault counter for LFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTLF_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTLF_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTLF_EN_1 = 1,
}
impl From<FCNTLF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLF_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTLF_EN`"]
pub type FCNTLF_EN_R = crate::R<bool, FCNTLF_EN_A>;
impl FCNTLF_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLF_EN_A {
        match self.bits {
            false => FCNTLF_EN_A::FCNTLF_EN_0,
            true => FCNTLF_EN_A::FCNTLF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLF_EN_0`"]
    #[inline(always)]
    pub fn is_fcntlf_en_0(&self) -> bool {
        *self == FCNTLF_EN_A::FCNTLF_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTLF_EN_1`"]
    #[inline(always)]
    pub fn is_fcntlf_en_1(&self) -> bool {
        *self == FCNTLF_EN_A::FCNTLF_EN_1
    }
}
#[doc = "Write proxy for field `FCNTLF_EN`"]
pub struct FCNTLF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTLF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTLF_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcntlf_en_0(self) -> &'a mut W {
        self.variant(FCNTLF_EN_A::FCNTLF_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcntlf_en_1(self) -> &'a mut W {
        self.variant(FCNTLF_EN_A::FCNTLF_EN_1)
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
#[doc = "Start flag counter for HFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCNTHF_A {
    #[doc = "0: 2048 cycles"]
    FCNTHF_0 = 0,
    #[doc = "1: 4096 cycles"]
    FCNTHF_1 = 1,
    #[doc = "2: 8192 cycles"]
    FCNTHF_2 = 2,
    #[doc = "3: 16384 cycles"]
    FCNTHF_3 = 3,
}
impl From<FCNTHF_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTHF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCNTHF`"]
pub type FCNTHF_R = crate::R<u8, FCNTHF_A>;
impl FCNTHF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF_A {
        match self.bits {
            0 => FCNTHF_A::FCNTHF_0,
            1 => FCNTHF_A::FCNTHF_1,
            2 => FCNTHF_A::FCNTHF_2,
            3 => FCNTHF_A::FCNTHF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF_0`"]
    #[inline(always)]
    pub fn is_fcnthf_0(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF_1`"]
    #[inline(always)]
    pub fn is_fcnthf_1(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_1
    }
    #[doc = "Checks if the value of the field is `FCNTHF_2`"]
    #[inline(always)]
    pub fn is_fcnthf_2(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_2
    }
    #[doc = "Checks if the value of the field is `FCNTHF_3`"]
    #[inline(always)]
    pub fn is_fcnthf_3(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_3
    }
}
#[doc = "Write proxy for field `FCNTHF`"]
pub struct FCNTHF_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf_0(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf_1(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf_2(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf_3(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reset start fault counter for HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCNTHF_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTHF_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTHF_1 = 1,
}
impl From<RFCNTHF_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTHF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RFCNTHF`"]
pub struct RFCNTHF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCNTHF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCNTHF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf_0(self) -> &'a mut W {
        self.variant(RFCNTHF_AW::RFCNTHF_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf_1(self) -> &'a mut W {
        self.variant(RFCNTHF_AW::RFCNTHF_1)
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
#[doc = "Enable start fault counter for HFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHF_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTHF_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTHF_EN_1 = 1,
}
impl From<FCNTHF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHF_EN`"]
pub type FCNTHF_EN_R = crate::R<bool, FCNTHF_EN_A>;
impl FCNTHF_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF_EN_A {
        match self.bits {
            false => FCNTHF_EN_A::FCNTHF_EN_0,
            true => FCNTHF_EN_A::FCNTHF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF_EN_0`"]
    #[inline(always)]
    pub fn is_fcnthf_en_0(&self) -> bool {
        *self == FCNTHF_EN_A::FCNTHF_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF_EN_1`"]
    #[inline(always)]
    pub fn is_fcnthf_en_1(&self) -> bool {
        *self == FCNTHF_EN_A::FCNTHF_EN_1
    }
}
#[doc = "Write proxy for field `FCNTHF_EN`"]
pub struct FCNTHF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHF_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf_en_0(self) -> &'a mut W {
        self.variant(FCNTHF_EN_A::FCNTHF_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf_en_1(self) -> &'a mut W {
        self.variant(FCNTHF_EN_A::FCNTHF_EN_1)
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
#[doc = "Start flag counter for HFXT2\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCNTHF2_A {
    #[doc = "0: 2048 cycles"]
    FCNTHF2_0 = 0,
    #[doc = "1: 4096 cycles"]
    FCNTHF2_1 = 1,
    #[doc = "2: 8192 cycles"]
    FCNTHF2_2 = 2,
    #[doc = "3: 16384 cycles"]
    FCNTHF2_3 = 3,
}
impl From<FCNTHF2_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTHF2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCNTHF2`"]
pub type FCNTHF2_R = crate::R<u8, FCNTHF2_A>;
impl FCNTHF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2_A {
        match self.bits {
            0 => FCNTHF2_A::FCNTHF2_0,
            1 => FCNTHF2_A::FCNTHF2_1,
            2 => FCNTHF2_A::FCNTHF2_2,
            3 => FCNTHF2_A::FCNTHF2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_0`"]
    #[inline(always)]
    pub fn is_fcnthf2_0(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_1`"]
    #[inline(always)]
    pub fn is_fcnthf2_1(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_1
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_2`"]
    #[inline(always)]
    pub fn is_fcnthf2_2(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_2
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_3`"]
    #[inline(always)]
    pub fn is_fcnthf2_3(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_3
    }
}
#[doc = "Write proxy for field `FCNTHF2`"]
pub struct FCNTHF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHF2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf2_0(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf2_1(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf2_2(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf2_3(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reset start fault counter for HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCNTHF2_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTHF2_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTHF2_1 = 1,
}
impl From<RFCNTHF2_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTHF2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RFCNTHF2`"]
pub struct RFCNTHF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCNTHF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCNTHF2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf2_0(self) -> &'a mut W {
        self.variant(RFCNTHF2_AW::RFCNTHF2_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf2_1(self) -> &'a mut W {
        self.variant(RFCNTHF2_AW::RFCNTHF2_1)
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
#[doc = "Enable start fault counter for HFXT2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHF2_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTHF2_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTHF2_EN_1 = 1,
}
impl From<FCNTHF2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHF2_EN`"]
pub type FCNTHF2_EN_R = crate::R<bool, FCNTHF2_EN_A>;
impl FCNTHF2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2_EN_A {
        match self.bits {
            false => FCNTHF2_EN_A::FCNTHF2_EN_0,
            true => FCNTHF2_EN_A::FCNTHF2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_EN_0`"]
    #[inline(always)]
    pub fn is_fcnthf2_en_0(&self) -> bool {
        *self == FCNTHF2_EN_A::FCNTHF2_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_EN_1`"]
    #[inline(always)]
    pub fn is_fcnthf2_en_1(&self) -> bool {
        *self == FCNTHF2_EN_A::FCNTHF2_EN_1
    }
}
#[doc = "Write proxy for field `FCNTHF2_EN`"]
pub struct FCNTHF2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNTHF2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNTHF2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf2_en_0(self) -> &'a mut W {
        self.variant(FCNTHF2_EN_A::FCNTHF2_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf2_en_1(self) -> &'a mut W {
        self.variant(FCNTHF2_EN_A::FCNTHF2_EN_1)
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
impl R {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&self) -> FCNTLF_R {
        FCNTLF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&self) -> FCNTLF_EN_R {
        FCNTLF_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&self) -> FCNTHF_R {
        FCNTHF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&self) -> FCNTHF_EN_R {
        FCNTHF_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&self) -> FCNTHF2_R {
        FCNTHF2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&self) -> FCNTHF2_EN_R {
        FCNTHF2_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&mut self) -> FCNTLF_W {
        FCNTLF_W { w: self }
    }
    #[doc = "Bit 2 - Reset start fault counter for LFXT"]
    #[inline(always)]
    pub fn rfcntlf(&mut self) -> RFCNTLF_W {
        RFCNTLF_W { w: self }
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&mut self) -> FCNTLF_EN_W {
        FCNTLF_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&mut self) -> FCNTHF_W {
        FCNTHF_W { w: self }
    }
    #[doc = "Bit 6 - Reset start fault counter for HFXT"]
    #[inline(always)]
    pub fn rfcnthf(&mut self) -> RFCNTHF_W {
        RFCNTHF_W { w: self }
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&mut self) -> FCNTHF_EN_W {
        FCNTHF_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&mut self) -> FCNTHF2_W {
        FCNTHF2_W { w: self }
    }
    #[doc = "Bit 10 - Reset start fault counter for HFXT2"]
    #[inline(always)]
    pub fn rfcnthf2(&mut self) -> RFCNTHF2_W {
        RFCNTHF2_W { w: self }
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&mut self) -> FCNTHF2_EN_W {
        FCNTHF2_EN_W { w: self }
    }
}
