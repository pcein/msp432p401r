#[doc = "Reader of register P10IV"]
pub type R = crate::R<u16, super::P10IV>;
#[doc = "Port 10 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P10IV_A {
    #[doc = "0: No interrupt pending"]
    P10IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 10.0 interrupt; Interrupt Flag: P10IFG0; Interrupt Priority: Highest"]
    P10IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 10.1 interrupt; Interrupt Flag: P10IFG1"]
    P10IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 10.2 interrupt; Interrupt Flag: P10IFG2"]
    P10IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 10.3 interrupt; Interrupt Flag: P10IFG3"]
    P10IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 10.4 interrupt; Interrupt Flag: P10IFG4"]
    P10IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 10.5 interrupt; Interrupt Flag: P10IFG5"]
    P10IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 10.6 interrupt; Interrupt Flag: P10IFG6"]
    P10IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 10.7 interrupt; Interrupt Flag: P10IFG7; Interrupt Priority: Lowest"]
    P10IV_16 = 16,
}
impl From<P10IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P10IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P10IV`"]
pub type P10IV_R = crate::R<u8, P10IV_A>;
impl P10IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P10IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P10IV_A::P10IV_0),
            2 => Val(P10IV_A::P10IV_2),
            4 => Val(P10IV_A::P10IV_4),
            6 => Val(P10IV_A::P10IV_6),
            8 => Val(P10IV_A::P10IV_8),
            10 => Val(P10IV_A::P10IV_10),
            12 => Val(P10IV_A::P10IV_12),
            14 => Val(P10IV_A::P10IV_14),
            16 => Val(P10IV_A::P10IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P10IV_0`"]
    #[inline(always)]
    pub fn is_p10iv_0(&self) -> bool {
        *self == P10IV_A::P10IV_0
    }
    #[doc = "Checks if the value of the field is `P10IV_2`"]
    #[inline(always)]
    pub fn is_p10iv_2(&self) -> bool {
        *self == P10IV_A::P10IV_2
    }
    #[doc = "Checks if the value of the field is `P10IV_4`"]
    #[inline(always)]
    pub fn is_p10iv_4(&self) -> bool {
        *self == P10IV_A::P10IV_4
    }
    #[doc = "Checks if the value of the field is `P10IV_6`"]
    #[inline(always)]
    pub fn is_p10iv_6(&self) -> bool {
        *self == P10IV_A::P10IV_6
    }
    #[doc = "Checks if the value of the field is `P10IV_8`"]
    #[inline(always)]
    pub fn is_p10iv_8(&self) -> bool {
        *self == P10IV_A::P10IV_8
    }
    #[doc = "Checks if the value of the field is `P10IV_10`"]
    #[inline(always)]
    pub fn is_p10iv_10(&self) -> bool {
        *self == P10IV_A::P10IV_10
    }
    #[doc = "Checks if the value of the field is `P10IV_12`"]
    #[inline(always)]
    pub fn is_p10iv_12(&self) -> bool {
        *self == P10IV_A::P10IV_12
    }
    #[doc = "Checks if the value of the field is `P10IV_14`"]
    #[inline(always)]
    pub fn is_p10iv_14(&self) -> bool {
        *self == P10IV_A::P10IV_14
    }
    #[doc = "Checks if the value of the field is `P10IV_16`"]
    #[inline(always)]
    pub fn is_p10iv_16(&self) -> bool {
        *self == P10IV_A::P10IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 10 interrupt vector value"]
    #[inline(always)]
    pub fn p10iv(&self) -> P10IV_R {
        P10IV_R::new((self.bits & 0x1f) as u8)
    }
}
