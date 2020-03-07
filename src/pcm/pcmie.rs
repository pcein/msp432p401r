#[doc = "Reader of register PCMIE"]
pub type R = crate::R<u32, super::PCMIE>;
#[doc = "Writer for register PCMIE"]
pub type W = crate::W<u32, super::PCMIE>;
#[doc = "Register PCMIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PCMIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPM invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_INVALID_TR_IE_A {
    #[doc = "0: Disabled"]
    LPM_INVALID_TR_IE_0 = 0,
    #[doc = "1: Enabled"]
    LPM_INVALID_TR_IE_1 = 1,
}
impl From<LPM_INVALID_TR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_INVALID_TR_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPM_INVALID_TR_IE`"]
pub type LPM_INVALID_TR_IE_R = crate::R<bool, LPM_INVALID_TR_IE_A>;
impl LPM_INVALID_TR_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_INVALID_TR_IE_A {
        match self.bits {
            false => LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0,
            true => LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_TR_IE_0`"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_0(&self) -> bool {
        *self == LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_TR_IE_1`"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_1(&self) -> bool {
        *self == LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1
    }
}
#[doc = "Write proxy for field `LPM_INVALID_TR_IE`"]
pub struct LPM_INVALID_TR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INVALID_TR_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_INVALID_TR_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_0(self) -> &'a mut W {
        self.variant(LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_1(self) -> &'a mut W {
        self.variant(LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "LPM invalid clock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_INVALID_CLK_IE_A {
    #[doc = "0: Disabled"]
    LPM_INVALID_CLK_IE_0 = 0,
    #[doc = "1: Enabled"]
    LPM_INVALID_CLK_IE_1 = 1,
}
impl From<LPM_INVALID_CLK_IE_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_INVALID_CLK_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPM_INVALID_CLK_IE`"]
pub type LPM_INVALID_CLK_IE_R = crate::R<bool, LPM_INVALID_CLK_IE_A>;
impl LPM_INVALID_CLK_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_INVALID_CLK_IE_A {
        match self.bits {
            false => LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0,
            true => LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_CLK_IE_0`"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_0(&self) -> bool {
        *self == LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_CLK_IE_1`"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_1(&self) -> bool {
        *self == LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1
    }
}
#[doc = "Write proxy for field `LPM_INVALID_CLK_IE`"]
pub struct LPM_INVALID_CLK_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INVALID_CLK_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_INVALID_CLK_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_0(self) -> &'a mut W {
        self.variant(LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_1(self) -> &'a mut W {
        self.variant(LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1)
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
#[doc = "Active mode invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM_INVALID_TR_IE_A {
    #[doc = "0: Disabled"]
    AM_INVALID_TR_IE_0 = 0,
    #[doc = "1: Enabled"]
    AM_INVALID_TR_IE_1 = 1,
}
impl From<AM_INVALID_TR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: AM_INVALID_TR_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AM_INVALID_TR_IE`"]
pub type AM_INVALID_TR_IE_R = crate::R<bool, AM_INVALID_TR_IE_A>;
impl AM_INVALID_TR_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM_INVALID_TR_IE_A {
        match self.bits {
            false => AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0,
            true => AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM_INVALID_TR_IE_0`"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_0(&self) -> bool {
        *self == AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0
    }
    #[doc = "Checks if the value of the field is `AM_INVALID_TR_IE_1`"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_1(&self) -> bool {
        *self == AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1
    }
}
#[doc = "Write proxy for field `AM_INVALID_TR_IE`"]
pub struct AM_INVALID_TR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_INVALID_TR_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM_INVALID_TR_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_0(self) -> &'a mut W {
        self.variant(AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_1(self) -> &'a mut W {
        self.variant(AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DC-DC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_ERROR_IE_A {
    #[doc = "0: Disabled"]
    DCDC_ERROR_IE_0 = 0,
    #[doc = "1: Enabled"]
    DCDC_ERROR_IE_1 = 1,
}
impl From<DCDC_ERROR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_ERROR_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCDC_ERROR_IE`"]
pub type DCDC_ERROR_IE_R = crate::R<bool, DCDC_ERROR_IE_A>;
impl DCDC_ERROR_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_ERROR_IE_A {
        match self.bits {
            false => DCDC_ERROR_IE_A::DCDC_ERROR_IE_0,
            true => DCDC_ERROR_IE_A::DCDC_ERROR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_ERROR_IE_0`"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_0(&self) -> bool {
        *self == DCDC_ERROR_IE_A::DCDC_ERROR_IE_0
    }
    #[doc = "Checks if the value of the field is `DCDC_ERROR_IE_1`"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_1(&self) -> bool {
        *self == DCDC_ERROR_IE_A::DCDC_ERROR_IE_1
    }
}
#[doc = "Write proxy for field `DCDC_ERROR_IE`"]
pub struct DCDC_ERROR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_ERROR_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_ERROR_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_0(self) -> &'a mut W {
        self.variant(DCDC_ERROR_IE_A::DCDC_ERROR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_1(self) -> &'a mut W {
        self.variant(DCDC_ERROR_IE_A::DCDC_ERROR_IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&self) -> LPM_INVALID_TR_IE_R {
        LPM_INVALID_TR_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&self) -> LPM_INVALID_CLK_IE_R {
        LPM_INVALID_CLK_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&self) -> AM_INVALID_TR_IE_R {
        AM_INVALID_TR_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&self) -> DCDC_ERROR_IE_R {
        DCDC_ERROR_IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&mut self) -> LPM_INVALID_TR_IE_W {
        LPM_INVALID_TR_IE_W { w: self }
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&mut self) -> LPM_INVALID_CLK_IE_W {
        LPM_INVALID_CLK_IE_W { w: self }
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&mut self) -> AM_INVALID_TR_IE_W {
        AM_INVALID_TR_IE_W { w: self }
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&mut self) -> DCDC_ERROR_IE_W {
        DCDC_ERROR_IE_W { w: self }
    }
}
