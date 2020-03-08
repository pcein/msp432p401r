#[doc = "Reader of register CSKEY"]
pub type R = crate::R<u32, super::CSKEY>;
#[doc = "Writer for register CSKEY"]
pub type W = crate::W<u32, super::CSKEY>;
#[doc = "Register CSKEY `reset()`'s with value 0xa596"]
impl crate::ResetValue for super::CSKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa596
    }
}
#[doc = "Reader of field `CSKEY`"]
pub type CSKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSKEY`"]
pub struct CSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CSKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&self) -> CSKEY_R {
        CSKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&mut self) -> CSKEY_W {
        CSKEY_W { w: self }
    }
}
