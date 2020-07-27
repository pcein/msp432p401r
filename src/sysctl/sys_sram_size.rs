#[doc = "Reader of register SYS_SRAM_SIZE"]
pub type R = crate::R<u32, super::SYS_SRAM_SIZE>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the size of SRAM on the device"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
