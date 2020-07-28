#[doc = "Reader of register FLCTL_PRGVER_TIMCTL"]
pub type R = crate::R<u32, super::FLCTL_PRGVER_TIMCTL>;
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<u8, u8>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOLD`"]
pub type HOLD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Length of the Hold phase for this operation"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
