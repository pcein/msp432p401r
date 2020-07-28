#[doc = "Reader of register ADC14MEM[%s]"]
pub type R = crate::R<u32, super::ADC14MEM>;
#[doc = "Writer for register ADC14MEM[%s]"]
pub type W = crate::W<u32, super::ADC14MEM>;
#[doc = "Register ADC14MEM[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14MEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Conversion_Results`"]
pub type CONVERSION_RESULTS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Conversion_Results`"]
pub struct CONVERSION_RESULTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVERSION_RESULTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&self) -> CONVERSION_RESULTS_R {
        CONVERSION_RESULTS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&mut self) -> CONVERSION_RESULTS_W {
        CONVERSION_RESULTS_W { w: self }
    }
}
