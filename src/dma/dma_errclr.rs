#[doc = "Reader of register DMA_ERRCLR"]
pub type R = crate::R<u32, super::DMA_ERRCLR>;
#[doc = "Writer for register DMA_ERRCLR"]
pub type W = crate::W<u32, super::DMA_ERRCLR>;
#[doc = "Register DMA_ERRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_ERRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRCLR_A {
    #[doc = "0: dma_err is LOW"]
    ERRCLR_0_READ = 0,
    #[doc = "1: dma_err is HIGH."]
    ERRCLR_1_READ = 1,
}
impl From<ERRCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRCLR`"]
pub type ERRCLR_R = crate::R<bool, ERRCLR_A>;
impl ERRCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRCLR_A {
        match self.bits {
            false => ERRCLR_A::ERRCLR_0_READ,
            true => ERRCLR_A::ERRCLR_1_READ,
        }
    }
    #[doc = "Checks if the value of the field is `ERRCLR_0_READ`"]
    #[inline(always)]
    pub fn is_errclr_0_read(&self) -> bool {
        *self == ERRCLR_A::ERRCLR_0_READ
    }
    #[doc = "Checks if the value of the field is `ERRCLR_1_READ`"]
    #[inline(always)]
    pub fn is_errclr_1_read(&self) -> bool {
        *self == ERRCLR_A::ERRCLR_1_READ
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRCLR_AW {
    #[doc = "0: No effect, status of dma_err is unchanged."]
    ERRCLR_0_WRITE = 0,
    #[doc = "1: Sets dma_err LOW."]
    ERRCLR_1_WRITE = 1,
}
impl From<ERRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERRCLR`"]
pub struct ERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect, status of dma_err is unchanged."]
    #[inline(always)]
    pub fn errclr_0_write(self) -> &'a mut W {
        self.variant(ERRCLR_AW::ERRCLR_0_WRITE)
    }
    #[doc = "Sets dma_err LOW."]
    #[inline(always)]
    pub fn errclr_1_write(self) -> &'a mut W {
        self.variant(ERRCLR_AW::ERRCLR_1_WRITE)
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
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&self) -> ERRCLR_R {
        ERRCLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ERRCLR_W {
        ERRCLR_W { w: self }
    }
}
