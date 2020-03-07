#[doc = "Reader of register RSTCTL_REBOOTRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_REBOOTRESET_STAT>;
#[doc = "Reader of field `REBOOT`"]
pub type REBOOT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates if Reboot reset was caused by the SYSCTL module."]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new((self.bits & 0x01) != 0)
    }
}
