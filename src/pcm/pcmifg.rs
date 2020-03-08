#[doc = "Reader of register PCMIFG"]
pub type R = crate::R<u32, super::PCMIFG>;
#[doc = "Reader of field `LPM_INVALID_TR_IFG`"]
pub type LPM_INVALID_TR_IFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPM_INVALID_CLK_IFG`"]
pub type LPM_INVALID_CLK_IFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `AM_INVALID_TR_IFG`"]
pub type AM_INVALID_TR_IFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_ERROR_IFG`"]
pub type DCDC_ERROR_IFG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - LPM invalid transition flag"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ifg(&self) -> LPM_INVALID_TR_IFG_R {
        LPM_INVALID_TR_IFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock flag"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ifg(&self) -> LPM_INVALID_CLK_IFG_R {
        LPM_INVALID_CLK_IFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition flag"]
    #[inline(always)]
    pub fn am_invalid_tr_ifg(&self) -> AM_INVALID_TR_IFG_R {
        AM_INVALID_TR_IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DC-DC error flag"]
    #[inline(always)]
    pub fn dcdc_error_ifg(&self) -> DCDC_ERROR_IFG_R {
        DCDC_ERROR_IFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
