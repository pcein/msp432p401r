#[doc = "Writer for register AESADOUT"]
pub type W = crate::W<u16, super::AESADOUT>;
#[doc = "Register AESADOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::AESADOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AESDOUT0x`"]
pub struct AESDOUT0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUT0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `AESDOUT1x`"]
pub struct AESDOUT1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUT1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0x(&mut self) -> AESDOUT0X_W {
        AESDOUT0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1x(&mut self) -> AESDOUT1X_W {
        AESDOUT1X_W { w: self }
    }
}
