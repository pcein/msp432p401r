#[doc = "Reader of register TAxCTL"]
pub type R = crate::R<u16, super::TAXCTL>;
#[doc = "Writer for register TAxCTL"]
pub type W = crate::W<u16, super::TAXCTL>;
#[doc = "Register TAxCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TAXCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIFG_A {
    #[doc = "0: No interrupt pending"]
    TAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    TAIFG_1 = 1,
}
impl From<TAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: TAIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAIFG`"]
pub type TAIFG_R = crate::R<bool, TAIFG_A>;
impl TAIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIFG_A {
        match self.bits {
            false => TAIFG_A::TAIFG_0,
            true => TAIFG_A::TAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIFG_0`"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == TAIFG_A::TAIFG_0
    }
    #[doc = "Checks if the value of the field is `TAIFG_1`"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == TAIFG_A::TAIFG_1
    }
}
#[doc = "Write proxy for field `TAIFG`"]
pub struct TAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
    #[doc = "0: Interrupt disabled"]
    TAIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    TAIE_1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAIE`"]
pub type TAIE_R = crate::R<bool, TAIE_A>;
impl TAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::TAIE_0,
            true => TAIE_A::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == TAIE_A::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == TAIE_A::TAIE_1
    }
}
#[doc = "Write proxy for field `TAIE`"]
pub struct TAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TACLR`"]
pub type TACLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACLR`"]
pub struct TACLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Stop mode: Timer is halted"]
    MC_0 = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    MC_1 = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    MC_2 = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, MC_A>;
impl MC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == MC_A::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == MC_A::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == MC_A::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == MC_A::MC_3
    }
}
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MC_A::MC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: /1"]
    ID_0 = 0,
    #[doc = "1: /2"]
    ID_1 = 1,
    #[doc = "2: /4"]
    ID_2 = 2,
    #[doc = "3: /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == ID_A::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == ID_A::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == ID_A::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == ID_A::ID_3
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut W {
        self.variant(ID_A::ID_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut W {
        self.variant(ID_A::ID_1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut W {
        self.variant(ID_A::ID_2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut W {
        self.variant(ID_A::ID_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: TAxCLK"]
    TASSEL_0 = 0,
    #[doc = "1: ACLK"]
    TASSEL_1 = 1,
    #[doc = "2: SMCLK"]
    TASSEL_2 = 2,
    #[doc = "3: INCLK"]
    TASSEL_3 = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TASSEL`"]
pub type TASSEL_R = crate::R<u8, TASSEL_A>;
impl TASSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TASSEL_0,
            1 => TASSEL_A::TASSEL_1,
            2 => TASSEL_A::TASSEL_2,
            3 => TASSEL_A::TASSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TASSEL_0`"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        *self == TASSEL_A::TASSEL_0
    }
    #[doc = "Checks if the value of the field is `TASSEL_1`"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        *self == TASSEL_A::TASSEL_1
    }
    #[doc = "Checks if the value of the field is `TASSEL_2`"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        *self == TASSEL_A::TASSEL_2
    }
    #[doc = "Checks if the value of the field is `TASSEL_3`"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        *self == TASSEL_A::TASSEL_3
    }
}
#[doc = "Write proxy for field `TASSEL`"]
pub struct TASSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_1)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_2)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TAIFG_W {
        TAIFG_W { w: self }
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W {
        TAIE_W { w: self }
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TACLR_W {
        TACLR_W { w: self }
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TASSEL_W {
        TASSEL_W { w: self }
    }
}
