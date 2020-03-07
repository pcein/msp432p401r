#[doc = "Reader of register ADC14LO0"]
pub type R = crate::R<u32, super::ADC14LO0>;
#[doc = "Writer for register ADC14LO0"]
pub type W = crate::W<u32, super::ADC14LO0>;
#[doc = "Register ADC14LO0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC14LO0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC14LO0`"]
pub type ADC14LO0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC14LO0`"]
pub struct ADC14LO0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14LO0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&self) -> ADC14LO0_R {
        ADC14LO0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&mut self) -> ADC14LO0_W {
        ADC14LO0_W { w: self }
    }
}
