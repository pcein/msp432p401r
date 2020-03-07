#[doc = "Reader of register UCAxIV"]
pub type R = crate::R<u16, super::UCAXIV>;
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    UCIV_0 = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCIV_2 = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCIV_4 = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCIV_6 = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCIV_8 = 8,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCIV`"]
pub type UCIV_R = crate::R<u16, UCIV_A>;
impl UCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UCIV_A::UCIV_0),
            2 => Val(UCIV_A::UCIV_2),
            4 => Val(UCIV_A::UCIV_4),
            6 => Val(UCIV_A::UCIV_6),
            8 => Val(UCIV_A::UCIV_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UCIV_0`"]
    #[inline(always)]
    pub fn is_uciv_0(&self) -> bool {
        *self == UCIV_A::UCIV_0
    }
    #[doc = "Checks if the value of the field is `UCIV_2`"]
    #[inline(always)]
    pub fn is_uciv_2(&self) -> bool {
        *self == UCIV_A::UCIV_2
    }
    #[doc = "Checks if the value of the field is `UCIV_4`"]
    #[inline(always)]
    pub fn is_uciv_4(&self) -> bool {
        *self == UCIV_A::UCIV_4
    }
    #[doc = "Checks if the value of the field is `UCIV_6`"]
    #[inline(always)]
    pub fn is_uciv_6(&self) -> bool {
        *self == UCIV_A::UCIV_6
    }
    #[doc = "Checks if the value of the field is `UCIV_8`"]
    #[inline(always)]
    pub fn is_uciv_8(&self) -> bool {
        *self == UCIV_A::UCIV_8
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
