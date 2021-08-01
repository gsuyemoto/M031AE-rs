#[doc = "Register `CLK_PWRCTL` reader"]
pub struct R(crate::R<CLK_PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PWRCTL` writer"]
pub struct W(crate::W<CLK_PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_PWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HXT Enable Bit (Write Protect)\nNote1 : reset by power on reset\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTEN_A {
    #[doc = "0: Eexternal high speed crystal (HXT) Disabled"]
    _0 = 0,
    #[doc = "1: External high speed crystal (HXT) Enabled"]
    _1 = 1,
}
impl From<HXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTEN` reader - HXT Enable Bit (Write Protect)\nNote1 : reset by power on reset\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTEN_R(crate::FieldReader<bool, HXTEN_A>);
impl HXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTEN_A {
        match self.bits {
            false => HXTEN_A::_0,
            true => HXTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTEN_A::_1
    }
}
impl core::ops::Deref for HXTEN_R {
    type Target = crate::FieldReader<bool, HXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTEN` writer - HXT Enable Bit (Write Protect)\nNote1 : reset by power on reset\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Eexternal high speed crystal (HXT) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTEN_A::_0)
    }
    #[doc = "External high speed crystal (HXT) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "LXT Enable Bit (Write Protect)\nNote1 : \nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTEN_A {
    #[doc = "0: External low speed crystal (LXT) Disabled"]
    _0 = 0,
    #[doc = "1: External low speed crystal (LXT) Enabled"]
    _1 = 1,
}
impl From<LXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTEN` reader - LXT Enable Bit (Write Protect)\nNote1 : \nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTEN_R(crate::FieldReader<bool, LXTEN_A>);
impl LXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTEN_A {
        match self.bits {
            false => LXTEN_A::_0,
            true => LXTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTEN_A::_1
    }
}
impl core::ops::Deref for LXTEN_R {
    type Target = crate::FieldReader<bool, LXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTEN` writer - LXT Enable Bit (Write Protect)\nNote1 : \nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External low speed crystal (LXT) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTEN_A::_0)
    }
    #[doc = "External low speed crystal (LXT) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "HIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRCEN_A {
    #[doc = "0: Internal high speed RC oscillator (HIRC) Disabled"]
    _0 = 0,
    #[doc = "1: Internal high speed RC oscillator (HIRC) Enabled"]
    _1 = 1,
}
impl From<HIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRCEN` reader - HIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HIRCEN_R(crate::FieldReader<bool, HIRCEN_A>);
impl HIRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRCEN_A {
        match self.bits {
            false => HIRCEN_A::_0,
            true => HIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HIRCEN_A::_1
    }
}
impl core::ops::Deref for HIRCEN_R {
    type Target = crate::FieldReader<bool, HIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRCEN` writer - HIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal high speed RC oscillator (HIRC) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HIRCEN_A::_0)
    }
    #[doc = "Internal high speed RC oscillator (HIRC) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HIRCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "LIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIRCEN_A {
    #[doc = "0: Internal low speed RC oscillator (LIRC) Disabled"]
    _0 = 0,
    #[doc = "1: Internal low speed RC oscillator (LIRC) Enabled"]
    _1 = 1,
}
impl From<LIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIRCEN` reader - LIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LIRCEN_R(crate::FieldReader<bool, LIRCEN_A>);
impl LIRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIRCEN_A {
        match self.bits {
            false => LIRCEN_A::_0,
            true => LIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LIRCEN_A::_1
    }
}
impl core::ops::Deref for LIRCEN_R {
    type Target = crate::FieldReader<bool, LIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIRCEN` writer - LIRC Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal low speed RC oscillator (LIRC) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIRCEN_A::_0)
    }
    #[doc = "Internal low speed RC oscillator (LIRC) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIRCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable the Wake-up Delay Counter (Write Protect)\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\nThe delayed clock cycle is 4096 clock cycles when chip works at external high speed crystal oscillator (HXT), and 512 clock cycles when chip works at internal high speed RC oscillator (HIRC).\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWKDLY_A {
    #[doc = "0: Clock cycles delay Disabled"]
    _0 = 0,
    #[doc = "1: Clock cycles delay Enabled"]
    _1 = 1,
}
impl From<PDWKDLY_A> for bool {
    #[inline(always)]
    fn from(variant: PDWKDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWKDLY` reader - Enable the Wake-up Delay Counter (Write Protect)\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\nThe delayed clock cycle is 4096 clock cycles when chip works at external high speed crystal oscillator (HXT), and 512 clock cycles when chip works at internal high speed RC oscillator (HIRC).\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKDLY_R(crate::FieldReader<bool, PDWKDLY_A>);
impl PDWKDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWKDLY_A {
        match self.bits {
            false => PDWKDLY_A::_0,
            true => PDWKDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDWKDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDWKDLY_A::_1
    }
}
impl core::ops::Deref for PDWKDLY_R {
    type Target = crate::FieldReader<bool, PDWKDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKDLY` writer - Enable the Wake-up Delay Counter (Write Protect)\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\nThe delayed clock cycle is 4096 clock cycles when chip works at external high speed crystal oscillator (HXT), and 512 clock cycles when chip works at internal high speed RC oscillator (HIRC).\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWKDLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock cycles delay Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDWKDLY_A::_0)
    }
    #[doc = "Clock cycles delay Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDWKDLY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\nNote 1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWKIEN_A {
    #[doc = "0: Power-down mode wake-up interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Power-down mode wake-up interrupt Enabled"]
    _1 = 1,
}
impl From<PDWKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWKIEN` reader - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\nNote 1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKIEN_R(crate::FieldReader<bool, PDWKIEN_A>);
impl PDWKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWKIEN_A {
        match self.bits {
            false => PDWKIEN_A::_0,
            true => PDWKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDWKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDWKIEN_A::_1
    }
}
impl core::ops::Deref for PDWKIEN_R {
    type Target = crate::FieldReader<bool, PDWKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKIEN` writer - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\nNote 1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power-down mode wake-up interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDWKIEN_A::_0)
    }
    #[doc = "Power-down mode wake-up interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDWKIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PDWKIF` reader - Power-down Mode Wake-up Interrupt Status\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \nThe flag is set if any wake-up source is occurred. Refer Power Modes and Wake-up Sources chapter.\nNote 1: Write 1 to clear the bit to 0.\nNote 2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
pub struct PDWKIF_R(crate::FieldReader<bool, bool>);
impl PDWKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDWKIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKIF` writer - Power-down Mode Wake-up Interrupt Status\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \nThe flag is set if any wake-up source is occurred. Refer Power Modes and Wake-up Sources chapter.\nNote 1: Write 1 to clear the bit to 0.\nNote 2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
pub struct PDWKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "System Power-down Enable (Write Protect)\nWhen this bit is set to 1, Power-down mode is enabled and chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode. If user disable LIRC before entering power-down mode, this bit should be set after LIRC disabled 50us.\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_A {
    #[doc = "0: Chip operating normally or chip in idle mode because of WFI command"]
    _0 = 0,
    #[doc = "1: Chip enters Power-down mode instant or wait CPU sleep command WFI"]
    _1 = 1,
}
impl From<PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN` reader - System Power-down Enable (Write Protect)\nWhen this bit is set to 1, Power-down mode is enabled and chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode. If user disable LIRC before entering power-down mode, this bit should be set after LIRC disabled 50us.\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDEN_R(crate::FieldReader<bool, PDEN_A>);
impl PDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_A {
        match self.bits {
            false => PDEN_A::_0,
            true => PDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDEN_A::_1
    }
}
impl core::ops::Deref for PDEN_R {
    type Target = crate::FieldReader<bool, PDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN` writer - System Power-down Enable (Write Protect)\nWhen this bit is set to 1, Power-down mode is enabled and chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode. If user disable LIRC before entering power-down mode, this bit should be set after LIRC disabled 50us.\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip operating normally or chip in idle mode because of WFI command"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDEN_A::_0)
    }
    #[doc = "Chip enters Power-down mode instant or wait CPU sleep command WFI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "HXT Gain Control Bit (Write Protect)\nThis is a protected register. Please refer to open lock sequence to program it.\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \nOthers: Reserved \nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HXTGAIN_A {
    #[doc = "0: HXT frequency is lower than from 4 MHz"]
    _0 = 0,
    #[doc = "1: HXT frequency is from 4 MHz to 8 MHz"]
    _1 = 1,
    #[doc = "2: HXT frequency is from 8 MHz to 12 MHz"]
    _2 = 2,
    #[doc = "3: HXT frequency is from 12 MHz to 16 MHz"]
    _3 = 3,
    #[doc = "4: HXT frequency is from 16 MHz to 24 MHz"]
    _4 = 4,
    #[doc = "7: HXT frequency is from 24 MHz to 32 MHz"]
    _7 = 7,
}
impl From<HXTGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: HXTGAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HXTGAIN` reader - HXT Gain Control Bit (Write Protect)\nThis is a protected register. Please refer to open lock sequence to program it.\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \nOthers: Reserved \nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTGAIN_R(crate::FieldReader<u8, HXTGAIN_A>);
impl HXTGAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HXTGAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HXTGAIN_A> {
        match self.bits {
            0 => Some(HXTGAIN_A::_0),
            1 => Some(HXTGAIN_A::_1),
            2 => Some(HXTGAIN_A::_2),
            3 => Some(HXTGAIN_A::_3),
            4 => Some(HXTGAIN_A::_4),
            7 => Some(HXTGAIN_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTGAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTGAIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HXTGAIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HXTGAIN_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == HXTGAIN_A::_4
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == HXTGAIN_A::_7
    }
}
impl core::ops::Deref for HXTGAIN_R {
    type Target = crate::FieldReader<u8, HXTGAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTGAIN` writer - HXT Gain Control Bit (Write Protect)\nThis is a protected register. Please refer to open lock sequence to program it.\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \nOthers: Reserved \nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTGAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTGAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HXT frequency is lower than from 4 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_0)
    }
    #[doc = "HXT frequency is from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_1)
    }
    #[doc = "HXT frequency is from 8 MHz to 12 MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_2)
    }
    #[doc = "HXT frequency is from 12 MHz to 16 MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_3)
    }
    #[doc = "HXT frequency is from 16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_4)
    }
    #[doc = "HXT frequency is from 24 MHz to 32 MHz"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "LXT Mode Selection\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTSELXT_A {
    #[doc = "0: LXT work as crystal mode. PF.4 and PF.5 are configured as external low speed crystal (LXT) pins"]
    _0 = 0,
    #[doc = "1: LXT work as external clock mode. PF.5 is configured as external clock input pin"]
    _1 = 1,
}
impl From<LXTSELXT_A> for bool {
    #[inline(always)]
    fn from(variant: LXTSELXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTSELXT` reader - LXT Mode Selection\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTSELXT_R(crate::FieldReader<bool, LXTSELXT_A>);
impl LXTSELXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTSELXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTSELXT_A {
        match self.bits {
            false => LXTSELXT_A::_0,
            true => LXTSELXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTSELXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTSELXT_A::_1
    }
}
impl core::ops::Deref for LXTSELXT_R {
    type Target = crate::FieldReader<bool, LXTSELXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTSELXT` writer - LXT Mode Selection\nNote 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTSELXT_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTSELXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTSELXT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LXT work as crystal mode. PF.4 and PF.5 are configured as external low speed crystal (LXT) pins"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTSELXT_A::_0)
    }
    #[doc = "LXT work as external clock mode. PF.5 is configured as external clock input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTSELXT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "LXT Gain Control Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LXTGAIN_A {
    #[doc = "0: LXT Crystal ESR = 35K, CL=12.5pF"]
    _0 = 0,
    #[doc = "2: LXT Crystal ESR = 70K, CL=12.5pF"]
    _2 = 2,
}
impl From<LXTGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: LXTGAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LXTGAIN` reader - LXT Gain Control Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTGAIN_R(crate::FieldReader<u8, LXTGAIN_A>);
impl LXTGAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LXTGAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LXTGAIN_A> {
        match self.bits {
            0 => Some(LXTGAIN_A::_0),
            2 => Some(LXTGAIN_A::_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTGAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == LXTGAIN_A::_2
    }
}
impl core::ops::Deref for LXTGAIN_R {
    type Target = crate::FieldReader<u8, LXTGAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTGAIN` writer - LXT Gain Control Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTGAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTGAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LXT Crystal ESR = 35K, CL=12.5pF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTGAIN_A::_0)
    }
    #[doc = "LXT Crystal ESR = 70K, CL=12.5pF"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(LXTGAIN_A::_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HXT Enable Bit (Write Protect) Note1 : reset by power on reset Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxten(&self) -> HXTEN_R {
        HXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LXT Enable Bit (Write Protect) Note1 : Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxten(&self) -> LXTEN_R {
        LXTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HIRC Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hircen(&self) -> HIRCEN_R {
        HIRCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LIRC Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lircen(&self) -> LIRCEN_R {
        LIRCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable the Wake-up Delay Counter (Write Protect) When the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable. The delayed clock cycle is 4096 clock cycles when chip works at external high speed crystal oscillator (HXT), and 512 clock cycles when chip works at internal high speed RC oscillator (HIRC). Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkdly(&self) -> PDWKDLY_R {
        PDWKDLY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect) Note 1: The interrupt will occur when both PDWKIF and PDWKIEN are high. Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkien(&self) -> PDWKIEN_R {
        PDWKIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power-down Mode Wake-up Interrupt Status Set by 'Power-down wake-up event', it indicates that resume from Power-down mode' The flag is set if any wake-up source is occurred. Refer Power Modes and Wake-up Sources chapter. Note 1: Write 1 to clear the bit to 0. Note 2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pdwkif(&self) -> PDWKIF_R {
        PDWKIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System Power-down Enable (Write Protect) When this bit is set to 1, Power-down mode is enabled and chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode. When chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down. In Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode. If user disable LIRC before entering power-down mode, this bit should be set after LIRC disabled 50us. In Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - HXT Gain Control Bit (Write Protect) This is a protected register. Please refer to open lock sequence to program it. Gain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. Others: Reserved Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxtgain(&self) -> HXTGAIN_R {
        HXTGAIN_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - LXT Mode Selection Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxtselxt(&self) -> LXTSELXT_R {
        LXTSELXT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - LXT Gain Control Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxtgain(&self) -> LXTGAIN_R {
        LXTGAIN_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HXT Enable Bit (Write Protect) Note1 : reset by power on reset Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxten(&mut self) -> HXTEN_W {
        HXTEN_W { w: self }
    }
    #[doc = "Bit 1 - LXT Enable Bit (Write Protect) Note1 : Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxten(&mut self) -> LXTEN_W {
        LXTEN_W { w: self }
    }
    #[doc = "Bit 2 - HIRC Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hircen(&mut self) -> HIRCEN_W {
        HIRCEN_W { w: self }
    }
    #[doc = "Bit 3 - LIRC Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lircen(&mut self) -> LIRCEN_W {
        LIRCEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the Wake-up Delay Counter (Write Protect) When the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable. The delayed clock cycle is 4096 clock cycles when chip works at external high speed crystal oscillator (HXT), and 512 clock cycles when chip works at internal high speed RC oscillator (HIRC). Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkdly(&mut self) -> PDWKDLY_W {
        PDWKDLY_W { w: self }
    }
    #[doc = "Bit 5 - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect) Note 1: The interrupt will occur when both PDWKIF and PDWKIEN are high. Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkien(&mut self) -> PDWKIEN_W {
        PDWKIEN_W { w: self }
    }
    #[doc = "Bit 6 - Power-down Mode Wake-up Interrupt Status Set by 'Power-down wake-up event', it indicates that resume from Power-down mode' The flag is set if any wake-up source is occurred. Refer Power Modes and Wake-up Sources chapter. Note 1: Write 1 to clear the bit to 0. Note 2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pdwkif(&mut self) -> PDWKIF_W {
        PDWKIF_W { w: self }
    }
    #[doc = "Bit 7 - System Power-down Enable (Write Protect) When this bit is set to 1, Power-down mode is enabled and chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode. When chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down. In Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode. If user disable LIRC before entering power-down mode, this bit should be set after LIRC disabled 50us. In Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bits 20:22 - HXT Gain Control Bit (Write Protect) This is a protected register. Please refer to open lock sequence to program it. Gain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. Others: Reserved Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxtgain(&mut self) -> HXTGAIN_W {
        HXTGAIN_W { w: self }
    }
    #[doc = "Bit 24 - LXT Mode Selection Note 2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxtselxt(&mut self) -> LXTSELXT_W {
        LXTSELXT_W { w: self }
    }
    #[doc = "Bits 25:26 - LXT Gain Control Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxtgain(&mut self) -> LXTGAIN_W {
        LXTGAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Power-down Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pwrctl](index.html) module"]
pub struct CLK_PWRCTL_SPEC;
impl crate::RegisterSpec for CLK_PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pwrctl::R](R) reader structure"]
impl crate::Readable for CLK_PWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pwrctl::W](W) writer structure"]
impl crate::Writable for CLK_PWRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PWRCTL to value 0x0231_0010"]
impl crate::Resettable for CLK_PWRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0231_0010
    }
}
