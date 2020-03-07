#[doc = "Reader of register UCBxADDMASK"]
pub type R = crate::R<u16, super::UCBXADDMASK>;
#[doc = "Writer for register UCBxADDMASK"]
pub type W = crate::W<u16, super::UCBXADDMASK>;
#[doc = "Register UCBxADDMASK `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::UCBXADDMASK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `ADDMASK`"]
pub type ADDMASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDMASK`"]
pub struct ADDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> ADDMASK_R {
        ADDMASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> ADDMASK_W {
        ADDMASK_W { w: self }
    }
}
