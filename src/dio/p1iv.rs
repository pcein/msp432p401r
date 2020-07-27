#[doc = "Reader of register P1IV"]
pub type R = crate::R<u16, super::P1IV>;
#[doc = "Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1IV_A {
    #[doc = "0: No interrupt pending"]
    P1IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1IV_16 = 16,
}
impl From<P1IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P1IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1IV`"]
pub type P1IV_R = crate::R<u8, P1IV_A>;
impl P1IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P1IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P1IV_A::P1IV_0),
            2 => Val(P1IV_A::P1IV_2),
            4 => Val(P1IV_A::P1IV_4),
            6 => Val(P1IV_A::P1IV_6),
            8 => Val(P1IV_A::P1IV_8),
            10 => Val(P1IV_A::P1IV_10),
            12 => Val(P1IV_A::P1IV_12),
            14 => Val(P1IV_A::P1IV_14),
            16 => Val(P1IV_A::P1IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P1IV_0`"]
    #[inline(always)]
    pub fn is_p1iv_0(&self) -> bool {
        *self == P1IV_A::P1IV_0
    }
    #[doc = "Checks if the value of the field is `P1IV_2`"]
    #[inline(always)]
    pub fn is_p1iv_2(&self) -> bool {
        *self == P1IV_A::P1IV_2
    }
    #[doc = "Checks if the value of the field is `P1IV_4`"]
    #[inline(always)]
    pub fn is_p1iv_4(&self) -> bool {
        *self == P1IV_A::P1IV_4
    }
    #[doc = "Checks if the value of the field is `P1IV_6`"]
    #[inline(always)]
    pub fn is_p1iv_6(&self) -> bool {
        *self == P1IV_A::P1IV_6
    }
    #[doc = "Checks if the value of the field is `P1IV_8`"]
    #[inline(always)]
    pub fn is_p1iv_8(&self) -> bool {
        *self == P1IV_A::P1IV_8
    }
    #[doc = "Checks if the value of the field is `P1IV_10`"]
    #[inline(always)]
    pub fn is_p1iv_10(&self) -> bool {
        *self == P1IV_A::P1IV_10
    }
    #[doc = "Checks if the value of the field is `P1IV_12`"]
    #[inline(always)]
    pub fn is_p1iv_12(&self) -> bool {
        *self == P1IV_A::P1IV_12
    }
    #[doc = "Checks if the value of the field is `P1IV_14`"]
    #[inline(always)]
    pub fn is_p1iv_14(&self) -> bool {
        *self == P1IV_A::P1IV_14
    }
    #[doc = "Checks if the value of the field is `P1IV_16`"]
    #[inline(always)]
    pub fn is_p1iv_16(&self) -> bool {
        *self == P1IV_A::P1IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1IV_R {
        P1IV_R::new((self.bits & 0x1f) as u8)
    }
}
