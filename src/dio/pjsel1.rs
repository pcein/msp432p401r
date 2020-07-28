#[doc = "Reader of register PJSEL1"]
pub type R = crate::R<u16, super::PJSEL1>;
#[doc = "Writer for register PJSEL1"]
pub type W = crate::W<u16, super::PJSEL1>;
#[doc = "Register PJSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PJSEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJSEL1`"]
pub type PJSEL1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJSEL1`"]
pub struct PJSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&self) -> PJSEL1_R {
        PJSEL1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&mut self) -> PJSEL1_W {
        PJSEL1_W { w: self }
    }
}
