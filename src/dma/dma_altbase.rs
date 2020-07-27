#[doc = "Reader of register DMA_ALTBASE"]
pub type R = crate::R<u32, super::DMA_ALTBASE>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of the alternate data structure"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
