#[doc = "Reader of register CSDCOERCAL0"]
pub type R = crate::R<u32, super::CSDCOERCAL0>;
#[doc = "Writer for register CSDCOERCAL0"]
pub type W = crate::W<u32, super::CSDCOERCAL0>;
#[doc = "Register CSDCOERCAL0 `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::CSDCOERCAL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `DCO_TCCAL`"]
pub type DCO_TCCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCO_TCCAL`"]
pub struct DCO_TCCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_TCCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DCO_FCAL_RSEL04`"]
pub type DCO_FCAL_RSEL04_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCO_FCAL_RSEL04`"]
pub struct DCO_FCAL_RSEL04_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_FCAL_RSEL04_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&self) -> DCO_TCCAL_R {
        DCO_TCCAL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&self) -> DCO_FCAL_RSEL04_R {
        DCO_FCAL_RSEL04_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&mut self) -> DCO_TCCAL_W {
        DCO_TCCAL_W { w: self }
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&mut self) -> DCO_FCAL_RSEL04_W {
        DCO_FCAL_RSEL04_W { w: self }
    }
}
