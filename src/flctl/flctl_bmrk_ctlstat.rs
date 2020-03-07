#[doc = "Reader of register FLCTL_BMRK_CTLSTAT"]
pub type R = crate::R<u32, super::FLCTL_BMRK_CTLSTAT>;
#[doc = "Writer for register FLCTL_BMRK_CTLSTAT"]
pub type W = crate::W<u32, super::FLCTL_BMRK_CTLSTAT>;
#[doc = "Register FLCTL_BMRK_CTLSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLCTL_BMRK_CTLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I_BMRK`"]
pub type I_BMRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I_BMRK`"]
pub struct I_BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> I_BMRK_W<'a> {
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
#[doc = "Reader of field `D_BMRK`"]
pub type D_BMRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D_BMRK`"]
pub struct D_BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> D_BMRK_W<'a> {
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
#[doc = "Reader of field `CMP_EN`"]
pub type CMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP_EN`"]
pub struct CMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EN_W<'a> {
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
#[doc = "Selects which benchmark register should be compared against the threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_SEL_A {
    #[doc = "0: Compares the Instruction Benchmark Register against the threshold value"]
    EN_1_0X0 = 0,
    #[doc = "1: Compares the Data Benchmark Register against the threshold value"]
    EN_2_0X1 = 1,
}
impl From<CMP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP_SEL`"]
pub type CMP_SEL_R = crate::R<bool, CMP_SEL_A>;
impl CMP_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_SEL_A {
        match self.bits {
            false => CMP_SEL_A::EN_1_0X0,
            true => CMP_SEL_A::EN_2_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_1_0X0`"]
    #[inline(always)]
    pub fn is_en_1_0x0(&self) -> bool {
        *self == CMP_SEL_A::EN_1_0X0
    }
    #[doc = "Checks if the value of the field is `EN_2_0X1`"]
    #[inline(always)]
    pub fn is_en_2_0x1(&self) -> bool {
        *self == CMP_SEL_A::EN_2_0X1
    }
}
#[doc = "Write proxy for field `CMP_SEL`"]
pub struct CMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compares the Instruction Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_1_0x0(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_1_0X0)
    }
    #[doc = "Compares the Data Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_2_0x1(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_2_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&self) -> I_BMRK_R {
        I_BMRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&self) -> D_BMRK_R {
        D_BMRK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CMP_EN_R {
        CMP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&self) -> CMP_SEL_R {
        CMP_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&mut self) -> I_BMRK_W {
        I_BMRK_W { w: self }
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&mut self) -> D_BMRK_W {
        D_BMRK_W { w: self }
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&mut self) -> CMP_EN_W {
        CMP_EN_W { w: self }
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&mut self) -> CMP_SEL_W {
        CMP_SEL_W { w: self }
    }
}
