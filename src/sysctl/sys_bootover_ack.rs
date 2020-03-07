#[doc = "Reader of register SYS_BOOTOVER_ACK"]
pub type R = crate::R<u32, super::SYS_BOOTOVER_ACK>;
#[doc = "Writer for register SYS_BOOTOVER_ACK"]
pub type W = crate::W<u32, super::SYS_BOOTOVER_ACK>;
#[doc = "Register SYS_BOOTOVER_ACK `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_BOOTOVER_ACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGVAL`"]
pub type REGVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGVAL`"]
pub struct REGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&self) -> REGVAL_R {
        REGVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&mut self) -> REGVAL_W {
        REGVAL_W { w: self }
    }
}
