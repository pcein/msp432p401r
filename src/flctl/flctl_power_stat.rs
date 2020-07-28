#[doc = "Reader of register FLCTL_POWER_STAT"]
pub type R = crate::R<u32, super::FLCTL_POWER_STAT>;
#[doc = "Flash power status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSTAT_A {
    #[doc = "0: Flash IP in power-down mode"]
    PSTAT_0 = 0,
    #[doc = "1: Flash IP Vdd domain power-up in progress"]
    PSTAT_1 = 1,
    #[doc = "2: PSS LDO_GOOD, IREF_OK and VREF_OK check in progress"]
    PSTAT_2 = 2,
    #[doc = "3: Flash IP SAFE_LV check in progress"]
    PSTAT_3 = 3,
    #[doc = "4: Flash IP Active"]
    PSTAT_4 = 4,
    #[doc = "5: Flash IP Active in Low-Frequency Active and Low-Frequency LPM0 modes."]
    PSTAT_5 = 5,
    #[doc = "6: Flash IP in Standby mode"]
    PSTAT_6 = 6,
    #[doc = "7: Flash IP in Current mirror boost state"]
    PSTAT_7 = 7,
}
impl From<PSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSTAT`"]
pub type PSTAT_R = crate::R<u8, PSTAT_A>;
impl PSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSTAT_A {
        match self.bits {
            0 => PSTAT_A::PSTAT_0,
            1 => PSTAT_A::PSTAT_1,
            2 => PSTAT_A::PSTAT_2,
            3 => PSTAT_A::PSTAT_3,
            4 => PSTAT_A::PSTAT_4,
            5 => PSTAT_A::PSTAT_5,
            6 => PSTAT_A::PSTAT_6,
            7 => PSTAT_A::PSTAT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSTAT_0`"]
    #[inline(always)]
    pub fn is_pstat_0(&self) -> bool {
        *self == PSTAT_A::PSTAT_0
    }
    #[doc = "Checks if the value of the field is `PSTAT_1`"]
    #[inline(always)]
    pub fn is_pstat_1(&self) -> bool {
        *self == PSTAT_A::PSTAT_1
    }
    #[doc = "Checks if the value of the field is `PSTAT_2`"]
    #[inline(always)]
    pub fn is_pstat_2(&self) -> bool {
        *self == PSTAT_A::PSTAT_2
    }
    #[doc = "Checks if the value of the field is `PSTAT_3`"]
    #[inline(always)]
    pub fn is_pstat_3(&self) -> bool {
        *self == PSTAT_A::PSTAT_3
    }
    #[doc = "Checks if the value of the field is `PSTAT_4`"]
    #[inline(always)]
    pub fn is_pstat_4(&self) -> bool {
        *self == PSTAT_A::PSTAT_4
    }
    #[doc = "Checks if the value of the field is `PSTAT_5`"]
    #[inline(always)]
    pub fn is_pstat_5(&self) -> bool {
        *self == PSTAT_A::PSTAT_5
    }
    #[doc = "Checks if the value of the field is `PSTAT_6`"]
    #[inline(always)]
    pub fn is_pstat_6(&self) -> bool {
        *self == PSTAT_A::PSTAT_6
    }
    #[doc = "Checks if the value of the field is `PSTAT_7`"]
    #[inline(always)]
    pub fn is_pstat_7(&self) -> bool {
        *self == PSTAT_A::PSTAT_7
    }
}
#[doc = "PSS FLDO GOOD status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOSTAT_A {
    #[doc = "0: FLDO not GOOD"]
    LDOSTAT_0 = 0,
    #[doc = "1: FLDO GOOD"]
    LDOSTAT_1 = 1,
}
impl From<LDOSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: LDOSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDOSTAT`"]
pub type LDOSTAT_R = crate::R<bool, LDOSTAT_A>;
impl LDOSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOSTAT_A {
        match self.bits {
            false => LDOSTAT_A::LDOSTAT_0,
            true => LDOSTAT_A::LDOSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LDOSTAT_0`"]
    #[inline(always)]
    pub fn is_ldostat_0(&self) -> bool {
        *self == LDOSTAT_A::LDOSTAT_0
    }
    #[doc = "Checks if the value of the field is `LDOSTAT_1`"]
    #[inline(always)]
    pub fn is_ldostat_1(&self) -> bool {
        *self == LDOSTAT_A::LDOSTAT_1
    }
}
#[doc = "PSS VREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFSTAT_A {
    #[doc = "0: Flash LDO not stable"]
    VREFSTAT_0 = 0,
    #[doc = "1: Flash LDO stable"]
    VREFSTAT_1 = 1,
}
impl From<VREFSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: VREFSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFSTAT`"]
pub type VREFSTAT_R = crate::R<bool, VREFSTAT_A>;
impl VREFSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFSTAT_A {
        match self.bits {
            false => VREFSTAT_A::VREFSTAT_0,
            true => VREFSTAT_A::VREFSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREFSTAT_0`"]
    #[inline(always)]
    pub fn is_vrefstat_0(&self) -> bool {
        *self == VREFSTAT_A::VREFSTAT_0
    }
    #[doc = "Checks if the value of the field is `VREFSTAT_1`"]
    #[inline(always)]
    pub fn is_vrefstat_1(&self) -> bool {
        *self == VREFSTAT_A::VREFSTAT_1
    }
}
#[doc = "PSS IREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTAT_A {
    #[doc = "0: IREF not stable"]
    IREFSTAT_0 = 0,
    #[doc = "1: IREF stable"]
    IREFSTAT_1 = 1,
}
impl From<IREFSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: IREFSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IREFSTAT`"]
pub type IREFSTAT_R = crate::R<bool, IREFSTAT_A>;
impl IREFSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFSTAT_A {
        match self.bits {
            false => IREFSTAT_A::IREFSTAT_0,
            true => IREFSTAT_A::IREFSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREFSTAT_0`"]
    #[inline(always)]
    pub fn is_irefstat_0(&self) -> bool {
        *self == IREFSTAT_A::IREFSTAT_0
    }
    #[doc = "Checks if the value of the field is `IREFSTAT_1`"]
    #[inline(always)]
    pub fn is_irefstat_1(&self) -> bool {
        *self == IREFSTAT_A::IREFSTAT_1
    }
}
#[doc = "PSS trim done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMSTAT_A {
    #[doc = "0: PSS trim not complete"]
    TRIMSTAT_0 = 0,
    #[doc = "1: PSS trim complete"]
    TRIMSTAT_1 = 1,
}
impl From<TRIMSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: TRIMSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIMSTAT`"]
pub type TRIMSTAT_R = crate::R<bool, TRIMSTAT_A>;
impl TRIMSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIMSTAT_A {
        match self.bits {
            false => TRIMSTAT_A::TRIMSTAT_0,
            true => TRIMSTAT_A::TRIMSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIMSTAT_0`"]
    #[inline(always)]
    pub fn is_trimstat_0(&self) -> bool {
        *self == TRIMSTAT_A::TRIMSTAT_0
    }
    #[doc = "Checks if the value of the field is `TRIMSTAT_1`"]
    #[inline(always)]
    pub fn is_trimstat_1(&self) -> bool {
        *self == TRIMSTAT_A::TRIMSTAT_1
    }
}
#[doc = "Indicates if Flash is being accessed in 2T mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_2T_A {
    #[doc = "0: Flash reads are in 1T mode"]
    RD_2T_0 = 0,
    #[doc = "1: Flash reads are in 2T mode"]
    RD_2T_1 = 1,
}
impl From<RD_2T_A> for bool {
    #[inline(always)]
    fn from(variant: RD_2T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RD_2T`"]
pub type RD_2T_R = crate::R<bool, RD_2T_A>;
impl RD_2T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_2T_A {
        match self.bits {
            false => RD_2T_A::RD_2T_0,
            true => RD_2T_A::RD_2T_1,
        }
    }
    #[doc = "Checks if the value of the field is `RD_2T_0`"]
    #[inline(always)]
    pub fn is_rd_2t_0(&self) -> bool {
        *self == RD_2T_A::RD_2T_0
    }
    #[doc = "Checks if the value of the field is `RD_2T_1`"]
    #[inline(always)]
    pub fn is_rd_2t_1(&self) -> bool {
        *self == RD_2T_A::RD_2T_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash power status"]
    #[inline(always)]
    pub fn pstat(&self) -> PSTAT_R {
        PSTAT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - PSS FLDO GOOD status"]
    #[inline(always)]
    pub fn ldostat(&self) -> LDOSTAT_R {
        LDOSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PSS VREF stable status"]
    #[inline(always)]
    pub fn vrefstat(&self) -> VREFSTAT_R {
        VREFSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PSS IREF stable status"]
    #[inline(always)]
    pub fn irefstat(&self) -> IREFSTAT_R {
        IREFSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PSS trim done status"]
    #[inline(always)]
    pub fn trimstat(&self) -> TRIMSTAT_R {
        TRIMSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates if Flash is being accessed in 2T mode"]
    #[inline(always)]
    pub fn rd_2t(&self) -> RD_2T_R {
        RD_2T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
