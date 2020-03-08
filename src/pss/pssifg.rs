#[doc = "Reader of register PSSIFG"]
pub type R = crate::R<u32, super::PSSIFG>;
#[doc = "High-side SVSM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHIFG_A {
    #[doc = "0: No interrupt pending"]
    SVSMHIFG_0 = 0,
    #[doc = "1: Interrupt due to SVSMH"]
    SVSMHIFG_1 = 1,
}
impl From<SVSMHIFG_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVSMHIFG`"]
pub type SVSMHIFG_R = crate::R<bool, SVSMHIFG_A>;
impl SVSMHIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHIFG_A {
        match self.bits {
            false => SVSMHIFG_A::SVSMHIFG_0,
            true => SVSMHIFG_A::SVSMHIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_0`"]
    #[inline(always)]
    pub fn is_svsmhifg_0(&self) -> bool {
        *self == SVSMHIFG_A::SVSMHIFG_0
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_1`"]
    #[inline(always)]
    pub fn is_svsmhifg_1(&self) -> bool {
        *self == SVSMHIFG_A::SVSMHIFG_1
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt flag"]
    #[inline(always)]
    pub fn svsmhifg(&self) -> SVSMHIFG_R {
        SVSMHIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
