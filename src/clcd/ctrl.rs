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
#[doc = "LCD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDEN`"]
pub type LCDEN_R = crate::R<bool, LCDEN_A>;
impl LCDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDEN_A {
        match self.bits {
            false => LCDEN_A::DISABLE,
            true => LCDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `LCDEN`"]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCDEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCDEN_A::ENABLE)
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
#[doc = "VI Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VISEL_A {
    #[doc = "0: On Vertical Sync"]
    ONVERTSYNC = 0,
    #[doc = "1: On Vertical Back Porch"]
    ONVERTBACKPORCH = 1,
    #[doc = "2: On Active Video"]
    ONACTIVEVIDEO = 2,
    #[doc = "3: On Vertical Front Porch"]
    ONVERTFRONTPORCH = 3,
}
impl From<VISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VISEL`"]
pub type VISEL_R = crate::R<u8, VISEL_A>;
impl VISEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VISEL_A {
        match self.bits {
            0 => VISEL_A::ONVERTSYNC,
            1 => VISEL_A::ONVERTBACKPORCH,
            2 => VISEL_A::ONACTIVEVIDEO,
            3 => VISEL_A::ONVERTFRONTPORCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONVERTSYNC`"]
    #[inline(always)]
    pub fn is_onvertsync(&self) -> bool {
        *self == VISEL_A::ONVERTSYNC
    }
    #[doc = "Checks if the value of the field is `ONVERTBACKPORCH`"]
    #[inline(always)]
    pub fn is_onvertbackporch(&self) -> bool {
        *self == VISEL_A::ONVERTBACKPORCH
    }
    #[doc = "Checks if the value of the field is `ONACTIVEVIDEO`"]
    #[inline(always)]
    pub fn is_onactivevideo(&self) -> bool {
        *self == VISEL_A::ONACTIVEVIDEO
    }
    #[doc = "Checks if the value of the field is `ONVERTFRONTPORCH`"]
    #[inline(always)]
    pub fn is_onvertfrontporch(&self) -> bool {
        *self == VISEL_A::ONVERTFRONTPORCH
    }
}
#[doc = "Write proxy for field `VISEL`"]
pub struct VISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VISEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On Vertical Sync"]
    #[inline(always)]
    pub fn onvertsync(self) -> &'a mut W {
        self.variant(VISEL_A::ONVERTSYNC)
    }
    #[doc = "On Vertical Back Porch"]
    #[inline(always)]
    pub fn onvertbackporch(self) -> &'a mut W {
        self.variant(VISEL_A::ONVERTBACKPORCH)
    }
    #[doc = "On Active Video"]
    #[inline(always)]
    pub fn onactivevideo(self) -> &'a mut W {
        self.variant(VISEL_A::ONACTIVEVIDEO)
    }
    #[doc = "On Vertical Front Porch"]
    #[inline(always)]
    pub fn onvertfrontporch(self) -> &'a mut W {
        self.variant(VISEL_A::ONVERTFRONTPORCH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Display Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISPTYPE_A {
    #[doc = "4: STN Color 8 bit"]
    STNCOLOR8BIT = 4,
    #[doc = "8: CLCD"]
    CLCD = 8,
}
impl From<DISPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DISPTYPE`"]
pub type DISPTYPE_R = crate::R<u8, DISPTYPE_A>;
impl DISPTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISPTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(DISPTYPE_A::STNCOLOR8BIT),
            8 => Val(DISPTYPE_A::CLCD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STNCOLOR8BIT`"]
    #[inline(always)]
    pub fn is_stncolor8bit(&self) -> bool {
        *self == DISPTYPE_A::STNCOLOR8BIT
    }
    #[doc = "Checks if the value of the field is `CLCD`"]
    #[inline(always)]
    pub fn is_clcd(&self) -> bool {
        *self == DISPTYPE_A::CLCD
    }
}
#[doc = "Write proxy for field `DISPTYPE`"]
pub struct DISPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISPTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STN Color 8 bit"]
    #[inline(always)]
    pub fn stncolor8bit(self) -> &'a mut W {
        self.variant(DISPTYPE_A::STNCOLOR8BIT)
    }
    #[doc = "CLCD"]
    #[inline(always)]
    pub fn clcd(self) -> &'a mut W {
        self.variant(DISPTYPE_A::CLCD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "BPP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BPP_A {
    #[doc = "0: BPP 1"]
    BPP1 = 0,
    #[doc = "1: BPP 2"]
    BPP2 = 1,
    #[doc = "2: BPP 4"]
    BPP4 = 2,
    #[doc = "3: BPP 8"]
    BPP8 = 3,
    #[doc = "4: BPP 16"]
    BPP16 = 4,
    #[doc = "5: BPP 24"]
    BPP24 = 5,
}
impl From<BPP_A> for u8 {
    #[inline(always)]
    fn from(variant: BPP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BPP`"]
pub type BPP_R = crate::R<u8, BPP_A>;
impl BPP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BPP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BPP_A::BPP1),
            1 => Val(BPP_A::BPP2),
            2 => Val(BPP_A::BPP4),
            3 => Val(BPP_A::BPP8),
            4 => Val(BPP_A::BPP16),
            5 => Val(BPP_A::BPP24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BPP1`"]
    #[inline(always)]
    pub fn is_bpp1(&self) -> bool {
        *self == BPP_A::BPP1
    }
    #[doc = "Checks if the value of the field is `BPP2`"]
    #[inline(always)]
    pub fn is_bpp2(&self) -> bool {
        *self == BPP_A::BPP2
    }
    #[doc = "Checks if the value of the field is `BPP4`"]
    #[inline(always)]
    pub fn is_bpp4(&self) -> bool {
        *self == BPP_A::BPP4
    }
    #[doc = "Checks if the value of the field is `BPP8`"]
    #[inline(always)]
    pub fn is_bpp8(&self) -> bool {
        *self == BPP_A::BPP8
    }
    #[doc = "Checks if the value of the field is `BPP16`"]
    #[inline(always)]
    pub fn is_bpp16(&self) -> bool {
        *self == BPP_A::BPP16
    }
    #[doc = "Checks if the value of the field is `BPP24`"]
    #[inline(always)]
    pub fn is_bpp24(&self) -> bool {
        *self == BPP_A::BPP24
    }
}
#[doc = "Write proxy for field `BPP`"]
pub struct BPP_W<'a> {
    w: &'a mut W,
}
impl<'a> BPP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BPP 1"]
    #[inline(always)]
    pub fn bpp1(self) -> &'a mut W {
        self.variant(BPP_A::BPP1)
    }
    #[doc = "BPP 2"]
    #[inline(always)]
    pub fn bpp2(self) -> &'a mut W {
        self.variant(BPP_A::BPP2)
    }
    #[doc = "BPP 4"]
    #[inline(always)]
    pub fn bpp4(self) -> &'a mut W {
        self.variant(BPP_A::BPP4)
    }
    #[doc = "BPP 8"]
    #[inline(always)]
    pub fn bpp8(self) -> &'a mut W {
        self.variant(BPP_A::BPP8)
    }
    #[doc = "BPP 16"]
    #[inline(always)]
    pub fn bpp16(self) -> &'a mut W {
        self.variant(BPP_A::BPP16)
    }
    #[doc = "BPP 24"]
    #[inline(always)]
    pub fn bpp24(self) -> &'a mut W {
        self.variant(BPP_A::BPP24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "MODE565\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE565_A {
    #[doc = "0: MODE 555"]
    MODE555 = 0,
    #[doc = "1: MODE 565"]
    MODE565 = 1,
}
impl From<MODE565_A> for bool {
    #[inline(always)]
    fn from(variant: MODE565_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE565`"]
pub type MODE565_R = crate::R<bool, MODE565_A>;
impl MODE565_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE565_A {
        match self.bits {
            false => MODE565_A::MODE555,
            true => MODE565_A::MODE565,
        }
    }
    #[doc = "Checks if the value of the field is `MODE555`"]
    #[inline(always)]
    pub fn is_mode555(&self) -> bool {
        *self == MODE565_A::MODE555
    }
    #[doc = "Checks if the value of the field is `MODE565`"]
    #[inline(always)]
    pub fn is_mode565(&self) -> bool {
        *self == MODE565_A::MODE565
    }
}
#[doc = "Write proxy for field `MODE565`"]
pub struct MODE565_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE565_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE565_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MODE 555"]
    #[inline(always)]
    pub fn mode555(self) -> &'a mut W {
        self.variant(MODE565_A::MODE555)
    }
    #[doc = "MODE 565"]
    #[inline(always)]
    pub fn mode565(self) -> &'a mut W {
        self.variant(MODE565_A::MODE565)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "EMODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMODE_A {
    #[doc = "0: LLBP"]
    LLBP = 0,
    #[doc = "1: BBBP"]
    BBBP = 1,
    #[doc = "2: LBBP"]
    LBBP = 2,
    #[doc = "3: RFU"]
    RFU = 3,
}
impl From<EMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMODE`"]
pub type EMODE_R = crate::R<u8, EMODE_A>;
impl EMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMODE_A {
        match self.bits {
            0 => EMODE_A::LLBP,
            1 => EMODE_A::BBBP,
            2 => EMODE_A::LBBP,
            3 => EMODE_A::RFU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LLBP`"]
    #[inline(always)]
    pub fn is_llbp(&self) -> bool {
        *self == EMODE_A::LLBP
    }
    #[doc = "Checks if the value of the field is `BBBP`"]
    #[inline(always)]
    pub fn is_bbbp(&self) -> bool {
        *self == EMODE_A::BBBP
    }
    #[doc = "Checks if the value of the field is `LBBP`"]
    #[inline(always)]
    pub fn is_lbbp(&self) -> bool {
        *self == EMODE_A::LBBP
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == EMODE_A::RFU
    }
}
#[doc = "Write proxy for field `EMODE`"]
pub struct EMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LLBP"]
    #[inline(always)]
    pub fn llbp(self) -> &'a mut W {
        self.variant(EMODE_A::LLBP)
    }
    #[doc = "BBBP"]
    #[inline(always)]
    pub fn bbbp(self) -> &'a mut W {
        self.variant(EMODE_A::BBBP)
    }
    #[doc = "LBBP"]
    #[inline(always)]
    pub fn lbbp(self) -> &'a mut W {
        self.variant(EMODE_A::LBBP)
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(EMODE_A::RFU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `C24`"]
pub type C24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C24`"]
pub struct C24_W<'a> {
    w: &'a mut W,
}
impl<'a> C24_W<'a> {
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
#[doc = "BURST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_A {
    #[doc = "0: WORDS4"]
    WORDS4 = 0,
    #[doc = "1: WORDS8"]
    WORDS8 = 1,
}
impl From<BURST_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<u8, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BURST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BURST_A::WORDS4),
            1 => Val(BURST_A::WORDS8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WORDS4`"]
    #[inline(always)]
    pub fn is_words4(&self) -> bool {
        *self == BURST_A::WORDS4
    }
    #[doc = "Checks if the value of the field is `WORDS8`"]
    #[inline(always)]
    pub fn is_words8(&self) -> bool {
        *self == BURST_A::WORDS8
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "WORDS4"]
    #[inline(always)]
    pub fn words4(self) -> &'a mut W {
        self.variant(BURST_A::WORDS4)
    }
    #[doc = "WORDS8"]
    #[inline(always)]
    pub fn words8(self) -> &'a mut W {
        self.variant(BURST_A::WORDS8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "LPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOL_A {
    #[doc = "0: ACTIVE HIGH"]
    ACTIVEHI = 0,
    #[doc = "1: ACTIVE LOW"]
    ACTIVELO = 1,
}
impl From<LPOL_A> for bool {
    #[inline(always)]
    fn from(variant: LPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPOL`"]
pub type LPOL_R = crate::R<bool, LPOL_A>;
impl LPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOL_A {
        match self.bits {
            false => LPOL_A::ACTIVEHI,
            true => LPOL_A::ACTIVELO,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHI`"]
    #[inline(always)]
    pub fn is_activehi(&self) -> bool {
        *self == LPOL_A::ACTIVEHI
    }
    #[doc = "Checks if the value of the field is `ACTIVELO`"]
    #[inline(always)]
    pub fn is_activelo(&self) -> bool {
        *self == LPOL_A::ACTIVELO
    }
}
#[doc = "Write proxy for field `LPOL`"]
pub struct LPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACTIVE HIGH"]
    #[inline(always)]
    pub fn activehi(self) -> &'a mut W {
        self.variant(LPOL_A::ACTIVEHI)
    }
    #[doc = "ACTIVE LOW"]
    #[inline(always)]
    pub fn activelo(self) -> &'a mut W {
        self.variant(LPOL_A::ACTIVELO)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PEN`"]
pub type PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN`"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - VI Select"]
    #[inline(always)]
    pub fn visel(&self) -> VISEL_R {
        VISEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Display Type"]
    #[inline(always)]
    pub fn disptype(&self) -> DISPTYPE_R {
        DISPTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - BPP"]
    #[inline(always)]
    pub fn bpp(&self) -> BPP_R {
        BPP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - MODE565"]
    #[inline(always)]
    pub fn mode565(&self) -> MODE565_R {
        MODE565_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - EMODE"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - C24"]
    #[inline(always)]
    pub fn c24(&self) -> C24_R {
        C24_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - BURST"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21 - LPOL"]
    #[inline(always)]
    pub fn lpol(&self) -> LPOL_R {
        LPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PEN"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bits 1:2 - VI Select"]
    #[inline(always)]
    pub fn visel(&mut self) -> VISEL_W {
        VISEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Display Type"]
    #[inline(always)]
    pub fn disptype(&mut self) -> DISPTYPE_W {
        DISPTYPE_W { w: self }
    }
    #[doc = "Bits 8:10 - BPP"]
    #[inline(always)]
    pub fn bpp(&mut self) -> BPP_W {
        BPP_W { w: self }
    }
    #[doc = "Bit 11 - MODE565"]
    #[inline(always)]
    pub fn mode565(&mut self) -> MODE565_W {
        MODE565_W { w: self }
    }
    #[doc = "Bits 12:13 - EMODE"]
    #[inline(always)]
    pub fn emode(&mut self) -> EMODE_W {
        EMODE_W { w: self }
    }
    #[doc = "Bit 15 - C24"]
    #[inline(always)]
    pub fn c24(&mut self) -> C24_W {
        C24_W { w: self }
    }
    #[doc = "Bits 19:20 - BURST"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 21 - LPOL"]
    #[inline(always)]
    pub fn lpol(&mut self) -> LPOL_W {
        LPOL_W { w: self }
    }
    #[doc = "Bit 22 - PEN"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
}
