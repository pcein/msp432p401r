#[doc = "Reader of register PEIN"]
pub type R = crate::R<u16, super::PEIN>;
#[doc = "Reader of field `P9IN`"]
pub type P9IN_R = crate::R<u8, u8>;
#[doc = "Reader of field `P10IN`"]
pub type P10IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Input"]
    #[inline(always)]
    pub fn p9in(&self) -> P9IN_R {
        P9IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Input"]
    #[inline(always)]
    pub fn p10in(&self) -> P10IN_R {
        P10IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
