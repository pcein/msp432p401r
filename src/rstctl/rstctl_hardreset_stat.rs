#[doc = "Reader of register RSTCTL_HARDRESET_STAT"]
pub type R = crate::R<u32, super::RSTCTL_HARDRESET_STAT>;
#[doc = "Reader of field `SRC0`"]
pub type SRC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC1`"]
pub type SRC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC2`"]
pub type SRC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC3`"]
pub type SRC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC4`"]
pub type SRC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC5`"]
pub type SRC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC6`"]
pub type SRC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC7`"]
pub type SRC7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC8`"]
pub type SRC8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC9`"]
pub type SRC9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC10`"]
pub type SRC10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC11`"]
pub type SRC11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC12`"]
pub type SRC12_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC13`"]
pub type SRC13_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC14`"]
pub type SRC14_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC15`"]
pub type SRC15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates that SRC0 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src0(&self) -> SRC0_R {
        SRC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that SRC1 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src1(&self) -> SRC1_R {
        SRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that SRC2 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src2(&self) -> SRC2_R {
        SRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that SRC3 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src3(&self) -> SRC3_R {
        SRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that SRC4 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src4(&self) -> SRC4_R {
        SRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that SRC5 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src5(&self) -> SRC5_R {
        SRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that SRC6 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src6(&self) -> SRC6_R {
        SRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that SRC7 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src7(&self) -> SRC7_R {
        SRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates that SRC8 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src8(&self) -> SRC8_R {
        SRC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates that SRC9 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src9(&self) -> SRC9_R {
        SRC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates that SRC10 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src10(&self) -> SRC10_R {
        SRC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates that SRC11 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src11(&self) -> SRC11_R {
        SRC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates that SRC12 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src12(&self) -> SRC12_R {
        SRC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicates that SRC13 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src13(&self) -> SRC13_R {
        SRC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Indicates that SRC14 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src14(&self) -> SRC14_R {
        SRC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicates that SRC15 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src15(&self) -> SRC15_R {
        SRC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
