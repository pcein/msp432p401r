#[doc = "Reader of register P6IV"]
pub type R = crate::R<u16, super::P6IV>;
#[doc = "Port 6 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P6IV_A {
    #[doc = "0: No interrupt pending"]
    P6IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    P6IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    P6IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    P6IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    P6IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    P6IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    P6IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    P6IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    P6IV_16 = 16,
}
impl From<P6IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P6IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P6IV`"]
pub type P6IV_R = crate::R<u8, P6IV_A>;
impl P6IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P6IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P6IV_A::P6IV_0),
            2 => Val(P6IV_A::P6IV_2),
            4 => Val(P6IV_A::P6IV_4),
            6 => Val(P6IV_A::P6IV_6),
            8 => Val(P6IV_A::P6IV_8),
            10 => Val(P6IV_A::P6IV_10),
            12 => Val(P6IV_A::P6IV_12),
            14 => Val(P6IV_A::P6IV_14),
            16 => Val(P6IV_A::P6IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P6IV_0`"]
    #[inline(always)]
    pub fn is_p6iv_0(&self) -> bool {
        *self == P6IV_A::P6IV_0
    }
    #[doc = "Checks if the value of the field is `P6IV_2`"]
    #[inline(always)]
    pub fn is_p6iv_2(&self) -> bool {
        *self == P6IV_A::P6IV_2
    }
    #[doc = "Checks if the value of the field is `P6IV_4`"]
    #[inline(always)]
    pub fn is_p6iv_4(&self) -> bool {
        *self == P6IV_A::P6IV_4
    }
    #[doc = "Checks if the value of the field is `P6IV_6`"]
    #[inline(always)]
    pub fn is_p6iv_6(&self) -> bool {
        *self == P6IV_A::P6IV_6
    }
    #[doc = "Checks if the value of the field is `P6IV_8`"]
    #[inline(always)]
    pub fn is_p6iv_8(&self) -> bool {
        *self == P6IV_A::P6IV_8
    }
    #[doc = "Checks if the value of the field is `P6IV_10`"]
    #[inline(always)]
    pub fn is_p6iv_10(&self) -> bool {
        *self == P6IV_A::P6IV_10
    }
    #[doc = "Checks if the value of the field is `P6IV_12`"]
    #[inline(always)]
    pub fn is_p6iv_12(&self) -> bool {
        *self == P6IV_A::P6IV_12
    }
    #[doc = "Checks if the value of the field is `P6IV_14`"]
    #[inline(always)]
    pub fn is_p6iv_14(&self) -> bool {
        *self == P6IV_A::P6IV_14
    }
    #[doc = "Checks if the value of the field is `P6IV_16`"]
    #[inline(always)]
    pub fn is_p6iv_16(&self) -> bool {
        *self == P6IV_A::P6IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6IV_R {
        P6IV_R::new((self.bits & 0x1f) as u8)
    }
}
