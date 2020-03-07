#[doc = "Reader of register PBIN"]
pub type R = crate::R<u16, super::PBIN>;
#[doc = "Reader of field `P3IN`"]
pub type P3IN_R = crate::R<u8, u8>;
#[doc = "Reader of field `P4IN`"]
pub type P4IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Input"]
    #[inline(always)]
    pub fn p3in(&self) -> P3IN_R {
        P3IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Input"]
    #[inline(always)]
    pub fn p4in(&self) -> P4IN_R {
        P4IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
