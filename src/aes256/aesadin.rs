#[doc = "Writer for register AESADIN"]
pub type W = crate::W<u16, super::AESADIN>;
#[doc = "Register AESADIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESADIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AESDIN0x`"]
pub struct AESDIN0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDIN0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `AESDIN1x`"]
pub struct AESDIN1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDIN1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0x(&mut self) -> AESDIN0X_W {
        AESDIN0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1x(&mut self) -> AESDIN1X_W {
        AESDIN1X_W { w: self }
    }
}
