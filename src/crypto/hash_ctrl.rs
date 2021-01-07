#[doc = "Reader of register HASH_CTRL"]
pub type R = crate::R<u32, super::HASH_CTRL>;
#[doc = "Writer for register HASH_CTRL"]
pub type W = crate::W<u32, super::HASH_CTRL>;
#[doc = "Register HASH_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Initialize. Initializes hash registers with standard constants.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::NOP,
            true => INIT_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == INIT_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == INIT_A::START
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(INIT_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(INIT_A::START)
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
#[doc = "XOR data with IV from cipher block. Useful when calculating HMAC to XOR the input pad and output pad.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XOR_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<XOR_A> for bool {
    #[inline(always)]
    fn from(variant: XOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XOR`"]
pub type XOR_R = crate::R<bool, XOR_A>;
impl XOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XOR_A {
        match self.bits {
            false => XOR_A::DIS,
            true => XOR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == XOR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == XOR_A::EN
    }
}
#[doc = "Write proxy for field `XOR`"]
pub struct XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> XOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(XOR_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(XOR_A::EN)
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
#[doc = "Hash function selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HASH_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: SHA-1."]
    SHA1 = 1,
    #[doc = "2: SHA 224."]
    SHA224 = 2,
    #[doc = "3: SHA 256."]
    SHA256 = 3,
    #[doc = "4: SHA 384."]
    SHA384 = 4,
    #[doc = "5: SHA 512."]
    SHA512 = 5,
}
impl From<HASH_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HASH`"]
pub type HASH_R = crate::R<u8, HASH_A>;
impl HASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HASH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HASH_A::DIS),
            1 => Val(HASH_A::SHA1),
            2 => Val(HASH_A::SHA224),
            3 => Val(HASH_A::SHA256),
            4 => Val(HASH_A::SHA384),
            5 => Val(HASH_A::SHA512),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HASH_A::DIS
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA224`"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == HASH_A::SHA224
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_A::SHA256
    }
    #[doc = "Checks if the value of the field is `SHA384`"]
    #[inline(always)]
    pub fn is_sha384(&self) -> bool {
        *self == HASH_A::SHA384
    }
    #[doc = "Checks if the value of the field is `SHA512`"]
    #[inline(always)]
    pub fn is_sha512(&self) -> bool {
        *self == HASH_A::SHA512
    }
}
#[doc = "Write proxy for field `HASH`"]
pub struct HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HASH_A::DIS)
    }
    #[doc = "SHA-1."]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(HASH_A::SHA1)
    }
    #[doc = "SHA 224."]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut W {
        self.variant(HASH_A::SHA224)
    }
    #[doc = "SHA 256."]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut W {
        self.variant(HASH_A::SHA256)
    }
    #[doc = "SHA 384."]
    #[inline(always)]
    pub fn sha384(self) -> &'a mut W {
        self.variant(HASH_A::SHA384)
    }
    #[doc = "SHA 512."]
    #[inline(always)]
    pub fn sha512(self) -> &'a mut W {
        self.variant(HASH_A::SHA512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Last Message Bit. This bit shall be set along with the HASH_MSG_SZ register prior to hashing the last 512 or 1024-bit block of the message data. It will allow automatic preprocessing of the last message padding, which includes the trailing bit 1, followed by the respective number of zero bits for the last block size and finally the message length represented in bytes. The bit will be automatically cleared at the same time the HASH DONE is set, designating the completion of the last message hash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_A {
    #[doc = "0: No Effect."]
    NOEFFECT = 0,
    #[doc = "1: Last Message Data."]
    LASTMSGDATA = 1,
}
impl From<LAST_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LAST`"]
pub type LAST_R = crate::R<bool, LAST_A>;
impl LAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_A {
        match self.bits {
            false => LAST_A::NOEFFECT,
            true => LAST_A::LASTMSGDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LAST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `LASTMSGDATA`"]
    #[inline(always)]
    pub fn is_last_msg_data(&self) -> bool {
        *self == LAST_A::LASTMSGDATA
    }
}
#[doc = "Write proxy for field `LAST`"]
pub struct LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LAST_A::NOEFFECT)
    }
    #[doc = "Last Message Data."]
    #[inline(always)]
    pub fn last_msg_data(self) -> &'a mut W {
        self.variant(LAST_A::LASTMSGDATA)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initialize. Initializes hash registers with standard constants."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOR data with IV from cipher block. Useful when calculating HMAC to XOR the input pad and output pad."]
    #[inline(always)]
    pub fn xor(&self) -> XOR_R {
        XOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Hash function selection."]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Last Message Bit. This bit shall be set along with the HASH_MSG_SZ register prior to hashing the last 512 or 1024-bit block of the message data. It will allow automatic preprocessing of the last message padding, which includes the trailing bit 1, followed by the respective number of zero bits for the last block size and finally the message length represented in bytes. The bit will be automatically cleared at the same time the HASH DONE is set, designating the completion of the last message hash."]
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize. Initializes hash registers with standard constants."]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - XOR data with IV from cipher block. Useful when calculating HMAC to XOR the input pad and output pad."]
    #[inline(always)]
    pub fn xor(&mut self) -> XOR_W {
        XOR_W { w: self }
    }
    #[doc = "Bits 2:4 - Hash function selection."]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W {
        HASH_W { w: self }
    }
    #[doc = "Bit 5 - Last Message Bit. This bit shall be set along with the HASH_MSG_SZ register prior to hashing the last 512 or 1024-bit block of the message data. It will allow automatic preprocessing of the last message padding, which includes the trailing bit 1, followed by the respective number of zero bits for the last block size and finally the message length represented in bytes. The bit will be automatically cleared at the same time the HASH DONE is set, designating the completion of the last message hash."]
    #[inline(always)]
    pub fn last(&mut self) -> LAST_W {
        LAST_W { w: self }
    }
}
