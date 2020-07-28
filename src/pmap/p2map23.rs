#[doc = "Reader of register P2MAP23"]
pub type R = crate::R<u16, super::P2MAP23>;
#[doc = "Writer for register P2MAP23"]
pub type W = crate::W<u16, super::P2MAP23>;
#[doc = "Register P2MAP23 `reset()`'s with value 0"]
impl crate::ResetValue for super::P2MAP23 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMAPx`"]
pub type PMAPX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PMAPx`"]
pub struct PMAPX_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&self) -> PMAPX_R {
        PMAPX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&mut self) -> PMAPX_W {
        PMAPX_W { w: self }
    }
}
