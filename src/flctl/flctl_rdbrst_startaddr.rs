#[doc = "Reader of register FLCTL_RDBRST_STARTADDR"]
pub type R = crate::R<u32, super::FLCTL_RDBRST_STARTADDR>;
#[doc = "Writer for register FLCTL_RDBRST_STARTADDR"]
pub type W = crate::W<u32, super::FLCTL_RDBRST_STARTADDR>;
#[doc = "Register FLCTL_RDBRST_STARTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_RDBRST_STARTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_ADDRESS`"]
pub type START_ADDRESS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `START_ADDRESS`"]
pub struct START_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> START_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&self) -> START_ADDRESS_R {
        START_ADDRESS_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&mut self) -> START_ADDRESS_W {
        START_ADDRESS_W { w: self }
    }
}
