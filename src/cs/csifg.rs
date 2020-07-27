#[doc = "Reader of register CSIFG"]
pub type R = crate::R<u32, super::CSIFG>;
#[doc = "LFXT oscillator fault flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTIFG_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    LFXTIFG_0 = 0,
    #[doc = "1: LFXT fault. A LFXT fault occurred after the last reset"]
    LFXTIFG_1 = 1,
}
impl From<LFXTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTIFG`"]
pub type LFXTIFG_R = crate::R<bool, LFXTIFG_A>;
impl LFXTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTIFG_A {
        match self.bits {
            false => LFXTIFG_A::LFXTIFG_0,
            true => LFXTIFG_A::LFXTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTIFG_0`"]
    #[inline(always)]
    pub fn is_lfxtifg_0(&self) -> bool {
        *self == LFXTIFG_A::LFXTIFG_0
    }
    #[doc = "Checks if the value of the field is `LFXTIFG_1`"]
    #[inline(always)]
    pub fn is_lfxtifg_1(&self) -> bool {
        *self == LFXTIFG_A::LFXTIFG_1
    }
}
#[doc = "HFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTIFG_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    HFXTIFG_0 = 0,
    #[doc = "1: HFXT fault. A HFXT fault occurred after the last reset"]
    HFXTIFG_1 = 1,
}
impl From<HFXTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTIFG`"]
pub type HFXTIFG_R = crate::R<bool, HFXTIFG_A>;
impl HFXTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTIFG_A {
        match self.bits {
            false => HFXTIFG_A::HFXTIFG_0,
            true => HFXTIFG_A::HFXTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTIFG_0`"]
    #[inline(always)]
    pub fn is_hfxtifg_0(&self) -> bool {
        *self == HFXTIFG_A::HFXTIFG_0
    }
    #[doc = "Checks if the value of the field is `HFXTIFG_1`"]
    #[inline(always)]
    pub fn is_hfxtifg_1(&self) -> bool {
        *self == HFXTIFG_A::HFXTIFG_1
    }
}
#[doc = "HFXT2 oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXT2IFG_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    HFXT2IFG_0 = 0,
    #[doc = "1: HFXT2 fault. A HFXT2 fault occurred after the last reset"]
    HFXT2IFG_1 = 1,
}
impl From<HFXT2IFG_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT2IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXT2IFG`"]
pub type HFXT2IFG_R = crate::R<bool, HFXT2IFG_A>;
impl HFXT2IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT2IFG_A {
        match self.bits {
            false => HFXT2IFG_A::HFXT2IFG_0,
            true => HFXT2IFG_A::HFXT2IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT2IFG_0`"]
    #[inline(always)]
    pub fn is_hfxt2ifg_0(&self) -> bool {
        *self == HFXT2IFG_A::HFXT2IFG_0
    }
    #[doc = "Checks if the value of the field is `HFXT2IFG_1`"]
    #[inline(always)]
    pub fn is_hfxt2ifg_1(&self) -> bool {
        *self == HFXT2IFG_A::HFXT2IFG_1
    }
}
#[doc = "DCO external resistor short circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOR_SHTIFG_A {
    #[doc = "0: DCO external resistor present"]
    DCOR_SHTIFG_0 = 0,
    #[doc = "1: DCO external resistor short circuit fault"]
    DCOR_SHTIFG_1 = 1,
}
impl From<DCOR_SHTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_SHTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCOR_SHTIFG`"]
pub type DCOR_SHTIFG_R = crate::R<bool, DCOR_SHTIFG_A>;
impl DCOR_SHTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_SHTIFG_A {
        match self.bits {
            false => DCOR_SHTIFG_A::DCOR_SHTIFG_0,
            true => DCOR_SHTIFG_A::DCOR_SHTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_SHTIFG_0`"]
    #[inline(always)]
    pub fn is_dcor_shtifg_0(&self) -> bool {
        *self == DCOR_SHTIFG_A::DCOR_SHTIFG_0
    }
    #[doc = "Checks if the value of the field is `DCOR_SHTIFG_1`"]
    #[inline(always)]
    pub fn is_dcor_shtifg_1(&self) -> bool {
        *self == DCOR_SHTIFG_A::DCOR_SHTIFG_1
    }
}
#[doc = "DCO external resistor open circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOR_OPNIFG_A {
    #[doc = "0: DCO external resistor present"]
    DCOR_OPNIFG_0 = 0,
    #[doc = "1: DCO external resistor open circuit fault"]
    DCOR_OPNIFG_1 = 1,
}
impl From<DCOR_OPNIFG_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_OPNIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCOR_OPNIFG`"]
pub type DCOR_OPNIFG_R = crate::R<bool, DCOR_OPNIFG_A>;
impl DCOR_OPNIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_OPNIFG_A {
        match self.bits {
            false => DCOR_OPNIFG_A::DCOR_OPNIFG_0,
            true => DCOR_OPNIFG_A::DCOR_OPNIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIFG_0`"]
    #[inline(always)]
    pub fn is_dcor_opnifg_0(&self) -> bool {
        *self == DCOR_OPNIFG_A::DCOR_OPNIFG_0
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIFG_1`"]
    #[inline(always)]
    pub fn is_dcor_opnifg_1(&self) -> bool {
        *self == DCOR_OPNIFG_A::DCOR_OPNIFG_1
    }
}
#[doc = "Start fault counter interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTLFIFG_A {
    #[doc = "0: Start counter not expired"]
    FCNTLFIFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTLFIFG_1 = 1,
}
impl From<FCNTLFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLFIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTLFIFG`"]
pub type FCNTLFIFG_R = crate::R<bool, FCNTLFIFG_A>;
impl FCNTLFIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLFIFG_A {
        match self.bits {
            false => FCNTLFIFG_A::FCNTLFIFG_0,
            true => FCNTLFIFG_A::FCNTLFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLFIFG_0`"]
    #[inline(always)]
    pub fn is_fcntlfifg_0(&self) -> bool {
        *self == FCNTLFIFG_A::FCNTLFIFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTLFIFG_1`"]
    #[inline(always)]
    pub fn is_fcntlfifg_1(&self) -> bool {
        *self == FCNTLFIFG_A::FCNTLFIFG_1
    }
}
#[doc = "Start fault counter interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHFIFG_A {
    #[doc = "0: Start counter not expired"]
    FCNTHFIFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTHFIFG_1 = 1,
}
impl From<FCNTHFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHFIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHFIFG`"]
pub type FCNTHFIFG_R = crate::R<bool, FCNTHFIFG_A>;
impl FCNTHFIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHFIFG_A {
        match self.bits {
            false => FCNTHFIFG_A::FCNTHFIFG_0,
            true => FCNTHFIFG_A::FCNTHFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHFIFG_0`"]
    #[inline(always)]
    pub fn is_fcnthfifg_0(&self) -> bool {
        *self == FCNTHFIFG_A::FCNTHFIFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTHFIFG_1`"]
    #[inline(always)]
    pub fn is_fcnthfifg_1(&self) -> bool {
        *self == FCNTHFIFG_A::FCNTHFIFG_1
    }
}
#[doc = "Start fault counter interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCNTHF2IFG_A {
    #[doc = "0: Start counter not expired"]
    FCNTHF2IFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTHF2IFG_1 = 1,
}
impl From<FCNTHF2IFG_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCNTHF2IFG`"]
pub type FCNTHF2IFG_R = crate::R<bool, FCNTHF2IFG_A>;
impl FCNTHF2IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2IFG_A {
        match self.bits {
            false => FCNTHF2IFG_A::FCNTHF2IFG_0,
            true => FCNTHF2IFG_A::FCNTHF2IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IFG_0`"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_0(&self) -> bool {
        *self == FCNTHF2IFG_A::FCNTHF2IFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IFG_1`"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_1(&self) -> bool {
        *self == FCNTHF2IFG_A::FCNTHF2IFG_1
    }
}
#[doc = "PLL out-of-lock interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLOOLIFG_A {
    #[doc = "0: No interrupt pending"]
    PLLOOLIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLOOLIFG_1 = 1,
}
impl From<PLLOOLIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOOLIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLOOLIFG`"]
pub type PLLOOLIFG_R = crate::R<bool, PLLOOLIFG_A>;
impl PLLOOLIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOOLIFG_A {
        match self.bits {
            false => PLLOOLIFG_A::PLLOOLIFG_0,
            true => PLLOOLIFG_A::PLLOOLIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOOLIFG_0`"]
    #[inline(always)]
    pub fn is_plloolifg_0(&self) -> bool {
        *self == PLLOOLIFG_A::PLLOOLIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLOOLIFG_1`"]
    #[inline(always)]
    pub fn is_plloolifg_1(&self) -> bool {
        *self == PLLOOLIFG_A::PLLOOLIFG_1
    }
}
#[doc = "PLL loss-of-signal interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLOSIFG_A {
    #[doc = "0: No interrupt pending"]
    PLLLOSIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLLOSIFG_1 = 1,
}
impl From<PLLLOSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOSIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLLOSIFG`"]
pub type PLLLOSIFG_R = crate::R<bool, PLLLOSIFG_A>;
impl PLLLOSIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOSIFG_A {
        match self.bits {
            false => PLLLOSIFG_A::PLLLOSIFG_0,
            true => PLLLOSIFG_A::PLLLOSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLLOSIFG_0`"]
    #[inline(always)]
    pub fn is_plllosifg_0(&self) -> bool {
        *self == PLLLOSIFG_A::PLLLOSIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLLOSIFG_1`"]
    #[inline(always)]
    pub fn is_plllosifg_1(&self) -> bool {
        *self == PLLLOSIFG_A::PLLLOSIFG_1
    }
}
#[doc = "PLL out-of-range interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLOORIFG_A {
    #[doc = "0: No interrupt pending"]
    PLLOORIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLOORIFG_1 = 1,
}
impl From<PLLOORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOORIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLOORIFG`"]
pub type PLLOORIFG_R = crate::R<bool, PLLOORIFG_A>;
impl PLLOORIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOORIFG_A {
        match self.bits {
            false => PLLOORIFG_A::PLLOORIFG_0,
            true => PLLOORIFG_A::PLLOORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOORIFG_0`"]
    #[inline(always)]
    pub fn is_plloorifg_0(&self) -> bool {
        *self == PLLOORIFG_A::PLLOORIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLOORIFG_1`"]
    #[inline(always)]
    pub fn is_plloorifg_1(&self) -> bool {
        *self == PLLOORIFG_A::PLLOORIFG_1
    }
}
#[doc = "REFCNT period counter expired\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIFG_A {
    #[doc = "0: REFCNT period counter not expired"]
    CALIFG_0 = 0,
    #[doc = "1: REFCNT period counter expired"]
    CALIFG_1 = 1,
}
impl From<CALIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CALIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALIFG`"]
pub type CALIFG_R = crate::R<bool, CALIFG_A>;
impl CALIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIFG_A {
        match self.bits {
            false => CALIFG_A::CALIFG_0,
            true => CALIFG_A::CALIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALIFG_0`"]
    #[inline(always)]
    pub fn is_califg_0(&self) -> bool {
        *self == CALIFG_A::CALIFG_0
    }
    #[doc = "Checks if the value of the field is `CALIFG_1`"]
    #[inline(always)]
    pub fn is_califg_1(&self) -> bool {
        *self == CALIFG_A::CALIFG_1
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtifg(&self) -> LFXTIFG_R {
        LFXTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtifg(&self) -> HFXTIFG_R {
        HFXTIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag"]
    #[inline(always)]
    pub fn hfxt2ifg(&self) -> HFXT2IFG_R {
        HFXT2IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DCO external resistor short circuit fault flag."]
    #[inline(always)]
    pub fn dcor_shtifg(&self) -> DCOR_SHTIFG_R {
        DCOR_SHTIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag."]
    #[inline(always)]
    pub fn dcor_opnifg(&self) -> DCOR_OPNIFG_R {
        DCOR_OPNIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt flag LFXT"]
    #[inline(always)]
    pub fn fcntlfifg(&self) -> FCNTLFIFG_R {
        FCNTLFIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt flag HFXT"]
    #[inline(always)]
    pub fn fcnthfifg(&self) -> FCNTHFIFG_R {
        FCNTHFIFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Start fault counter interrupt flag HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ifg(&self) -> FCNTHF2IFG_R {
        FCNTHF2IFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt flag"]
    #[inline(always)]
    pub fn plloolifg(&self) -> PLLOOLIFG_R {
        PLLOOLIFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt flag"]
    #[inline(always)]
    pub fn plllosifg(&self) -> PLLLOSIFG_R {
        PLLLOSIFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt flag"]
    #[inline(always)]
    pub fn plloorifg(&self) -> PLLOORIFG_R {
        PLLOORIFG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter expired"]
    #[inline(always)]
    pub fn califg(&self) -> CALIFG_R {
        CALIFG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
