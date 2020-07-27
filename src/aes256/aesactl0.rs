#[doc = "Reader of register AESACTL0"]
pub type R = crate::R<u16, super::AESACTL0>;
#[doc = "Writer for register AESACTL0"]
pub type W = crate::W<u16, super::AESACTL0>;
#[doc = "Register AESACTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESOPX_A {
    #[doc = "0: Encryption"]
    AESOPX_0 = 0,
    #[doc = "1: Decryption. The provided key is the same key used for encryption"]
    AESOPX_1 = 1,
    #[doc = "2: Generate first round key required for decryption"]
    AESOPX_2 = 2,
    #[doc = "3: Decryption. The provided key is the first round key required for decryption"]
    AESOPX_3 = 3,
}
impl From<AESOPX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESOPX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESOPx`"]
pub type AESOPX_R = crate::R<u8, AESOPX_A>;
impl AESOPX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESOPX_A {
        match self.bits {
            0 => AESOPX_A::AESOPX_0,
            1 => AESOPX_A::AESOPX_1,
            2 => AESOPX_A::AESOPX_2,
            3 => AESOPX_A::AESOPX_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESOPX_0`"]
    #[inline(always)]
    pub fn is_aesopx_0(&self) -> bool {
        *self == AESOPX_A::AESOPX_0
    }
    #[doc = "Checks if the value of the field is `AESOPX_1`"]
    #[inline(always)]
    pub fn is_aesopx_1(&self) -> bool {
        *self == AESOPX_A::AESOPX_1
    }
    #[doc = "Checks if the value of the field is `AESOPX_2`"]
    #[inline(always)]
    pub fn is_aesopx_2(&self) -> bool {
        *self == AESOPX_A::AESOPX_2
    }
    #[doc = "Checks if the value of the field is `AESOPX_3`"]
    #[inline(always)]
    pub fn is_aesopx_3(&self) -> bool {
        *self == AESOPX_A::AESOPX_3
    }
}
#[doc = "Write proxy for field `AESOPx`"]
pub struct AESOPX_W<'a> {
    w: &'a mut W,
}
impl<'a> AESOPX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESOPX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn aesopx_0(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_0)
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn aesopx_1(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_1)
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_2(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_2)
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_3(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "AES key length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESKLX_A {
    #[doc = "0: AES128. The key size is 128 bit"]
    AESKLX_0 = 0,
    #[doc = "1: AES192. The key size is 192 bit."]
    AESKLX_1 = 1,
    #[doc = "2: AES256. The key size is 256 bit"]
    AESKLX_2 = 2,
}
impl From<AESKLX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESKLX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKLx`"]
pub type AESKLX_R = crate::R<u8, AESKLX_A>;
impl AESKLX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESKLX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKLX_A::AESKLX_0),
            1 => Val(AESKLX_A::AESKLX_1),
            2 => Val(AESKLX_A::AESKLX_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKLX_0`"]
    #[inline(always)]
    pub fn is_aesklx_0(&self) -> bool {
        *self == AESKLX_A::AESKLX_0
    }
    #[doc = "Checks if the value of the field is `AESKLX_1`"]
    #[inline(always)]
    pub fn is_aesklx_1(&self) -> bool {
        *self == AESKLX_A::AESKLX_1
    }
    #[doc = "Checks if the value of the field is `AESKLX_2`"]
    #[inline(always)]
    pub fn is_aesklx_2(&self) -> bool {
        *self == AESKLX_A::AESKLX_2
    }
}
#[doc = "Write proxy for field `AESKLx`"]
pub struct AESKLX_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKLX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKLX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn aesklx_0(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_0)
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn aesklx_1(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_1)
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn aesklx_2(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "AES cipher mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESCMX_A {
    #[doc = "0: ECB"]
    AESCMX_0 = 0,
    #[doc = "1: CBC"]
    AESCMX_1 = 1,
    #[doc = "2: OFB"]
    AESCMX_2 = 2,
    #[doc = "3: CFB"]
    AESCMX_3 = 3,
}
impl From<AESCMX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESCMX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESCMx`"]
pub type AESCMX_R = crate::R<u8, AESCMX_A>;
impl AESCMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCMX_A {
        match self.bits {
            0 => AESCMX_A::AESCMX_0,
            1 => AESCMX_A::AESCMX_1,
            2 => AESCMX_A::AESCMX_2,
            3 => AESCMX_A::AESCMX_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESCMX_0`"]
    #[inline(always)]
    pub fn is_aescmx_0(&self) -> bool {
        *self == AESCMX_A::AESCMX_0
    }
    #[doc = "Checks if the value of the field is `AESCMX_1`"]
    #[inline(always)]
    pub fn is_aescmx_1(&self) -> bool {
        *self == AESCMX_A::AESCMX_1
    }
    #[doc = "Checks if the value of the field is `AESCMX_2`"]
    #[inline(always)]
    pub fn is_aescmx_2(&self) -> bool {
        *self == AESCMX_A::AESCMX_2
    }
    #[doc = "Checks if the value of the field is `AESCMX_3`"]
    #[inline(always)]
    pub fn is_aescmx_3(&self) -> bool {
        *self == AESCMX_A::AESCMX_3
    }
}
#[doc = "Write proxy for field `AESCMx`"]
pub struct AESCMX_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESCMX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn aescmx_0(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_0)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn aescmx_1(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_1)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn aescmx_2(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_2)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn aescmx_3(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "AES software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESSWRST_A {
    #[doc = "0: No reset"]
    AESSWRST_0 = 0,
    #[doc = "1: Reset AES accelerator module"]
    AESSWRST_1 = 1,
}
impl From<AESSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: AESSWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESSWRST`"]
pub type AESSWRST_R = crate::R<bool, AESSWRST_A>;
impl AESSWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSWRST_A {
        match self.bits {
            false => AESSWRST_A::AESSWRST_0,
            true => AESSWRST_A::AESSWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESSWRST_0`"]
    #[inline(always)]
    pub fn is_aesswrst_0(&self) -> bool {
        *self == AESSWRST_A::AESSWRST_0
    }
    #[doc = "Checks if the value of the field is `AESSWRST_1`"]
    #[inline(always)]
    pub fn is_aesswrst_1(&self) -> bool {
        *self == AESSWRST_A::AESSWRST_1
    }
}
#[doc = "Write proxy for field `AESSWRST`"]
pub struct AESSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESSWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset"]
    #[inline(always)]
    pub fn aesswrst_0(self) -> &'a mut W {
        self.variant(AESSWRST_A::AESSWRST_0)
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn aesswrst_1(self) -> &'a mut W {
        self.variant(AESSWRST_A::AESSWRST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "AES ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESRDYIFG_A {
    #[doc = "0: No interrupt pending"]
    AESRDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    AESRDYIFG_1 = 1,
}
impl From<AESRDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESRDYIFG`"]
pub type AESRDYIFG_R = crate::R<bool, AESRDYIFG_A>;
impl AESRDYIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIFG_A {
        match self.bits {
            false => AESRDYIFG_A::AESRDYIFG_0,
            true => AESRDYIFG_A::AESRDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_0`"]
    #[inline(always)]
    pub fn is_aesrdyifg_0(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_0
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_1`"]
    #[inline(always)]
    pub fn is_aesrdyifg_1(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_1
    }
}
#[doc = "Write proxy for field `AESRDYIFG`"]
pub struct AESRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESRDYIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_0(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_1(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "AES error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESERRFG_A {
    #[doc = "0: No error"]
    AESERRFG_0 = 0,
    #[doc = "1: Error occurred"]
    AESERRFG_1 = 1,
}
impl From<AESERRFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESERRFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESERRFG`"]
pub type AESERRFG_R = crate::R<bool, AESERRFG_A>;
impl AESERRFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESERRFG_A {
        match self.bits {
            false => AESERRFG_A::AESERRFG_0,
            true => AESERRFG_A::AESERRFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESERRFG_0`"]
    #[inline(always)]
    pub fn is_aeserrfg_0(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_0
    }
    #[doc = "Checks if the value of the field is `AESERRFG_1`"]
    #[inline(always)]
    pub fn is_aeserrfg_1(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_1
    }
}
#[doc = "Write proxy for field `AESERRFG`"]
pub struct AESERRFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESERRFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESERRFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn aeserrfg_0(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_0)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn aeserrfg_1(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "AES ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    AESRDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    AESRDYIE_1 = 1,
}
impl From<AESRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESRDYIE`"]
pub type AESRDYIE_R = crate::R<bool, AESRDYIE_A>;
impl AESRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIE_A {
        match self.bits {
            false => AESRDYIE_A::AESRDYIE_0,
            true => AESRDYIE_A::AESRDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESRDYIE_0`"]
    #[inline(always)]
    pub fn is_aesrdyie_0(&self) -> bool {
        *self == AESRDYIE_A::AESRDYIE_0
    }
    #[doc = "Checks if the value of the field is `AESRDYIE_1`"]
    #[inline(always)]
    pub fn is_aesrdyie_1(&self) -> bool {
        *self == AESRDYIE_A::AESRDYIE_1
    }
}
#[doc = "Write proxy for field `AESRDYIE`"]
pub struct AESRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn aesrdyie_0(self) -> &'a mut W {
        self.variant(AESRDYIE_A::AESRDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn aesrdyie_1(self) -> &'a mut W {
        self.variant(AESRDYIE_A::AESRDYIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "AES cipher mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESCMEN_A {
    #[doc = "0: No DMA triggers are generated"]
    AESCMEN_0 = 0,
    #[doc = "1: DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    AESCMEN_1 = 1,
}
impl From<AESCMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESCMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESCMEN`"]
pub type AESCMEN_R = crate::R<bool, AESCMEN_A>;
impl AESCMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCMEN_A {
        match self.bits {
            false => AESCMEN_A::AESCMEN_0,
            true => AESCMEN_A::AESCMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESCMEN_0`"]
    #[inline(always)]
    pub fn is_aescmen_0(&self) -> bool {
        *self == AESCMEN_A::AESCMEN_0
    }
    #[doc = "Checks if the value of the field is `AESCMEN_1`"]
    #[inline(always)]
    pub fn is_aescmen_1(&self) -> bool {
        *self == AESCMEN_A::AESCMEN_1
    }
}
#[doc = "Write proxy for field `AESCMEN`"]
pub struct AESCMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESCMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_0(self) -> &'a mut W {
        self.variant(AESCMEN_A::AESCMEN_0)
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_1(self) -> &'a mut W {
        self.variant(AESCMEN_A::AESCMEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&self) -> AESOPX_R {
        AESOPX_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&self) -> AESKLX_R {
        AESKLX_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&self) -> AESCMX_R {
        AESCMX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AESSWRST_R {
        AESSWRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AESRDYIFG_R {
        AESRDYIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AESERRFG_R {
        AESERRFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AESRDYIE_R {
        AESRDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AESCMEN_R {
        AESCMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&mut self) -> AESOPX_W {
        AESOPX_W { w: self }
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&mut self) -> AESKLX_W {
        AESKLX_W { w: self }
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&mut self) -> AESCMX_W {
        AESCMX_W { w: self }
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AESSWRST_W {
        AESSWRST_W { w: self }
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AESRDYIFG_W {
        AESRDYIFG_W { w: self }
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AESERRFG_W {
        AESERRFG_W { w: self }
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AESRDYIE_W {
        AESRDYIE_W { w: self }
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AESCMEN_W {
        AESCMEN_W { w: self }
    }
}
