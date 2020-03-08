#[doc = "Reader of register DMA_WAITSTAT"]
pub type R = crate::R<u32, super::DMA_WAITSTAT>;
#[doc = "Channel wait on request status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WAITREQ_A {
    #[doc = "0: dma_waitonreq\\[C\\]
is LOW."]
    WAITREQ_0 = 0,
    #[doc = "1: dma_waitonreq\\[C\\]
is HIGH."]
    WAITREQ_1 = 1,
}
impl From<WAITREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: WAITREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAITREQ`"]
pub type WAITREQ_R = crate::R<u32, WAITREQ_A>;
impl WAITREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, WAITREQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAITREQ_A::WAITREQ_0),
            1 => Val(WAITREQ_A::WAITREQ_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAITREQ_0`"]
    #[inline(always)]
    pub fn is_waitreq_0(&self) -> bool {
        *self == WAITREQ_A::WAITREQ_0
    }
    #[doc = "Checks if the value of the field is `WAITREQ_1`"]
    #[inline(always)]
    pub fn is_waitreq_1(&self) -> bool {
        *self == WAITREQ_A::WAITREQ_1
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel wait on request status."]
    #[inline(always)]
    pub fn waitreq(&self) -> WAITREQ_R {
        WAITREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
