#[doc = "Reader of register FLCTL_ERSVER_TIMCTL"]
pub type R = crate::R<u32, super::FLCTL_ERSVER_TIMCTL>;
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
}
