#[doc = "Reader of register PMAPCTL"]
pub type R = crate::R<u16, super::PMAPCTL>;
#[doc = "Writer for register PMAPCTL"]
pub type W = crate::W<u16, super::PMAPCTL>;
#[doc = "Register PMAPCTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PMAPCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Port mapping lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAPLOCKED_A {
    #[doc = "0: Access to mapping registers is granted"]
    PMAPLOCKED_0 = 0,
    #[doc = "1: Access to mapping registers is locked"]
    PMAPLOCKED_1 = 1,
}
impl From<PMAPLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: PMAPLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMAPLOCKED`"]
pub type PMAPLOCKED_R = crate::R<bool, PMAPLOCKED_A>;
impl PMAPLOCKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAPLOCKED_A {
        match self.bits {
            false => PMAPLOCKED_A::PMAPLOCKED_0,
            true => PMAPLOCKED_A::PMAPLOCKED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMAPLOCKED_0`"]
    #[inline(always)]
    pub fn is_pmaplocked_0(&self) -> bool {
        *self == PMAPLOCKED_A::PMAPLOCKED_0
    }
    #[doc = "Checks if the value of the field is `PMAPLOCKED_1`"]
    #[inline(always)]
    pub fn is_pmaplocked_1(&self) -> bool {
        *self == PMAPLOCKED_A::PMAPLOCKED_1
    }
}
#[doc = "Port mapping reconfiguration control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAPRECFG_A {
    #[doc = "0: Configuration allowed only once"]
    PMAPRECFG_0 = 0,
    #[doc = "1: Allow reconfiguration of port mapping"]
    PMAPRECFG_1 = 1,
}
impl From<PMAPRECFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMAPRECFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMAPRECFG`"]
pub type PMAPRECFG_R = crate::R<bool, PMAPRECFG_A>;
impl PMAPRECFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAPRECFG_A {
        match self.bits {
            false => PMAPRECFG_A::PMAPRECFG_0,
            true => PMAPRECFG_A::PMAPRECFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMAPRECFG_0`"]
    #[inline(always)]
    pub fn is_pmaprecfg_0(&self) -> bool {
        *self == PMAPRECFG_A::PMAPRECFG_0
    }
    #[doc = "Checks if the value of the field is `PMAPRECFG_1`"]
    #[inline(always)]
    pub fn is_pmaprecfg_1(&self) -> bool {
        *self == PMAPRECFG_A::PMAPRECFG_1
    }
}
#[doc = "Write proxy for field `PMAPRECFG`"]
pub struct PMAPRECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPRECFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMAPRECFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configuration allowed only once"]
    #[inline(always)]
    pub fn pmaprecfg_0(self) -> &'a mut W {
        self.variant(PMAPRECFG_A::PMAPRECFG_0)
    }
    #[doc = "Allow reconfiguration of port mapping"]
    #[inline(always)]
    pub fn pmaprecfg_1(self) -> &'a mut W {
        self.variant(PMAPRECFG_A::PMAPRECFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port mapping lock bit"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PMAPLOCKED_R {
        PMAPLOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PMAPRECFG_R {
        PMAPRECFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&mut self) -> PMAPRECFG_W {
        PMAPRECFG_W { w: self }
    }
}
