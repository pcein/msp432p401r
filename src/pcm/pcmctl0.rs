#[doc = "Reader of register PCMCTL0"]
pub type R = crate::R<u32, super::PCMCTL0>;
#[doc = "Writer for register PCMCTL0"]
pub type W = crate::W<u32, super::PCMCTL0>;
#[doc = "Register PCMCTL0 `reset()`'s with value 0xa596_0000"]
impl crate::ResetValue for super::PCMCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa596_0000
    }
}
#[doc = "Active Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMR_A {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    AMR_0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    AMR_1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    AMR_4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    AMR_5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    AMR_8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    AMR_9 = 9,
}
impl From<AMR_A> for u8 {
    #[inline(always)]
    fn from(variant: AMR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AMR`"]
pub type AMR_R = crate::R<u8, AMR_A>;
impl AMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AMR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AMR_A::AMR_0),
            1 => Val(AMR_A::AMR_1),
            4 => Val(AMR_A::AMR_4),
            5 => Val(AMR_A::AMR_5),
            8 => Val(AMR_A::AMR_8),
            9 => Val(AMR_A::AMR_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AMR_0`"]
    #[inline(always)]
    pub fn is_amr_0(&self) -> bool {
        *self == AMR_A::AMR_0
    }
    #[doc = "Checks if the value of the field is `AMR_1`"]
    #[inline(always)]
    pub fn is_amr_1(&self) -> bool {
        *self == AMR_A::AMR_1
    }
    #[doc = "Checks if the value of the field is `AMR_4`"]
    #[inline(always)]
    pub fn is_amr_4(&self) -> bool {
        *self == AMR_A::AMR_4
    }
    #[doc = "Checks if the value of the field is `AMR_5`"]
    #[inline(always)]
    pub fn is_amr_5(&self) -> bool {
        *self == AMR_A::AMR_5
    }
    #[doc = "Checks if the value of the field is `AMR_8`"]
    #[inline(always)]
    pub fn is_amr_8(&self) -> bool {
        *self == AMR_A::AMR_8
    }
    #[doc = "Checks if the value of the field is `AMR_9`"]
    #[inline(always)]
    pub fn is_amr_9(&self) -> bool {
        *self == AMR_A::AMR_9
    }
}
#[doc = "Write proxy for field `AMR`"]
pub struct AMR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LDO based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_0(self) -> &'a mut W {
        self.variant(AMR_A::AMR_0)
    }
    #[doc = "LDO based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_1(self) -> &'a mut W {
        self.variant(AMR_A::AMR_1)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_4(self) -> &'a mut W {
        self.variant(AMR_A::AMR_4)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_5(self) -> &'a mut W {
        self.variant(AMR_A::AMR_5)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_8(self) -> &'a mut W {
        self.variant(AMR_A::AMR_8)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_9(self) -> &'a mut W {
        self.variant(AMR_A::AMR_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Low Power Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMR_A {
    #[doc = "0: LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    LPMR_0 = 0,
    #[doc = "10: LPM3.5. Core voltage setting 0."]
    LPMR_10 = 10,
    #[doc = "12: LPM4.5"]
    LPMR_12 = 12,
}
impl From<LPMR_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMR`"]
pub type LPMR_R = crate::R<u8, LPMR_A>;
impl LPMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPMR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPMR_A::LPMR_0),
            10 => Val(LPMR_A::LPMR_10),
            12 => Val(LPMR_A::LPMR_12),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPMR_0`"]
    #[inline(always)]
    pub fn is_lpmr_0(&self) -> bool {
        *self == LPMR_A::LPMR_0
    }
    #[doc = "Checks if the value of the field is `LPMR_10`"]
    #[inline(always)]
    pub fn is_lpmr_10(&self) -> bool {
        *self == LPMR_A::LPMR_10
    }
    #[doc = "Checks if the value of the field is `LPMR_12`"]
    #[inline(always)]
    pub fn is_lpmr_12(&self) -> bool {
        *self == LPMR_A::LPMR_12
    }
}
#[doc = "Write proxy for field `LPMR`"]
pub struct LPMR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    #[inline(always)]
    pub fn lpmr_0(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_0)
    }
    #[doc = "LPM3.5. Core voltage setting 0."]
    #[inline(always)]
    pub fn lpmr_10(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_10)
    }
    #[doc = "LPM4.5"]
    #[inline(always)]
    pub fn lpmr_12(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Current Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPM_A {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    CPM_0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    CPM_1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    CPM_4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    CPM_5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    CPM_8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    CPM_9 = 9,
    #[doc = "16: LDO based LPM0 at Core voltage setting 0."]
    CPM_16 = 16,
    #[doc = "17: LDO based LPM0 at Core voltage setting 1."]
    CPM_17 = 17,
    #[doc = "20: DC-DC based LPM0 at Core voltage setting 0."]
    CPM_20 = 20,
    #[doc = "21: DC-DC based LPM0 at Core voltage setting 1."]
    CPM_21 = 21,
    #[doc = "24: Low-Frequency LPM0 at Core voltage setting 0."]
    CPM_24 = 24,
    #[doc = "25: Low-Frequency LPM0 at Core voltage setting 1."]
    CPM_25 = 25,
    #[doc = "32: LPM3"]
    CPM_32 = 32,
}
impl From<CPM_A> for u8 {
    #[inline(always)]
    fn from(variant: CPM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPM`"]
pub type CPM_R = crate::R<u8, CPM_A>;
impl CPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPM_A::CPM_0),
            1 => Val(CPM_A::CPM_1),
            4 => Val(CPM_A::CPM_4),
            5 => Val(CPM_A::CPM_5),
            8 => Val(CPM_A::CPM_8),
            9 => Val(CPM_A::CPM_9),
            16 => Val(CPM_A::CPM_16),
            17 => Val(CPM_A::CPM_17),
            20 => Val(CPM_A::CPM_20),
            21 => Val(CPM_A::CPM_21),
            24 => Val(CPM_A::CPM_24),
            25 => Val(CPM_A::CPM_25),
            32 => Val(CPM_A::CPM_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPM_0`"]
    #[inline(always)]
    pub fn is_cpm_0(&self) -> bool {
        *self == CPM_A::CPM_0
    }
    #[doc = "Checks if the value of the field is `CPM_1`"]
    #[inline(always)]
    pub fn is_cpm_1(&self) -> bool {
        *self == CPM_A::CPM_1
    }
    #[doc = "Checks if the value of the field is `CPM_4`"]
    #[inline(always)]
    pub fn is_cpm_4(&self) -> bool {
        *self == CPM_A::CPM_4
    }
    #[doc = "Checks if the value of the field is `CPM_5`"]
    #[inline(always)]
    pub fn is_cpm_5(&self) -> bool {
        *self == CPM_A::CPM_5
    }
    #[doc = "Checks if the value of the field is `CPM_8`"]
    #[inline(always)]
    pub fn is_cpm_8(&self) -> bool {
        *self == CPM_A::CPM_8
    }
    #[doc = "Checks if the value of the field is `CPM_9`"]
    #[inline(always)]
    pub fn is_cpm_9(&self) -> bool {
        *self == CPM_A::CPM_9
    }
    #[doc = "Checks if the value of the field is `CPM_16`"]
    #[inline(always)]
    pub fn is_cpm_16(&self) -> bool {
        *self == CPM_A::CPM_16
    }
    #[doc = "Checks if the value of the field is `CPM_17`"]
    #[inline(always)]
    pub fn is_cpm_17(&self) -> bool {
        *self == CPM_A::CPM_17
    }
    #[doc = "Checks if the value of the field is `CPM_20`"]
    #[inline(always)]
    pub fn is_cpm_20(&self) -> bool {
        *self == CPM_A::CPM_20
    }
    #[doc = "Checks if the value of the field is `CPM_21`"]
    #[inline(always)]
    pub fn is_cpm_21(&self) -> bool {
        *self == CPM_A::CPM_21
    }
    #[doc = "Checks if the value of the field is `CPM_24`"]
    #[inline(always)]
    pub fn is_cpm_24(&self) -> bool {
        *self == CPM_A::CPM_24
    }
    #[doc = "Checks if the value of the field is `CPM_25`"]
    #[inline(always)]
    pub fn is_cpm_25(&self) -> bool {
        *self == CPM_A::CPM_25
    }
    #[doc = "Checks if the value of the field is `CPM_32`"]
    #[inline(always)]
    pub fn is_cpm_32(&self) -> bool {
        *self == CPM_A::CPM_32
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
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&self) -> AMR_R {
        AMR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&self) -> LPMR_R {
        LPMR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Current Power Mode"]
    #[inline(always)]
    pub fn cpm(&self) -> CPM_R {
        CPM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PCMKEY_R {
        PCMKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&mut self) -> AMR_W {
        AMR_W { w: self }
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&mut self) -> LPMR_W {
        LPMR_W { w: self }
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PCMKEY_W {
        PCMKEY_W { w: self }
    }
}
