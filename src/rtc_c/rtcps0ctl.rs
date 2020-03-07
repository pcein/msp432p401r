#[doc = "Reader of register RTCPS0CTL"]
pub type R = crate::R<u16, super::RTCPS0CTL>;
#[doc = "Writer for register RTCPS0CTL"]
pub type W = crate::W<u16, super::RTCPS0CTL>;
#[doc = "Register RTCPS0CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS0CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale timer 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0PSIFG_A {
    #[doc = "0: No time event occurred"]
    RT0PSIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RT0PSIFG_1 = 1,
}
impl From<RT0PSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RT0PSIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT0PSIFG`"]
pub type RT0PSIFG_R = crate::R<bool, RT0PSIFG_A>;
impl RT0PSIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSIFG_A {
        match self.bits {
            false => RT0PSIFG_A::RT0PSIFG_0,
            true => RT0PSIFG_A::RT0PSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSIFG_0`"]
    #[inline(always)]
    pub fn is_rt0psifg_0(&self) -> bool {
        *self == RT0PSIFG_A::RT0PSIFG_0
    }
    #[doc = "Checks if the value of the field is `RT0PSIFG_1`"]
    #[inline(always)]
    pub fn is_rt0psifg_1(&self) -> bool {
        *self == RT0PSIFG_A::RT0PSIFG_1
    }
}
#[doc = "Write proxy for field `RT0PSIFG`"]
pub struct RT0PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_0(self) -> &'a mut W {
        self.variant(RT0PSIFG_A::RT0PSIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_1(self) -> &'a mut W {
        self.variant(RT0PSIFG_A::RT0PSIFG_1)
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
#[doc = "Prescale timer 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0PSIE_A {
    #[doc = "0: Interrupt not enabled"]
    RT0PSIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RT0PSIE_1 = 1,
}
impl From<RT0PSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RT0PSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT0PSIE`"]
pub type RT0PSIE_R = crate::R<bool, RT0PSIE_A>;
impl RT0PSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSIE_A {
        match self.bits {
            false => RT0PSIE_A::RT0PSIE_0,
            true => RT0PSIE_A::RT0PSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSIE_0`"]
    #[inline(always)]
    pub fn is_rt0psie_0(&self) -> bool {
        *self == RT0PSIE_A::RT0PSIE_0
    }
    #[doc = "Checks if the value of the field is `RT0PSIE_1`"]
    #[inline(always)]
    pub fn is_rt0psie_1(&self) -> bool {
        *self == RT0PSIE_A::RT0PSIE_1
    }
}
#[doc = "Write proxy for field `RT0PSIE`"]
pub struct RT0PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rt0psie_0(self) -> &'a mut W {
        self.variant(RT0PSIE_A::RT0PSIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rt0psie_1(self) -> &'a mut W {
        self.variant(RT0PSIE_A::RT0PSIE_1)
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
#[doc = "Prescale timer 0 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0IP_A {
    #[doc = "0: Divide by 2"]
    RT0IP_0 = 0,
    #[doc = "1: Divide by 4"]
    RT0IP_1 = 1,
    #[doc = "2: Divide by 8"]
    RT0IP_2 = 2,
    #[doc = "3: Divide by 16"]
    RT0IP_3 = 3,
    #[doc = "4: Divide by 32"]
    RT0IP_4 = 4,
    #[doc = "5: Divide by 64"]
    RT0IP_5 = 5,
    #[doc = "6: Divide by 128"]
    RT0IP_6 = 6,
    #[doc = "7: Divide by 256"]
    RT0IP_7 = 7,
}
impl From<RT0IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0IP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT0IP`"]
pub type RT0IP_R = crate::R<u8, RT0IP_A>;
impl RT0IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0IP_A {
        match self.bits {
            0 => RT0IP_A::RT0IP_0,
            1 => RT0IP_A::RT0IP_1,
            2 => RT0IP_A::RT0IP_2,
            3 => RT0IP_A::RT0IP_3,
            4 => RT0IP_A::RT0IP_4,
            5 => RT0IP_A::RT0IP_5,
            6 => RT0IP_A::RT0IP_6,
            7 => RT0IP_A::RT0IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT0IP_0`"]
    #[inline(always)]
    pub fn is_rt0ip_0(&self) -> bool {
        *self == RT0IP_A::RT0IP_0
    }
    #[doc = "Checks if the value of the field is `RT0IP_1`"]
    #[inline(always)]
    pub fn is_rt0ip_1(&self) -> bool {
        *self == RT0IP_A::RT0IP_1
    }
    #[doc = "Checks if the value of the field is `RT0IP_2`"]
    #[inline(always)]
    pub fn is_rt0ip_2(&self) -> bool {
        *self == RT0IP_A::RT0IP_2
    }
    #[doc = "Checks if the value of the field is `RT0IP_3`"]
    #[inline(always)]
    pub fn is_rt0ip_3(&self) -> bool {
        *self == RT0IP_A::RT0IP_3
    }
    #[doc = "Checks if the value of the field is `RT0IP_4`"]
    #[inline(always)]
    pub fn is_rt0ip_4(&self) -> bool {
        *self == RT0IP_A::RT0IP_4
    }
    #[doc = "Checks if the value of the field is `RT0IP_5`"]
    #[inline(always)]
    pub fn is_rt0ip_5(&self) -> bool {
        *self == RT0IP_A::RT0IP_5
    }
    #[doc = "Checks if the value of the field is `RT0IP_6`"]
    #[inline(always)]
    pub fn is_rt0ip_6(&self) -> bool {
        *self == RT0IP_A::RT0IP_6
    }
    #[doc = "Checks if the value of the field is `RT0IP_7`"]
    #[inline(always)]
    pub fn is_rt0ip_7(&self) -> bool {
        *self == RT0IP_A::RT0IP_7
    }
}
#[doc = "Write proxy for field `RT0IP`"]
pub struct RT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0IP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn rt0ip_0(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_0)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn rt0ip_1(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_1)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn rt0ip_2(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_2)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn rt0ip_3(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_3)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn rt0ip_4(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_4)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn rt0ip_5(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_5)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn rt0ip_6(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_6)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn rt0ip_7(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> RT0PSIFG_R {
        RT0PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&self) -> RT0PSIE_R {
        RT0PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&self) -> RT0IP_R {
        RT0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> RT0PSIFG_W {
        RT0PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> RT0PSIE_W {
        RT0PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> RT0IP_W {
        RT0IP_W { w: self }
    }
}
