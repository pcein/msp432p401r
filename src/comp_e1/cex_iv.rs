#[doc = "Reader of register CExIV"]
pub type R = crate::R<u16, super::CEXIV>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CEIV_A {
    #[doc = "0: No interrupt pending"]
    CEIV_0 = 0,
    #[doc = "2: Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    CEIV_2 = 2,
    #[doc = "4: Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    CEIV_4 = 4,
    #[doc = "10: Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    CEIV_10 = 10,
}
impl From<CEIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CEIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIV`"]
pub type CEIV_R = crate::R<u16, CEIV_A>;
impl CEIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CEIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEIV_A::CEIV_0),
            2 => Val(CEIV_A::CEIV_2),
            4 => Val(CEIV_A::CEIV_4),
            10 => Val(CEIV_A::CEIV_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CEIV_0`"]
    #[inline(always)]
    pub fn is_ceiv_0(&self) -> bool {
        *self == CEIV_A::CEIV_0
    }
    #[doc = "Checks if the value of the field is `CEIV_2`"]
    #[inline(always)]
    pub fn is_ceiv_2(&self) -> bool {
        *self == CEIV_A::CEIV_2
    }
    #[doc = "Checks if the value of the field is `CEIV_4`"]
    #[inline(always)]
    pub fn is_ceiv_4(&self) -> bool {
        *self == CEIV_A::CEIV_4
    }
    #[doc = "Checks if the value of the field is `CEIV_10`"]
    #[inline(always)]
    pub fn is_ceiv_10(&self) -> bool {
        *self == CEIV_A::CEIV_10
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&self) -> CEIV_R {
        CEIV_R::new((self.bits & 0xffff) as u16)
    }
}
