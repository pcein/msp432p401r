#[doc = "Reader of register P4IV"]
pub type R = crate::R<u16, super::P4IV>;
#[doc = "Port 4 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4IV_A {
    #[doc = "0: No interrupt pending"]
    P4IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4IV_16 = 16,
}
impl From<P4IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P4IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4IV`"]
pub type P4IV_R = crate::R<u8, P4IV_A>;
impl P4IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P4IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P4IV_A::P4IV_0),
            2 => Val(P4IV_A::P4IV_2),
            4 => Val(P4IV_A::P4IV_4),
            6 => Val(P4IV_A::P4IV_6),
            8 => Val(P4IV_A::P4IV_8),
            10 => Val(P4IV_A::P4IV_10),
            12 => Val(P4IV_A::P4IV_12),
            14 => Val(P4IV_A::P4IV_14),
            16 => Val(P4IV_A::P4IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P4IV_0`"]
    #[inline(always)]
    pub fn is_p4iv_0(&self) -> bool {
        *self == P4IV_A::P4IV_0
    }
    #[doc = "Checks if the value of the field is `P4IV_2`"]
    #[inline(always)]
    pub fn is_p4iv_2(&self) -> bool {
        *self == P4IV_A::P4IV_2
    }
    #[doc = "Checks if the value of the field is `P4IV_4`"]
    #[inline(always)]
    pub fn is_p4iv_4(&self) -> bool {
        *self == P4IV_A::P4IV_4
    }
    #[doc = "Checks if the value of the field is `P4IV_6`"]
    #[inline(always)]
    pub fn is_p4iv_6(&self) -> bool {
        *self == P4IV_A::P4IV_6
    }
    #[doc = "Checks if the value of the field is `P4IV_8`"]
    #[inline(always)]
    pub fn is_p4iv_8(&self) -> bool {
        *self == P4IV_A::P4IV_8
    }
    #[doc = "Checks if the value of the field is `P4IV_10`"]
    #[inline(always)]
    pub fn is_p4iv_10(&self) -> bool {
        *self == P4IV_A::P4IV_10
    }
    #[doc = "Checks if the value of the field is `P4IV_12`"]
    #[inline(always)]
    pub fn is_p4iv_12(&self) -> bool {
        *self == P4IV_A::P4IV_12
    }
    #[doc = "Checks if the value of the field is `P4IV_14`"]
    #[inline(always)]
    pub fn is_p4iv_14(&self) -> bool {
        *self == P4IV_A::P4IV_14
    }
    #[doc = "Checks if the value of the field is `P4IV_16`"]
    #[inline(always)]
    pub fn is_p4iv_16(&self) -> bool {
        *self == P4IV_A::P4IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4IV_R {
        P4IV_R::new((self.bits & 0x1f) as u8)
    }
}
