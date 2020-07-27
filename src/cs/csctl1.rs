#[doc = "Reader of register CSCTL1"]
pub type R = crate::R<u32, super::CSCTL1>;
#[doc = "Writer for register CSCTL1"]
pub type W = crate::W<u32, super::CSCTL1>;
#[doc = "Register CSCTL1 `reset()`'s with value 0x33"]
impl crate::ResetValue for super::CSCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33
    }
}
#[doc = "Selects the MCLK source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELM_A {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    SELM_0 = 0,
    #[doc = "1: `1`"]
    SELM_1 = 1,
    #[doc = "2: `10`"]
    SELM_2 = 2,
    #[doc = "3: `11`"]
    SELM_3 = 3,
    #[doc = "4: `100`"]
    SELM_4 = 4,
    #[doc = "5: when HFXT available, otherwise DCOCLK"]
    SELM_5 = 5,
    #[doc = "6: when HFXT2 available, otherwise DCOCLK"]
    SELM_6 = 6,
    #[doc = "7: for future use. Defaults to DCOCLK. Not recommended for use\r\r\nto ensure future compatibilities."]
    SELM_7 = 7,
}
impl From<SELM_A> for u8 {
    #[inline(always)]
    fn from(variant: SELM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELM`"]
pub type SELM_R = crate::R<u8, SELM_A>;
impl SELM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELM_A {
        match self.bits {
            0 => SELM_A::SELM_0,
            1 => SELM_A::SELM_1,
            2 => SELM_A::SELM_2,
            3 => SELM_A::SELM_3,
            4 => SELM_A::SELM_4,
            5 => SELM_A::SELM_5,
            6 => SELM_A::SELM_6,
            7 => SELM_A::SELM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELM_0`"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == SELM_A::SELM_0
    }
    #[doc = "Checks if the value of the field is `SELM_1`"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == SELM_A::SELM_1
    }
    #[doc = "Checks if the value of the field is `SELM_2`"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == SELM_A::SELM_2
    }
    #[doc = "Checks if the value of the field is `SELM_3`"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == SELM_A::SELM_3
    }
    #[doc = "Checks if the value of the field is `SELM_4`"]
    #[inline(always)]
    pub fn is_selm_4(&self) -> bool {
        *self == SELM_A::SELM_4
    }
    #[doc = "Checks if the value of the field is `SELM_5`"]
    #[inline(always)]
    pub fn is_selm_5(&self) -> bool {
        *self == SELM_A::SELM_5
    }
    #[doc = "Checks if the value of the field is `SELM_6`"]
    #[inline(always)]
    pub fn is_selm_6(&self) -> bool {
        *self == SELM_A::SELM_6
    }
    #[doc = "Checks if the value of the field is `SELM_7`"]
    #[inline(always)]
    pub fn is_selm_7(&self) -> bool {
        *self == SELM_A::SELM_7
    }
}
#[doc = "Write proxy for field `SELM`"]
pub struct SELM_W<'a> {
    w: &'a mut W,
}
impl<'a> SELM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut W {
        self.variant(SELM_A::SELM_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut W {
        self.variant(SELM_A::SELM_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut W {
        self.variant(SELM_A::SELM_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut W {
        self.variant(SELM_A::SELM_3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn selm_4(self) -> &'a mut W {
        self.variant(SELM_A::SELM_4)
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn selm_5(self) -> &'a mut W {
        self.variant(SELM_A::SELM_5)
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn selm_6(self) -> &'a mut W {
        self.variant(SELM_A::SELM_6)
    }
    #[doc = "for future use. Defaults to DCOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn selm_7(self) -> &'a mut W {
        self.variant(SELM_A::SELM_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects the SMCLK and HSMCLK source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    SELS_0 = 0,
    #[doc = "1: `1`"]
    SELS_1 = 1,
    #[doc = "2: `10`"]
    SELS_2 = 2,
    #[doc = "3: `11`"]
    SELS_3 = 3,
    #[doc = "4: `100`"]
    SELS_4 = 4,
    #[doc = "5: when HFXT available, otherwise DCOCLK"]
    SELS_5 = 5,
    #[doc = "6: when HFXT2 available, otherwise DCOCLK"]
    SELS_6 = 6,
    #[doc = "7: for furture use. Defaults to DCOCLK. Do not use to ensure future compatibilities."]
    SELS_7 = 7,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELS`"]
pub type SELS_R = crate::R<u8, SELS_A>;
impl SELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELS_A {
        match self.bits {
            0 => SELS_A::SELS_0,
            1 => SELS_A::SELS_1,
            2 => SELS_A::SELS_2,
            3 => SELS_A::SELS_3,
            4 => SELS_A::SELS_4,
            5 => SELS_A::SELS_5,
            6 => SELS_A::SELS_6,
            7 => SELS_A::SELS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELS_0`"]
    #[inline(always)]
    pub fn is_sels_0(&self) -> bool {
        *self == SELS_A::SELS_0
    }
    #[doc = "Checks if the value of the field is `SELS_1`"]
    #[inline(always)]
    pub fn is_sels_1(&self) -> bool {
        *self == SELS_A::SELS_1
    }
    #[doc = "Checks if the value of the field is `SELS_2`"]
    #[inline(always)]
    pub fn is_sels_2(&self) -> bool {
        *self == SELS_A::SELS_2
    }
    #[doc = "Checks if the value of the field is `SELS_3`"]
    #[inline(always)]
    pub fn is_sels_3(&self) -> bool {
        *self == SELS_A::SELS_3
    }
    #[doc = "Checks if the value of the field is `SELS_4`"]
    #[inline(always)]
    pub fn is_sels_4(&self) -> bool {
        *self == SELS_A::SELS_4
    }
    #[doc = "Checks if the value of the field is `SELS_5`"]
    #[inline(always)]
    pub fn is_sels_5(&self) -> bool {
        *self == SELS_A::SELS_5
    }
    #[doc = "Checks if the value of the field is `SELS_6`"]
    #[inline(always)]
    pub fn is_sels_6(&self) -> bool {
        *self == SELS_A::SELS_6
    }
    #[doc = "Checks if the value of the field is `SELS_7`"]
    #[inline(always)]
    pub fn is_sels_7(&self) -> bool {
        *self == SELS_A::SELS_7
    }
}
#[doc = "Write proxy for field `SELS`"]
pub struct SELS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn sels_0(self) -> &'a mut W {
        self.variant(SELS_A::SELS_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sels_1(self) -> &'a mut W {
        self.variant(SELS_A::SELS_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sels_2(self) -> &'a mut W {
        self.variant(SELS_A::SELS_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sels_3(self) -> &'a mut W {
        self.variant(SELS_A::SELS_3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sels_4(self) -> &'a mut W {
        self.variant(SELS_A::SELS_4)
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn sels_5(self) -> &'a mut W {
        self.variant(SELS_A::SELS_5)
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn sels_6(self) -> &'a mut W {
        self.variant(SELS_A::SELS_6)
    }
    #[doc = "for furture use. Defaults to DCOCLK. Do not use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sels_7(self) -> &'a mut W {
        self.variant(SELS_A::SELS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELA_A {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    SELA_0 = 0,
    #[doc = "1: `1`"]
    SELA_1 = 1,
    #[doc = "2: `10`"]
    SELA_2 = 2,
    #[doc = "3: for future use. Defaults to REFOCLK. Not recommended\r\r\nfor use to ensure future compatibilities."]
    SELA_3 = 3,
    #[doc = "4: for future use. Defaults to REFOCLK. Not recommended\r\r\nfor use to ensure future compatibilities."]
    SELA_4 = 4,
    #[doc = "5: for future use. Defaults to REFOCLK. Not recommended\r\r\nfor use to ensure future compatibilities."]
    SELA_5 = 5,
    #[doc = "6: for future use. Defaults to REFOCLK. Not recommended\r\r\nfor use to ensure future compatibilities."]
    SELA_6 = 6,
    #[doc = "7: for future use. Defaults to REFOCLK. Not recommended\r\r\nfor use to ensure future compatibilities."]
    SELA_7 = 7,
}
impl From<SELA_A> for u8 {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELA`"]
pub type SELA_R = crate::R<u8, SELA_A>;
impl SELA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELA_A {
        match self.bits {
            0 => SELA_A::SELA_0,
            1 => SELA_A::SELA_1,
            2 => SELA_A::SELA_2,
            3 => SELA_A::SELA_3,
            4 => SELA_A::SELA_4,
            5 => SELA_A::SELA_5,
            6 => SELA_A::SELA_6,
            7 => SELA_A::SELA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELA_0`"]
    #[inline(always)]
    pub fn is_sela_0(&self) -> bool {
        *self == SELA_A::SELA_0
    }
    #[doc = "Checks if the value of the field is `SELA_1`"]
    #[inline(always)]
    pub fn is_sela_1(&self) -> bool {
        *self == SELA_A::SELA_1
    }
    #[doc = "Checks if the value of the field is `SELA_2`"]
    #[inline(always)]
    pub fn is_sela_2(&self) -> bool {
        *self == SELA_A::SELA_2
    }
    #[doc = "Checks if the value of the field is `SELA_3`"]
    #[inline(always)]
    pub fn is_sela_3(&self) -> bool {
        *self == SELA_A::SELA_3
    }
    #[doc = "Checks if the value of the field is `SELA_4`"]
    #[inline(always)]
    pub fn is_sela_4(&self) -> bool {
        *self == SELA_A::SELA_4
    }
    #[doc = "Checks if the value of the field is `SELA_5`"]
    #[inline(always)]
    pub fn is_sela_5(&self) -> bool {
        *self == SELA_A::SELA_5
    }
    #[doc = "Checks if the value of the field is `SELA_6`"]
    #[inline(always)]
    pub fn is_sela_6(&self) -> bool {
        *self == SELA_A::SELA_6
    }
    #[doc = "Checks if the value of the field is `SELA_7`"]
    #[inline(always)]
    pub fn is_sela_7(&self) -> bool {
        *self == SELA_A::SELA_7
    }
}
#[doc = "Write proxy for field `SELA`"]
pub struct SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> SELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn sela_0(self) -> &'a mut W {
        self.variant(SELA_A::SELA_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sela_1(self) -> &'a mut W {
        self.variant(SELA_A::SELA_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sela_2(self) -> &'a mut W {
        self.variant(SELA_A::SELA_2)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_3(self) -> &'a mut W {
        self.variant(SELA_A::SELA_3)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_4(self) -> &'a mut W {
        self.variant(SELA_A::SELA_4)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_5(self) -> &'a mut W {
        self.variant(SELA_A::SELA_5)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_6(self) -> &'a mut W {
        self.variant(SELA_A::SELA_6)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_7(self) -> &'a mut W {
        self.variant(SELA_A::SELA_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Selects the BCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELB_A {
    #[doc = "0: LFXTCLK"]
    SELB_0 = 0,
    #[doc = "1: REFOCLK"]
    SELB_1 = 1,
}
impl From<SELB_A> for bool {
    #[inline(always)]
    fn from(variant: SELB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELB`"]
pub type SELB_R = crate::R<bool, SELB_A>;
impl SELB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELB_A {
        match self.bits {
            false => SELB_A::SELB_0,
            true => SELB_A::SELB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SELB_0`"]
    #[inline(always)]
    pub fn is_selb_0(&self) -> bool {
        *self == SELB_A::SELB_0
    }
    #[doc = "Checks if the value of the field is `SELB_1`"]
    #[inline(always)]
    pub fn is_selb_1(&self) -> bool {
        *self == SELB_A::SELB_1
    }
}
#[doc = "Write proxy for field `SELB`"]
pub struct SELB_W<'a> {
    w: &'a mut W,
}
impl<'a> SELB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LFXTCLK"]
    #[inline(always)]
    pub fn selb_0(self) -> &'a mut W {
        self.variant(SELB_A::SELB_0)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn selb_1(self) -> &'a mut W {
        self.variant(SELB_A::SELB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "MCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: f(MCLK)/1"]
    DIVM_0 = 0,
    #[doc = "1: f(MCLK)/2"]
    DIVM_1 = 1,
    #[doc = "2: f(MCLK)/4"]
    DIVM_2 = 2,
    #[doc = "3: f(MCLK)/8"]
    DIVM_3 = 3,
    #[doc = "4: f(MCLK)/16"]
    DIVM_4 = 4,
    #[doc = "5: f(MCLK)/32"]
    DIVM_5 = 5,
    #[doc = "6: f(MCLK)/64"]
    DIVM_6 = 6,
    #[doc = "7: f(MCLK)/128"]
    DIVM_7 = 7,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVM`"]
pub type DIVM_R = crate::R<u8, DIVM_A>;
impl DIVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::DIVM_0,
            1 => DIVM_A::DIVM_1,
            2 => DIVM_A::DIVM_2,
            3 => DIVM_A::DIVM_3,
            4 => DIVM_A::DIVM_4,
            5 => DIVM_A::DIVM_5,
            6 => DIVM_A::DIVM_6,
            7 => DIVM_A::DIVM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVM_0`"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVM_A::DIVM_0
    }
    #[doc = "Checks if the value of the field is `DIVM_1`"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVM_A::DIVM_1
    }
    #[doc = "Checks if the value of the field is `DIVM_2`"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVM_A::DIVM_2
    }
    #[doc = "Checks if the value of the field is `DIVM_3`"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVM_A::DIVM_3
    }
    #[doc = "Checks if the value of the field is `DIVM_4`"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == DIVM_A::DIVM_4
    }
    #[doc = "Checks if the value of the field is `DIVM_5`"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == DIVM_A::DIVM_5
    }
    #[doc = "Checks if the value of the field is `DIVM_6`"]
    #[inline(always)]
    pub fn is_divm_6(&self) -> bool {
        *self == DIVM_A::DIVM_6
    }
    #[doc = "Checks if the value of the field is `DIVM_7`"]
    #[inline(always)]
    pub fn is_divm_7(&self) -> bool {
        *self == DIVM_A::DIVM_7
    }
}
#[doc = "Write proxy for field `DIVM`"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f(MCLK)/1"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "f(MCLK)/2"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "f(MCLK)/4"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "f(MCLK)/8"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_3)
    }
    #[doc = "f(MCLK)/16"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_4)
    }
    #[doc = "f(MCLK)/32"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_5)
    }
    #[doc = "f(MCLK)/64"]
    #[inline(always)]
    pub fn divm_6(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_6)
    }
    #[doc = "f(MCLK)/128"]
    #[inline(always)]
    pub fn divm_7(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "HSMCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVHS_A {
    #[doc = "0: f(HSMCLK)/1"]
    DIVHS_0 = 0,
    #[doc = "1: f(HSMCLK)/2"]
    DIVHS_1 = 1,
    #[doc = "2: f(HSMCLK)/4"]
    DIVHS_2 = 2,
    #[doc = "3: f(HSMCLK)/8"]
    DIVHS_3 = 3,
    #[doc = "4: f(HSMCLK)/16"]
    DIVHS_4 = 4,
    #[doc = "5: f(HSMCLK)/32"]
    DIVHS_5 = 5,
    #[doc = "6: f(HSMCLK)/64"]
    DIVHS_6 = 6,
    #[doc = "7: f(HSMCLK)/128"]
    DIVHS_7 = 7,
}
impl From<DIVHS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVHS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVHS`"]
pub type DIVHS_R = crate::R<u8, DIVHS_A>;
impl DIVHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVHS_A {
        match self.bits {
            0 => DIVHS_A::DIVHS_0,
            1 => DIVHS_A::DIVHS_1,
            2 => DIVHS_A::DIVHS_2,
            3 => DIVHS_A::DIVHS_3,
            4 => DIVHS_A::DIVHS_4,
            5 => DIVHS_A::DIVHS_5,
            6 => DIVHS_A::DIVHS_6,
            7 => DIVHS_A::DIVHS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVHS_0`"]
    #[inline(always)]
    pub fn is_divhs_0(&self) -> bool {
        *self == DIVHS_A::DIVHS_0
    }
    #[doc = "Checks if the value of the field is `DIVHS_1`"]
    #[inline(always)]
    pub fn is_divhs_1(&self) -> bool {
        *self == DIVHS_A::DIVHS_1
    }
    #[doc = "Checks if the value of the field is `DIVHS_2`"]
    #[inline(always)]
    pub fn is_divhs_2(&self) -> bool {
        *self == DIVHS_A::DIVHS_2
    }
    #[doc = "Checks if the value of the field is `DIVHS_3`"]
    #[inline(always)]
    pub fn is_divhs_3(&self) -> bool {
        *self == DIVHS_A::DIVHS_3
    }
    #[doc = "Checks if the value of the field is `DIVHS_4`"]
    #[inline(always)]
    pub fn is_divhs_4(&self) -> bool {
        *self == DIVHS_A::DIVHS_4
    }
    #[doc = "Checks if the value of the field is `DIVHS_5`"]
    #[inline(always)]
    pub fn is_divhs_5(&self) -> bool {
        *self == DIVHS_A::DIVHS_5
    }
    #[doc = "Checks if the value of the field is `DIVHS_6`"]
    #[inline(always)]
    pub fn is_divhs_6(&self) -> bool {
        *self == DIVHS_A::DIVHS_6
    }
    #[doc = "Checks if the value of the field is `DIVHS_7`"]
    #[inline(always)]
    pub fn is_divhs_7(&self) -> bool {
        *self == DIVHS_A::DIVHS_7
    }
}
#[doc = "Write proxy for field `DIVHS`"]
pub struct DIVHS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVHS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f(HSMCLK)/1"]
    #[inline(always)]
    pub fn divhs_0(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_0)
    }
    #[doc = "f(HSMCLK)/2"]
    #[inline(always)]
    pub fn divhs_1(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_1)
    }
    #[doc = "f(HSMCLK)/4"]
    #[inline(always)]
    pub fn divhs_2(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_2)
    }
    #[doc = "f(HSMCLK)/8"]
    #[inline(always)]
    pub fn divhs_3(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_3)
    }
    #[doc = "f(HSMCLK)/16"]
    #[inline(always)]
    pub fn divhs_4(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_4)
    }
    #[doc = "f(HSMCLK)/32"]
    #[inline(always)]
    pub fn divhs_5(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_5)
    }
    #[doc = "f(HSMCLK)/64"]
    #[inline(always)]
    pub fn divhs_6(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_6)
    }
    #[doc = "f(HSMCLK)/128"]
    #[inline(always)]
    pub fn divhs_7(self) -> &'a mut W {
        self.variant(DIVHS_A::DIVHS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "ACLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: f(ACLK)/1"]
    DIVA_0 = 0,
    #[doc = "1: f(ACLK)/2"]
    DIVA_1 = 1,
    #[doc = "2: f(ACLK)/4"]
    DIVA_2 = 2,
    #[doc = "3: f(ACLK)/8"]
    DIVA_3 = 3,
    #[doc = "4: f(ACLK)/16"]
    DIVA_4 = 4,
    #[doc = "5: f(ACLK)/32"]
    DIVA_5 = 5,
    #[doc = "6: f(ACLK)/64"]
    DIVA_6 = 6,
    #[doc = "7: f(ACLK)/128"]
    DIVA_7 = 7,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::DIVA_0,
            1 => DIVA_A::DIVA_1,
            2 => DIVA_A::DIVA_2,
            3 => DIVA_A::DIVA_3,
            4 => DIVA_A::DIVA_4,
            5 => DIVA_A::DIVA_5,
            6 => DIVA_A::DIVA_6,
            7 => DIVA_A::DIVA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVA_0`"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "Checks if the value of the field is `DIVA_1`"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "Checks if the value of the field is `DIVA_2`"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "Checks if the value of the field is `DIVA_3`"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
    #[doc = "Checks if the value of the field is `DIVA_4`"]
    #[inline(always)]
    pub fn is_diva_4(&self) -> bool {
        *self == DIVA_A::DIVA_4
    }
    #[doc = "Checks if the value of the field is `DIVA_5`"]
    #[inline(always)]
    pub fn is_diva_5(&self) -> bool {
        *self == DIVA_A::DIVA_5
    }
    #[doc = "Checks if the value of the field is `DIVA_6`"]
    #[inline(always)]
    pub fn is_diva_6(&self) -> bool {
        *self == DIVA_A::DIVA_6
    }
    #[doc = "Checks if the value of the field is `DIVA_7`"]
    #[inline(always)]
    pub fn is_diva_7(&self) -> bool {
        *self == DIVA_A::DIVA_7
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f(ACLK)/1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "f(ACLK)/2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "f(ACLK)/4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "f(ACLK)/8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_3)
    }
    #[doc = "f(ACLK)/16"]
    #[inline(always)]
    pub fn diva_4(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_4)
    }
    #[doc = "f(ACLK)/32"]
    #[inline(always)]
    pub fn diva_5(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_5)
    }
    #[doc = "f(ACLK)/64"]
    #[inline(always)]
    pub fn diva_6(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_6)
    }
    #[doc = "f(ACLK)/128"]
    #[inline(always)]
    pub fn diva_7(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "SMCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: f(SMCLK)/1"]
    DIVS_0 = 0,
    #[doc = "1: f(SMCLK)/2"]
    DIVS_1 = 1,
    #[doc = "2: f(SMCLK)/4"]
    DIVS_2 = 2,
    #[doc = "3: f(SMCLK)/8"]
    DIVS_3 = 3,
    #[doc = "4: f(SMCLK)/16"]
    DIVS_4 = 4,
    #[doc = "5: f(SMCLK)/32"]
    DIVS_5 = 5,
    #[doc = "6: f(SMCLK)/64"]
    DIVS_6 = 6,
    #[doc = "7: f(SMCLK)/128"]
    DIVS_7 = 7,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVS`"]
