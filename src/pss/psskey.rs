#[doc = "Reader of register PSSKEY"]
pub type R = crate::R<u32, super::PSSKEY>;
#[doc = "Writer for register PSSKEY"]
pub type W = crate::W<u32, super::PSSKEY>;
#[doc = "Register PSSKEY `reset()`'s with value 0xa596"]
impl crate::ResetValue for super::PSSKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa596
    }
}
#[doc = "Reader of field `PSSKEY`"]
pub type PSSKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSSKEY`"]
pub struct PSSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&self) -> PSSKEY_R {
        PSSKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&mut self) -> PSSKEY_W {
        PSSKEY_W { w: self }
    }
}
