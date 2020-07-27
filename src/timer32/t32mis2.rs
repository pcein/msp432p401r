#[doc = "Reader of register T32MIS2"]
pub type R = crate::R<u32, super::T32MIS2>;
#[doc = "Reader of field `IFG`"]
pub type IFG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enabled interrupt status"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new((self.bits & 0x01) != 0)
    }
}
