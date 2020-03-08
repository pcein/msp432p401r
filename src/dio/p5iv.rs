#[doc = "Reader of register P5IV"]
pub type R = crate::R<u16, super::P5IV>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P5IV_A {
    #[doc = "0: No interrupt pending"]
    P5IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5IV_16 = 16,
}
impl From<P5IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P5IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P5IV`"]
pub type P5IV_R = crate::R<u8, P5IV_A>;
impl P5IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P5IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P5IV_A::P5IV_0),
            2 => Val(P5IV_A::P5IV_2),
            4 => Val(P5IV_A::P5IV_4),
            6 => Val(P5IV_A::P5IV_6),
            8 => Val(P5IV_A::P5IV_8),
            10 => Val(P5IV_A::P5IV_10),
            12 => Val(P5IV_A::P5IV_12),
            14 => Val(P5IV_A::P5IV_14),
            16 => Val(P5IV_A::P5IV_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P5IV_0`"]
    #[inline(always)]
    pub fn is_p5iv_0(&self) -> bool {
        *self == P5IV_A::P5IV_0
    }
    #[doc = "Checks if the value of the field is `P5IV_2`"]
    #[inline(always)]
    pub fn is_p5iv_2(&self) -> bool {
        *self == P5IV_A::P5IV_2
    }
    #[doc = "Checks if the value of the field is `P5IV_4`"]
    #[inline(always)]
    pub fn is_p5iv_4(&self) -> bool {
        *self == P5IV_A::P5IV_4
    }
    #[doc = "Checks if the value of the field is `P5IV_6`"]
    #[inline(always)]
    pub fn is_p5iv_6(&self) -> bool {
        *self == P5IV_A::P5IV_6
    }
    #[doc = "Checks if the value of the field is `P5IV_8`"]
    #[inline(always)]
    pub fn is_p5iv_8(&self) -> bool {
        *self == P5IV_A::P5IV_8
    }
    #[doc = "Checks if the value of the field is `P5IV_10`"]
    #[inline(always)]
    pub fn is_p5iv_10(&self) -> bool {
        *self == P5IV_A::P5IV_10
    }
    #[doc = "Checks if the value of the field is `P5IV_12`"]
    #[inline(always)]
    pub fn is_p5iv_12(&self) -> bool {
        *self == P5IV_A::P5IV_12
    }
    #[doc = "Checks if the value of the field is `P5IV_14`"]
    #[inline(always)]
    pub fn is_p5iv_14(&self) -> bool {
        *self == P5IV_A::P5IV_14
    }
    #[doc = "Checks if the value of the field is `P5IV_16`"]
    #[inline(always)]
    pub fn is_p5iv_16(&self) -> bool {
        *self == P5IV_A::P5IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5IV_R {
        P5IV_R::new((self.bits & 0x1f) as u8)
    }
}
