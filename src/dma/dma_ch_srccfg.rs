#[doc = "Reader of register DMA_CH_SRCCFG[%s]"]
pub type R = crate::R<u32, super::DMA_CH_SRCCFG>;
#[doc = "Writer for register DMA_CH_SRCCFG[%s]"]
pub type W = crate::W<u32, super::DMA_CH_SRCCFG>;
#[doc = "Register DMA_CH_SRCCFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CH_SRCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_SRC`"]
pub type DMA_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_SRC`"]
pub struct DMA_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Device level DMA source mapping to channel input"]
    #[inline(always)]
    pub fn dma_src(&self) -> DMA_SRC_R {
        DMA_SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Device level DMA source mapping to channel input"]
    #[inline(always)]
    pub fn dma_src(&mut self) -> DMA_SRC_W {
        DMA_SRC_W { w: self }
    }
}
