#[doc = "Reader of register PAIN"]
pub type R = crate::R<u16, super::PAIN>;
#[doc = "Reader of field `P1IN`"]
pub type P1IN_R = crate::R<u8, u8>;
#[doc = "Reader of field `P2IN`"]
pub type P2IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Input"]
    #[inline(always)]
    pub fn p1in(&self) -> P1IN_R {
        P1IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Input"]
    #[inline(always)]
    pub fn p2in(&self) -> P2IN_R {
        P2IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
