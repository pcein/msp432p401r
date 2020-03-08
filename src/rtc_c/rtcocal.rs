#[doc = "Reader of register RTCOCAL"]
pub type R = crate::R<u16, super::RTCOCAL>;
#[doc = "Writer for register RTCOCAL"]
pub type W = crate::W<u16, super::RTCOCAL>;
#[doc = "Register RTCOCAL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCOCAL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCOCAL`"]
pub type RTCOCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCOCAL`"]
pub struct RTCOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Real-time clock offset error calibration sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOCALS_A {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    RTCOCALS_0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    RTCOCALS_1 = 1,
}
impl From<RTCOCALS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOCALS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCOCALS`"]
pub type RTCOCALS_R = crate::R<bool, RTCOCALS_A>;
impl RTCOCALS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOCALS_A {
        match self.bits {
            false => RTCOCALS_A::RTCOCALS_0,
            true => RTCOCALS_A::RTCOCALS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_0`"]
    #[inline(always)]
    pub fn is_rtcocals_0(&self) -> bool {
        *self == RTCOCALS_A::RTCOCALS_0
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_1`"]
    #[inline(always)]
    pub fn is_rtcocals_1(&self) -> bool {
        *self == RTCOCALS_A::RTCOCALS_1
    }
}
#[doc = "Write proxy for field `RTCOCALS`"]
pub struct RTCOCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCALS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCOCALS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn rtcocals_0(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_0)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn rtcocals_1(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&self) -> RTCOCAL_R {
        RTCOCAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&mut self) -> RTCOCAL_W {
        RTCOCAL_W { w: self }
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W {
        RTCOCALS_W { w: self }
    }
}
