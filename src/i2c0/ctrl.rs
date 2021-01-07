#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_EN_A {
    #[doc = "0: Disable I2C."]
    DIS = 0,
    #[doc = "1: enable I2C."]
    EN = 1,
}
impl From<I2C_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_EN`"]
pub type I2C_EN_R = crate::R<bool, I2C_EN_A>;
impl I2C_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_EN_A {
        match self.bits {
            false => I2C_EN_A::DIS,
            true => I2C_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C_EN_A::EN
    }
}
#[doc = "Write proxy for field `I2C_EN`"]
pub struct I2C_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C_EN_A::DIS)
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C_EN_A::EN)
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
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST_A {
    #[doc = "0: Slave Mode."]
    SLAVE_MODE = 0,
    #[doc = "1: Master Mode."]
    MASTER_MODE = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MST`"]
pub type MST_R = crate::R<bool, MST_A>;
impl MST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::SLAVE_MODE,
            true => MST_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MST_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MST_A::MASTER_MODE
    }
}
#[doc = "Write proxy for field `MST`"]
pub struct MST_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MST_A::SLAVE_MODE)
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MST_A::MASTER_MODE)
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
#[doc = "General Call Address Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_CALL_ADDR_A {
    #[doc = "0: Ignore Gneral Call Address."]
    DIS = 0,
    #[doc = "1: Acknowledge general call address."]
    EN = 1,
}
impl From<GEN_CALL_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_CALL_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GEN_CALL_ADDR`"]
pub type GEN_CALL_ADDR_R = crate::R<bool, GEN_CALL_ADDR_A>;
impl GEN_CALL_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_CALL_ADDR_A {
        match self.bits {
            false => GEN_CALL_ADDR_A::DIS,
            true => GEN_CALL_ADDR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GEN_CALL_ADDR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GEN_CALL_ADDR_A::EN
    }
}
#[doc = "Write proxy for field `GEN_CALL_ADDR`"]
pub struct GEN_CALL_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_CALL_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEN_CALL_ADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GEN_CALL_ADDR_A::DIS)
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GEN_CALL_ADDR_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Interactive Receive Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MODE_A {
    #[doc = "0: Disable Interactive Receive Mode."]
    DIS = 0,
    #[doc = "1: Enable Interactive Receive Mode."]
    EN = 1,
}
impl From<RX_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_MODE`"]
pub type RX_MODE_R = crate::R<bool, RX_MODE_A>;
impl RX_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MODE_A {
        match self.bits {
            false => RX_MODE_A::DIS,
            true => RX_MODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_MODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_MODE_A::EN
    }
}
#[doc = "Write proxy for field `RX_MODE`"]
pub struct RX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_MODE_A::DIS)
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_MODE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MODE_ACK_A {
    #[doc = "0: return ACK (pulling SDA LOW)."]
    ACK = 0,
    #[doc = "1: return NACK (leaving SDA HIGH)."]
    NACK = 1,
}
impl From<RX_MODE_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MODE_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_MODE_ACK`"]
pub type RX_MODE_ACK_R = crate::R<bool, RX_MODE_ACK_A>;
impl RX_MODE_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MODE_ACK_A {
        match self.bits {
            false => RX_MODE_ACK_A::ACK,
            true => RX_MODE_ACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == RX_MODE_ACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == RX_MODE_ACK_A::NACK
    }
}
#[doc = "Write proxy for field `RX_MODE_ACK`"]
pub struct RX_MODE_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MODE_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MODE_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(RX_MODE_ACK_A::ACK)
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(RX_MODE_ACK_A::NACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "SCL Output. This bits control SCL output when SWOE =1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_OUT_A {
    #[doc = "0: Drive SCL low."]
    DRIVE_SCL_LOW = 0,
    #[doc = "1: Release SCL."]
    RELEASE_SCL = 1,
}
impl From<SCL_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCL_OUT`"]
pub type SCL_OUT_R = crate::R<bool, SCL_OUT_A>;
impl SCL_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_OUT_A {
        match self.bits {
            false => SCL_OUT_A::DRIVE_SCL_LOW,
            true => SCL_OUT_A::RELEASE_SCL,
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_SCL_LOW`"]
    #[inline(always)]
    pub fn is_drive_scl_low(&self) -> bool {
        *self == SCL_OUT_A::DRIVE_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `RELEASE_SCL`"]
    #[inline(always)]
    pub fn is_release_scl(&self) -> bool {
        *self == SCL_OUT_A::RELEASE_SCL
    }
}
#[doc = "Write proxy for field `SCL_OUT`"]
pub struct SCL_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn drive_scl_low(self) -> &'a mut W {
        self.variant(SCL_OUT_A::DRIVE_SCL_LOW)
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn release_scl(self) -> &'a mut W {
        self.variant(SCL_OUT_A::RELEASE_SCL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "SDA Output. This bits control SDA output when SWOE = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_OUT_A {
    #[doc = "0: Drive SDA low."]
    DRIVE_SDA_LOW = 0,
    #[doc = "1: Release SDA."]
    RELEASE_SDA = 1,
}
impl From<SDA_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDA_OUT`"]
pub type SDA_OUT_R = crate::R<bool, SDA_OUT_A>;
impl SDA_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_OUT_A {
        match self.bits {
            false => SDA_OUT_A::DRIVE_SDA_LOW,
            true => SDA_OUT_A::RELEASE_SDA,
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_SDA_LOW`"]
    #[inline(always)]
    pub fn is_drive_sda_low(&self) -> bool {
        *self == SDA_OUT_A::DRIVE_SDA_LOW
    }
    #[doc = "Checks if the value of the field is `RELEASE_SDA`"]
    #[inline(always)]
    pub fn is_release_sda(&self) -> bool {
        *self == SDA_OUT_A::RELEASE_SDA
    }
}
#[doc = "Write proxy for field `SDA_OUT`"]
pub struct SDA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDA_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn drive_sda_low(self) -> &'a mut W {
        self.variant(SDA_OUT_A::DRIVE_SDA_LOW)
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn release_sda(self) -> &'a mut W {
        self.variant(SDA_OUT_A::RELEASE_SDA)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCL`"]
pub type SCL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDA`"]
pub type SDA_R = crate::R<bool, bool>;
#[doc = "Software Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_OUT_EN_A {
    #[doc = "0: I2C Outputs SCLO and SDAO disabled."]
    OUTPUTS_DISABLE = 0,
    #[doc = "1: I2C Outputs SCLO and SDAO enabled."]
    OUTPUTS_ENABLE = 1,
}
impl From<SW_OUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SW_OUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW_OUT_EN`"]
pub type SW_OUT_EN_R = crate::R<bool, SW_OUT_EN_A>;
impl SW_OUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_OUT_EN_A {
        match self.bits {
            false => SW_OUT_EN_A::OUTPUTS_DISABLE,
            true => SW_OUT_EN_A::OUTPUTS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUTS_DISABLE`"]
    #[inline(always)]
    pub fn is_outputs_disable(&self) -> bool {
        *self == SW_OUT_EN_A::OUTPUTS_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUTS_ENABLE`"]
    #[inline(always)]
    pub fn is_outputs_enable(&self) -> bool {
        *self == SW_OUT_EN_A::OUTPUTS_ENABLE
    }
}
#[doc = "Write proxy for field `SW_OUT_EN`"]
pub struct SW_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_OUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_OUT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn outputs_disable(self) -> &'a mut W {
        self.variant(SW_OUT_EN_A::OUTPUTS_DISABLE)
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn outputs_enable(self) -> &'a mut W {
        self.variant(SW_OUT_EN_A::OUTPUTS_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match(GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Write."]
    WRITE = 0,
    #[doc = "1: Read."]
    READ = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, READ_A>;
impl READ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::WRITE,
            true => READ_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == READ_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == READ_A::READ
    }
}
#[doc = "This bit will disable slave clock stretching when set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_CLK_STRECH_DIS_A {
    #[doc = "0: Slave clock stretching enabled."]
    EN = 0,
    #[doc = "1: Slave clock stretching disabled."]
    DIS = 1,
}
impl From<SCL_CLK_STRECH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_CLK_STRECH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCL_CLK_STRECH_DIS`"]
pub type SCL_CLK_STRECH_DIS_R = crate::R<bool, SCL_CLK_STRECH_DIS_A>;
impl SCL_CLK_STRECH_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_CLK_STRECH_DIS_A {
        match self.bits {
            false => SCL_CLK_STRECH_DIS_A::EN,
            true => SCL_CLK_STRECH_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCL_CLK_STRECH_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCL_CLK_STRECH_DIS_A::DIS
    }
}
#[doc = "Write proxy for field `SCL_CLK_STRECH_DIS`"]
pub struct SCL_CLK_STRECH_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_CLK_STRECH_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_CLK_STRECH_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCL_CLK_STRECH_DIS_A::EN)
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCL_CLK_STRECH_DIS_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_PP_MODE_A {
    #[doc = "0: Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    DIS = 0,
    #[doc = "1: Non-standard push-pull operation: drive low for 0, drive high for 1"]
    EN = 1,
}
impl From<SCL_PP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_PP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCL_PP_MODE`"]
pub type SCL_PP_MODE_R = crate::R<bool, SCL_PP_MODE_A>;
impl SCL_PP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_PP_MODE_A {
        match self.bits {
            false => SCL_PP_MODE_A::DIS,
            true => SCL_PP_MODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCL_PP_MODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCL_PP_MODE_A::EN
    }
}
#[doc = "Write proxy for field `SCL_PP_MODE`"]
pub struct SCL_PP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_PP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_PP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCL_PP_MODE_A::DIS)
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCL_PP_MODE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Hs-mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_MODE_A {
    #[doc = "0: Hs-mode disabled."]
    DIS = 0,
    #[doc = "1: Hs-mode enabled."]
    EN = 1,
}
impl From<HS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: HS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HS_MODE`"]
pub type HS_MODE_R = crate::R<bool, HS_MODE_A>;
impl HS_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_MODE_A {
        match self.bits {
            false => HS_MODE_A::DIS,
            true => HS_MODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HS_MODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HS_MODE_A::EN
    }
}
#[doc = "Write proxy for field `HS_MODE`"]
pub struct HS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hs-mode disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HS_MODE_A::DIS)
    }
    #[doc = "Hs-mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HS_MODE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2C_EN_R {
        I2C_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gen_call_addr(&self) -> GEN_CALL_ADDR_R {
        GEN_CALL_ADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn rx_mode(&self) -> RX_MODE_R {
        RX_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    pub fn rx_mode_ack(&self) -> RX_MODE_ACK_R {
        RX_MODE_ACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    pub fn scl_out(&self) -> SCL_OUT_R {
        SCL_OUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    pub fn sda_out(&self) -> SDA_OUT_R {
        SDA_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCL status. This bit reflects the logic gate of SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDA status. THis bit reflects the logic gate of SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn sw_out_en(&self) -> SW_OUT_EN_R {
        SW_OUT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match(GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn scl_clk_strech_dis(&self) -> SCL_CLK_STRECH_DIS_R {
        SCL_CLK_STRECH_DIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    pub fn scl_pp_mode(&self) -> SCL_PP_MODE_R {
        SCL_PP_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hs-mode Enable."]
    #[inline(always)]
    pub fn hs_mode(&self) -> HS_MODE_R {
        HS_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn i2c_en(&mut self) -> I2C_EN_W {
        I2C_EN_W { w: self }
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst(&mut self) -> MST_W {
        MST_W { w: self }
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gen_call_addr(&mut self) -> GEN_CALL_ADDR_W {
        GEN_CALL_ADDR_W { w: self }
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RX_MODE_W {
        RX_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    pub fn rx_mode_ack(&mut self) -> RX_MODE_ACK_W {
        RX_MODE_ACK_W { w: self }
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    pub fn scl_out(&mut self) -> SCL_OUT_W {
        SCL_OUT_W { w: self }
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    pub fn sda_out(&mut self) -> SDA_OUT_W {
        SDA_OUT_W { w: self }
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn sw_out_en(&mut self) -> SW_OUT_EN_W {
        SW_OUT_EN_W { w: self }
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn scl_clk_strech_dis(&mut self) -> SCL_CLK_STRECH_DIS_W {
        SCL_CLK_STRECH_DIS_W { w: self }
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    pub fn scl_pp_mode(&mut self) -> SCL_PP_MODE_W {
        SCL_PP_MODE_W { w: self }
    }
    #[doc = "Bit 15 - Hs-mode Enable."]
    #[inline(always)]
    pub fn hs_mode(&mut self) -> HS_MODE_W {
        HS_MODE_W { w: self }
    }
}
