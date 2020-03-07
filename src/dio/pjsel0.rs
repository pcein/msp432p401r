#[doc = "Reader of register PJSEL0"]
pub type R = crate::R<u16, super::PJSEL0>;
#[doc = "Writer for register PJSEL0"]
pub type W = crate::W<u16, super::PJSEL0>;
#[doc = "Register PJSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PJSEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJSEL0`"]
pub type PJSEL0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJSEL0`"]
pub struct PJSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Select 0"]
    #[inline(always)]
    pub fn pjsel0(&self) -> PJSEL0_R {
        PJSEL0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 0"]
    #[inline(always)]
    pub fn pjsel0(&mut self) -> PJSEL0_W {
        PJSEL0_W { w: self }
    }
}
