#[doc = "Reader of register DMA_INT0_SRCFLG"]
pub type R = crate::R<u32, super::DMA_INT0_SRCFLG>;
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3`"]
pub type CH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4`"]
pub type CH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5`"]
pub type CH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH6`"]
pub type CH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH7`"]
pub type CH7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH8`"]
pub type CH8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH9`"]
pub type CH9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH10`"]
pub type CH10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH11`"]
pub type CH11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH12`"]
pub type CH12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH13`"]
pub type CH13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH14`"]
pub type CH14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH15`"]
pub type CH15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH16`"]
pub type CH16_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH17`"]
pub type CH17_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH18`"]
pub type CH18_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH19`"]
pub type CH19_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH20`"]
pub type CH20_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH21`"]
pub type CH21_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH22`"]
pub type CH22_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH23`"]
pub type CH23_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH24`"]
pub type CH24_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH25`"]
pub type CH25_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH26`"]
pub type CH26_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH27`"]
pub type CH27_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH28`"]
pub type CH28_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH29`"]
pub type CH29_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH30`"]
pub type CH30_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH31`"]
pub type CH31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
