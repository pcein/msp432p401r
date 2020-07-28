#[doc = "Reader of register RTCTIM0"]
pub type R = crate::R<u16, super::RTCTIM0>;
#[doc = "Writer for register RTCTIM0"]
pub type W = crate::W<u16, super::RTCTIM0>;
#[doc = "Register RTCTIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCTIM0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Seconds`"]
pub type SECONDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Seconds`"]
pub struct SECONDS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
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
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&mut self) -> SECONDS_W {
        SECONDS_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
}
