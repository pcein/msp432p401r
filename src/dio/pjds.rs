#[doc = "Reader of register PJDS"]
pub type R = crate::R<u16, super::PJDS>;
#[doc = "Writer for register PJDS"]
pub type W = crate::W<u16, super::PJDS>;
#[doc = "Register PJDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PJDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJDS`"]
pub type PJDS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJDS`"]
pub struct PJDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&self) -> PJDS_R {
        PJDS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&mut self) -> PJDS_W {
        PJDS_W { w: self }
    }
}
