#[doc = "Writer for register AESAXIN"]
pub type W = crate::W<u16, super::AESAXIN>;
#[doc = "Register AESAXIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAXIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AESXIN0x`"]
pub struct AESXIN0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `AESXIN1x`"]
pub struct AESXIN1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0x(&mut self) -> AESXIN0X_W {
        AESXIN0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1x(&mut self) -> AESXIN1X_W {
        AESXIN1X_W { w: self }
    }
}
