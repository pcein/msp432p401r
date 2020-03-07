#[doc = "Reader of register PCIN"]
pub type R = crate::R<u16, super::PCIN>;
#[doc = "Reader of field `P5IN`"]
pub type P5IN_R = crate::R<u8, u8>;
#[doc = "Reader of field `P6IN`"]
pub type P6IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Input"]
    #[inline(always)]
    pub fn p5in(&self) -> P5IN_R {
        P5IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Input"]
    #[inline(always)]
    pub fn p6in(&self) -> P6IN_R {
        P6IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
