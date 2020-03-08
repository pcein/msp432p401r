#[doc = "Reader of register PCMCTL1"]
pub type R = crate::R<u32, super::PCMCTL1>;
#[doc = "Writer for register PCMCTL1"]
pub type W = crate::W<u32, super::PCMCTL1>;
#[doc = "Register PCMCTL1 `reset()`'s with value 0xa596_0000"]
impl crate::ResetValue for super::PCMCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa596_0000
    }
}
#[doc = "Lock LPM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKLPM5_A {
    #[doc = "0: LPMx.5 configuration defaults to reset condition"]
    LOCKLPM5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    LOCKLPM5_1 = 1,
}
impl From<LOCKLPM5_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKLPM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKLPM5`"]
pub type LOCKLPM5_R = crate::R<bool, LOCKLPM5_A>;
impl LOCKLPM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKLPM5_A {
        match self.bits {
            false => LOCKLPM5_A::LOCKLPM5_0,
            true => LOCKLPM5_A::LOCKLPM5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_0`"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_0
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_1`"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_1
    }
}
#[doc = "Write proxy for field `LOCKLPM5`"]
pub struct LOCKLPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLPM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKLPM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPMx.5 configuration defaults to reset condition"]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_0)
    }
    #[doc = "LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Lock Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKBKUP_A {
    #[doc = "0: Backup domain configuration defaults to reset condition"]
    LOCKBKUP_0 = 0,
    #[doc = "1: Backup domain configuration remains locked during LPM3.5 entry and exit"]
    LOCKBKUP_1 = 1,
}
impl From<LOCKBKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKBKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKBKUP`"]
pub type LOCKBKUP_R = crate::R<bool, LOCKBKUP_A>;
impl LOCKBKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKBKUP_A {
        match self.bits {
            false => LOCKBKUP_A::LOCKBKUP_0,
            true => LOCKBKUP_A::LOCKBKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKBKUP_0`"]
    #[inline(always)]
    pub fn is_lockbkup_0(&self) -> bool {
        *self == LOCKBKUP_A::LOCKBKUP_0
    }
    #[doc = "Checks if the value of the field is `LOCKBKUP_1`"]
    #[inline(always)]
    pub fn is_lockbkup_1(&self) -> bool {
        *self == LOCKBKUP_A::LOCKBKUP_1
    }
}
#[doc = "Write proxy for field `LOCKBKUP`"]
pub struct LOCKBKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKBKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKBKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Backup domain configuration defaults to reset condition"]
    #[inline(always)]
    pub fn lockbkup_0(self) -> &'a mut W {
        self.variant(LOCKBKUP_A::LOCKBKUP_0)
    }
    #[doc = "Backup domain configuration remains locked during LPM3.5 entry and exit"]
    #[inline(always)]
    pub fn lockbkup_1(self) -> &'a mut W {
        self.variant(LOCKBKUP_A::LOCKBKUP_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Force LPM entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_LPM_ENTRY_A {
    #[doc = "0: PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    FORCE_LPM_ENTRY_0 = 0,
    #[doc = "1: PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    FORCE_LPM_ENTRY_1 = 1,
}
impl From<FORCE_LPM_ENTRY_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_LPM_ENTRY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCE_LPM_ENTRY`"]
pub type FORCE_LPM_ENTRY_R = crate::R<bool, FORCE_LPM_ENTRY_A>;
impl FORCE_LPM_ENTRY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_LPM_ENTRY_A {
        match self.bits {
            false => FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0,
            true => FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_LPM_ENTRY_0`"]
    #[inline(always)]
    pub fn is_force_lpm_entry_0(&self) -> bool {
        *self == FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0
    }
    #[doc = "Checks if the value of the field is `FORCE_LPM_ENTRY_1`"]
    #[inline(always)]
    pub fn is_force_lpm_entry_1(&self) -> bool {
        *self == FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1
    }
}
#[doc = "Write proxy for field `FORCE_LPM_ENTRY`"]
pub struct FORCE_LPM_ENTRY_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_LPM_ENTRY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_LPM_ENTRY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    #[inline(always)]
    pub fn force_lpm_entry_0(self) -> &'a mut W {
        self.variant(FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0)
    }
    #[doc = "PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    #[inline(always)]
    pub fn force_lpm_entry_1(self) -> &'a mut W {
        self.variant(FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PMR_BUSY`"]
pub type PMR_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMR_BUSY`"]
pub struct PMR_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> PMR_BUSY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCMKEY`"]
pub type PCMKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCMKEY`"]
pub struct PCMKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&self) -> LOCKBKUP_R {
        LOCKBKUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&self) -> FORCE_LPM_ENTRY_R {
        FORCE_LPM_ENTRY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&self) -> PMR_BUSY_R {
        PMR_BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PCMKEY_R {
        PCMKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W {
        LOCKLPM5_W { w: self }
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&mut self) -> LOCKBKUP_W {
        LOCKBKUP_W { w: self }
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&mut self) -> FORCE_LPM_ENTRY_W {
        FORCE_LPM_ENTRY_W { w: self }
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&mut self) -> PMR_BUSY_W {
        PMR_BUSY_W { w: self }
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PCMKEY_W {
        PCMKEY_W { w: self }
    }
}
