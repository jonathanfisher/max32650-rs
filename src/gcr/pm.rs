#[doc = "Reader of register PM"]
pub type R = crate::R<u32, super::PM>;
#[doc = "Writer for register PM"]
pub type W = crate::W<u32, super::PM>;
#[doc = "Register PM `reset()`'s with value 0"]
impl crate::ResetValue for super::PM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Active Mode."]
    ACTIVE = 0,
    #[doc = "3: Shutdown Mode."]
    SHUTDOWN = 3,
    #[doc = "4: Backup Mode."]
    BACKUP = 4,
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
            0 => Val(MODE_A::ACTIVE),
            3 => Val(MODE_A::SHUTDOWN),
            4 => Val(MODE_A::BACKUP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == MODE_A::SHUTDOWN
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == MODE_A::BACKUP
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
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(MODE_A::SHUTDOWN)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut W {
        self.variant(MODE_A::BACKUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOWKEN_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<GPIOWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOWKEN`"]
pub type GPIOWKEN_R = crate::R<bool, GPIOWKEN_A>;
impl GPIOWKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOWKEN_A {
        match self.bits {
            false => GPIOWKEN_A::DIS,
            true => GPIOWKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIOWKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIOWKEN_A::EN
    }
}
#[doc = "Write proxy for field `GPIOWKEN`"]
pub struct GPIOWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOWKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIOWKEN_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIOWKEN_A::EN)
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
#[doc = "RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCWKEN_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<RTCWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCWKEN`"]
pub type RTCWKEN_R = crate::R<bool, RTCWKEN_A>;
impl RTCWKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCWKEN_A {
        match self.bits {
            false => RTCWKEN_A::DIS,
            true => RTCWKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RTCWKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RTCWKEN_A::EN
    }
}
#[doc = "Write proxy for field `RTCWKEN`"]
pub struct RTCWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCWKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTCWKEN_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTCWKEN_A::EN)
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
#[doc = "USB Wake Up Enable. This bit enables USB activity as wakeup source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBWKEN_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<USBWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBWKEN`"]
pub type USBWKEN_R = crate::R<bool, USBWKEN_A>;
impl USBWKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBWKEN_A {
        match self.bits {
            false => USBWKEN_A::DIS,
            true => USBWKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == USBWKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == USBWKEN_A::EN
    }
}
#[doc = "Write proxy for field `USBWKEN`"]
pub struct USBWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBWKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(USBWKEN_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(USBWKEN_A::EN)
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
#[doc = "HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRCPD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<HIRCPD_A> for bool {
    #[inline(always)]
    fn from(variant: HIRCPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRCPD`"]
pub type HIRCPD_R = crate::R<bool, HIRCPD_A>;
impl HIRCPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRCPD_A {
        match self.bits {
            false => HIRCPD_A::ACTIVE,
            true => HIRCPD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HIRCPD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == HIRCPD_A::DEEPSLEEP
    }
}
#[doc = "Write proxy for field `HIRCPD`"]
pub struct HIRCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRCPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRCPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HIRCPD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(HIRCPD_A::DEEPSLEEP)
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
#[doc = "96MHz power down. This bit selects 96MHz HIRC power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC96MPD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<HIRC96MPD_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC96MPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC96MPD`"]
pub type HIRC96MPD_R = crate::R<bool, HIRC96MPD_A>;
impl HIRC96MPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC96MPD_A {
        match self.bits {
            false => HIRC96MPD_A::ACTIVE,
            true => HIRC96MPD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HIRC96MPD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == HIRC96MPD_A::DEEPSLEEP
    }
}
#[doc = "Write proxy for field `HIRC96MPD`"]
pub struct HIRC96MPD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC96MPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC96MPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HIRC96MPD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(HIRC96MPD_A::DEEPSLEEP)
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
#[doc = "8MHz power down. This bit selects 8MHz HIRC power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC8MPD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<HIRC8MPD_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC8MPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC8MPD`"]
pub type HIRC8MPD_R = crate::R<bool, HIRC8MPD_A>;
impl HIRC8MPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC8MPD_A {
        match self.bits {
            false => HIRC8MPD_A::ACTIVE,
            true => HIRC8MPD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HIRC8MPD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == HIRC8MPD_A::DEEPSLEEP
    }
}
#[doc = "Write proxy for field `HIRC8MPD`"]
pub struct HIRC8MPD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC8MPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC8MPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HIRC8MPD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(HIRC8MPD_A::DEEPSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpiowken(&self) -> GPIOWKEN_R {
        GPIOWKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtcwken(&self) -> RTCWKEN_R {
        RTCWKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB Wake Up Enable. This bit enables USB activity as wakeup source."]
    #[inline(always)]
    pub fn usbwken(&self) -> USBWKEN_R {
        USBWKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hircpd(&self) -> HIRCPD_R {
        HIRCPD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 96MHz power down. This bit selects 96MHz HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hirc96mpd(&self) -> HIRC96MPD_R {
        HIRC96MPD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 8MHz power down. This bit selects 8MHz HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hirc8mpd(&self) -> HIRC8MPD_R {
        HIRC8MPD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpiowken(&mut self) -> GPIOWKEN_W {
        GPIOWKEN_W { w: self }
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtcwken(&mut self) -> RTCWKEN_W {
        RTCWKEN_W { w: self }
    }
    #[doc = "Bit 6 - USB Wake Up Enable. This bit enables USB activity as wakeup source."]
    #[inline(always)]
    pub fn usbwken(&mut self) -> USBWKEN_W {
        USBWKEN_W { w: self }
    }
    #[doc = "Bit 15 - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hircpd(&mut self) -> HIRCPD_W {
        HIRCPD_W { w: self }
    }
    #[doc = "Bit 16 - 96MHz power down. This bit selects 96MHz HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hirc96mpd(&mut self) -> HIRC96MPD_W {
        HIRC96MPD_W { w: self }
    }
    #[doc = "Bit 17 - 8MHz power down. This bit selects 8MHz HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hirc8mpd(&mut self) -> HIRC8MPD_W {
        HIRC8MPD_W { w: self }
    }
}
