#[doc = "Reader of register SYS_WDTRESET_CTL"]
pub type R = crate::R<u32, super::SYS_WDTRESET_CTL>;
#[doc = "Writer for register SYS_WDTRESET_CTL"]
pub type W = crate::W<u32, super::SYS_WDTRESET_CTL>;
#[doc = "Register SYS_WDTRESET_CTL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::SYS_WDTRESET_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "WDT timeout reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: WDT timeout event generates Soft reset"]
    TIMEOUT_0 = 0,
    #[doc = "1: WDT timeout event generates Hard reset"]
    TIMEOUT_1 = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::TIMEOUT_0,
            true => TIMEOUT_A::TIMEOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_0`"]
    #[inline(always)]
    pub fn is_timeout_0(&self) -> bool {
        *self == TIMEOUT_A::TIMEOUT_0
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_1`"]
    #[inline(always)]
    pub fn is_timeout_1(&self) -> bool {
        *self == TIMEOUT_A::TIMEOUT_1
    }
}
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WDT timeout event generates Soft reset"]
    #[inline(always)]
    pub fn timeout_0(self) -> &'a mut W {
        self.variant(TIMEOUT_A::TIMEOUT_0)
    }
    #[doc = "WDT timeout event generates Hard reset"]
    #[inline(always)]
    pub fn timeout_1(self) -> &'a mut W {
        self.variant(TIMEOUT_A::TIMEOUT_1)
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
#[doc = "WDT password violation reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIOLATION_A {
    #[doc = "0: WDT password violation event generates Soft reset"]
    VIOLATION_0 = 0,
    #[doc = "1: WDT password violation event generates Hard reset"]
    VIOLATION_1 = 1,
}
impl From<VIOLATION_A> for bool {
    #[inline(always)]
    fn from(variant: VIOLATION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIOLATION`"]
pub type VIOLATION_R = crate::R<bool, VIOLATION_A>;
impl VIOLATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIOLATION_A {
        match self.bits {
            false => VIOLATION_A::VIOLATION_0,
            true => VIOLATION_A::VIOLATION_1,
        }
    }
    #[doc = "Checks if the value of the field is `VIOLATION_0`"]
    #[inline(always)]
    pub fn is_violation_0(&self) -> bool {
        *self == VIOLATION_A::VIOLATION_0
    }
    #[doc = "Checks if the value of the field is `VIOLATION_1`"]
    #[inline(always)]
    pub fn is_violation_1(&self) -> bool {
        *self == VIOLATION_A::VIOLATION_1
    }
}
#[doc = "Write proxy for field `VIOLATION`"]
pub struct VIOLATION_W<'a> {
    w: &'a mut W,
}
impl<'a> VIOLATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIOLATION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WDT password violation event generates Soft reset"]
    #[inline(always)]
    pub fn violation_0(self) -> &'a mut W {
        self.variant(VIOLATION_A::VIOLATION_0)
    }
    #[doc = "WDT password violation event generates Hard reset"]
    #[inline(always)]
    pub fn violation_1(self) -> &'a mut W {
        self.variant(VIOLATION_A::VIOLATION_1)
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
impl R {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&self) -> VIOLATION_R {
        VIOLATION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&mut self) -> VIOLATION_W {
        VIOLATION_W { w: self }
    }
}
