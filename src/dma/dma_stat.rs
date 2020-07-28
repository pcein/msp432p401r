#[doc = "Reader of register DMA_STAT"]
pub type R = crate::R<u32, super::DMA_STAT>;
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTEN_A {
    #[doc = "0: Controller disabled"]
    MASTEN_0 = 0,
    #[doc = "1: Controller enabled"]
    MASTEN_1 = 1,
}
impl From<MASTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MASTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTEN`"]
pub type MASTEN_R = crate::R<bool, MASTEN_A>;
impl MASTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTEN_A {
        match self.bits {
            false => MASTEN_A::MASTEN_0,
            true => MASTEN_A::MASTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTEN_0`"]
    #[inline(always)]
    pub fn is_masten_0(&self) -> bool {
        *self == MASTEN_A::MASTEN_0
    }
    #[doc = "Checks if the value of the field is `MASTEN_1`"]
    #[inline(always)]
    pub fn is_masten_1(&self) -> bool {
        *self == MASTEN_A::MASTEN_1
    }
}
#[doc = "Current state of the control state machine. State can be one of the following:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: idle"]
    STATE_0 = 0,
    #[doc = "1: reading channel controller data"]
    STATE_1 = 1,
    #[doc = "2: reading source data end pointer"]
    STATE_2 = 2,
    #[doc = "3: reading destination data end pointer"]
    STATE_3 = 3,
    #[doc = "4: reading source data"]
    STATE_4 = 4,
    #[doc = "5: writing destination data"]
    STATE_5 = 5,
    #[doc = "6: waiting for DMA request to clear"]
    STATE_6 = 6,
    #[doc = "7: writing channel controller data"]
    STATE_7 = 7,
    #[doc = "8: stalled"]
    STATE_8 = 8,
    #[doc = "9: done"]
    STATE_9 = 9,
    #[doc = "10: peripheral scatter-gather transition"]
    STATE_10 = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::STATE_0),
            1 => Val(STATE_A::STATE_1),
            2 => Val(STATE_A::STATE_2),
            3 => Val(STATE_A::STATE_3),
            4 => Val(STATE_A::STATE_4),
            5 => Val(STATE_A::STATE_5),
            6 => Val(STATE_A::STATE_6),
            7 => Val(STATE_A::STATE_7),
            8 => Val(STATE_A::STATE_8),
            9 => Val(STATE_A::STATE_9),
            10 => Val(STATE_A::STATE_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE_0`"]
    #[inline(always)]
    pub fn is_state_0(&self) -> bool {
        *self == STATE_A::STATE_0
    }
    #[doc = "Checks if the value of the field is `STATE_1`"]
    #[inline(always)]
    pub fn is_state_1(&self) -> bool {
        *self == STATE_A::STATE_1
    }
    #[doc = "Checks if the value of the field is `STATE_2`"]
    #[inline(always)]
    pub fn is_state_2(&self) -> bool {
        *self == STATE_A::STATE_2
    }
    #[doc = "Checks if the value of the field is `STATE_3`"]
    #[inline(always)]
    pub fn is_state_3(&self) -> bool {
        *self == STATE_A::STATE_3
    }
    #[doc = "Checks if the value of the field is `STATE_4`"]
    #[inline(always)]
    pub fn is_state_4(&self) -> bool {
        *self == STATE_A::STATE_4
    }
    #[doc = "Checks if the value of the field is `STATE_5`"]
    #[inline(always)]
    pub fn is_state_5(&self) -> bool {
        *self == STATE_A::STATE_5
    }
    #[doc = "Checks if the value of the field is `STATE_6`"]
    #[inline(always)]
    pub fn is_state_6(&self) -> bool {
        *self == STATE_A::STATE_6
    }
    #[doc = "Checks if the value of the field is `STATE_7`"]
    #[inline(always)]
    pub fn is_state_7(&self) -> bool {
        *self == STATE_A::STATE_7
    }
    #[doc = "Checks if the value of the field is `STATE_8`"]
    #[inline(always)]
    pub fn is_state_8(&self) -> bool {
        *self == STATE_A::STATE_8
    }
    #[doc = "Checks if the value of the field is `STATE_9`"]
    #[inline(always)]
    pub fn is_state_9(&self) -> bool {
        *self == STATE_A::STATE_9
    }
    #[doc = "Checks if the value of the field is `STATE_10`"]
    #[inline(always)]
    pub fn is_state_10(&self) -> bool {
        *self == STATE_A::STATE_10
    }
}
#[doc = "Number of available DMA channels minus one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMACHANS_A {
    #[doc = "0: Controller configured to use 1 DMA channel"]
    DMACHANS_0 = 0,
    #[doc = "1: Controller configured to use 2 DMA channels"]
    DMACHANS_1 = 1,
    #[doc = "30: Controller configured to use 31 DMA channels"]
    DMACHANS_30 = 30,
    #[doc = "31: Controller configured to use 32 DMA channels"]
    DMACHANS_31 = 31,
}
impl From<DMACHANS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMACHANS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMACHANS`"]
pub type DMACHANS_R = crate::R<u8, DMACHANS_A>;
impl DMACHANS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMACHANS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMACHANS_A::DMACHANS_0),
            1 => Val(DMACHANS_A::DMACHANS_1),
            30 => Val(DMACHANS_A::DMACHANS_30),
            31 => Val(DMACHANS_A::DMACHANS_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMACHANS_0`"]
    #[inline(always)]
    pub fn is_dmachans_0(&self) -> bool {
        *self == DMACHANS_A::DMACHANS_0
    }
    #[doc = "Checks if the value of the field is `DMACHANS_1`"]
    #[inline(always)]
    pub fn is_dmachans_1(&self) -> bool {
        *self == DMACHANS_A::DMACHANS_1
    }
    #[doc = "Checks if the value of the field is `DMACHANS_30`"]
    #[inline(always)]
    pub fn is_dmachans_30(&self) -> bool {
        *self == DMACHANS_A::DMACHANS_30
    }
    #[doc = "Checks if the value of the field is `DMACHANS_31`"]
    #[inline(always)]
    pub fn is_dmachans_31(&self) -> bool {
        *self == DMACHANS_A::DMACHANS_31
    }
}
#[doc = "To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TESTSTAT_A {
    #[doc = "0: Controller does not include the integration test logic"]
    TESTSTAT_0 = 0,
    #[doc = "1: Controller includes the integration test logic"]
    TESTSTAT_1 = 1,
}
impl From<TESTSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: TESTSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TESTSTAT`"]
pub type TESTSTAT_R = crate::R<u8, TESTSTAT_A>;
impl TESTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TESTSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TESTSTAT_A::TESTSTAT_0),
            1 => Val(TESTSTAT_A::TESTSTAT_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TESTSTAT_0`"]
    #[inline(always)]
    pub fn is_teststat_0(&self) -> bool {
        *self == TESTSTAT_A::TESTSTAT_0
    }
    #[doc = "Checks if the value of the field is `TESTSTAT_1`"]
    #[inline(always)]
    pub fn is_teststat_1(&self) -> bool {
        *self == TESTSTAT_A::TESTSTAT_1
    }
}
impl R {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn masten(&self) -> MASTEN_R {
        MASTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Current state of the control state machine. State can be one of the following:"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of available DMA channels minus one."]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved."]
    #[inline(always)]
    pub fn teststat(&self) -> TESTSTAT_R {
        TESTSTAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
