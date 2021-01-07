#[doc = "Reader of register TRANS"]
pub type R = crate::R<u16, super::TRANS>;
#[doc = "Writer for register TRANS"]
pub type W = crate::W<u16, super::TRANS>;
#[doc = "Register TRANS `reset()`'s with value 0"]
impl crate::ResetValue for super::TRANS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_EN_A {
    #[doc = "1: `1`"]
    DMA_TRANSFER = 1,
    #[doc = "0: `0`"]
    NON_DMA_TRANSFER = 0,
}
impl From<DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_EN`"]
pub type DMA_EN_R = crate::R<bool, DMA_EN_A>;
impl DMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_EN_A {
        match self.bits {
            true => DMA_EN_A::DMA_TRANSFER,
            false => DMA_EN_A::NON_DMA_TRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER`"]
    #[inline(always)]
    pub fn is_dma_transfer(&self) -> bool {
        *self == DMA_EN_A::DMA_TRANSFER
    }
    #[doc = "Checks if the value of the field is `NON_DMA_TRANSFER`"]
    #[inline(always)]
    pub fn is_non_dma_transfer(&self) -> bool {
        *self == DMA_EN_A::NON_DMA_TRANSFER
    }
}
#[doc = "Write proxy for field `DMA_EN`"]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dma_transfer(self) -> &'a mut W {
        self.variant(DMA_EN_A::DMA_TRANSFER)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn non_dma_transfer(self) -> &'a mut W {
        self.variant(DMA_EN_A::NON_DMA_TRANSFER)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Block Count Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLK_CNT_EN_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<BLK_CNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK_CNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLK_CNT_EN`"]
pub type BLK_CNT_EN_R = crate::R<bool, BLK_CNT_EN_A>;
impl BLK_CNT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK_CNT_EN_A {
        match self.bits {
            true => BLK_CNT_EN_A::ENABLE,
            false => BLK_CNT_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BLK_CNT_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BLK_CNT_EN_A::DISABLE
    }
}
#[doc = "Write proxy for field `BLK_CNT_EN`"]
pub struct BLK_CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_CNT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLK_CNT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BLK_CNT_EN_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BLK_CNT_EN_A::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Auto CMD Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUTO_CMD_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    CMD12 = 1,
    #[doc = "2: `10`"]
    CMD23 = 2,
}
impl From<AUTO_CMD_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTO_CMD_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUTO_CMD_EN`"]
pub type AUTO_CMD_EN_R = crate::R<u8, AUTO_CMD_EN_A>;
impl AUTO_CMD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUTO_CMD_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUTO_CMD_EN_A::DISABLE),
            1 => Val(AUTO_CMD_EN_A::CMD12),
            2 => Val(AUTO_CMD_EN_A::CMD23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTO_CMD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CMD12`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == AUTO_CMD_EN_A::CMD12
    }
    #[doc = "Checks if the value of the field is `CMD23`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == AUTO_CMD_EN_A::CMD23
    }
}
#[doc = "Write proxy for field `AUTO_CMD_EN`"]
pub struct AUTO_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_CMD_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTO_CMD_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut W {
        self.variant(AUTO_CMD_EN_A::CMD12)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut W {
        self.variant(AUTO_CMD_EN_A::CMD23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Data Transfer Direction Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WRITE_A {
    #[doc = "1: `1`"]
    READ = 1,
    #[doc = "0: `0`"]
    WRITE = 0,
}
impl From<READ_WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_WRITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ_WRITE`"]
pub type READ_WRITE_R = crate::R<bool, READ_WRITE_A>;
impl READ_WRITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_WRITE_A {
        match self.bits {
            true => READ_WRITE_A::READ,
            false => READ_WRITE_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == READ_WRITE_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == READ_WRITE_A::WRITE
    }
}
#[doc = "Write proxy for field `READ_WRITE`"]
pub struct READ_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WRITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_WRITE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(READ_WRITE_A::READ)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(READ_WRITE_A::WRITE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Multi / Single Block Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTI_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<MULTI_A> for bool {
    #[inline(always)]
    fn from(variant: MULTI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MULTI`"]
pub type MULTI_R = crate::R<bool, MULTI_A>;
impl MULTI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULTI_A {
        match self.bits {
            true => MULTI_A::ENABLE,
            false => MULTI_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MULTI_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MULTI_A::DISABLE
    }
}
#[doc = "Write proxy for field `MULTI`"]
pub struct MULTI_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MULTI_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MULTI_A::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable."]
    #[inline(always)]
    pub fn blk_cnt_en(&self) -> BLK_CNT_EN_R {
        BLK_CNT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable."]
    #[inline(always)]
    pub fn auto_cmd_en(&self) -> AUTO_CMD_EN_R {
        AUTO_CMD_EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select."]
    #[inline(always)]
    pub fn multi(&self) -> MULTI_R {
        MULTI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable."]
    #[inline(always)]
    pub fn blk_cnt_en(&mut self) -> BLK_CNT_EN_W {
        BLK_CNT_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto CMD Enable."]
    #[inline(always)]
    pub fn auto_cmd_en(&mut self) -> AUTO_CMD_EN_W {
        AUTO_CMD_EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select."]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W {
        READ_WRITE_W { w: self }
    }
    #[doc = "Bit 5 - Multi / Single Block Select."]
    #[inline(always)]
    pub fn multi(&mut self) -> MULTI_W {
        MULTI_W { w: self }
    }
}
