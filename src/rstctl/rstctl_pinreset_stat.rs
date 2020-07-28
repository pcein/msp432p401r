#[doc = "Reader of register RSTCTL_PINRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_PINRESET_STAT>;
#[doc = "Reader of field `RSTNMI`"]
pub type RSTNMI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - POR was caused by RSTn/NMI pin based reset event"]
    #[inline(always)]
    pub fn rstnmi(&self) -> RSTNMI_R {
        RSTNMI_R::new((self.bits & 0x01) != 0)
    }
}
