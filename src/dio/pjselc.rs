#[doc = "Reader of register PJSELC"]
pub type R = crate::R<u16, super::PJSELC>;
#[doc = "Writer for register PJSELC"]
pub type W = crate::W<u16, super::PJSELC>;
#[doc = "Register PJSELC `reset()`'s with value 0"]
impl crate::ResetValue for super::PJSELC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJSELC`"]
pub type PJSELC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJSELC`"]
pub struct PJSELC_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&self) -> PJSELC_R {
        PJSELC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&mut self) -> PJSELC_W {
        PJSELC_W { w: self }
    }
}
