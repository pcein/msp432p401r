#[doc = "Reader of register UCBxSTATW"]
pub type R = crate::R<u16, super::UCBXSTATW>;
#[doc = "Writer for register UCBxSTATW"]
pub type W = crate::W<u16, super::UCBXSTATW>;
#[doc = "Register UCBxSTATW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCBXSTATW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBBUSY_A {
    #[doc = "0: Bus inactive"]
    UCBBUSY_0 = 0,
    #[doc = "1: Bus busy"]
    UCBBUSY_1 = 1,
}
impl From<UCBBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: UCBBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCBBUSY`"]
pub type UCBBUSY_R = crate::R<bool, UCBBUSY_A>;
impl UCBBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBBUSY_A {
        match self.bits {
            false => UCBBUSY_A::UCBBUSY_0,
            true => UCBBUSY_A::UCBBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBBUSY_0`"]
    #[inline(always)]
    pub fn is_ucbbusy_0(&self) -> bool {
        *self == UCBBUSY_A::UCBBUSY_0
    }
    #[doc = "Checks if the value of the field is `UCBBUSY_1`"]
    #[inline(always)]
    pub fn is_ucbbusy_1(&self) -> bool {
        *self == UCBBUSY_A::UCBBUSY_1
    }
}
#[doc = "General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCGC_A {
    #[doc = "0: No general call address received"]
    UCGC_0 = 0,
    #[doc = "1: General call address received"]
    UCGC_1 = 1,
}
impl From<UCGC_A> for bool {
    #[inline(always)]
    fn from(variant: UCGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCGC`"]
pub type UCGC_R = crate::R<bool, UCGC_A>;
impl UCGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGC_A {
        match self.bits {
            false => UCGC_A::UCGC_0,
            true => UCGC_A::UCGC_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCGC_0`"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == UCGC_A::UCGC_0
    }
    #[doc = "Checks if the value of the field is `UCGC_1`"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == UCGC_A::UCGC_1
    }
}
#[doc = "SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSCLLOW_A {
    #[doc = "0: SCL is not held low"]
    UCSCLLOW_0 = 0,
    #[doc = "1: SCL is held low"]
    UCSCLLOW_1 = 1,
}
impl From<UCSCLLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UCSCLLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCSCLLOW`"]
pub type UCSCLLOW_R = crate::R<bool, UCSCLLOW_A>;
impl UCSCLLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSCLLOW_A {
        match self.bits {
            false => UCSCLLOW_A::UCSCLLOW_0,
            true => UCSCLLOW_A::UCSCLLOW_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_0`"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == UCSCLLOW_A::UCSCLLOW_0
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_1`"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == UCSCLLOW_A::UCSCLLOW_1
    }
}
#[doc = "Reader of field `UCBCNT`"]
pub type UCBCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 4 - Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UCBCNT_R {
        UCBCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
