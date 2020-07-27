#[doc = "Writer for register DMA_ENACLR"]
pub type W = crate::W<u32, super::DMA_ENACLR>;
#[doc = "Register DMA_ENACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_ENACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLR_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_ENASET Register to enable DMA channels."]
    CLR_0 = 0,
    #[doc = "1: Disables channel C.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
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
    #[doc = "No effect. Use the DMA_ENASET Register to enable DMA channels."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_0)
    }
    #[doc = "Disables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
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
    #[doc = "Bits 0:31 - Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
}
