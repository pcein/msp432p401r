#[doc = "Reader of register T32RIS1"]
pub type R = crate::R<u32, super::T32RIS1>;
#[doc = "Reader of field `RAW_IFG`"]
pub type RAW_IFG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw interrupt status"]
    #[inline(always)]
    pub fn raw_ifg(&self) -> RAW_IFG_R {
        RAW_IFG_R::new((self.bits & 0x01) != 0)
    }
}
