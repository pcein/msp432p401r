#[doc = "Reader of register CSDCOERCAL1"]
pub type R = crate::R<u32, super::CSDCOERCAL1>;
#[doc = "Writer for register CSDCOERCAL1"]
pub type W = crate::W<u32, super::CSDCOERCAL1>;
#[doc = "Register CSDCOERCAL1 `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::CSDCOERCAL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `DCO_FCAL_RSEL5`"]
pub type DCO_FCAL_RSEL5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCO_FCAL_RSEL5`"]
pub struct DCO_FCAL_RSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_FCAL_RSEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&self) -> DCO_FCAL_RSEL5_R {
        DCO_FCAL_RSEL5_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&mut self) -> DCO_FCAL_RSEL5_W {
        DCO_FCAL_RSEL5_W { w: self }
    }
}
