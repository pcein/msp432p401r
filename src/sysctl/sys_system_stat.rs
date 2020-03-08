#[doc = "Reader of register SYS_SYSTEM_STAT"]
pub type R = crate::R<u32, super::SYS_SYSTEM_STAT>;
#[doc = "Reader of field `DBG_SEC_ACT`"]
pub type DBG_SEC_ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `JTAG_SWD_LOCK_ACT`"]
pub type JTAG_SWD_LOCK_ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `IP_PROT_ACT`"]
pub type IP_PROT_ACT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - Debug Security active"]
    #[inline(always)]
    pub fn dbg_sec_act(&self) -> DBG_SEC_ACT_R {
        DBG_SEC_ACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates if JTAG and SWD Lock is active"]
    #[inline(always)]
    pub fn jtag_swd_lock_act(&self) -> JTAG_SWD_LOCK_ACT_R {
        JTAG_SWD_LOCK_ACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates if IP protection is active"]
    #[inline(always)]
    pub fn ip_prot_act(&self) -> IP_PROT_ACT_R {
        IP_PROT_ACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
