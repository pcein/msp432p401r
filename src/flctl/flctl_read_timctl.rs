#[doc = "Reader of register FLCTL_READ_TIMCTL"]
pub type R = crate::R<u32, super::FLCTL_READ_TIMCTL>;
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<u8, u8>;
#[doc = "Reader of field `IREF_BOOST1`"]
pub type IREF_BOOST1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SETUP_LONG`"]
pub type SETUP_LONG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Configures the length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Length of the IREF_BOOST1 signal of the IP"]
    #[inline(always)]
    pub fn iref_boost1(&self) -> IREF_BOOST1_R {
        IREF_BOOST1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Length of the Setup time into read mode when the device is recovering from one of the following conditions: Moving from Power-down or Standby back to Active and device is not trimmed. Moving from standby to active state in low-frequency active mode. Recovering from the LDO Boost operation after a Mass Erase."]
    #[inline(always)]
    pub fn setup_long(&self) -> SETUP_LONG_R {
        SETUP_LONG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
