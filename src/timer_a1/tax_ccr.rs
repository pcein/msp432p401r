#[doc = "Reader of register TAxCCR[%s]"]
pub type R = crate::R<u16, super::TAXCCR>;
#[doc = "Writer for register TAxCCR[%s]"]
pub type W = crate::W<u16, super::TAXCCR>;
#[doc = "Register TAxCCR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TAXCCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAxR`"]
pub type TAXR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TAxR`"]
pub struct TAXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAXR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&self) -> TAXR_R {
        TAXR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&mut self) -> TAXR_W {
        TAXR_W { w: self }
    }
}
