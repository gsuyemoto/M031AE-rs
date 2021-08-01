#[doc = "Register `CLK_APBCLK0` reader"]
pub struct R(crate::R<CLK_APBCLK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_APBCLK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_APBCLK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_APBCLK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_APBCLK0` writer"]
pub struct W(crate::W<CLK_APBCLK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_APBCLK0_SPEC>;
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
impl From<crate::W<CLK_APBCLK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_APBCLK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Timer Clock Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: This bit is reset by power on reset, Watchdog reset or software chip reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCKEN_A {
    #[doc = "0: Watchdog timer clock Disabled"]
    _0 = 0,
    #[doc = "1: Watchdog timer clock Enabled"]
    _1 = 1,
}
impl From<WDTCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCKEN` reader - Watchdog Timer Clock Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: This bit is reset by power on reset, Watchdog reset or software chip reset."]
pub struct WDTCKEN_R(crate::FieldReader<bool, WDTCKEN_A>);
impl WDTCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCKEN_A {
        match self.bits {
            false => WDTCKEN_A::_0,
            true => WDTCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDTCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDTCKEN_A::_1
    }
}
impl core::ops::Deref for WDTCKEN_R {
    type Target = crate::FieldReader<bool, WDTCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTCKEN` writer - Watchdog Timer Clock Enable Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: This bit is reset by power on reset, Watchdog reset or software chip reset."]
pub struct WDTCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog timer clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTCKEN_A::_0)
    }
    #[doc = "Watchdog timer clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTCKEN_A::_1)
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
#[doc = "RTC Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCKEN_A {
    #[doc = "0: RTC clock Disabled"]
    _0 = 0,
    #[doc = "1: RTC clock Enabled"]
    _1 = 1,
}
impl From<RTCCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCKEN` reader - RTC Clock Enable Bit"]
pub struct RTCCKEN_R(crate::FieldReader<bool, RTCCKEN_A>);
impl RTCCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCKEN_A {
        match self.bits {
            false => RTCCKEN_A::_0,
            true => RTCCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTCCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTCCKEN_A::_1
    }
}
impl core::ops::Deref for RTCCKEN_R {
    type Target = crate::FieldReader<bool, RTCCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCKEN` writer - RTC Clock Enable Bit"]
pub struct RTCCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCKEN_A::_0)
    }
    #[doc = "RTC clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCKEN_A::_1)
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
#[doc = "Timer0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0CKEN_A {
    #[doc = "0: Timer0 clock Disabled"]
    _0 = 0,
    #[doc = "1: Timer0 clock Enabled"]
    _1 = 1,
}
impl From<TMR0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0CKEN` reader - Timer0 Clock Enable Bit"]
pub struct TMR0CKEN_R(crate::FieldReader<bool, TMR0CKEN_A>);
impl TMR0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0CKEN_A {
        match self.bits {
            false => TMR0CKEN_A::_0,
            true => TMR0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR0CKEN_A::_1
    }
}
impl core::ops::Deref for TMR0CKEN_R {
    type Target = crate::FieldReader<bool, TMR0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0CKEN` writer - Timer0 Clock Enable Bit"]
pub struct TMR0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0CKEN_A::_0)
    }
    #[doc = "Timer0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0CKEN_A::_1)
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
#[doc = "Timer1 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1CKEN_A {
    #[doc = "0: Timer1 clock Disabled"]
    _0 = 0,
    #[doc = "1: Timer1 clock Enabled"]
    _1 = 1,
}
impl From<TMR1CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1CKEN` reader - Timer1 Clock Enable Bit"]
pub struct TMR1CKEN_R(crate::FieldReader<bool, TMR1CKEN_A>);
impl TMR1CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR1CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1CKEN_A {
        match self.bits {
            false => TMR1CKEN_A::_0,
            true => TMR1CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR1CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR1CKEN_A::_1
    }
}
impl core::ops::Deref for TMR1CKEN_R {
    type Target = crate::FieldReader<bool, TMR1CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1CKEN` writer - Timer1 Clock Enable Bit"]
pub struct TMR1CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer1 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1CKEN_A::_0)
    }
    #[doc = "Timer1 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1CKEN_A::_1)
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
#[doc = "Timer2 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2CKEN_A {
    #[doc = "0: Timer2 clock Disabled"]
    _0 = 0,
    #[doc = "1: Timer2 clock Enabled"]
    _1 = 1,
}
impl From<TMR2CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2CKEN` reader - Timer2 Clock Enable Bit"]
pub struct TMR2CKEN_R(crate::FieldReader<bool, TMR2CKEN_A>);
impl TMR2CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR2CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2CKEN_A {
        match self.bits {
            false => TMR2CKEN_A::_0,
            true => TMR2CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR2CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR2CKEN_A::_1
    }
}
impl core::ops::Deref for TMR2CKEN_R {
    type Target = crate::FieldReader<bool, TMR2CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2CKEN` writer - Timer2 Clock Enable Bit"]
pub struct TMR2CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer2 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2CKEN_A::_0)
    }
    #[doc = "Timer2 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2CKEN_A::_1)
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
#[doc = "Timer3 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3CKEN_A {
    #[doc = "0: Timer3 clock Disabled"]
    _0 = 0,
    #[doc = "1: Timer3 clock Enabled"]
    _1 = 1,
}
impl From<TMR3CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR3CKEN` reader - Timer3 Clock Enable Bit"]
pub struct TMR3CKEN_R(crate::FieldReader<bool, TMR3CKEN_A>);
impl TMR3CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR3CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3CKEN_A {
        match self.bits {
            false => TMR3CKEN_A::_0,
            true => TMR3CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR3CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR3CKEN_A::_1
    }
}
impl core::ops::Deref for TMR3CKEN_R {
    type Target = crate::FieldReader<bool, TMR3CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR3CKEN` writer - Timer3 Clock Enable Bit"]
pub struct TMR3CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer3 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3CKEN_A::_0)
    }
    #[doc = "Timer3 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3CKEN_A::_1)
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
#[doc = "CLKO Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOCKEN_A {
    #[doc = "0: CLKO clock Disabled"]
    _0 = 0,
    #[doc = "1: CLKO clock Enabled"]
    _1 = 1,
}
impl From<CLKOCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOCKEN` reader - CLKO Clock Enable Bit"]
pub struct CLKOCKEN_R(crate::FieldReader<bool, CLKOCKEN_A>);
impl CLKOCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKOCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOCKEN_A {
        match self.bits {
            false => CLKOCKEN_A::_0,
            true => CLKOCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKOCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKOCKEN_A::_1
    }
}
impl core::ops::Deref for CLKOCKEN_R {
    type Target = crate::FieldReader<bool, CLKOCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOCKEN` writer - CLKO Clock Enable Bit"]
pub struct CLKOCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CLKO clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOCKEN_A::_0)
    }
    #[doc = "CLKO clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOCKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Analog Comparator 0/1 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP01CKEN_A {
    #[doc = "0: Analog comparator 0/1 clock Disabled"]
    _0 = 0,
    #[doc = "1: Analog comparator 0/1 clock Enabled"]
    _1 = 1,
}
impl From<ACMP01CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP01CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP01CKEN` reader - Analog Comparator 0/1 Clock Enable Bit"]
pub struct ACMP01CKEN_R(crate::FieldReader<bool, ACMP01CKEN_A>);
impl ACMP01CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP01CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP01CKEN_A {
        match self.bits {
            false => ACMP01CKEN_A::_0,
            true => ACMP01CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMP01CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMP01CKEN_A::_1
    }
}
impl core::ops::Deref for ACMP01CKEN_R {
    type Target = crate::FieldReader<bool, ACMP01CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP01CKEN` writer - Analog Comparator 0/1 Clock Enable Bit"]
pub struct ACMP01CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP01CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP01CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog comparator 0/1 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP01CKEN_A::_0)
    }
    #[doc = "Analog comparator 0/1 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP01CKEN_A::_1)
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
#[doc = "I2C0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0CKEN_A {
    #[doc = "0: I2C0 clock Disabled"]
    _0 = 0,
    #[doc = "1: I2C0 clock Enabled"]
    _1 = 1,
}
impl From<I2C0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0CKEN` reader - I2C0 Clock Enable Bit"]
pub struct I2C0CKEN_R(crate::FieldReader<bool, I2C0CKEN_A>);
impl I2C0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0CKEN_A {
        match self.bits {
            false => I2C0CKEN_A::_0,
            true => I2C0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C0CKEN_A::_1
    }
}
impl core::ops::Deref for I2C0CKEN_R {
    type Target = crate::FieldReader<bool, I2C0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0CKEN` writer - I2C0 Clock Enable Bit"]
pub struct I2C0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0CKEN_A::_0)
    }
    #[doc = "I2C0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "I2C1 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1CKEN_A {
    #[doc = "0: I2C1 clock Disabled"]
    _0 = 0,
    #[doc = "1: I2C1 clock Enabled"]
    _1 = 1,
}
impl From<I2C1CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1CKEN` reader - I2C1 Clock Enable Bit"]
pub struct I2C1CKEN_R(crate::FieldReader<bool, I2C1CKEN_A>);
impl I2C1CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1CKEN_A {
        match self.bits {
            false => I2C1CKEN_A::_0,
            true => I2C1CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C1CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C1CKEN_A::_1
    }
}
impl core::ops::Deref for I2C1CKEN_R {
    type Target = crate::FieldReader<bool, I2C1CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1CKEN` writer - I2C1 Clock Enable Bit"]
pub struct I2C1CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C1 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1CKEN_A::_0)
    }
    #[doc = "I2C1 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "QSPI0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0CKEN_A {
    #[doc = "0: QSPI0 clock Disabled"]
    _0 = 0,
    #[doc = "1: QSPI0 clock Enabled"]
    _1 = 1,
}
impl From<QSPI0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPI0CKEN` reader - QSPI0 Clock Enable Bit"]
pub struct QSPI0CKEN_R(crate::FieldReader<bool, QSPI0CKEN_A>);
impl QSPI0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QSPI0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI0CKEN_A {
        match self.bits {
            false => QSPI0CKEN_A::_0,
            true => QSPI0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == QSPI0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == QSPI0CKEN_A::_1
    }
}
impl core::ops::Deref for QSPI0CKEN_R {
    type Target = crate::FieldReader<bool, QSPI0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI0CKEN` writer - QSPI0 Clock Enable Bit"]
pub struct QSPI0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPI0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QSPI0CKEN_A::_0)
    }
    #[doc = "QSPI0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QSPI0CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "SPI0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0CKEN_A {
    #[doc = "0: SPI0 clock Disabled"]
    _0 = 0,
    #[doc = "1: SPI0 clock Enabled"]
    _1 = 1,
}
impl From<SPI0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0CKEN` reader - SPI0 Clock Enable Bit"]
pub struct SPI0CKEN_R(crate::FieldReader<bool, SPI0CKEN_A>);
impl SPI0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0CKEN_A {
        match self.bits {
            false => SPI0CKEN_A::_0,
            true => SPI0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0CKEN_A::_1
    }
}
impl core::ops::Deref for SPI0CKEN_R {
    type Target = crate::FieldReader<bool, SPI0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0CKEN` writer - SPI0 Clock Enable Bit"]
pub struct SPI0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0CKEN_A::_0)
    }
    #[doc = "SPI0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "UART0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0CKEN_A {
    #[doc = "0: UART0 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART0 clock Enabled"]
    _1 = 1,
}
impl From<UART0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0CKEN` reader - UART0 Clock Enable Bit"]
pub struct UART0CKEN_R(crate::FieldReader<bool, UART0CKEN_A>);
impl UART0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0CKEN_A {
        match self.bits {
            false => UART0CKEN_A::_0,
            true => UART0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0CKEN_A::_1
    }
}
impl core::ops::Deref for UART0CKEN_R {
    type Target = crate::FieldReader<bool, UART0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0CKEN` writer - UART0 Clock Enable Bit"]
pub struct UART0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0CKEN_A::_0)
    }
    #[doc = "UART0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "UART1 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1CKEN_A {
    #[doc = "0: UART1 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART1 clock Enabled"]
    _1 = 1,
}
impl From<UART1CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART1CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1CKEN` reader - UART1 Clock Enable Bit"]
pub struct UART1CKEN_R(crate::FieldReader<bool, UART1CKEN_A>);
impl UART1CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1CKEN_A {
        match self.bits {
            false => UART1CKEN_A::_0,
            true => UART1CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART1CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART1CKEN_A::_1
    }
}
impl core::ops::Deref for UART1CKEN_R {
    type Target = crate::FieldReader<bool, UART1CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1CKEN` writer - UART1 Clock Enable Bit"]
pub struct UART1CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART1 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1CKEN_A::_0)
    }
    #[doc = "UART1 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "UART2 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2CKEN_A {
    #[doc = "0: UART2 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART2 clock Enabled"]
    _1 = 1,
}
impl From<UART2CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART2CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2CKEN` reader - UART2 Clock Enable Bit"]
pub struct UART2CKEN_R(crate::FieldReader<bool, UART2CKEN_A>);
impl UART2CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2CKEN_A {
        match self.bits {
            false => UART2CKEN_A::_0,
            true => UART2CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART2CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART2CKEN_A::_1
    }
}
impl core::ops::Deref for UART2CKEN_R {
    type Target = crate::FieldReader<bool, UART2CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2CKEN` writer - UART2 Clock Enable Bit"]
pub struct UART2CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART2 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2CKEN_A::_0)
    }
    #[doc = "UART2 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "UART3 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3CKEN_A {
    #[doc = "0: UART3 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART3 clock Enabled"]
    _1 = 1,
}
impl From<UART3CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART3CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART3CKEN` reader - UART3 Clock Enable Bit"]
pub struct UART3CKEN_R(crate::FieldReader<bool, UART3CKEN_A>);
impl UART3CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART3CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3CKEN_A {
        match self.bits {
            false => UART3CKEN_A::_0,
            true => UART3CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART3CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART3CKEN_A::_1
    }
}
impl core::ops::Deref for UART3CKEN_R {
    type Target = crate::FieldReader<bool, UART3CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3CKEN` writer - UART3 Clock Enable Bit"]
pub struct UART3CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART3 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3CKEN_A::_0)
    }
    #[doc = "UART3 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "UART4 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4CKEN_A {
    #[doc = "0: UART4 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART4 clock Enabled"]
    _1 = 1,
}
impl From<UART4CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART4CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4CKEN` reader - UART4 Clock Enable Bit"]
pub struct UART4CKEN_R(crate::FieldReader<bool, UART4CKEN_A>);
impl UART4CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4CKEN_A {
        match self.bits {
            false => UART4CKEN_A::_0,
            true => UART4CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART4CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART4CKEN_A::_1
    }
}
impl core::ops::Deref for UART4CKEN_R {
    type Target = crate::FieldReader<bool, UART4CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4CKEN` writer - UART4 Clock Enable Bit"]
pub struct UART4CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART4 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART4CKEN_A::_0)
    }
    #[doc = "UART4 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART4CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "UART5 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART5CKEN_A {
    #[doc = "0: UART5 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART5 clock Enabled"]
    _1 = 1,
}
impl From<UART5CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART5CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART5CKEN` reader - UART5 Clock Enable Bit"]
pub struct UART5CKEN_R(crate::FieldReader<bool, UART5CKEN_A>);
impl UART5CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5CKEN_A {
        match self.bits {
            false => UART5CKEN_A::_0,
            true => UART5CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART5CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART5CKEN_A::_1
    }
}
impl core::ops::Deref for UART5CKEN_R {
    type Target = crate::FieldReader<bool, UART5CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5CKEN` writer - UART5 Clock Enable Bit"]
pub struct UART5CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART5 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART5CKEN_A::_0)
    }
    #[doc = "UART5 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART5CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "UART6 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART6CKEN_A {
    #[doc = "0: UART6 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART6 clock Enabled"]
    _1 = 1,
}
impl From<UART6CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART6CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART6CKEN` reader - UART6 Clock Enable Bit"]
pub struct UART6CKEN_R(crate::FieldReader<bool, UART6CKEN_A>);
impl UART6CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART6CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART6CKEN_A {
        match self.bits {
            false => UART6CKEN_A::_0,
            true => UART6CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART6CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART6CKEN_A::_1
    }
}
impl core::ops::Deref for UART6CKEN_R {
    type Target = crate::FieldReader<bool, UART6CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART6CKEN` writer - UART6 Clock Enable Bit"]
pub struct UART6CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART6CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART6CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART6 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART6CKEN_A::_0)
    }
    #[doc = "UART6 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART6CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "UART7 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART7CKEN_A {
    #[doc = "0: UART7 clock Disabled"]
    _0 = 0,
    #[doc = "1: UART7 clock Enabled"]
    _1 = 1,
}
impl From<UART7CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART7CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART7CKEN` reader - UART7 Clock Enable Bit"]
pub struct UART7CKEN_R(crate::FieldReader<bool, UART7CKEN_A>);
impl UART7CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART7CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART7CKEN_A {
        match self.bits {
            false => UART7CKEN_A::_0,
            true => UART7CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART7CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART7CKEN_A::_1
    }
}
impl core::ops::Deref for UART7CKEN_R {
    type Target = crate::FieldReader<bool, UART7CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7CKEN` writer - UART7 Clock Enable Bit"]
pub struct UART7CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART7 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART7CKEN_A::_0)
    }
    #[doc = "UART7 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART7CKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "USB Device Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDCKEN_A {
    #[doc = "0: USB Device clock Disabled"]
    _0 = 0,
    #[doc = "1: USB Device clock Enabled"]
    _1 = 1,
}
impl From<USBDCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBDCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDCKEN` reader - USB Device Clock Enable Bit"]
pub struct USBDCKEN_R(crate::FieldReader<bool, USBDCKEN_A>);
impl USBDCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBDCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDCKEN_A {
        match self.bits {
            false => USBDCKEN_A::_0,
            true => USBDCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBDCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBDCKEN_A::_1
    }
}
impl core::ops::Deref for USBDCKEN_R {
    type Target = crate::FieldReader<bool, USBDCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDCKEN` writer - USB Device Clock Enable Bit"]
pub struct USBDCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB Device clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDCKEN_A::_0)
    }
    #[doc = "USB Device clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDCKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Analog-digital-converter (ADC) Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCKEN_A {
    #[doc = "0: ADC clock Disabled"]
    _0 = 0,
    #[doc = "1: ADC clock Enabled"]
    _1 = 1,
}
impl From<ADCCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCKEN` reader - Analog-digital-converter (ADC) Clock Enable Bit"]
pub struct ADCCKEN_R(crate::FieldReader<bool, ADCCKEN_A>);
impl ADCCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCKEN_A {
        match self.bits {
            false => ADCCKEN_A::_0,
            true => ADCCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCCKEN_A::_1
    }
}
impl core::ops::Deref for ADCCKEN_R {
    type Target = crate::FieldReader<bool, ADCCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCCKEN` writer - Analog-digital-converter (ADC) Clock Enable Bit"]
pub struct ADCCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCCKEN_A::_0)
    }
    #[doc = "ADC clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCCKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timer Clock Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: This bit is reset by power on reset, Watchdog reset or software chip reset."]
    #[inline(always)]
    pub fn wdtcken(&self) -> WDTCKEN_R {
        WDTCKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Clock Enable Bit"]
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr0cken(&self) -> TMR0CKEN_R {
        TMR0CKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer1 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr1cken(&self) -> TMR1CKEN_R {
        TMR1CKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer2 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr2cken(&self) -> TMR2CKEN_R {
        TMR2CKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer3 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr3cken(&self) -> TMR3CKEN_R {
        TMR3CKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CLKO Clock Enable Bit"]
    #[inline(always)]
    pub fn clkocken(&self) -> CLKOCKEN_R {
        CLKOCKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Clock Enable Bit"]
    #[inline(always)]
    pub fn acmp01cken(&self) -> ACMP01CKEN_R {
        ACMP01CKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 Clock Enable Bit"]
    #[inline(always)]
    pub fn i2c0cken(&self) -> I2C0CKEN_R {
        I2C0CKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C1 Clock Enable Bit"]
    #[inline(always)]
    pub fn i2c1cken(&self) -> I2C1CKEN_R {
        I2C1CKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - QSPI0 Clock Enable Bit"]
    #[inline(always)]
    pub fn qspi0cken(&self) -> QSPI0CKEN_R {
        QSPI0CKEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI0 Clock Enable Bit"]
    #[inline(always)]
    pub fn spi0cken(&self) -> SPI0CKEN_R {
        SPI0CKEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart0cken(&self) -> UART0CKEN_R {
        UART0CKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart1cken(&self) -> UART1CKEN_R {
        UART1CKEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart2cken(&self) -> UART2CKEN_R {
        UART2CKEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART3 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart3cken(&self) -> UART3CKEN_R {
        UART3CKEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART4 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart4cken(&self) -> UART4CKEN_R {
        UART4CKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - UART5 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart5cken(&self) -> UART5CKEN_R {
        UART5CKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UART6 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart6cken(&self) -> UART6CKEN_R {
        UART6CKEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UART7 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart7cken(&self) -> UART7CKEN_R {
        UART7CKEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB Device Clock Enable Bit"]
    #[inline(always)]
    pub fn usbdcken(&self) -> USBDCKEN_R {
        USBDCKEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Analog-digital-converter (ADC) Clock Enable Bit"]
    #[inline(always)]
    pub fn adccken(&self) -> ADCCKEN_R {
        ADCCKEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Clock Enable Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: This bit is reset by power on reset, Watchdog reset or software chip reset."]
    #[inline(always)]
    pub fn wdtcken(&mut self) -> WDTCKEN_W {
        WDTCKEN_W { w: self }
    }
    #[doc = "Bit 1 - RTC Clock Enable Bit"]
    #[inline(always)]
    pub fn rtccken(&mut self) -> RTCCKEN_W {
        RTCCKEN_W { w: self }
    }
    #[doc = "Bit 2 - Timer0 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr0cken(&mut self) -> TMR0CKEN_W {
        TMR0CKEN_W { w: self }
    }
    #[doc = "Bit 3 - Timer1 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr1cken(&mut self) -> TMR1CKEN_W {
        TMR1CKEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer2 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr2cken(&mut self) -> TMR2CKEN_W {
        TMR2CKEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer3 Clock Enable Bit"]
    #[inline(always)]
    pub fn tmr3cken(&mut self) -> TMR3CKEN_W {
        TMR3CKEN_W { w: self }
    }
    #[doc = "Bit 6 - CLKO Clock Enable Bit"]
    #[inline(always)]
    pub fn clkocken(&mut self) -> CLKOCKEN_W {
        CLKOCKEN_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Clock Enable Bit"]
    #[inline(always)]
    pub fn acmp01cken(&mut self) -> ACMP01CKEN_W {
        ACMP01CKEN_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 Clock Enable Bit"]
    #[inline(always)]
    pub fn i2c0cken(&mut self) -> I2C0CKEN_W {
        I2C0CKEN_W { w: self }
    }
    #[doc = "Bit 9 - I2C1 Clock Enable Bit"]
    #[inline(always)]
    pub fn i2c1cken(&mut self) -> I2C1CKEN_W {
        I2C1CKEN_W { w: self }
    }
    #[doc = "Bit 12 - QSPI0 Clock Enable Bit"]
    #[inline(always)]
    pub fn qspi0cken(&mut self) -> QSPI0CKEN_W {
        QSPI0CKEN_W { w: self }
    }
    #[doc = "Bit 13 - SPI0 Clock Enable Bit"]
    #[inline(always)]
    pub fn spi0cken(&mut self) -> SPI0CKEN_W {
        SPI0CKEN_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart0cken(&mut self) -> UART0CKEN_W {
        UART0CKEN_W { w: self }
    }
    #[doc = "Bit 17 - UART1 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart1cken(&mut self) -> UART1CKEN_W {
        UART1CKEN_W { w: self }
    }
    #[doc = "Bit 18 - UART2 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart2cken(&mut self) -> UART2CKEN_W {
        UART2CKEN_W { w: self }
    }
    #[doc = "Bit 19 - UART3 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart3cken(&mut self) -> UART3CKEN_W {
        UART3CKEN_W { w: self }
    }
    #[doc = "Bit 20 - UART4 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart4cken(&mut self) -> UART4CKEN_W {
        UART4CKEN_W { w: self }
    }
    #[doc = "Bit 21 - UART5 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart5cken(&mut self) -> UART5CKEN_W {
        UART5CKEN_W { w: self }
    }
    #[doc = "Bit 22 - UART6 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart6cken(&mut self) -> UART6CKEN_W {
        UART6CKEN_W { w: self }
    }
    #[doc = "Bit 23 - UART7 Clock Enable Bit"]
    #[inline(always)]
    pub fn uart7cken(&mut self) -> UART7CKEN_W {
        UART7CKEN_W { w: self }
    }
    #[doc = "Bit 27 - USB Device Clock Enable Bit"]
    #[inline(always)]
    pub fn usbdcken(&mut self) -> USBDCKEN_W {
        USBDCKEN_W { w: self }
    }
    #[doc = "Bit 28 - Analog-digital-converter (ADC) Clock Enable Bit"]
    #[inline(always)]
    pub fn adccken(&mut self) -> ADCCKEN_W {
        ADCCKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Devices Clock Enable Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_apbclk0](index.html) module"]
pub struct CLK_APBCLK0_SPEC;
impl crate::RegisterSpec for CLK_APBCLK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_apbclk0::R](R) reader structure"]
impl crate::Readable for CLK_APBCLK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_apbclk0::W](W) writer structure"]
impl crate::Writable for CLK_APBCLK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_APBCLK0 to value 0x01"]
impl crate::Resettable for CLK_APBCLK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
