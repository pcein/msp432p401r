#[doc = "Reader of register PJIN"]
pub type R = crate::R<u16, super::PJIN>;
#[doc = "Reader of field `PJIN`"]
pub type PJIN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Input"]
    #[inline(always)]
    pub fn pjin(&self) -> PJIN_R {
        PJIN_R::new((self.bits & 0xffff) as u16)
    }
}
