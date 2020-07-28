#[doc = "Writer for register AESAXDIN"]
pub type W = crate::W<u16, super::AESAXDIN>;
#[doc = "Register AESAXDIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAXDIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AESXDIN0x`"]
pub struct AESXDIN0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXDIN0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `AESXDIN1x`"]
pub struct AESXDIN1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXDIN1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0x(&mut self) -> AESXDIN0X_W {
        AESXDIN0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1x(&mut self) -> AESXDIN1X_W {
        AESXDIN1X_W { w: self }
    }
}
