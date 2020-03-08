#[doc = "Reader of register RTCAMINHR"]
pub type R = crate::R<u16, super::RTCAMINHR>;
#[doc = "Writer for register RTCAMINHR"]
pub type W = crate::W<u16, super::RTCAMINHR>;
#[doc = "Register RTCAMINHR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCAMINHR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Minutes`"]
pub type MINUTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Minutes`"]
pub struct MINUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `MINAE`"]
pub type MINAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINAE`"]
pub struct MINAE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `Hours`"]
pub type HOURS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Hours`"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOURAE`"]
pub type HOURAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOURAE`"]
pub struct HOURAE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURAE_W<'a> {
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
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MINAE_R {
        MINAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HOURAE_R {
        HOURAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MINAE_W {
        MINAE_W { w: self }
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HOURAE_W {
        HOURAE_W { w: self }
    }
}
