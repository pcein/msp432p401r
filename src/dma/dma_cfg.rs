#[doc = "Writer for register DMA_CFG"]
pub type W = crate::W<u32, super::DMA_CFG>;
#[doc = "Register DMA_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTEN_AW {
    #[doc = "0: Controller disabled"]
    MASTEN_0 = 0,
    #[doc = "1: Controller enabled"]
    MASTEN_1 = 1,
}
impl From<MASTEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MASTEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MASTEN`"]
pub struct MASTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Controller disabled"]
    #[inline(always)]
    pub fn masten_0(self) -> &'a mut W {
        self.variant(MASTEN_AW::MASTEN_0)
    }
    #[doc = "Controller enabled"]
    #[inline(always)]
    pub fn masten_1(self) -> &'a mut W {
        self.variant(MASTEN_AW::MASTEN_1)
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
#[doc = "Write proxy for field `CHPROTCTRL`"]
pub struct CHPROTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPROTCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn masten(&mut self) -> MASTEN_W {
        MASTEN_W { w: self }
    }
    #[doc = "Bits 5:7 - Sets the AHB-Lite protection by controlling the HPROT\\[3:1\\]
signal levels as follows: Bit \\[7\\]
Controls HPROT\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HPROT\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HPROT\\[1\\]
to indicate if a privileged access is occurring. Note: When bit \\[n\\]
= 1 then the corresponding HPROT is HIGH. When bit \\[n\\]
= 0 then the corresponding HPROT is LOW."]
    #[inline(always)]
    pub fn chprotctrl(&mut self) -> CHPROTCTRL_W {
        CHPROTCTRL_W { w: self }
    }
}
