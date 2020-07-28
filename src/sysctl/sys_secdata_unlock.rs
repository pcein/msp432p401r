#[doc = "Reader of register SYS_SECDATA_UNLOCK"]
pub type R = crate::R<u32, super::SYS_SECDATA_UNLOCK>;
#[doc = "Writer for register SYS_SECDATA_UNLOCK"]
pub type W = crate::W<u32, super::SYS_SECDATA_UNLOCK>;
#[doc = "Register SYS_SECDATA_UNLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_SECDATA_UNLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UNLKEY`"]
pub type UNLKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UNLKEY`"]
pub struct UNLKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Unlock key"]
    #[inline(always)]
    pub fn unlkey(&self) -> UNLKEY_R {
        UNLKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlock key"]
    #[inline(always)]
    pub fn unlkey(&mut self) -> UNLKEY_W {
        UNLKEY_W { w: self }
    }
}
