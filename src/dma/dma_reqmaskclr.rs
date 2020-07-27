#[doc = "Writer for register DMA_REQMASKCLR"]
pub type W = crate::W<u32, super::DMA_REQMASKCLR>;
#[doc = "Register DMA_REQMASKCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_REQMASKCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req and dma_sreq.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLR_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_REQMASKSET Register to disable dma_req and\r\r\ndma_sreq from generating requests."]
    CLR_0 = 0,
    #[doc = "1: Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    CLR_1 = 1,
}
impl From<CLR_AW> for u32 {
    #[inline(always)]
    fn from(variant: CLR_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CLR`"]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_REQMASKSET Register to disable dma_req and dma_sreq from generating requests."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_0)
    }
    #[doc = "Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req and dma_sreq."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
}
