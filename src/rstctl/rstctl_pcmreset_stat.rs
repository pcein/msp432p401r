#[doc = "Reader of register RSTCTL_PCMRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_PCMRESET_STAT>;
#[doc = "Reader of field `LPM35`"]
pub type LPM35_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPM45`"]
pub type LPM45_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by PCM due to an exit from LPM3.5"]
    #[inline(always)]
    pub fn lpm35(&self) -> LPM35_R {
        LPM35_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if POR was caused by PCM due to an exit from LPM4.5"]
    #[inline(always)]
    pub fn lpm45(&self) -> LPM45_R {
        LPM45_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
