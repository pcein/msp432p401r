#[doc = "Reader of register PSSCLRIFG"]
pub type R = crate::R<u32, super::PSSCLRIFG>;
#[doc = "Writer for register PSSCLRIFG"]
pub type W = crate::W<u32, super::PSSCLRIFG>;
#[doc = "Register PSSCLRIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSCLRIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SVSMH clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRSVSMHIFG_AW {
    #[doc = "0: No effect"]
    CLRSVSMHIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLRSVSMHIFG_1 = 1,
}
impl From<CLRSVSMHIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRSVSMHIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLRSVSMHIFG`"]
pub struct CLRSVSMHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSVSMHIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRSVSMHIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrsvsmhifg_0(self) -> &'a mut W {
        self.variant(CLRSVSMHIFG_AW::CLRSVSMHIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg_1(self) -> &'a mut W {
        self.variant(CLRSVSMHIFG_AW::CLRSVSMHIFG_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 1 - SVSMH clear interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg(&mut self) -> CLRSVSMHIFG_W {
        CLRSVSMHIFG_W { w: self }
    }
}
