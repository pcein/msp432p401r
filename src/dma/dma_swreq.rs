#[doc = "Writer for register DMA_SWREQ"]
pub type W = crate::W<u32, super::DMA_SWREQ>;
#[doc = "Register DMA_SWREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SWREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHNL_SW_REQ_AW {
    #[doc = "0: Does not create a DMA request for the channel"]
    CHNL_SW_REQ_0 = 0,
    #[doc = "1: Creates a DMA request for the channel"]
    CHNL_SW_REQ_1 = 1,
}
impl From<CHNL_SW_REQ_AW> for u32 {
    #[inline(always)]
    fn from(variant: CHNL_SW_REQ_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CHNL_SW_REQ`"]
pub struct CHNL_SW_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_SW_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHNL_SW_REQ_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Does not create a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_0(self) -> &'a mut W {
        self.variant(CHNL_SW_REQ_AW::CHNL_SW_REQ_0)
    }
    #[doc = "Creates a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_1(self) -> &'a mut W {
        self.variant(CHNL_SW_REQ_AW::CHNL_SW_REQ_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel."]
    #[inline(always)]
    pub fn chnl_sw_req(&mut self) -> CHNL_SW_REQ_W {
        CHNL_SW_REQ_W { w: self }
    }
}
