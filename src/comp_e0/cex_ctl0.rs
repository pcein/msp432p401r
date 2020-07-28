#[doc = "Reader of register CExCTL0"]
pub type R = crate::R<u16, super::CEXCTL0>;
#[doc = "Writer for register CExCTL0"]
pub type W = crate::W<u16, super::CEXCTL0>;
#[doc = "Register CExCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CEXCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel input selected for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEIPSEL_A {
    #[doc = "0: Channel 0 selected"]
    CEIPSEL_0 = 0,
    #[doc = "1: Channel 1 selected"]
    CEIPSEL_1 = 1,
    #[doc = "2: Channel 2 selected"]
    CEIPSEL_2 = 2,
    #[doc = "3: Channel 3 selected"]
    CEIPSEL_3 = 3,
    #[doc = "4: Channel 4 selected"]
    CEIPSEL_4 = 4,
    #[doc = "5: Channel 5 selected"]
    CEIPSEL_5 = 5,
    #[doc = "6: Channel 6 selected"]
    CEIPSEL_6 = 6,
    #[doc = "7: Channel 7 selected"]
    CEIPSEL_7 = 7,
    #[doc = "8: Channel 8 selected"]
    CEIPSEL_8 = 8,
    #[doc = "9: Channel 9 selected"]
    CEIPSEL_9 = 9,
    #[doc = "10: Channel 10 selected"]
    CEIPSEL_10 = 10,
    #[doc = "11: Channel 11 selected"]
    CEIPSEL_11 = 11,
    #[doc = "12: Channel 12 selected"]
    CEIPSEL_12 = 12,
    #[doc = "13: Channel 13 selected"]
    CEIPSEL_13 = 13,
    #[doc = "14: Channel 14 selected"]
    CEIPSEL_14 = 14,
    #[doc = "15: Channel 15 selected"]
    CEIPSEL_15 = 15,
}
impl From<CEIPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEIPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIPSEL`"]
pub type CEIPSEL_R = crate::R<u8, CEIPSEL_A>;
impl CEIPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIPSEL_A {
        match self.bits {
            0 => CEIPSEL_A::CEIPSEL_0,
            1 => CEIPSEL_A::CEIPSEL_1,
            2 => CEIPSEL_A::CEIPSEL_2,
            3 => CEIPSEL_A::CEIPSEL_3,
            4 => CEIPSEL_A::CEIPSEL_4,
            5 => CEIPSEL_A::CEIPSEL_5,
            6 => CEIPSEL_A::CEIPSEL_6,
            7 => CEIPSEL_A::CEIPSEL_7,
            8 => CEIPSEL_A::CEIPSEL_8,
            9 => CEIPSEL_A::CEIPSEL_9,
            10 => CEIPSEL_A::CEIPSEL_10,
            11 => CEIPSEL_A::CEIPSEL_11,
            12 => CEIPSEL_A::CEIPSEL_12,
            13 => CEIPSEL_A::CEIPSEL_13,
            14 => CEIPSEL_A::CEIPSEL_14,
            15 => CEIPSEL_A::CEIPSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_0`"]
    #[inline(always)]
    pub fn is_ceipsel_0(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_0
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_1`"]
    #[inline(always)]
    pub fn is_ceipsel_1(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_1
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_2`"]
    #[inline(always)]
    pub fn is_ceipsel_2(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_2
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_3`"]
    #[inline(always)]
    pub fn is_ceipsel_3(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_3
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_4`"]
    #[inline(always)]
    pub fn is_ceipsel_4(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_4
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_5`"]
    #[inline(always)]
    pub fn is_ceipsel_5(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_5
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_6`"]
    #[inline(always)]
    pub fn is_ceipsel_6(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_6
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_7`"]
    #[inline(always)]
    pub fn is_ceipsel_7(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_7
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_8`"]
    #[inline(always)]
    pub fn is_ceipsel_8(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_8
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_9`"]
    #[inline(always)]
    pub fn is_ceipsel_9(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_9
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_10`"]
    #[inline(always)]
    pub fn is_ceipsel_10(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_10
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_11`"]
    #[inline(always)]
    pub fn is_ceipsel_11(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_11
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_12`"]
    #[inline(always)]
    pub fn is_ceipsel_12(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_12
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_13`"]
    #[inline(always)]
    pub fn is_ceipsel_13(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_13
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_14`"]
    #[inline(always)]
    pub fn is_ceipsel_14(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_14
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_15`"]
    #[inline(always)]
    pub fn is_ceipsel_15(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_15
    }
}
#[doc = "Write proxy for field `CEIPSEL`"]
pub struct CEIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIPSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ceipsel_0(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ceipsel_1(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ceipsel_2(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ceipsel_3(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ceipsel_4(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ceipsel_5(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ceipsel_6(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ceipsel_7(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_7)
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn ceipsel_8(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_8)
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn ceipsel_9(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_9)
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn ceipsel_10(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_10)
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn ceipsel_11(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_11)
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn ceipsel_12(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_12)
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn ceipsel_13(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_13)
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn ceipsel_14(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_14)
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn ceipsel_15(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Channel input enable for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIPEN_A {
    #[doc = "0: Selected analog input channel for V+ terminal is disabled"]
    CEIPEN_0 = 0,
    #[doc = "1: Selected analog input channel for V+ terminal is enabled"]
    CEIPEN_1 = 1,
}
impl From<CEIPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEIPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEIPEN`"]
pub type CEIPEN_R = crate::R<bool, CEIPEN_A>;
impl CEIPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIPEN_A {
        match self.bits {
            false => CEIPEN_A::CEIPEN_0,
            true => CEIPEN_A::CEIPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIPEN_0`"]
    #[inline(always)]
    pub fn is_ceipen_0(&self) -> bool {
        *self == CEIPEN_A::CEIPEN_0
    }
    #[doc = "Checks if the value of the field is `CEIPEN_1`"]
    #[inline(always)]
    pub fn is_ceipen_1(&self) -> bool {
        *self == CEIPEN_A::CEIPEN_1
    }
}
#[doc = "Write proxy for field `CEIPEN`"]
pub struct CEIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected analog input channel for V+ terminal is disabled"]
    #[inline(always)]
    pub fn ceipen_0(self) -> &'a mut W {
        self.variant(CEIPEN_A::CEIPEN_0)
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled"]
    #[inline(always)]
    pub fn ceipen_1(self) -> &'a mut W {
        self.variant(CEIPEN_A::CEIPEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Channel input selected for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEIMSEL_A {
    #[doc = "0: Channel 0 selected"]
    CEIMSEL_0 = 0,
    #[doc = "1: Channel 1 selected"]
    CEIMSEL_1 = 1,
    #[doc = "2: Channel 2 selected"]
    CEIMSEL_2 = 2,
    #[doc = "3: Channel 3 selected"]
    CEIMSEL_3 = 3,
    #[doc = "4: Channel 4 selected"]
    CEIMSEL_4 = 4,
    #[doc = "5: Channel 5 selected"]
    CEIMSEL_5 = 5,
    #[doc = "6: Channel 6 selected"]
    CEIMSEL_6 = 6,
    #[doc = "7: Channel 7 selected"]
    CEIMSEL_7 = 7,
    #[doc = "8: Channel 8 selected"]
    CEIMSEL_8 = 8,
    #[doc = "9: Channel 9 selected"]
    CEIMSEL_9 = 9,
    #[doc = "10: Channel 10 selected"]
    CEIMSEL_10 = 10,
    #[doc = "11: Channel 11 selected"]
    CEIMSEL_11 = 11,
    #[doc = "12: Channel 12 selected"]
    CEIMSEL_12 = 12,
    #[doc = "13: Channel 13 selected"]
    CEIMSEL_13 = 13,
    #[doc = "14: Channel 14 selected"]
    CEIMSEL_14 = 14,
    #[doc = "15: Channel 15 selected"]
    CEIMSEL_15 = 15,
}
impl From<CEIMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEIMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIMSEL`"]
pub type CEIMSEL_R = crate::R<u8, CEIMSEL_A>;
impl CEIMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIMSEL_A {
        match self.bits {
            0 => CEIMSEL_A::CEIMSEL_0,
            1 => CEIMSEL_A::CEIMSEL_1,
            2 => CEIMSEL_A::CEIMSEL_2,
            3 => CEIMSEL_A::CEIMSEL_3,
            4 => CEIMSEL_A::CEIMSEL_4,
            5 => CEIMSEL_A::CEIMSEL_5,
            6 => CEIMSEL_A::CEIMSEL_6,
            7 => CEIMSEL_A::CEIMSEL_7,
            8 => CEIMSEL_A::CEIMSEL_8,
            9 => CEIMSEL_A::CEIMSEL_9,
            10 => CEIMSEL_A::CEIMSEL_10,
            11 => CEIMSEL_A::CEIMSEL_11,
            12 => CEIMSEL_A::CEIMSEL_12,
            13 => CEIMSEL_A::CEIMSEL_13,
            14 => CEIMSEL_A::CEIMSEL_14,
            15 => CEIMSEL_A::CEIMSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_0`"]
    #[inline(always)]
    pub fn is_ceimsel_0(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_0
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_1`"]
    #[inline(always)]
    pub fn is_ceimsel_1(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_1
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_2`"]
    #[inline(always)]
    pub fn is_ceimsel_2(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_2
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_3`"]
    #[inline(always)]
    pub fn is_ceimsel_3(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_3
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_4`"]
    #[inline(always)]
    pub fn is_ceimsel_4(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_4
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_5`"]
    #[inline(always)]
    pub fn is_ceimsel_5(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_5
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_6`"]
    #[inline(always)]
    pub fn is_ceimsel_6(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_6
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_7`"]
    #[inline(always)]
    pub fn is_ceimsel_7(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_7
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_8`"]
    #[inline(always)]
    pub fn is_ceimsel_8(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_8
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_9`"]
    #[inline(always)]
    pub fn is_ceimsel_9(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_9
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_10`"]
    #[inline(always)]
    pub fn is_ceimsel_10(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_10
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_11`"]
    #[inline(always)]
    pub fn is_ceimsel_11(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_11
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_12`"]
    #[inline(always)]
    pub fn is_ceimsel_12(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_12
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_13`"]
    #[inline(always)]
    pub fn is_ceimsel_13(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_13
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_14`"]
    #[inline(always)]
    pub fn is_ceimsel_14(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_14
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_15`"]
    #[inline(always)]
    pub fn is_ceimsel_15(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_15
    }
}
#[doc = "Write proxy for field `CEIMSEL`"]
pub struct CEIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIMSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ceimsel_0(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ceimsel_1(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ceimsel_2(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ceimsel_3(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ceimsel_4(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ceimsel_5(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ceimsel_6(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ceimsel_7(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_7)
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn ceimsel_8(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_8)
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn ceimsel_9(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_9)
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn ceimsel_10(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_10)
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn ceimsel_11(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_11)
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn ceimsel_12(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_12)
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn ceimsel_13(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_13)
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn ceimsel_14(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_14)
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn ceimsel_15(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Channel input enable for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIMEN_A {
    #[doc = "0: Selected analog input channel for V- terminal is disabled"]
    CEIMEN_0 = 0,
    #[doc = "1: Selected analog input channel for V- terminal is enabled"]
    CEIMEN_1 = 1,
}
impl From<CEIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEIMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEIMEN`"]
pub type CEIMEN_R = crate::R<bool, CEIMEN_A>;
impl CEIMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIMEN_A {
        match self.bits {
            false => CEIMEN_A::CEIMEN_0,
            true => CEIMEN_A::CEIMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIMEN_0`"]
    #[inline(always)]
    pub fn is_ceimen_0(&self) -> bool {
        *self == CEIMEN_A::CEIMEN_0
    }
    #[doc = "Checks if the value of the field is `CEIMEN_1`"]
    #[inline(always)]
    pub fn is_ceimen_1(&self) -> bool {
        *self == CEIMEN_A::CEIMEN_1
    }
}
#[doc = "Write proxy for field `CEIMEN`"]
pub struct CEIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected analog input channel for V- terminal is disabled"]
    #[inline(always)]
    pub fn ceimen_0(self) -> &'a mut W {
        self.variant(CEIMEN_A::CEIMEN_0)
    }
    #[doc = "Selected analog input channel for V- terminal is enabled"]
    #[inline(always)]
    pub fn ceimen_1(self) -> &'a mut W {
        self.variant(CEIMEN_A::CEIMEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn ceipsel(&self) -> CEIPSEL_R {
        CEIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn ceipen(&self) -> CEIPEN_R {
        CEIPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn ceimsel(&self) -> CEIMSEL_R {
        CEIMSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn ceimen(&self) -> CEIMEN_R {
        CEIMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn ceipsel(&mut self) -> CEIPSEL_W {
        CEIPSEL_W { w: self }
    }
    #[doc = "Bit 7 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn ceipen(&mut self) -> CEIPEN_W {
        CEIPEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn ceimsel(&mut self) -> CEIMSEL_W {
        CEIMSEL_W { w: self }
    }
    #[doc = "Bit 15 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn ceimen(&mut self) -> CEIMEN_W {
        CEIMEN_W { w: self }
    }
}
