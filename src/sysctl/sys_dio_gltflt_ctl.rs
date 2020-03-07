#[doc = "Reader of register SYS_DIO_GLTFLT_CTL"]
pub type R = crate::R<u32, super::SYS_DIO_GLTFLT_CTL>;
#[doc = "Writer for register SYS_DIO_GLTFLT_CTL"]
pub type W = crate::W<u32, super::SYS_DIO_GLTFLT_CTL>;
#[doc = "Register SYS_DIO_GLTFLT_CTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SYS_DIO_GLTFLT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Glitch filter enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLTCH_EN_A {
    #[doc = "0: Disables glitch filter on the digital I/Os"]
    GLTCH_EN_0 = 0,
    #[doc = "1: Enables glitch filter on the digital I/Os"]
    GLTCH_EN_1 = 1,
}
impl From<GLTCH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GLTCH_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GLTCH_EN`"]
pub type GLTCH_EN_R = crate::R<bool, GLTCH_EN_A>;
impl GLTCH_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLTCH_EN_A {
        match self.bits {
            false => GLTCH_EN_A::GLTCH_EN_0,
            true => GLTCH_EN_A::GLTCH_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GLTCH_EN_0`"]
    #[inline(always)]
    pub fn is_gltch_en_0(&self) -> bool {
        *self == GLTCH_EN_A::GLTCH_EN_0
    }
    #[doc = "Checks if the value of the field is `GLTCH_EN_1`"]
    #[inline(always)]
    pub fn is_gltch_en_1(&self) -> bool {
        *self == GLTCH_EN_A::GLTCH_EN_1
    }
}
#[doc = "Write proxy for field `GLTCH_EN`"]
pub struct GLTCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLTCH_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLTCH_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_0(self) -> &'a mut W {
        self.variant(GLTCH_EN_A::GLTCH_EN_0)
    }
    #[doc = "Enables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_1(self) -> &'a mut W {
        self.variant(GLTCH_EN_A::GLTCH_EN_1)
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
impl R {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&self) -> GLTCH_EN_R {
        GLTCH_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&mut self) -> GLTCH_EN_W {
        GLTCH_EN_W { w: self }
    }
}
