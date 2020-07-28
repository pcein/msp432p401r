#[doc = "Reader of register PJDIR"]
pub type R = crate::R<u16, super::PJDIR>;
#[doc = "Writer for register PJDIR"]
pub type W = crate::W<u16, super::PJDIR>;
#[doc = "Register PJDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PJDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJDIR`"]
pub type PJDIR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJDIR`"]
pub struct PJDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&self) -> PJDIR_R {
        PJDIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&mut self) -> PJDIR_W {
        PJDIR_W { w: self }
    }
}
