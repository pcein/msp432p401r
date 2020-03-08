#[doc = "Reader of register SYS_FLASH_SIZE"]
pub type R = crate::R<u32, super::SYS_FLASH_SIZE>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash User Region size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
