#[doc = "Reader of register TAxEX0"]
pub type R = crate::R<u16, super::TAXEX0>;
#[doc = "Writer for register TAxEX0"]
pub type W = crate::W<u16, super::TAXEX0>;
#[doc = "Register TAxEX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TAXEX0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAIDEX_A {
    #[doc = "0: Divide by 1"]
    TAIDEX_0 = 0,
    #[doc = "1: Divide by 2"]
    TAIDEX_1 = 1,
    #[doc = "2: Divide by 3"]
    TAIDEX_2 = 2,
    #[doc = "3: Divide by 4"]
    TAIDEX_3 = 3,
    #[doc = "4: Divide by 5"]
    TAIDEX_4 = 4,
    #[doc = "5: Divide by 6"]
    TAIDEX_5 = 5,
    #[doc = "6: Divide by 7"]
    TAIDEX_6 = 6,
    #[doc = "7: Divide by 8"]
    TAIDEX_7 = 7,
}
impl From<TAIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAIDEX`"]
pub type TAIDEX_R = crate::R<u8, TAIDEX_A>;
impl TAIDEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIDEX_A {
        match self.bits {
            0 => TAIDEX_A::TAIDEX_0,
            1 => TAIDEX_A::TAIDEX_1,
            2 => TAIDEX_A::TAIDEX_2,
            3 => TAIDEX_A::TAIDEX_3,
            4 => TAIDEX_A::TAIDEX_4,
            5 => TAIDEX_A::TAIDEX_5,
            6 => TAIDEX_A::TAIDEX_6,
            7 => TAIDEX_A::TAIDEX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TAIDEX_0`"]
    #[inline(always)]
    pub fn is_taidex_0(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_0
    }
    #[doc = "Checks if the value of the field is `TAIDEX_1`"]
    #[inline(always)]
    pub fn is_taidex_1(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_1
    }
    #[doc = "Checks if the value of the field is `TAIDEX_2`"]
    #[inline(always)]
    pub fn is_taidex_2(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_2
    }
    #[doc = "Checks if the value of the field is `TAIDEX_3`"]
    #[inline(always)]
    pub fn is_taidex_3(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_3
    }
    #[doc = "Checks if the value of the field is `TAIDEX_4`"]
    #[inline(always)]
    pub fn is_taidex_4(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_4
    }
    #[doc = "Checks if the value of the field is `TAIDEX_5`"]
    #[inline(always)]
    pub fn is_taidex_5(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_5
    }
    #[doc = "Checks if the value of the field is `TAIDEX_6`"]
    #[inline(always)]
    pub fn is_taidex_6(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_6
    }
    #[doc = "Checks if the value of the field is `TAIDEX_7`"]
    #[inline(always)]
    pub fn is_taidex_7(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_7
    }
}
#[doc = "Write proxy for field `TAIDEX`"]
pub struct TAIDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIDEX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn taidex_0(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn taidex_1(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn taidex_2(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn taidex_3(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn taidex_4(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn taidex_5(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn taidex_6(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn taidex_7(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&self) -> TAIDEX_R {
        TAIDEX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TAIDEX_W {
        TAIDEX_W { w: self }
    }
}
