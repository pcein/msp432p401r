#[doc = "Writer for register PCMCLRIFG"]
pub type W = crate::W<u32, super::PCMCLRIFG>;
#[doc = "Register PCMCLRIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCMCLRIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLR_LPM_INVALID_TR_IFG`"]
pub struct CLR_LPM_INVALID_TR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LPM_INVALID_TR_IFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CLR_LPM_INVALID_CLK_IFG`"]
pub struct CLR_LPM_INVALID_CLK_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LPM_INVALID_CLK_IFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CLR_AM_INVALID_TR_IFG`"]
pub struct CLR_AM_INVALID_TR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_AM_INVALID_TR_IFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CLR_DCDC_ERROR_IFG`"]
pub struct CLR_DCDC_ERROR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DCDC_ERROR_IFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear LPM invalid transition flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_tr_ifg(&mut self) -> CLR_LPM_INVALID_TR_IFG_W {
        CLR_LPM_INVALID_TR_IFG_W { w: self }
    }
    #[doc = "Bit 1 - Clear LPM invalid clock flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_clk_ifg(&mut self) -> CLR_LPM_INVALID_CLK_IFG_W {
        CLR_LPM_INVALID_CLK_IFG_W { w: self }
    }
    #[doc = "Bit 2 - Clear active mode invalid transition flag"]
    #[inline(always)]
    pub fn clr_am_invalid_tr_ifg(&mut self) -> CLR_AM_INVALID_TR_IFG_W {
        CLR_AM_INVALID_TR_IFG_W { w: self }
    }
    #[doc = "Bit 6 - Clear DC-DC error flag"]
    #[inline(always)]
    pub fn clr_dcdc_error_ifg(&mut self) -> CLR_DCDC_ERROR_IFG_W {
        CLR_DCDC_ERROR_IFG_W { w: self }
    }
}