pub type DIVS_R = crate::R<u8, DIVS_A>;
impl DIVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::DIVS_0,
            1 => DIVS_A::DIVS_1,
            2 => DIVS_A::DIVS_2,
            3 => DIVS_A::DIVS_3,
            4 => DIVS_A::DIVS_4,
            5 => DIVS_A::DIVS_5,
            6 => DIVS_A::DIVS_6,
            7 => DIVS_A::DIVS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVS_0`"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVS_A::DIVS_0
    }
    #[doc = "Checks if the value of the field is `DIVS_1`"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVS_A::DIVS_1
    }
    #[doc = "Checks if the value of the field is `DIVS_2`"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVS_A::DIVS_2
    }
    #[doc = "Checks if the value of the field is `DIVS_3`"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVS_A::DIVS_3
    }
    #[doc = "Checks if the value of the field is `DIVS_4`"]
    #[inline(always)]
    pub fn is_divs_4(&self) -> bool {
        *self == DIVS_A::DIVS_4
    }
    #[doc = "Checks if the value of the field is `DIVS_5`"]
    #[inline(always)]
    pub fn is_divs_5(&self) -> bool {
        *self == DIVS_A::DIVS_5
    }
    #[doc = "Checks if the value of the field is `DIVS_6`"]
    #[inline(always)]
    pub fn is_divs_6(&self) -> bool {
        *self == DIVS_A::DIVS_6
    }
    #[doc = "Checks if the value of the field is `DIVS_7`"]
    #[inline(always)]
    pub fn is_divs_7(&self) -> bool {
        *self == DIVS_A::DIVS_7
    }
}
#[doc = "Write proxy for field `DIVS`"]
pub struct DIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f(SMCLK)/1"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "f(SMCLK)/2"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "f(SMCLK)/4"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "f(SMCLK)/8"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_3)
    }
    #[doc = "f(SMCLK)/16"]
    #[inline(always)]
    pub fn divs_4(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_4)
    }
    #[doc = "f(SMCLK)/32"]
    #[inline(always)]
    pub fn divs_5(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_5)
    }
    #[doc = "f(SMCLK)/64"]
    #[inline(always)]
    pub fn divs_6(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_6)
    }
    #[doc = "f(SMCLK)/128"]
    #[inline(always)]
    pub fn divs_7(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selects the SMCLK and HSMCLK source"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Selects the BCLK source"]
    #[inline(always)]
    pub fn selb(&self) -> SELB_R {
        SELB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - HSMCLK source divider"]
    #[inline(always)]
    pub fn divhs(&self) -> DIVHS_R {
        DIVHS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&mut self) -> SELM_W {
        SELM_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects the SMCLK and HSMCLK source"]
    #[inline(always)]
    pub fn sels(&mut self) -> SELS_W {
        SELS_W { w: self }
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W {
        SELA_W { w: self }
    }
    #[doc = "Bit 12 - Selects the BCLK source"]
    #[inline(always)]
    pub fn selb(&mut self) -> SELB_W {
        SELB_W { w: self }
    }
    #[doc = "Bits 16:18 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bits 20:22 - HSMCLK source divider"]
    #[inline(always)]
    pub fn divhs(&mut self) -> DIVHS_W {
        DIVHS_W { w: self }
    }
    #[doc = "Bits 24:26 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 28:30 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W { w: self }
    }
}
