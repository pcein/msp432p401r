#[doc = "Reader of register CSCTL0"]
pub type R = crate::R<u32, super::CSCTL0>;
#[doc = "Writer for register CSCTL0"]
pub type W = crate::W<u32, super::CSCTL0>;
#[doc = "Register CSCTL0 `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::CSCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `DCOTUNE`"]
pub type DCOTUNE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCOTUNE`"]
pub struct DCOTUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOTUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "DCO frequency range select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: Nominal DCO Frequency Range (MHz): 1 to 2"]
    DCORSEL_0 = 0,
    #[doc = "1: Nominal DCO Frequency Range (MHz): 2 to 4"]
    DCORSEL_1 = 1,
    #[doc = "2: Nominal DCO Frequency Range (MHz): 4 to 8"]
    DCORSEL_2 = 2,
    #[doc = "3: Nominal DCO Frequency Range (MHz): 8 to 16"]
    DCORSEL_3 = 3,
    #[doc = "4: Nominal DCO Frequency Range (MHz): 16 to 32"]
    DCORSEL_4 = 4,
    #[doc = "5: Nominal DCO Frequency Range (MHz): 32 to 64"]
    DCORSEL_5 = 5,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCORSEL`"]
pub type DCORSEL_R = crate::R<u8, DCORSEL_A>;
impl DCORSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCORSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCORSEL_A::DCORSEL_0),
            1 => Val(DCORSEL_A::DCORSEL_1),
            2 => Val(DCORSEL_A::DCORSEL_2),
            3 => Val(DCORSEL_A::DCORSEL_3),
            4 => Val(DCORSEL_A::DCORSEL_4),
            5 => Val(DCORSEL_A::DCORSEL_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
}
#[doc = "Write proxy for field `DCORSEL`"]
pub struct DCORSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCORSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCORSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 1 to 2"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 2 to 4"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 4 to 8"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 8 to 16"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 16 to 32"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 32 to 64"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Enables the DCO external resistor mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCORES_A {
    #[doc = "0: Internal resistor mode"]
    DCORES_0 = 0,
    #[doc = "1: External resistor mode"]
    DCORES_1 = 1,
}
impl From<DCORES_A> for bool {
    #[inline(always)]
    fn from(variant: DCORES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCORES`"]
pub type DCORES_R = crate::R<bool, DCORES_A>;
impl DCORES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORES_A {
        match self.bits {
            false => DCORES_A::DCORES_0,
            true => DCORES_A::DCORES_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCORES_0`"]
    #[inline(always)]
    pub fn is_dcores_0(&self) -> bool {
        *self == DCORES_A::DCORES_0
    }
    #[doc = "Checks if the value of the field is `DCORES_1`"]
    #[inline(always)]
    pub fn is_dcores_1(&self) -> bool {
        *self == DCORES_A::DCORES_1
    }
}
#[doc = "Write proxy for field `DCORES`"]
pub struct DCORES_W<'a> {
    w: &'a mut W,
}
impl<'a> DCORES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCORES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal resistor mode"]
    #[inline(always)]
    pub fn dcores_0(self) -> &'a mut W {
        self.variant(DCORES_A::DCORES_0)
    }
    #[doc = "External resistor mode"]
    #[inline(always)]
    pub fn dcores_1(self) -> &'a mut W {
        self.variant(DCORES_A::DCORES_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Enables the DCO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOEN_A {
    #[doc = "0: DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    DCOEN_0 = 0,
    #[doc = "1: DCO is on"]
    DCOEN_1 = 1,
}
impl From<DCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCOEN`"]
pub type DCOEN_R = crate::R<bool, DCOEN_A>;
impl DCOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOEN_A {
        match self.bits {
            false => DCOEN_A::DCOEN_0,
            true => DCOEN_A::DCOEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOEN_0`"]
    #[inline(always)]
    pub fn is_dcoen_0(&self) -> bool {
        *self == DCOEN_A::DCOEN_0
    }
    #[doc = "Checks if the value of the field is `DCOEN_1`"]
    #[inline(always)]
    pub fn is_dcoen_1(&self) -> bool {
        *self == DCOEN_A::DCOEN_1
    }
}
#[doc = "Write proxy for field `DCOEN`"]
pub struct DCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    #[inline(always)]
    pub fn dcoen_0(self) -> &'a mut W {
        self.variant(DCOEN_A::DCOEN_0)
    }
    #[doc = "DCO is on"]
    #[inline(always)]
    pub fn dcoen_1(self) -> &'a mut W {
        self.variant(DCOEN_A::DCOEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&self) -> DCOTUNE_R {
        DCOTUNE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&self) -> DCORES_R {
        DCORES_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&self) -> DCOEN_R {
        DCOEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&mut self) -> DCOTUNE_W {
        DCOTUNE_W { w: self }
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W {
        DCORSEL_W { w: self }
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&mut self) -> DCORES_W {
        DCORES_W { w: self }
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&mut self) -> DCOEN_W {
        DCOEN_W { w: self }
    }
}
