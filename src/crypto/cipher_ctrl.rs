#[doc = "Reader of register CIPHER_CTRL"]
pub type R = crate::R<u32, super::CIPHER_CTRL>;
#[doc = "Writer for register CIPHER_CTRL"]
pub type W = crate::W<u32, super::CIPHER_CTRL>;
#[doc = "Register CIPHER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CIPHER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Encrypt. Select encryption or decryption of input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENC_A {
    #[doc = "0: Encrypt."]
    ENCRYPT = 0,
    #[doc = "1: Decrypt."]
    DECRYPT = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<bool, ENC_A>;
impl ENC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::ENCRYPT,
            true => ENC_A::DECRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPT`"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == ENC_A::ENCRYPT
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == ENC_A::DECRYPT
    }
}
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Encrypt."]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut W {
        self.variant(ENC_A::ENCRYPT)
    }
    #[doc = "Decrypt."]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut W {
        self.variant(ENC_A::DECRYPT)
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
#[doc = "Load Key from crypto DMA. This bit is automatically cleared by hardware after the DMA has completed loading the key. When the DMA operation is done, it sets the appropriate crypto DMA Done flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<KEY_A> for bool {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY_A {
        match self.bits {
            false => KEY_A::COMPLETE,
            true => KEY_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == KEY_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == KEY_A::START
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(KEY_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(KEY_A::START)
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
#[doc = "Source of Random key.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: User cipher key (0x4000_1060)."]
    CIPHERKEY = 0,
    #[doc = "2: Key from battery-backed register file (0x4000_5000 to 0x4000_501F)."]
    REGFILE = 2,
    #[doc = "3: Key from battery-backed register file (0x4000_5020 to 0x4000_502F)."]
    QSPIKEY_REGFILE = 3,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::CIPHERKEY),
            2 => Val(SRC_A::REGFILE),
            3 => Val(SRC_A::QSPIKEY_REGFILE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CIPHERKEY`"]
    #[inline(always)]
    pub fn is_cipher_key(&self) -> bool {
        *self == SRC_A::CIPHERKEY
    }
    #[doc = "Checks if the value of the field is `REGFILE`"]
    #[inline(always)]
    pub fn is_reg_file(&self) -> bool {
        *self == SRC_A::REGFILE
    }
    #[doc = "Checks if the value of the field is `QSPIKEY_REGFILE`"]
    #[inline(always)]
    pub fn is_qspi_key_reg_file(&self) -> bool {
        *self == SRC_A::QSPIKEY_REGFILE
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "User cipher key (0x4000_1060)."]
    #[inline(always)]
    pub fn cipher_key(self) -> &'a mut W {
        self.variant(SRC_A::CIPHERKEY)
    }
    #[doc = "Key from battery-backed register file (0x4000_5000 to 0x4000_501F)."]
    #[inline(always)]
    pub fn reg_file(self) -> &'a mut W {
        self.variant(SRC_A::REGFILE)
    }
    #[doc = "Key from battery-backed register file (0x4000_5020 to 0x4000_502F)."]
    #[inline(always)]
    pub fn qspi_key_reg_file(self) -> &'a mut W {
        self.variant(SRC_A::QSPIKEY_REGFILE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Cipher Operation Select. Symmetric Block Cipher algorithm selection or memory operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIPHER_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: AES 128."]
    AES128 = 1,
    #[doc = "2: AES 192."]
    AES192 = 2,
    #[doc = "3: AES 256."]
    AES256 = 3,
    #[doc = "4: DES."]
    DES = 4,
    #[doc = "5: Triple DES."]
    TDES = 5,
}
impl From<CIPHER_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIPHER`"]
pub type CIPHER_R = crate::R<u8, CIPHER_A>;
impl CIPHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIPHER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIPHER_A::DIS),
            1 => Val(CIPHER_A::AES128),
            2 => Val(CIPHER_A::AES192),
            3 => Val(CIPHER_A::AES256),
            4 => Val(CIPHER_A::DES),
            5 => Val(CIPHER_A::TDES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CIPHER_A::DIS
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == CIPHER_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == CIPHER_A::AES256
    }
    #[doc = "Checks if the value of the field is `DES`"]
    #[inline(always)]
    pub fn is_des(&self) -> bool {
        *self == CIPHER_A::DES
    }
    #[doc = "Checks if the value of the field is `TDES`"]
    #[inline(always)]
    pub fn is_tdes(&self) -> bool {
        *self == CIPHER_A::TDES
    }
}
#[doc = "Write proxy for field `CIPHER`"]
pub struct CIPHER_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIPHER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CIPHER_A::DIS)
    }
    #[doc = "AES 128."]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(CIPHER_A::AES128)
    }
    #[doc = "AES 192."]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(CIPHER_A::AES192)
    }
    #[doc = "AES 256."]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(CIPHER_A::AES256)
    }
    #[doc = "DES."]
    #[inline(always)]
    pub fn des(self) -> &'a mut W {
        self.variant(CIPHER_A::DES)
    }
    #[doc = "Triple DES."]
    #[inline(always)]
    pub fn tdes(self) -> &'a mut W {
        self.variant(CIPHER_A::TDES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Mode Select. Mode of operation for block cipher or memory operation. DES/TDES cannot be used in CFB, OFB or CTR modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: ECB Mode."]
    ECB = 0,
    #[doc = "1: CBC Mode."]
    CBC = 1,
    #[doc = "2: CFB (AES only)."]
    CFB = 2,
    #[doc = "3: OFB (AES only)."]
    OFB = 3,
    #[doc = "4: CTR (AES only)."]
    CTR = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::ECB),
            1 => Val(MODE_A::CBC),
            2 => Val(MODE_A::CFB),
            3 => Val(MODE_A::OFB),
            4 => Val(MODE_A::CTR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == MODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == MODE_A::CBC
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == MODE_A::CFB
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == MODE_A::OFB
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == MODE_A::CTR
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ECB Mode."]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(MODE_A::ECB)
    }
    #[doc = "CBC Mode."]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(MODE_A::CBC)
    }
    #[doc = "CFB (AES only)."]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(MODE_A::CFB)
    }
    #[doc = "OFB (AES only)."]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(MODE_A::OFB)
    }
    #[doc = "CTR (AES only)."]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(MODE_A::CTR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Encrypt. Select encryption or decryption of input data."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Key from crypto DMA. This bit is automatically cleared by hardware after the DMA has completed loading the key. When the DMA operation is done, it sets the appropriate crypto DMA Done flag."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Source of Random key."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Cipher Operation Select. Symmetric Block Cipher algorithm selection or memory operation."]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Mode Select. Mode of operation for block cipher or memory operation. DES/TDES cannot be used in CFB, OFB or CTR modes."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Encrypt. Select encryption or decryption of input data."]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 1 - Load Key from crypto DMA. This bit is automatically cleared by hardware after the DMA has completed loading the key. When the DMA operation is done, it sets the appropriate crypto DMA Done flag."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bits 2:3 - Source of Random key."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bits 4:6 - Cipher Operation Select. Symmetric Block Cipher algorithm selection or memory operation."]
    #[inline(always)]
    pub fn cipher(&mut self) -> CIPHER_W {
        CIPHER_W { w: self }
    }
    #[doc = "Bits 8:10 - Mode Select. Mode of operation for block cipher or memory operation. DES/TDES cannot be used in CFB, OFB or CTR modes."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
