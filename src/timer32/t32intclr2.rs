#[doc = "Writer for register T32INTCLR2"]
pub type W = crate::W<u32, super::T32INTCLR2>;
#[doc = "Register T32INTCLR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::T32INTCLR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTCLR`"]
pub struct INTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write clears the interrupt output"]
    #[inline(always)]
    pub fn intclr(&mut self) -> INTCLR_W {
        INTCLR_W { w: self }
    }
}
