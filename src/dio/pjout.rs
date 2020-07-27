#[doc = "Reader of register PJOUT"]
pub type R = crate::R<u16, super::PJOUT>;
#[doc = "Writer for register PJOUT"]
pub type W = crate::W<u16, super::PJOUT>;
#[doc = "Register PJOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PJOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJOUT`"]
pub type PJOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJOUT`"]
pub struct PJOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&self) -> PJOUT_R {
        PJOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&mut self) -> PJOUT_W {
        PJOUT_W { w: self }
    }
}
