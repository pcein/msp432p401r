#[doc = "Reader of register DMA_DEVICE_CFG"]
pub type R = crate::R<u32, super::DMA_DEVICE_CFG>;
#[doc = "Reader of field `NUM_DMA_CHANNELS`"]
pub type NUM_DMA_CHANNELS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_SRC_PER_CHANNEL`"]
pub type NUM_SRC_PER_CHANNEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of DMA channels available"]
    #[inline(always)]
    pub fn num_dma_channels(&self) -> NUM_DMA_CHANNELS_R {
        NUM_DMA_CHANNELS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of DMA sources per channel"]
    #[inline(always)]
    pub fn num_src_per_channel(&self) -> NUM_SRC_PER_CHANNEL_R {
        NUM_SRC_PER_CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
