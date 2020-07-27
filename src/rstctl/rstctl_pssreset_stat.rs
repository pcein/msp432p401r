#[doc = "Reader of register RSTCTL_PSSRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_PSSRESET_STAT>;
#[doc = "Reader of field `SVSMH`"]
pub type SVSMH_R = crate::R<bool, bool>;
#[doc = "Reader of field `BGREF`"]
pub type BGREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCCDET`"]
pub type VCCDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `SVSL`"]
pub type SVSL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Indicates if POR was caused by an SVSMH trip condition int the PSS"]
    #[inline(always)]
    pub fn svsmh(&self) -> SVSMH_R {
        SVSMH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if POR was caused by a BGREF not okay condition in the PSS"]
    #[inline(always)]
    pub fn bgref(&self) -> BGREF_R {
        BGREF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates if POR was caused by a VCCDET trip condition in the PSS"]
    #[inline(always)]
    pub fn vccdet(&self) -> VCCDET_R {
        VCCDET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates if POR was caused by an SVSL trip condition in the PSS"]
    #[inline(always)]
    pub fn svsl(&self) -> SVSL_R {
        SVSL_R::new((self.bits & 0x01) != 0)
    }
}
