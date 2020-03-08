#[doc = "Reader of register DMA_ALTSET"]
pub type R = crate::R<u32, super::DMA_ALTSET>;
#[doc = "Writer for register DMA_ALTSET"]
pub type W = crate::W<u32, super::DMA_ALTSET>;
#[doc = "Register DMA_ALTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_ALTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_A {
    #[doc = "0: DMA channel C is using the primary data structure."]
    SET_0_READ = 0,
    #[doc = "1: DMA channel C is using the alternate data structure."]
    SET_1_READ = 1,
}
impl From<SET_A> for u32 {
    #[inline(always)]
    fn from(variant: SET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SET`"]
pub type SET_R = crate::R<u32, SET_A>;
impl SET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SET_A::SET_0_READ),
            1 => Val(SET_A::SET_1_READ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_0_READ`"]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SET_A::SET_0_READ
    }
    #[doc = "Checks if the value of the field is `SET_1_READ`"]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SET_A::SET_1_READ
    }
}
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    SEL_0_WRITE = 0,
    #[doc = "1: Selects the alternate data structure for channel C.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    SEL_1_WRITE = 1,
}
impl From<SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `SET`"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    #[inline(always)]
    pub fn sel_0_write(self) -> &'a mut W {
        self.variant(SET_AW::SEL_0_WRITE)
    }
    #[doc = "Selects the alternate data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn sel_1_write(self) -> &'a mut W {
        self.variant(SET_AW::SEL_1_WRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
}
