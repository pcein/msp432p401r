#[doc = "Reader of register PJREN"]
pub type R = crate::R<u16, super::PJREN>;
#[doc = "Writer for register PJREN"]
pub type W = crate::W<u16, super::PJREN>;
#[doc = "Register PJREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PJREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJREN`"]
pub type PJREN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PJREN`"]
pub struct PJREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&self) -> PJREN_R {
        PJREN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&mut self) -> PJREN_W {
        PJREN_W { w: self }
    }
}
