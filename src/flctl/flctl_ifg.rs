#[doc = "Reader of register FLCTL_IFG"]
pub type R = crate::R<u32, super::FLCTL_IFG>;
#[doc = "Reader of field `RDBRST`"]
pub type RDBRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVPRE`"]
pub type AVPRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVPST`"]
pub type AVPST_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRG`"]
pub type PRG_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRGB`"]
pub type PRGB_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BMRK`"]
pub type BMRK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRG_ERR`"]
pub type PRG_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - If set to 1, indicates that the Read Burst/Compare operation is complete"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set to 1, indicates that the pre-program verify operation has detected an error"]
    #[inline(always)]
    pub fn avpre(&self) -> AVPRE_R {
        AVPRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If set to 1, indicates that the post-program verify operation has failed comparison"]
    #[inline(always)]
    pub fn avpst(&self) -> AVPST_R {
        AVPST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set to 1, indicates that a word Program operation is complete"]
    #[inline(always)]
    pub fn prg(&self) -> PRG_R {
        PRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If set to 1, indicates that the configured Burst Program operation is complete"]
    #[inline(always)]
    pub fn prgb(&self) -> PRGB_R {
        PRGB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, indicates that the Erase operation is complete"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1, indicates that a Benchmark Compare match occurred"]
    #[inline(always)]
    pub fn bmrk(&self) -> BMRK_R {
        BMRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1, indicates a word composition error in full word write mode (possible data loss due to writes crossing over to a new 128bit boundary before full word has been composed)"]
    #[inline(always)]
    pub fn prg_err(&self) -> PRG_ERR_R {
        PRG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
