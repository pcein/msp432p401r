#[doc = "Reader of register AESACTL1"]
pub type R = crate::R<u16, super::AESACTL1>;
#[doc = "Writer for register AESACTL1"]
pub type W = crate::W<u16, super::AESACTL1>;
#[doc = "Register AESACTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESBLKCNTx`"]
pub type AESBLKCNTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESBLKCNTx`"]
pub struct AESBLKCNTX_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&self) -> AESBLKCNTX_R {
        AESBLKCNTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&mut self) -> AESBLKCNTX_W {
        AESBLKCNTX_W { w: self }
    }
}
