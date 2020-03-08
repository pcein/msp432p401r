#[doc = "Reader of register RTCTCMP"]
pub type R = crate::R<u16, super::RTCTCMP>;
#[doc = "Writer for register RTCTCMP"]
pub type W = crate::W<u16, super::RTCTCMP>;
#[doc = "Register RTCTCMP `reset()`'s with value 0x4000"]
impl crate::ResetValue for super::RTCTCMP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000
    }
}
#[doc = "Reader of field `RTCTCMP`"]
pub type RTCTCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCTCMP`"]
pub struct RTCTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Real-time clock temperature compensation write OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTCOK_A {
    #[doc = "0: Write to RTCTCMPx is unsuccessful"]
    RTCTCOK_0 = 0,
    #[doc = "1: Write to RTCTCMPx is successful"]
    RTCTCOK_1 = 1,
}
impl From<RTCTCOK_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTCOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCTCOK`"]
pub type RTCTCOK_R = crate::R<bool, RTCTCOK_A>;
impl RTCTCOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTCOK_A {
        match self.bits {
            false => RTCTCOK_A::RTCTCOK_0,
            true => RTCTCOK_A::RTCTCOK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTCOK_0`"]
    #[inline(always)]
    pub fn is_rtctcok_0(&self) -> bool {
        *self == RTCTCOK_A::RTCTCOK_0
    }
    #[doc = "Checks if the value of the field is `RTCTCOK_1`"]
    #[inline(always)]
    pub fn is_rtctcok_1(&self) -> bool {
        *self == RTCTCOK_A::RTCTCOK_1
    }
}
#[doc = "Reader of field `RTCTCRDY`"]
pub type RTCTCRDY_R = crate::R<bool, bool>;
#[doc = "Real-time clock temperature compensation sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTCMPS_A {
    #[doc = "0: Down calibration. Frequency adjusted down"]
    RTCTCMPS_0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up"]
    RTCTCMPS_1 = 1,
}
impl From<RTCTCMPS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTCMPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCTCMPS`"]
pub type RTCTCMPS_R = crate::R<bool, RTCTCMPS_A>;
impl RTCTCMPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTCMPS_A {
        match self.bits {
            false => RTCTCMPS_A::RTCTCMPS_0,
            true => RTCTCMPS_A::RTCTCMPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTCMPS_0`"]
    #[inline(always)]
    pub fn is_rtctcmps_0(&self) -> bool {
        *self == RTCTCMPS_A::RTCTCMPS_0
    }
    #[doc = "Checks if the value of the field is `RTCTCMPS_1`"]
    #[inline(always)]
    pub fn is_rtctcmps_1(&self) -> bool {
        *self == RTCTCMPS_A::RTCTCMPS_1
    }
}
#[doc = "Write proxy for field `RTCTCMPS`"]
pub struct RTCTCMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTCMPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Down calibration. Frequency adjusted down"]
    #[inline(always)]
    pub fn rtctcmps_0(self) -> &'a mut W {
        self.variant(RTCTCMPS_A::RTCTCMPS_0)
    }
    #[doc = "Up calibration. Frequency adjusted up"]
    #[inline(always)]
    pub fn rtctcmps_1(self) -> &'a mut W {
        self.variant(RTCTCMPS_A::RTCTCMPS_1)
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
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&self) -> RTCTCMP_R {
        RTCTCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - Real-time clock temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RTCTCOK_R {
        RTCTCOK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Real-time clock temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RTCTCRDY_R {
        RTCTCRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RTCTCMPS_R {
        RTCTCMPS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&mut self) -> RTCTCMP_W {
        RTCTCMP_W { w: self }
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RTCTCMPS_W {
        RTCTCMPS_W { w: self }
    }
}
