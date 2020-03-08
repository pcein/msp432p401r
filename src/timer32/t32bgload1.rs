#[doc = "Reader of register T32BGLOAD1"]
pub type R = crate::R<u32, super::T32BGLOAD1>;
#[doc = "Writer for register T32BGLOAD1"]
pub type W = crate::W<u32, super::T32BGLOAD1>;
#[doc = "Register T32BGLOAD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::T32BGLOAD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BGLOAD`"]
pub type BGLOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BGLOAD`"]
pub struct BGLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BGLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&self) -> BGLOAD_R {
        BGLOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&mut self) -> BGLOAD_W {
        BGLOAD_W { w: self }
    }
}
