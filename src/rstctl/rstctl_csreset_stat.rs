#[doc = "Reader of register RSTCTL_CSRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_CSRESET_STAT>;
#[doc = "Reader of field `DCOR_SHT`"]
pub type DCOR_SHT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by DCO short circuit fault in the external resistor mode"]
    #[inline(always)]
    pub fn dcor_sht(&self) -> DCOR_SHT_R {
        DCOR_SHT_R::new((self.bits & 0x01) != 0)
    }
}
