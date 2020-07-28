#[doc = "Reader of register PMAPKEYID"]
pub type R = crate::R<u16, super::PMAPKEYID>;
#[doc = "Writer for register PMAPKEYID"]
pub type W = crate::W<u16, super::PMAPKEYID>;
#[doc = "Register PMAPKEYID `reset()`'s with value 0x96a5"]
impl crate::ResetValue for super::PMAPKEYID {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x96a5
    }
}
#[doc = "Reader of field `PMAPKEY`"]
pub type PMAPKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PMAPKEY`"]
pub struct PMAPKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port mapping controller write access key"]
    #[inline(always)]
    pub fn pmapkey(&self) -> PMAPKEY_R {
        PMAPKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port mapping controller write access key"]
    #[inline(always)]
    pub fn pmapkey(&mut self) -> PMAPKEY_W {
        PMAPKEY_W { w: self }
    }
}
