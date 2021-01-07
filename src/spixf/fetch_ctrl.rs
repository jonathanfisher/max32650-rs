#[doc = "Reader of register FETCH_CTRL"]
pub type R = crate::R<u32, super::FETCH_CTRL>;
#[doc = "Writer for register FETCH_CTRL"]
pub type W = crate::W<u32, super::FETCH_CTRL>;
#[doc = "Register FETCH_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FETCH_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDVAL`"]
pub type CMDVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDVAL`"]
pub struct CMDVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Command Width. Number of data I/O used to send commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_WIDTH_A {
    #[doc = "0: Single SDIO."]
    SINGLE = 0,
    #[doc = "1: Dual SDIO."]
    DUAL_IO = 1,
    #[doc = "2: Quad SDIO."]
    QUAD_IO = 2,
    #[doc = "3: Invalid."]
    INVALID = 3,
}
impl From<CMD_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD_WIDTH`"]
pub type CMD_WIDTH_R = crate::R<u8, CMD_WIDTH_A>;
impl CMD_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_WIDTH_A {
        match self.bits {
            0 => CMD_WIDTH_A::SINGLE,
            1 => CMD_WIDTH_A::DUAL_IO,
            2 => CMD_WIDTH_A::QUAD_IO,
            3 => CMD_WIDTH_A::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CMD_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == CMD_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == CMD_WIDTH_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == CMD_WIDTH_A::INVALID
    }
}
#[doc = "Write proxy for field `CMD_WIDTH`"]
pub struct CMD_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_WIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single SDIO."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::SINGLE)
    }
    #[doc = "Dual SDIO."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad SDIO."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::QUAD_IO)
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::INVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Address Width. Number of data I/O used to send address, and mode/dummy clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDR_WIDTH_A {
    #[doc = "0: Single SDIO."]
    SINGLE = 0,
    #[doc = "1: Dual SDIO."]
    DUAL_IO = 1,
    #[doc = "2: Quad SDIO."]
    QUAD_IO = 2,
    #[doc = "3: Invalid."]
    INVALID = 3,
}
impl From<ADDR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDR_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADDR_WIDTH`"]
pub type ADDR_WIDTH_R = crate::R<u8, ADDR_WIDTH_A>;
impl ADDR_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_WIDTH_A {
        match self.bits {
            0 => ADDR_WIDTH_A::SINGLE,
            1 => ADDR_WIDTH_A::DUAL_IO,
            2 => ADDR_WIDTH_A::QUAD_IO,
            3 => ADDR_WIDTH_A::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ADDR_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == ADDR_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == ADDR_WIDTH_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == ADDR_WIDTH_A::INVALID
    }
}
#[doc = "Write proxy for field `ADDR_WIDTH`"]
pub struct ADDR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_WIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single SDIO."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::SINGLE)
    }
    #[doc = "Dual SDIO."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad SDIO."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::QUAD_IO)
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::INVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Data Width. Number of data I/O used to receive data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: Single SDIO."]
    SINGLE = 0,
    #[doc = "1: Dual SDIO."]
    DUAL_IO = 1,
    #[doc = "2: Quad SDIO."]
    QUAD_IO = 2,
    #[doc = "3: Invalid."]
    INVALID = 3,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATA_WIDTH`"]
pub type DATA_WIDTH_R = crate::R<u8, DATA_WIDTH_A>;
impl DATA_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_WIDTH_A {
        match self.bits {
            0 => DATA_WIDTH_A::SINGLE,
            1 => DATA_WIDTH_A::DUAL_IO,
            2 => DATA_WIDTH_A::QUAD_IO,
            3 => DATA_WIDTH_A::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DATA_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == DATA_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == DATA_WIDTH_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == DATA_WIDTH_A::INVALID
    }
}
#[doc = "Write proxy for field `DATA_WIDTH`"]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_WIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single SDIO."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::SINGLE)
    }
    #[doc = "Dual SDIO."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad SDIO."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::QUAD_IO)
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::INVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Four Byte Address Mode. Enables 4-byte Flash Address Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOUR_BYTE_ADDR_A {
    #[doc = "0: 3 Byte Address Mode."]
    _3 = 0,
    #[doc = "1: 4 Byte Address Mode."]
    _4 = 1,
}
impl From<FOUR_BYTE_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: FOUR_BYTE_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FOUR_BYTE_ADDR`"]
pub type FOUR_BYTE_ADDR_R = crate::R<bool, FOUR_BYTE_ADDR_A>;
impl FOUR_BYTE_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOUR_BYTE_ADDR_A {
        match self.bits {
            false => FOUR_BYTE_ADDR_A::_3,
            true => FOUR_BYTE_ADDR_A::_4,
        }
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == FOUR_BYTE_ADDR_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FOUR_BYTE_ADDR_A::_4
    }
}
#[doc = "Write proxy for field `FOUR_BYTE_ADDR`"]
pub struct FOUR_BYTE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOUR_BYTE_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOUR_BYTE_ADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "3 Byte Address Mode."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FOUR_BYTE_ADDR_A::_3)
    }
    #[doc = "4 Byte Address Mode."]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FOUR_BYTE_ADDR_A::_4)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Command Value sent to target to initiate fetching from SPI flash."]
    #[inline(always)]
    pub fn cmdval(&self) -> CMDVAL_R {
        CMDVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Command Width. Number of data I/O used to send commands."]
    #[inline(always)]
    pub fn cmd_width(&self) -> CMD_WIDTH_R {
        CMD_WIDTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Address Width. Number of data I/O used to send address, and mode/dummy clocks."]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Data Width. Number of data I/O used to receive data."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Four Byte Address Mode. Enables 4-byte Flash Address Mode."]
    #[inline(always)]
    pub fn four_byte_addr(&self) -> FOUR_BYTE_ADDR_R {
        FOUR_BYTE_ADDR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Value sent to target to initiate fetching from SPI flash."]
    #[inline(always)]
    pub fn cmdval(&mut self) -> CMDVAL_W {
        CMDVAL_W { w: self }
    }
    #[doc = "Bits 8:9 - Command Width. Number of data I/O used to send commands."]
    #[inline(always)]
    pub fn cmd_width(&mut self) -> CMD_WIDTH_W {
        CMD_WIDTH_W { w: self }
    }
    #[doc = "Bits 10:11 - Address Width. Number of data I/O used to send address, and mode/dummy clocks."]
    #[inline(always)]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W {
        ADDR_WIDTH_W { w: self }
    }
    #[doc = "Bits 12:13 - Data Width. Number of data I/O used to receive data."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 16 - Four Byte Address Mode. Enables 4-byte Flash Address Mode."]
    #[inline(always)]
    pub fn four_byte_addr(&mut self) -> FOUR_BYTE_ADDR_W {
        FOUR_BYTE_ADDR_W { w: self }
    }
}
