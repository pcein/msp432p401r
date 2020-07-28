#[doc = "Reader of register P3IV"]
pub type R = crate::R<u16, super::P3IV>;
#[doc = "Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3IV_A {
    #[doc = "0: No interrupt pending"]
    P3IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3IV_16 = 16,
}
impl From<P3IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P3IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3IV`"]
pub type P3IV_R = crate::R<u8, P3IV_A>;
impl P3IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P3IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P3IV_A::P3IV_0),
            2 => Val(P3IV_A::P3IV_2),
            4 => Val(P3IV_A::P3IV_4),
            6 => Val(P3IV_A::P3IV_6),
            8 => Val(P3IV_A::P3IV_8),
            10 => Val(P3IV_A::P3IV_10),
            12 => Val(P3IV_A::P3IV_12),
            14 => Val(P3IV_A::P3IV_14),
            16 => Val(P3IV_A::P3IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P3IV_0`"]
    #[inline(always)]
    pub fn is_p3iv_0(&self) -> bool {
        *self == P3IV_A::P3IV_0
    }
    #[doc = "Checks if the value of the field is `P3IV_2`"]
    #[inline(always)]
    pub fn is_p3iv_2(&self) -> bool {
        *self == P3IV_A::P3IV_2
    }
    #[doc = "Checks if the value of the field is `P3IV_4`"]
    #[inline(always)]
    pub fn is_p3iv_4(&self) -> bool {
        *self == P3IV_A::P3IV_4
    }
    #[doc = "Checks if the value of the field is `P3IV_6`"]
    #[inline(always)]
    pub fn is_p3iv_6(&self) -> bool {
        *self == P3IV_A::P3IV_6
    }
    #[doc = "Checks if the value of the field is `P3IV_8`"]
    #[inline(always)]
    pub fn is_p3iv_8(&self) -> bool {
        *self == P3IV_A::P3IV_8
    }
    #[doc = "Checks if the value of the field is `P3IV_10`"]
    #[inline(always)]
    pub fn is_p3iv_10(&self) -> bool {
        *self == P3IV_A::P3IV_10
    }
    #[doc = "Checks if the value of the field is `P3IV_12`"]
    #[inline(always)]
    pub fn is_p3iv_12(&self) -> bool {
        *self == P3IV_A::P3IV_12
    }
    #[doc = "Checks if the value of the field is `P3IV_14`"]
    #[inline(always)]
    pub fn is_p3iv_14(&self) -> bool {
        *self == P3IV_A::P3IV_14
    }
    #[doc = "Checks if the value of the field is `P3IV_16`"]
    #[inline(always)]
    pub fn is_p3iv_16(&self) -> bool {
        *self == P3IV_A::P3IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3IV_R {
        P3IV_R::new((self.bits & 0x1f) as u8)
    }
}
