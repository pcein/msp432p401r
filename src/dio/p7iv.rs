#[doc = "Reader of register P7IV"]
pub type R = crate::R<u16, super::P7IV>;
#[doc = "Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P7IV_A {
    #[doc = "0: No interrupt pending"]
    P7IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7IV_16 = 16,
}
impl From<P7IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P7IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P7IV`"]
pub type P7IV_R = crate::R<u8, P7IV_A>;
impl P7IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P7IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P7IV_A::P7IV_0),
            2 => Val(P7IV_A::P7IV_2),
            4 => Val(P7IV_A::P7IV_4),
            6 => Val(P7IV_A::P7IV_6),
            8 => Val(P7IV_A::P7IV_8),
            10 => Val(P7IV_A::P7IV_10),
            12 => Val(P7IV_A::P7IV_12),
            14 => Val(P7IV_A::P7IV_14),
            16 => Val(P7IV_A::P7IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P7IV_0`"]
    #[inline(always)]
    pub fn is_p7iv_0(&self) -> bool {
        *self == P7IV_A::P7IV_0
    }
    #[doc = "Checks if the value of the field is `P7IV_2`"]
    #[inline(always)]
    pub fn is_p7iv_2(&self) -> bool {
        *self == P7IV_A::P7IV_2
    }
    #[doc = "Checks if the value of the field is `P7IV_4`"]
    #[inline(always)]
    pub fn is_p7iv_4(&self) -> bool {
        *self == P7IV_A::P7IV_4
    }
    #[doc = "Checks if the value of the field is `P7IV_6`"]
    #[inline(always)]
    pub fn is_p7iv_6(&self) -> bool {
        *self == P7IV_A::P7IV_6
    }
    #[doc = "Checks if the value of the field is `P7IV_8`"]
    #[inline(always)]
    pub fn is_p7iv_8(&self) -> bool {
        *self == P7IV_A::P7IV_8
    }
    #[doc = "Checks if the value of the field is `P7IV_10`"]
    #[inline(always)]
    pub fn is_p7iv_10(&self) -> bool {
        *self == P7IV_A::P7IV_10
    }
    #[doc = "Checks if the value of the field is `P7IV_12`"]
    #[inline(always)]
    pub fn is_p7iv_12(&self) -> bool {
        *self == P7IV_A::P7IV_12
    }
    #[doc = "Checks if the value of the field is `P7IV_14`"]
    #[inline(always)]
    pub fn is_p7iv_14(&self) -> bool {
        *self == P7IV_A::P7IV_14
    }
    #[doc = "Checks if the value of the field is `P7IV_16`"]
    #[inline(always)]
    pub fn is_p7iv_16(&self) -> bool {
        *self == P7IV_A::P7IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7IV_R {
        P7IV_R::new((self.bits & 0x1f) as u8)
    }
}
