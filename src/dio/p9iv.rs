#[doc = "Reader of register P9IV"]
pub type R = crate::R<u16, super::P9IV>;
#[doc = "Port 9 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P9IV_A {
    #[doc = "0: No interrupt pending"]
    P9IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    P9IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    P9IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    P9IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    P9IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    P9IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    P9IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    P9IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    P9IV_16 = 16,
}
impl From<P9IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P9IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P9IV`"]
pub type P9IV_R = crate::R<u8, P9IV_A>;
impl P9IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P9IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P9IV_A::P9IV_0),
            2 => Val(P9IV_A::P9IV_2),
            4 => Val(P9IV_A::P9IV_4),
            6 => Val(P9IV_A::P9IV_6),
            8 => Val(P9IV_A::P9IV_8),
            10 => Val(P9IV_A::P9IV_10),
            12 => Val(P9IV_A::P9IV_12),
            14 => Val(P9IV_A::P9IV_14),
            16 => Val(P9IV_A::P9IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P9IV_0`"]
    #[inline(always)]
    pub fn is_p9iv_0(&self) -> bool {
        *self == P9IV_A::P9IV_0
    }
    #[doc = "Checks if the value of the field is `P9IV_2`"]
    #[inline(always)]
    pub fn is_p9iv_2(&self) -> bool {
        *self == P9IV_A::P9IV_2
    }
    #[doc = "Checks if the value of the field is `P9IV_4`"]
    #[inline(always)]
    pub fn is_p9iv_4(&self) -> bool {
        *self == P9IV_A::P9IV_4
    }
    #[doc = "Checks if the value of the field is `P9IV_6`"]
    #[inline(always)]
    pub fn is_p9iv_6(&self) -> bool {
        *self == P9IV_A::P9IV_6
    }
    #[doc = "Checks if the value of the field is `P9IV_8`"]
    #[inline(always)]
    pub fn is_p9iv_8(&self) -> bool {
        *self == P9IV_A::P9IV_8
    }
    #[doc = "Checks if the value of the field is `P9IV_10`"]
    #[inline(always)]
    pub fn is_p9iv_10(&self) -> bool {
        *self == P9IV_A::P9IV_10
    }
    #[doc = "Checks if the value of the field is `P9IV_12`"]
    #[inline(always)]
    pub fn is_p9iv_12(&self) -> bool {
        *self == P9IV_A::P9IV_12
    }
    #[doc = "Checks if the value of the field is `P9IV_14`"]
    #[inline(always)]
    pub fn is_p9iv_14(&self) -> bool {
        *self == P9IV_A::P9IV_14
    }
    #[doc = "Checks if the value of the field is `P9IV_16`"]
    #[inline(always)]
    pub fn is_p9iv_16(&self) -> bool {
        *self == P9IV_A::P9IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 9 interrupt vector value"]
    #[inline(always)]
    pub fn p9iv(&self) -> P9IV_R {
        P9IV_R::new((self.bits & 0x1f) as u8)
    }
}
