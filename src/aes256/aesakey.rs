#[doc = "Writer for register AESAKEY"]
pub type W = crate::W<u16, super::AESAKEY>;
#[doc = "Register AESAKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAKEY {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AESKEY0x`"]
pub struct AESKEY0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `AESKEY1x`"]
pub struct AESKEY1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0x(&mut self) -> AESKEY0X_W {
        AESKEY0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1x(&mut self) -> AESKEY1X_W {
        AESKEY1X_W { w: self }
    }
}
