#[doc = "Reader of register PDIN"]
pub type R = crate::R<u16, super::PDIN>;
#[doc = "Reader of field `P7IN`"]
pub type P7IN_R = crate::R<u8, u8>;
#[doc = "Reader of field `P8IN`"]
pub type P8IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Input"]
    #[inline(always)]
    pub fn p7in(&self) -> P7IN_R {
        P7IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Input"]
    #[inline(always)]
    pub fn p8in(&self) -> P8IN_R {
        P8IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
