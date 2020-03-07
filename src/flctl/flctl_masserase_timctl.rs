#[doc = "Reader of register FLCTL_MASSERASE_TIMCTL"]
pub type R = crate::R<u32, super::FLCTL_MASSERASE_TIMCTL>;
#[doc = "Reader of field `BOOST_ACTIVE`"]
pub type BOOST_ACTIVE_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOST_HOLD`"]
pub type BOOST_HOLD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Length of the time for which LDO Boost Signal is kept active"]
    #[inline(always)]
    pub fn boost_active(&self) -> BOOST_ACTIVE_R {
        BOOST_ACTIVE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length for which Flash deactivates the LDO Boost signal before processing any new commands"]
    #[inline(always)]
    pub fn boost_hold(&self) -> BOOST_HOLD_R {
        BOOST_HOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
