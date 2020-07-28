#[doc = "Reader of register CSSTAT"]
pub type R = crate::R<u32, super::CSSTAT>;
#[doc = "DCO status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCO_ON_A {
    #[doc = "0: Inactive"]
    DCO_ON_0 = 0,
    #[doc = "1: Active"]
    DCO_ON_1 = 1,
}
impl From<DCO_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DCO_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCO_ON`"]
pub type DCO_ON_R = crate::R<bool, DCO_ON_A>;
impl DCO_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCO_ON_A {
        match self.bits {
            false => DCO_ON_A::DCO_ON_0,
            true => DCO_ON_A::DCO_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCO_ON_0`"]
    #[inline(always)]
    pub fn is_dco_on_0(&self) -> bool {
        *self == DCO_ON_A::DCO_ON_0
    }
    #[doc = "Checks if the value of the field is `DCO_ON_1`"]
    #[inline(always)]
    pub fn is_dco_on_1(&self) -> bool {
        *self == DCO_ON_A::DCO_ON_1
    }
}
#[doc = "DCO bias status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOBIAS_ON_A {
    #[doc = "0: Inactive"]
    DCOBIAS_ON_0 = 0,
    #[doc = "1: Active"]
    DCOBIAS_ON_1 = 1,
}
impl From<DCOBIAS_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DCOBIAS_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCOBIAS_ON`"]
pub type DCOBIAS_ON_R = crate::R<bool, DCOBIAS_ON_A>;
impl DCOBIAS_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOBIAS_ON_A {
        match self.bits {
            false => DCOBIAS_ON_A::DCOBIAS_ON_0,
            true => DCOBIAS_ON_A::DCOBIAS_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOBIAS_ON_0`"]
    #[inline(always)]
    pub fn is_dcobias_on_0(&self) -> bool {
        *self == DCOBIAS_ON_A::DCOBIAS_ON_0
    }
    #[doc = "Checks if the value of the field is `DCOBIAS_ON_1`"]
    #[inline(always)]
    pub fn is_dcobias_on_1(&self) -> bool {
        *self == DCOBIAS_ON_A::DCOBIAS_ON_1
    }
}
#[doc = "HFXT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXT_ON_A {
    #[doc = "0: Inactive"]
    HFXT_ON_0 = 0,
    #[doc = "1: Active"]
    HFXT_ON_1 = 1,
}
impl From<HFXT_ON_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXT_ON`"]
pub type HFXT_ON_R = crate::R<bool, HFXT_ON_A>;
impl HFXT_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT_ON_A {
        match self.bits {
            false => HFXT_ON_A::HFXT_ON_0,
            true => HFXT_ON_A::HFXT_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT_ON_0`"]
    #[inline(always)]
    pub fn is_hfxt_on_0(&self) -> bool {
        *self == HFXT_ON_A::HFXT_ON_0
    }
    #[doc = "Checks if the value of the field is `HFXT_ON_1`"]
    #[inline(always)]
    pub fn is_hfxt_on_1(&self) -> bool {
        *self == HFXT_ON_A::HFXT_ON_1
    }
}
#[doc = "HFXT2 status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXT2_ON_A {
    #[doc = "0: Inactive"]
    HFXT2_ON_0 = 0,
    #[doc = "1: Active"]
    HFXT2_ON_1 = 1,
}
impl From<HFXT2_ON_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT2_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXT2_ON`"]
pub type HFXT2_ON_R = crate::R<bool, HFXT2_ON_A>;
impl HFXT2_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT2_ON_A {
        match self.bits {
            false => HFXT2_ON_A::HFXT2_ON_0,
            true => HFXT2_ON_A::HFXT2_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT2_ON_0`"]
    #[inline(always)]
    pub fn is_hfxt2_on_0(&self) -> bool {
        *self == HFXT2_ON_A::HFXT2_ON_0
    }
    #[doc = "Checks if the value of the field is `HFXT2_ON_1`"]
    #[inline(always)]
    pub fn is_hfxt2_on_1(&self) -> bool {
        *self == HFXT2_ON_A::HFXT2_ON_1
    }
}
#[doc = "MODOSC status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODOSC_ON_A {
    #[doc = "0: Inactive"]
    MODOSC_ON_0 = 0,
    #[doc = "1: Active"]
    MODOSC_ON_1 = 1,
}
impl From<MODOSC_ON_A> for bool {
    #[inline(always)]
    fn from(variant: MODOSC_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODOSC_ON`"]
pub type MODOSC_ON_R = crate::R<bool, MODOSC_ON_A>;
impl MODOSC_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODOSC_ON_A {
        match self.bits {
            false => MODOSC_ON_A::MODOSC_ON_0,
            true => MODOSC_ON_A::MODOSC_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODOSC_ON_0`"]
    #[inline(always)]
    pub fn is_modosc_on_0(&self) -> bool {
        *self == MODOSC_ON_A::MODOSC_ON_0
    }
    #[doc = "Checks if the value of the field is `MODOSC_ON_1`"]
    #[inline(always)]
    pub fn is_modosc_on_1(&self) -> bool {
        *self == MODOSC_ON_A::MODOSC_ON_1
    }
}
#[doc = "VLO status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLO_ON_A {
    #[doc = "0: Inactive"]
    VLO_ON_0 = 0,
    #[doc = "1: Active"]
    VLO_ON_1 = 1,
}
impl From<VLO_ON_A> for bool {
    #[inline(always)]
    fn from(variant: VLO_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLO_ON`"]
pub type VLO_ON_R = crate::R<bool, VLO_ON_A>;
impl VLO_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLO_ON_A {
        match self.bits {
            false => VLO_ON_A::VLO_ON_0,
            true => VLO_ON_A::VLO_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLO_ON_0`"]
    #[inline(always)]
    pub fn is_vlo_on_0(&self) -> bool {
        *self == VLO_ON_A::VLO_ON_0
    }
    #[doc = "Checks if the value of the field is `VLO_ON_1`"]
    #[inline(always)]
    pub fn is_vlo_on_1(&self) -> bool {
        *self == VLO_ON_A::VLO_ON_1
    }
}
#[doc = "LFXT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXT_ON_A {
    #[doc = "0: Inactive"]
    LFXT_ON_0 = 0,
    #[doc = "1: Active"]
    LFXT_ON_1 = 1,
}
impl From<LFXT_ON_A> for bool {
    #[inline(always)]
    fn from(variant: LFXT_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXT_ON`"]
pub type LFXT_ON_R = crate::R<bool, LFXT_ON_A>;
impl LFXT_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXT_ON_A {
        match self.bits {
            false => LFXT_ON_A::LFXT_ON_0,
            true => LFXT_ON_A::LFXT_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXT_ON_0`"]
    #[inline(always)]
    pub fn is_lfxt_on_0(&self) -> bool {
        *self == LFXT_ON_A::LFXT_ON_0
    }
    #[doc = "Checks if the value of the field is `LFXT_ON_1`"]
    #[inline(always)]
    pub fn is_lfxt_on_1(&self) -> bool {
        *self == LFXT_ON_A::LFXT_ON_1
    }
}
#[doc = "REFO status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFO_ON_A {
    #[doc = "0: Inactive"]
    REFO_ON_0 = 0,
    #[doc = "1: Active"]
    REFO_ON_1 = 1,
}
impl From<REFO_ON_A> for bool {
    #[inline(always)]
    fn from(variant: REFO_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFO_ON`"]
pub type REFO_ON_R = crate::R<bool, REFO_ON_A>;
impl REFO_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFO_ON_A {
        match self.bits {
            false => REFO_ON_A::REFO_ON_0,
            true => REFO_ON_A::REFO_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFO_ON_0`"]
    #[inline(always)]
    pub fn is_refo_on_0(&self) -> bool {
        *self == REFO_ON_A::REFO_ON_0
    }
    #[doc = "Checks if the value of the field is `REFO_ON_1`"]
    #[inline(always)]
    pub fn is_refo_on_1(&self) -> bool {
        *self == REFO_ON_A::REFO_ON_1
    }
}
#[doc = "ACLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACLK_ON_A {
    #[doc = "0: Inactive"]
    ACLK_ON_0 = 0,
    #[doc = "1: Active"]
    ACLK_ON_1 = 1,
}
impl From<ACLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: ACLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACLK_ON`"]
pub type ACLK_ON_R = crate::R<bool, ACLK_ON_A>;
impl ACLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLK_ON_A {
        match self.bits {
            false => ACLK_ON_A::ACLK_ON_0,
            true => ACLK_ON_A::ACLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACLK_ON_0`"]
    #[inline(always)]
    pub fn is_aclk_on_0(&self) -> bool {
        *self == ACLK_ON_A::ACLK_ON_0
    }
    #[doc = "Checks if the value of the field is `ACLK_ON_1`"]
    #[inline(always)]
    pub fn is_aclk_on_1(&self) -> bool {
        *self == ACLK_ON_A::ACLK_ON_1
    }
}
#[doc = "MCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_ON_A {
    #[doc = "0: Inactive"]
    MCLK_ON_0 = 0,
    #[doc = "1: Active"]
    MCLK_ON_1 = 1,
}
impl From<MCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLK_ON`"]
pub type MCLK_ON_R = crate::R<bool, MCLK_ON_A>;
impl MCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_ON_A {
        match self.bits {
            false => MCLK_ON_A::MCLK_ON_0,
            true => MCLK_ON_A::MCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLK_ON_0`"]
    #[inline(always)]
    pub fn is_mclk_on_0(&self) -> bool {
        *self == MCLK_ON_A::MCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `MCLK_ON_1`"]
    #[inline(always)]
    pub fn is_mclk_on_1(&self) -> bool {
        *self == MCLK_ON_A::MCLK_ON_1
    }
}
#[doc = "HSMCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMCLK_ON_A {
    #[doc = "0: Inactive"]
    HSMCLK_ON_0 = 0,
    #[doc = "1: Active"]
    HSMCLK_ON_1 = 1,
}
impl From<HSMCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSMCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSMCLK_ON`"]
pub type HSMCLK_ON_R = crate::R<bool, HSMCLK_ON_A>;
impl HSMCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMCLK_ON_A {
        match self.bits {
            false => HSMCLK_ON_A::HSMCLK_ON_0,
            true => HSMCLK_ON_A::HSMCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSMCLK_ON_0`"]
    #[inline(always)]
    pub fn is_hsmclk_on_0(&self) -> bool {
        *self == HSMCLK_ON_A::HSMCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `HSMCLK_ON_1`"]
    #[inline(always)]
    pub fn is_hsmclk_on_1(&self) -> bool {
        *self == HSMCLK_ON_A::HSMCLK_ON_1
    }
}
#[doc = "SMCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLK_ON_A {
    #[doc = "0: Inactive"]
    SMCLK_ON_0 = 0,
    #[doc = "1: Active"]
    SMCLK_ON_1 = 1,
}
impl From<SMCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLK_ON`"]
pub type SMCLK_ON_R = crate::R<bool, SMCLK_ON_A>;
impl SMCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLK_ON_A {
        match self.bits {
            false => SMCLK_ON_A::SMCLK_ON_0,
            true => SMCLK_ON_A::SMCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLK_ON_0`"]
    #[inline(always)]
    pub fn is_smclk_on_0(&self) -> bool {
        *self == SMCLK_ON_A::SMCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `SMCLK_ON_1`"]
    #[inline(always)]
    pub fn is_smclk_on_1(&self) -> bool {
        *self == SMCLK_ON_A::SMCLK_ON_1
    }
}
#[doc = "MODCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODCLK_ON_A {
    #[doc = "0: Inactive"]
    MODCLK_ON_0 = 0,
    #[doc = "1: Active"]
    MODCLK_ON_1 = 1,
}
impl From<MODCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: MODCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODCLK_ON`"]
pub type MODCLK_ON_R = crate::R<bool, MODCLK_ON_A>;
impl MODCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODCLK_ON_A {
        match self.bits {
            false => MODCLK_ON_A::MODCLK_ON_0,
            true => MODCLK_ON_A::MODCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODCLK_ON_0`"]
    #[inline(always)]
    pub fn is_modclk_on_0(&self) -> bool {
        *self == MODCLK_ON_A::MODCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `MODCLK_ON_1`"]
    #[inline(always)]
    pub fn is_modclk_on_1(&self) -> bool {
        *self == MODCLK_ON_A::MODCLK_ON_1
    }
}
#[doc = "VLOCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLOCLK_ON_A {
    #[doc = "0: Inactive"]
    VLOCLK_ON_0 = 0,
    #[doc = "1: Active"]
    VLOCLK_ON_1 = 1,
}
impl From<VLOCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: VLOCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLOCLK_ON`"]
pub type VLOCLK_ON_R = crate::R<bool, VLOCLK_ON_A>;
impl VLOCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLOCLK_ON_A {
        match self.bits {
            false => VLOCLK_ON_A::VLOCLK_ON_0,
            true => VLOCLK_ON_A::VLOCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLOCLK_ON_0`"]
    #[inline(always)]
    pub fn is_vloclk_on_0(&self) -> bool {
        *self == VLOCLK_ON_A::VLOCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `VLOCLK_ON_1`"]
    #[inline(always)]
    pub fn is_vloclk_on_1(&self) -> bool {
        *self == VLOCLK_ON_A::VLOCLK_ON_1
    }
}
#[doc = "LFXTCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTCLK_ON_A {
    #[doc = "0: Inactive"]
    LFXTCLK_ON_0 = 0,
    #[doc = "1: Active"]
    LFXTCLK_ON_1 = 1,
}
impl From<LFXTCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTCLK_ON`"]
pub type LFXTCLK_ON_R = crate::R<bool, LFXTCLK_ON_A>;
impl LFXTCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTCLK_ON_A {
        match self.bits {
            false => LFXTCLK_ON_A::LFXTCLK_ON_0,
            true => LFXTCLK_ON_A::LFXTCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTCLK_ON_0`"]
    #[inline(always)]
    pub fn is_lfxtclk_on_0(&self) -> bool {
        *self == LFXTCLK_ON_A::LFXTCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `LFXTCLK_ON_1`"]
    #[inline(always)]
    pub fn is_lfxtclk_on_1(&self) -> bool {
        *self == LFXTCLK_ON_A::LFXTCLK_ON_1
    }
}
#[doc = "REFOCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOCLK_ON_A {
    #[doc = "0: Inactive"]
    REFOCLK_ON_0 = 0,
    #[doc = "1: Active"]
    REFOCLK_ON_1 = 1,
}
impl From<REFOCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: REFOCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFOCLK_ON`"]
pub type REFOCLK_ON_R = crate::R<bool, REFOCLK_ON_A>;
impl REFOCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOCLK_ON_A {
        match self.bits {
            false => REFOCLK_ON_A::REFOCLK_ON_0,
            true => REFOCLK_ON_A::REFOCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOCLK_ON_0`"]
    #[inline(always)]
    pub fn is_refoclk_on_0(&self) -> bool {
        *self == REFOCLK_ON_A::REFOCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `REFOCLK_ON_1`"]
    #[inline(always)]
    pub fn is_refoclk_on_1(&self) -> bool {
        *self == REFOCLK_ON_A::REFOCLK_ON_1
    }
}
#[doc = "ACLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACLK_READY_A {
    #[doc = "0: Not ready"]
    ACLK_READY_0 = 0,
    #[doc = "1: Ready"]
    ACLK_READY_1 = 1,
}
impl From<ACLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: ACLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACLK_READY`"]
pub type ACLK_READY_R = crate::R<bool, ACLK_READY_A>;
impl ACLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLK_READY_A {
        match self.bits {
            false => ACLK_READY_A::ACLK_READY_0,
            true => ACLK_READY_A::ACLK_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACLK_READY_0`"]
    #[inline(always)]
    pub fn is_aclk_ready_0(&self) -> bool {
        *self == ACLK_READY_A::ACLK_READY_0
    }
    #[doc = "Checks if the value of the field is `ACLK_READY_1`"]
    #[inline(always)]
    pub fn is_aclk_ready_1(&self) -> bool {
        *self == ACLK_READY_A::ACLK_READY_1
    }
}
#[doc = "MCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_READY_A {
    #[doc = "0: Not ready"]
    MCLK_READY_0 = 0,
    #[doc = "1: Ready"]
    MCLK_READY_1 = 1,
}
impl From<MCLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLK_READY`"]
pub type MCLK_READY_R = crate::R<bool, MCLK_READY_A>;
impl MCLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_READY_A {
        match self.bits {
            false => MCLK_READY_A::MCLK_READY_0,
            true => MCLK_READY_A::MCLK_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLK_READY_0`"]
    #[inline(always)]
    pub fn is_mclk_ready_0(&self) -> bool {
        *self == MCLK_READY_A::MCLK_READY_0
    }
    #[doc = "Checks if the value of the field is `MCLK_READY_1`"]
    #[inline(always)]
    pub fn is_mclk_ready_1(&self) -> bool {
        *self == MCLK_READY_A::MCLK_READY_1
    }
}
#[doc = "HSMCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMCLK_READY_A {
    #[doc = "0: Not ready"]
    HSMCLK_READY_0 = 0,
    #[doc = "1: Ready"]
    HSMCLK_READY_1 = 1,
}
impl From<HSMCLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: HSMCLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSMCLK_READY`"]
pub type HSMCLK_READY_R = crate::R<bool, HSMCLK_READY_A>;
impl HSMCLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMCLK_READY_A {
        match self.bits {
            false => HSMCLK_READY_A::HSMCLK_READY_0,
            true => HSMCLK_READY_A::HSMCLK_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSMCLK_READY_0`"]
    #[inline(always)]
    pub fn is_hsmclk_ready_0(&self) -> bool {
        *self == HSMCLK_READY_A::HSMCLK_READY_0
    }
    #[doc = "Checks if the value of the field is `HSMCLK_READY_1`"]
    #[inline(always)]
    pub fn is_hsmclk_ready_1(&self) -> bool {
        *self == HSMCLK_READY_A::HSMCLK_READY_1
    }
}
#[doc = "SMCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLK_READY_A {
    #[doc = "0: Not ready"]
    SMCLK_READY_0 = 0,
    #[doc = "1: Ready"]
    SMCLK_READY_1 = 1,
}
impl From<SMCLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLK_READY`"]
pub type SMCLK_READY_R = crate::R<bool, SMCLK_READY_A>;
impl SMCLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLK_READY_A {
        match self.bits {
            false => SMCLK_READY_A::SMCLK_READY_0,
            true => SMCLK_READY_A::SMCLK_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLK_READY_0`"]
    #[inline(always)]
    pub fn is_smclk_ready_0(&self) -> bool {
        *self == SMCLK_READY_A::SMCLK_READY_0
    }
    #[doc = "Checks if the value of the field is `SMCLK_READY_1`"]
    #[inline(always)]
    pub fn is_smclk_ready_1(&self) -> bool {
        *self == SMCLK_READY_A::SMCLK_READY_1
    }
}
#[doc = "BCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLK_READY_A {
    #[doc = "0: Not ready"]
    BCLK_READY_0 = 0,
    #[doc = "1: Ready"]
    BCLK_READY_1 = 1,
}
impl From<BCLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: BCLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCLK_READY`"]
pub type BCLK_READY_R = crate::R<bool, BCLK_READY_A>;
impl BCLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCLK_READY_A {
        match self.bits {
            false => BCLK_READY_A::BCLK_READY_0,
            true => BCLK_READY_A::BCLK_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCLK_READY_0`"]
    #[inline(always)]
    pub fn is_bclk_ready_0(&self) -> bool {
        *self == BCLK_READY_A::BCLK_READY_0
    }
    #[doc = "Checks if the value of the field is `BCLK_READY_1`"]
    #[inline(always)]
    pub fn is_bclk_ready_1(&self) -> bool {
        *self == BCLK_READY_A::BCLK_READY_1
    }
}
impl R {
    #[doc = "Bit 0 - DCO status"]
    #[inline(always)]
    pub fn dco_on(&self) -> DCO_ON_R {
        DCO_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCO bias status"]
    #[inline(always)]
    pub fn dcobias_on(&self) -> DCOBIAS_ON_R {
        DCOBIAS_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXT status"]
    #[inline(always)]
    pub fn hfxt_on(&self) -> HFXT_ON_R {
        HFXT_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFXT2 status"]
    #[inline(always)]
    pub fn hfxt2_on(&self) -> HFXT2_ON_R {
        HFXT2_ON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MODOSC status"]
    #[inline(always)]
    pub fn modosc_on(&self) -> MODOSC_ON_R {
        MODOSC_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VLO status"]
    #[inline(always)]
    pub fn vlo_on(&self) -> VLO_ON_R {
        VLO_ON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFXT status"]
    #[inline(always)]
    pub fn lfxt_on(&self) -> LFXT_ON_R {
        LFXT_ON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - REFO status"]
    #[inline(always)]
    pub fn refo_on(&self) -> REFO_ON_R {
        REFO_ON_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ACLK system clock status"]
    #[inline(always)]
    pub fn aclk_on(&self) -> ACLK_ON_R {
        ACLK_ON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MCLK system clock status"]
    #[inline(always)]
    pub fn mclk_on(&self) -> MCLK_ON_R {
        MCLK_ON_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSMCLK system clock status"]
    #[inline(always)]
    pub fn hsmclk_on(&self) -> HSMCLK_ON_R {
        HSMCLK_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SMCLK system clock status"]
    #[inline(always)]
    pub fn smclk_on(&self) -> SMCLK_ON_R {
        SMCLK_ON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MODCLK system clock status"]
    #[inline(always)]
    pub fn modclk_on(&self) -> MODCLK_ON_R {
        MODCLK_ON_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VLOCLK system clock status"]
    #[inline(always)]
    pub fn vloclk_on(&self) -> VLOCLK_ON_R {
        VLOCLK_ON_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LFXTCLK system clock status"]
    #[inline(always)]
    pub fn lfxtclk_on(&self) -> LFXTCLK_ON_R {
        LFXTCLK_ON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - REFOCLK system clock status"]
    #[inline(always)]
    pub fn refoclk_on(&self) -> REFOCLK_ON_R {
        REFOCLK_ON_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ACLK Ready status"]
    #[inline(always)]
    pub fn aclk_ready(&self) -> ACLK_READY_R {
        ACLK_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MCLK Ready status"]
    #[inline(always)]
    pub fn mclk_ready(&self) -> MCLK_READY_R {
        MCLK_READY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HSMCLK Ready status"]
    #[inline(always)]
    pub fn hsmclk_ready(&self) -> HSMCLK_READY_R {
        HSMCLK_READY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SMCLK Ready status"]
    #[inline(always)]
    pub fn smclk_ready(&self) -> SMCLK_READY_R {
        SMCLK_READY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - BCLK Ready status"]
    #[inline(always)]
    pub fn bclk_ready(&self) -> BCLK_READY_R {
        BCLK_READY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
