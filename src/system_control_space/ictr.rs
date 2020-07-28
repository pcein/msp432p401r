#[doc = "Reader of register ICTR"]
pub type R = crate::R<u32, super::ICTR>;
#[doc = "Reader of field `INTLINESNUM`"]
pub type INTLINESNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Total number of interrupt lines in groups of 32."]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x1f) as u8)
    }
}
