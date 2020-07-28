#[doc = "Reader of register T32LOAD1"]
pub type R = crate::R<u32, super::T32LOAD1>;
#[doc = "Writer for register T32LOAD1"]
pub type W = crate::W<u32, super::T32LOAD1>;
#[doc = "Register T32LOAD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::T32LOAD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
}
