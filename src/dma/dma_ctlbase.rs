#[doc = "Reader of register DMA_CTLBASE"]
pub type R = crate::R<u32, super::DMA_CTLBASE>;
#[doc = "Writer for register DMA_CTLBASE"]
pub type W = crate::W<u32, super::DMA_CTLBASE>;
#[doc = "Register DMA_CTLBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CTLBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - Pointer to the base address of the primary data structure."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:31 - Pointer to the base address of the primary data structure."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
