#[doc = "Reader of register PSSIE"]
pub type R = crate::R<u32, super::PSSIE>;
#[doc = "Writer for register PSSIE"]
pub type W = crate::W<u32, super::PSSIE>;
#[doc = "Register PSSIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High-side SVSM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHIE_A {
    #[doc = "0: Interrupt disabled"]
    SVSMHIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    SVSMHIE_1 = 1,
}
impl From<SVSMHIE_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVSMHIE`"]
pub type SVSMHIE_R = crate::R<bool, SVSMHIE_A>;
impl SVSMHIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHIE_A {
        match self.bits {
            false => SVSMHIE_A::SVSMHIE_0,
            true => SVSMHIE_A::SVSMHIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHIE_0`"]
    #[inline(always)]
    pub fn is_svsmhie_0(&self) -> bool {
        *self == SVSMHIE_A::SVSMHIE_0
    }
    #[doc = "Checks if the value of the field is `SVSMHIE_1`"]
    #[inline(always)]
    pub fn is_svsmhie_1(&self) -> bool {
        *self == SVSMHIE_A::SVSMHIE_1
    }
}
#[doc = "Write proxy for field `SVSMHIE`"]
pub struct SVSMHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMHIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn svsmhie_0(self) -> &'a mut W {
        self.variant(SVSMHIE_A::SVSMHIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn svsmhie_1(self) -> &'a mut W {
        self.variant(SVSMHIE_A::SVSMHIE_1)
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
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&self) -> SVSMHIE_R {
        SVSMHIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&mut self) -> SVSMHIE_W {
        SVSMHIE_W { w: self }
    }
}
