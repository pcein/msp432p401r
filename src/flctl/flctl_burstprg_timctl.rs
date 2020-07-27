#[doc = "Reader of register FLCTL_BURSTPRG_TIMCTL"]
pub type R = crate::R<u32, super::FLCTL_BURSTPRG_TIMCTL>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 8:27 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 8) & 0x000f_ffff) as u32)
    }
}
