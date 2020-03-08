#[doc = "Writer for register CSSETIFG"]
pub type W = crate::W<u32, super::CSSETIFG>;
#[doc = "Register CSSETIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CSSETIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_LFXTIFG_AW {
    #[doc = "0: No effect"]
    SET_LFXTIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_LFXTIFG_1 = 1,
}
impl From<SET_LFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_LFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_LFXTIFG`"]
pub struct SET_LFXTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_LFXTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_LFXTIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_lfxtifg_0(self) -> &'a mut W {
        self.variant(SET_LFXTIFG_AW::SET_LFXTIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg_1(self) -> &'a mut W {
        self.variant(SET_LFXTIFG_AW::SET_LFXTIFG_1)
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
#[doc = "Set HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_HFXTIFG_AW {
    #[doc = "0: No effect"]
    SET_HFXTIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_HFXTIFG_1 = 1,
}
impl From<SET_HFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_HFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_HFXTIFG`"]
pub struct SET_HFXTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_HFXTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_HFXTIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxtifg_0(self) -> &'a mut W {
        self.variant(SET_HFXTIFG_AW::SET_HFXTIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg_1(self) -> &'a mut W {
        self.variant(SET_HFXTIFG_AW::SET_HFXTIFG_1)
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
#[doc = "Set HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_HFXT2IFG_AW {
    #[doc = "0: No effect"]
    SET_HFXT2IFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_HFXT2IFG_1 = 1,
}
impl From<SET_HFXT2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_HFXT2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_HFXT2IFG`"]
pub struct SET_HFXT2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_HFXT2IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_HFXT2IFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxt2ifg_0(self) -> &'a mut W {
        self.variant(SET_HFXT2IFG_AW::SET_HFXT2IFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg_1(self) -> &'a mut W {
        self.variant(SET_HFXT2IFG_AW::SET_HFXT2IFG_1)
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
#[doc = "Set DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_DCOR_OPNIFG_AW {
    #[doc = "0: No effect"]
    SET_DCOR_OPNIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_DCOR_OPNIFG_1 = 1,
}
impl From<SET_DCOR_OPNIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_DCOR_OPNIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_DCOR_OPNIFG`"]
pub struct SET_DCOR_OPNIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_DCOR_OPNIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_DCOR_OPNIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_dcor_opnifg_0(self) -> &'a mut W {
        self.variant(SET_DCOR_OPNIFG_AW::SET_DCOR_OPNIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_dcor_opnifg_1(self) -> &'a mut W {
        self.variant(SET_DCOR_OPNIFG_AW::SET_DCOR_OPNIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "REFCNT period counter set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_CALIFG_AW {
    #[doc = "0: No effect"]
    SET_CALIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_CALIFG_1 = 1,
}
impl From<SET_CALIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_CALIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_CALIFG`"]
pub struct SET_CALIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_CALIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_CALIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_califg_0(self) -> &'a mut W {
        self.variant(SET_CALIFG_AW::SET_CALIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_califg_1(self) -> &'a mut W {
        self.variant(SET_CALIFG_AW::SET_CALIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Start fault counter set interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_FCNTHFIFG_AW {
    #[doc = "0: No effect"]
    SET_FCNTHFIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTHFIFG_1 = 1,
}
impl From<SET_FCNTHFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTHFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_FCNTHFIFG`"]
pub struct SET_FCNTHFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_FCNTHFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_FCNTHFIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthfifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTHFIFG_AW::SET_FCNTHFIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthfifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTHFIFG_AW::SET_FCNTHFIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Start fault counter set interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_FCNTHF2IFG_AW {
    #[doc = "0: No effect"]
    SET_FCNTHF2IFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTHF2IFG_1 = 1,
}
impl From<SET_FCNTHF2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTHF2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_FCNTHF2IFG`"]
pub struct SET_FCNTHF2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_FCNTHF2IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_FCNTHF2IFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTHF2IFG_AW::SET_FCNTHF2IFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTHF2IFG_AW::SET_FCNTHF2IFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Start fault counter set interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_FCNTLFIFG_AW {
    #[doc = "0: No effect"]
    SET_FCNTLFIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTLFIFG_1 = 1,
}
impl From<SET_FCNTLFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTLFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_FCNTLFIFG`"]
pub struct SET_FCNTLFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_FCNTLFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_FCNTLFIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcntlfifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTLFIFG_AW::SET_FCNTLFIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcntlfifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTLFIFG_AW::SET_FCNTLFIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "PLL out-of-lock set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_PLLOOLIFG_AW {
    #[doc = "0: No effect"]
    SET_PLLOOLIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLOOLIFG_1 = 1,
}
impl From<SET_PLLOOLIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLOOLIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_PLLOOLIFG`"]
pub struct SET_PLLOOLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_PLLOOLIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_PLLOOLIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloolifg_0(self) -> &'a mut W {
        self.variant(SET_PLLOOLIFG_AW::SET_PLLOOLIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg_1(self) -> &'a mut W {
        self.variant(SET_PLLOOLIFG_AW::SET_PLLOOLIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "PLL loss-of-signal set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_PLLLOSIFG_AW {
    #[doc = "0: No effect"]
    SET_PLLLOSIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLLOSIFG_1 = 1,
}
impl From<SET_PLLLOSIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLLOSIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_PLLLOSIFG`"]
pub struct SET_PLLLOSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_PLLLOSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_PLLLOSIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plllosifg_0(self) -> &'a mut W {
        self.variant(SET_PLLLOSIFG_AW::SET_PLLLOSIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg_1(self) -> &'a mut W {
        self.variant(SET_PLLLOSIFG_AW::SET_PLLLOSIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PLL out-of-range set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_PLLOORIFG_AW {
    #[doc = "0: No effect"]
    SET_PLLOORIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLOORIFG_1 = 1,
}
impl From<SET_PLLOORIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLOORIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SET_PLLOORIFG`"]
pub struct SET_PLLOORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_PLLOORIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_PLLOORIFG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloorifg_0(self) -> &'a mut W {
        self.variant(SET_PLLOORIFG_AW::SET_PLLOORIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg_1(self) -> &'a mut W {
        self.variant(SET_PLLOORIFG_AW::SET_PLLOORIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg(&mut self) -> SET_LFXTIFG_W {
        SET_LFXTIFG_W { w: self }
    }
    #[doc = "Bit 1 - Set HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg(&mut self) -> SET_HFXTIFG_W {
        SET_HFXTIFG_W { w: self }
    }
    #[doc = "Bit 2 - Set HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg(&mut self) -> SET_HFXT2IFG_W {
        SET_HFXT2IFG_W { w: self }
    }
    #[doc = "Bit 6 - Set DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn set_dcor_opnifg(&mut self) -> SET_DCOR_OPNIFG_W {
        SET_DCOR_OPNIFG_W { w: self }
    }
    #[doc = "Bit 15 - REFCNT period counter set interrupt flag"]
    #[inline(always)]
    pub fn set_califg(&mut self) -> SET_CALIFG_W {
        SET_CALIFG_W { w: self }
    }
    #[doc = "Bit 9 - Start fault counter set interrupt flag HFXT"]
    #[inline(always)]
    pub fn set_fcnthfifg(&mut self) -> SET_FCNTHFIFG_W {
        SET_FCNTHFIFG_W { w: self }
    }
    #[doc = "Bit 10 - Start fault counter set interrupt flag HFXT2"]
    #[inline(always)]
    pub fn set_fcnthf2ifg(&mut self) -> SET_FCNTHF2IFG_W {
        SET_FCNTHF2IFG_W { w: self }
    }
    #[doc = "Bit 8 - Start fault counter set interrupt flag LFXT"]
    #[inline(always)]
    pub fn set_fcntlfifg(&mut self) -> SET_FCNTLFIFG_W {
        SET_FCNTLFIFG_W { w: self }
    }
    #[doc = "Bit 12 - PLL out-of-lock set interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg(&mut self) -> SET_PLLOOLIFG_W {
        SET_PLLOOLIFG_W { w: self }
    }
    #[doc = "Bit 13 - PLL loss-of-signal set interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg(&mut self) -> SET_PLLLOSIFG_W {
        SET_PLLLOSIFG_W { w: self }
    }
    #[doc = "Bit 14 - PLL out-of-range set interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg(&mut self) -> SET_PLLOORIFG_W {
        SET_PLLOORIFG_W { w: self }
    }
}
