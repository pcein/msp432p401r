#[doc = "Reader of register RTCPS"]
pub type R = crate::R<u16, super::RTCPS>;
#[doc = "Writer for register RTCPS"]
pub type W = crate::W<u16, super::RTCPS>;
#[doc = "Register RTCPS `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT0PS`"]
pub type RT0PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RT0PS`"]
pub struct RT0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RT1PS`"]
pub type RT1PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RT1PS`"]
pub struct RT1PS_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&self) -> RT0PS_R {
        RT0PS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&self) -> RT1PS_R {
        RT1PS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&mut self) -> RT0PS_W {
        RT0PS_W { w: self }
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&mut self) -> RT1PS_W {
        RT1PS_W { w: self }
    }
}
