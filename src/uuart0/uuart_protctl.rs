#[doc = "Register `UUART_PROTCTL` reader"]
pub struct R(crate::R<UUART_PROTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_PROTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_PROTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_PROTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_PROTCTL` writer"]
pub struct W(crate::W<UUART_PROTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_PROTCTL_SPEC>;
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
impl From<crate::W<UUART_PROTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_PROTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop Bits\nThis bit defines the number of stop bits in an UART frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPB_A {
    #[doc = "0: The number of stop bits is 1"]
    _0 = 0,
    #[doc = "1: The number of stop bits is 2"]
    _1 = 1,
}
impl From<STOPB_A> for bool {
    #[inline(always)]
    fn from(variant: STOPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPB` reader - Stop Bits\nThis bit defines the number of stop bits in an UART frame."]
pub struct STOPB_R(crate::FieldReader<bool, STOPB_A>);
impl STOPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPB_A {
        match self.bits {
            false => STOPB_A::_0,
            true => STOPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOPB_A::_1
    }
}
impl core::ops::Deref for STOPB_R {
    type Target = crate::FieldReader<bool, STOPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPB` writer - Stop Bits\nThis bit defines the number of stop bits in an UART frame."]
pub struct STOPB_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The number of stop bits is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPB_A::_0)
    }
    #[doc = "The number of stop bits is 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPB_A::_1)
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
#[doc = "Parity Enable Bit\nThis bit defines the parity bit is enabled in an UART frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYEN_A {
    #[doc = "0: The parity bit Disabled"]
    _0 = 0,
    #[doc = "1: The parity bit Enabled"]
    _1 = 1,
}
impl From<PARITYEN_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYEN` reader - Parity Enable Bit\nThis bit defines the parity bit is enabled in an UART frame."]
pub struct PARITYEN_R(crate::FieldReader<bool, PARITYEN_A>);
impl PARITYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYEN_A {
        match self.bits {
            false => PARITYEN_A::_0,
            true => PARITYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PARITYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PARITYEN_A::_1
    }
}
impl core::ops::Deref for PARITYEN_R {
    type Target = crate::FieldReader<bool, PARITYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITYEN` writer - Parity Enable Bit\nThis bit defines the parity bit is enabled in an UART frame."]
pub struct PARITYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The parity bit Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARITYEN_A::_0)
    }
    #[doc = "The parity bit Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARITYEN_A::_1)
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
#[doc = "Even Parity Enable Bit\nNote: This bit has effect only when PARITYEN is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENPARITY_A {
    #[doc = "0: Odd number of logic 1's is transmitted and checked in each word"]
    _0 = 0,
    #[doc = "1: Even number of logic 1's is transmitted and checked in each word"]
    _1 = 1,
}
impl From<EVENPARITY_A> for bool {
    #[inline(always)]
    fn from(variant: EVENPARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENPARITY` reader - Even Parity Enable Bit\nNote: This bit has effect only when PARITYEN is set."]
pub struct EVENPARITY_R(crate::FieldReader<bool, EVENPARITY_A>);
impl EVENPARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENPARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENPARITY_A {
        match self.bits {
            false => EVENPARITY_A::_0,
            true => EVENPARITY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EVENPARITY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EVENPARITY_A::_1
    }
}
impl core::ops::Deref for EVENPARITY_R {
    type Target = crate::FieldReader<bool, EVENPARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENPARITY` writer - Even Parity Enable Bit\nNote: This bit has effect only when PARITYEN is set."]
pub struct EVENPARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENPARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENPARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Odd number of logic 1's is transmitted and checked in each word"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EVENPARITY_A::_0)
    }
    #[doc = "Even number of logic 1's is transmitted and checked in each word"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EVENPARITY_A::_1)
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
#[doc = "nRTS Auto-flow Control Enable Bit\nNote: This bit has effect only when the RTSAUDIREN is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSAUTOEN_A {
    #[doc = "0: nRTS auto-flow control Disabled"]
    _0 = 0,
    #[doc = "1: nRTS auto-flow control Enabled"]
    _1 = 1,
}
impl From<RTSAUTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSAUTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSAUTOEN` reader - nRTS Auto-flow Control Enable Bit\nNote: This bit has effect only when the RTSAUDIREN is not set."]
pub struct RTSAUTOEN_R(crate::FieldReader<bool, RTSAUTOEN_A>);
impl RTSAUTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSAUTOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSAUTOEN_A {
        match self.bits {
            false => RTSAUTOEN_A::_0,
            true => RTSAUTOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTSAUTOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTSAUTOEN_A::_1
    }
}
impl core::ops::Deref for RTSAUTOEN_R {
    type Target = crate::FieldReader<bool, RTSAUTOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSAUTOEN` writer - nRTS Auto-flow Control Enable Bit\nNote: This bit has effect only when the RTSAUDIREN is not set."]
pub struct RTSAUTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSAUTOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSAUTOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nRTS auto-flow control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSAUTOEN_A::_0)
    }
    #[doc = "nRTS auto-flow control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSAUTOEN_A::_1)
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
#[doc = "nCTS Auto-flow Control Enable Bit\nWhen nCTS auto-flow is enabled, the UART will send data to external device when nCTS input assert (UART will not send data to device if nCTS input is dis-asserted).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSAUTOEN_A {
    #[doc = "0: nCTS auto-flow control Disabled"]
    _0 = 0,
    #[doc = "1: nCTS auto-flow control Enabled"]
    _1 = 1,
}
impl From<CTSAUTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSAUTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSAUTOEN` reader - nCTS Auto-flow Control Enable Bit\nWhen nCTS auto-flow is enabled, the UART will send data to external device when nCTS input assert (UART will not send data to device if nCTS input is dis-asserted)."]
pub struct CTSAUTOEN_R(crate::FieldReader<bool, CTSAUTOEN_A>);
impl CTSAUTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSAUTOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSAUTOEN_A {
        match self.bits {
            false => CTSAUTOEN_A::_0,
            true => CTSAUTOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSAUTOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSAUTOEN_A::_1
    }
}
impl core::ops::Deref for CTSAUTOEN_R {
    type Target = crate::FieldReader<bool, CTSAUTOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSAUTOEN` writer - nCTS Auto-flow Control Enable Bit\nWhen nCTS auto-flow is enabled, the UART will send data to external device when nCTS input assert (UART will not send data to device if nCTS input is dis-asserted)."]
pub struct CTSAUTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSAUTOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSAUTOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nCTS auto-flow control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSAUTOEN_A::_0)
    }
    #[doc = "nCTS auto-flow control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSAUTOEN_A::_1)
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
#[doc = "nRTS Auto Direction Enable Bit\nWhen nRTS auto direction is enabled, if the transmitted bytes in the TX buffer is empty, the UART asserted nRTS signal automatically.\nNote 1: This bit is used for nRTS auto direction control for RS485.\nNote 2: This bit has effect only when the RTSAUTOEN is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSAUDIREN_A {
    #[doc = "0: nRTS auto direction control Disabled"]
    _0 = 0,
    #[doc = "1: nRTS auto direction control Enabled"]
    _1 = 1,
}
impl From<RTSAUDIREN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSAUDIREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSAUDIREN` reader - nRTS Auto Direction Enable Bit\nWhen nRTS auto direction is enabled, if the transmitted bytes in the TX buffer is empty, the UART asserted nRTS signal automatically.\nNote 1: This bit is used for nRTS auto direction control for RS485.\nNote 2: This bit has effect only when the RTSAUTOEN is not set."]
pub struct RTSAUDIREN_R(crate::FieldReader<bool, RTSAUDIREN_A>);
impl RTSAUDIREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSAUDIREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSAUDIREN_A {
        match self.bits {
            false => RTSAUDIREN_A::_0,
            true => RTSAUDIREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTSAUDIREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTSAUDIREN_A::_1
    }
}
impl core::ops::Deref for RTSAUDIREN_R {
    type Target = crate::FieldReader<bool, RTSAUDIREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSAUDIREN` writer - nRTS Auto Direction Enable Bit\nWhen nRTS auto direction is enabled, if the transmitted bytes in the TX buffer is empty, the UART asserted nRTS signal automatically.\nNote 1: This bit is used for nRTS auto direction control for RS485.\nNote 2: This bit has effect only when the RTSAUTOEN is not set."]
pub struct RTSAUDIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSAUDIREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSAUDIREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nRTS auto direction control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSAUDIREN_A::_0)
    }
    #[doc = "nRTS auto direction control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSAUDIREN_A::_1)
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
#[doc = "Auto-baud Rate Detect Enable Bit\nNote: When the auto - baud rate detect operation finishes, hardware will clear this bit. The associated interrupt ABRDETIF (UUART_PROTST\\[9\\]) will be generated (If ARBIEN (UUART_PROTIEN \\[1\\]) is enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABREN_A {
    #[doc = "0: Auto-baud rate detect function Disabled"]
    _0 = 0,
    #[doc = "1: Auto-baud rate detect function Enabled"]
    _1 = 1,
}
impl From<ABREN_A> for bool {
    #[inline(always)]
    fn from(variant: ABREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABREN` reader - Auto-baud Rate Detect Enable Bit\nNote: When the auto - baud rate detect operation finishes, hardware will clear this bit. The associated interrupt ABRDETIF (UUART_PROTST\\[9\\]) will be generated (If ARBIEN (UUART_PROTIEN \\[1\\]) is enabled)."]
pub struct ABREN_R(crate::FieldReader<bool, ABREN_A>);
impl ABREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABREN_A {
        match self.bits {
            false => ABREN_A::_0,
            true => ABREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABREN_A::_1
    }
}
impl core::ops::Deref for ABREN_R {
    type Target = crate::FieldReader<bool, ABREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABREN` writer - Auto-baud Rate Detect Enable Bit\nNote: When the auto - baud rate detect operation finishes, hardware will clear this bit. The associated interrupt ABRDETIF (UUART_PROTST\\[9\\]) will be generated (If ARBIEN (UUART_PROTIEN \\[1\\]) is enabled)."]
pub struct ABREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate detect function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABREN_A::_0)
    }
    #[doc = "Auto-baud rate detect function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABREN_A::_1)
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
#[doc = "Data Wake-up Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATWKEN_A {
    #[doc = "0: Data wake-up mode Disabled"]
    _0 = 0,
    #[doc = "1: Data wake-up mode Enabled"]
    _1 = 1,
}
impl From<DATWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: DATWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATWKEN` reader - Data Wake-up Mode Enable Bit"]
pub struct DATWKEN_R(crate::FieldReader<bool, DATWKEN_A>);
impl DATWKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATWKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATWKEN_A {
        match self.bits {
            false => DATWKEN_A::_0,
            true => DATWKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATWKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATWKEN_A::_1
    }
}
impl core::ops::Deref for DATWKEN_R {
    type Target = crate::FieldReader<bool, DATWKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATWKEN` writer - Data Wake-up Mode Enable Bit"]
pub struct DATWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATWKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data wake-up mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATWKEN_A::_0)
    }
    #[doc = "Data wake-up mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATWKEN_A::_1)
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
#[doc = "nCTS Wake-up Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSWKEN_A {
    #[doc = "0: nCTS wake-up mode Disabled"]
    _0 = 0,
    #[doc = "1: nCTS wake-up mode Enabled"]
    _1 = 1,
}
impl From<CTSWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSWKEN` reader - nCTS Wake-up Mode Enable Bit"]
pub struct CTSWKEN_R(crate::FieldReader<bool, CTSWKEN_A>);
impl CTSWKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSWKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSWKEN_A {
        match self.bits {
            false => CTSWKEN_A::_0,
            true => CTSWKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSWKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSWKEN_A::_1
    }
}
impl core::ops::Deref for CTSWKEN_R {
    type Target = crate::FieldReader<bool, CTSWKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSWKEN` writer - nCTS Wake-up Mode Enable Bit"]
pub struct CTSWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSWKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nCTS wake-up mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSWKEN_A::_0)
    }
    #[doc = "nCTS wake-up mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSWKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `WAKECNT` reader - Wake-up Counter\nThese bits field indicate how many clock cycle selected by fPDS_CNT do the slave can get the 1st bit (start bit) when the device is wake-up from Power-down mode."]
pub struct WAKECNT_R(crate::FieldReader<u8, u8>);
impl WAKECNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAKECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKECNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKECNT` writer - Wake-up Counter\nThese bits field indicate how many clock cycle selected by fPDS_CNT do the slave can get the 1st bit (start bit) when the device is wake-up from Power-down mode."]
pub struct WAKECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `BRDETITV` reader - Baud Rate Detection Interval \nThis bit fields indicate how many clock cycle selected by TMCNTSRC (UUART_BRGEN \\[5\\]) does the slave calculates the baud rate in one bits. The order of the bus shall be 1 and 0 step by step (e.g. the input data pattern shall be 0x55). The user can read the value to know the current input baud rate of the bus whenever the ABRDETIF (UUART_PROTSTS\\[9\\]) is set.\nNote: This bit can be cleared to 0 by software writing '0' to the BRDETITV."]
pub struct BRDETITV_R(crate::FieldReader<u16, u16>);
impl BRDETITV_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRDETITV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRDETITV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRDETITV` writer - Baud Rate Detection Interval \nThis bit fields indicate how many clock cycle selected by TMCNTSRC (UUART_BRGEN \\[5\\]) does the slave calculates the baud rate in one bits. The order of the bus shall be 1 and 0 step by step (e.g. the input data pattern shall be 0x55). The user can read the value to know the current input baud rate of the bus whenever the ABRDETIF (UUART_PROTSTS\\[9\\]) is set.\nNote: This bit can be cleared to 0 by software writing '0' to the BRDETITV."]
pub struct BRDETITV_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDETITV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Stick Parity Enable Bit\nNote: Refer to RS-485 Support section for detailed information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STICKEN_A {
    #[doc = "0: Stick parity Disabled"]
    _0 = 0,
    #[doc = "1: Stick parity Enabled"]
    _1 = 1,
}
impl From<STICKEN_A> for bool {
    #[inline(always)]
    fn from(variant: STICKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STICKEN` reader - Stick Parity Enable Bit\nNote: Refer to RS-485 Support section for detailed information."]
pub struct STICKEN_R(crate::FieldReader<bool, STICKEN_A>);
impl STICKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STICKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STICKEN_A {
        match self.bits {
            false => STICKEN_A::_0,
            true => STICKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STICKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STICKEN_A::_1
    }
}
impl core::ops::Deref for STICKEN_R {
    type Target = crate::FieldReader<bool, STICKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STICKEN` writer - Stick Parity Enable Bit\nNote: Refer to RS-485 Support section for detailed information."]
pub struct STICKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STICKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STICKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stick parity Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STICKEN_A::_0)
    }
    #[doc = "Stick parity Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STICKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Transmit Break Control Enable Bit\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCEN_A {
    #[doc = "0: Transmit Break Control Disabled"]
    _0 = 0,
    #[doc = "1: Transmit Break Control Enabled"]
    _1 = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCEN` reader - Transmit Break Control Enable Bit\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
pub struct BCEN_R(crate::FieldReader<bool, BCEN_A>);
impl BCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::_0,
            true => BCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCEN_A::_1
    }
}
impl core::ops::Deref for BCEN_R {
    type Target = crate::FieldReader<bool, BCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCEN` writer - Transmit Break Control Enable Bit\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
pub struct BCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit Break Control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCEN_A::_0)
    }
    #[doc = "Transmit Break Control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "UART Protocol Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTEN_A {
    #[doc = "0: UART Protocol Disabled"]
    _0 = 0,
    #[doc = "1: UART Protocol Enabled"]
    _1 = 1,
}
impl From<PROTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PROTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTEN` reader - UART Protocol Enable Bit"]
pub struct PROTEN_R(crate::FieldReader<bool, PROTEN_A>);
impl PROTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTEN_A {
        match self.bits {
            false => PROTEN_A::_0,
            true => PROTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROTEN_A::_1
    }
}
impl core::ops::Deref for PROTEN_R {
    type Target = crate::FieldReader<bool, PROTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTEN` writer - UART Protocol Enable Bit"]
pub struct PROTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART Protocol Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTEN_A::_0)
    }
    #[doc = "UART Protocol Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Stop Bits This bit defines the number of stop bits in an UART frame."]
    #[inline(always)]
    pub fn stopb(&self) -> STOPB_R {
        STOPB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Enable Bit This bit defines the parity bit is enabled in an UART frame."]
    #[inline(always)]
    pub fn parityen(&self) -> PARITYEN_R {
        PARITYEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Even Parity Enable Bit Note: This bit has effect only when PARITYEN is set."]
    #[inline(always)]
    pub fn evenparity(&self) -> EVENPARITY_R {
        EVENPARITY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nRTS Auto-flow Control Enable Bit Note: This bit has effect only when the RTSAUDIREN is not set."]
    #[inline(always)]
    pub fn rtsautoen(&self) -> RTSAUTOEN_R {
        RTSAUTOEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - nCTS Auto-flow Control Enable Bit When nCTS auto-flow is enabled, the UART will send data to external device when nCTS input assert (UART will not send data to device if nCTS input is dis-asserted)."]
    #[inline(always)]
    pub fn ctsautoen(&self) -> CTSAUTOEN_R {
        CTSAUTOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - nRTS Auto Direction Enable Bit When nRTS auto direction is enabled, if the transmitted bytes in the TX buffer is empty, the UART asserted nRTS signal automatically. Note 1: This bit is used for nRTS auto direction control for RS485. Note 2: This bit has effect only when the RTSAUTOEN is not set."]
    #[inline(always)]
    pub fn rtsaudiren(&self) -> RTSAUDIREN_R {
        RTSAUDIREN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Auto-baud Rate Detect Enable Bit Note: When the auto - baud rate detect operation finishes, hardware will clear this bit. The associated interrupt ABRDETIF (UUART_PROTST\\[9\\]) will be generated (If ARBIEN (UUART_PROTIEN \\[1\\]) is enabled)."]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Wake-up Mode Enable Bit"]
    #[inline(always)]
    pub fn datwken(&self) -> DATWKEN_R {
        DATWKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - nCTS Wake-up Mode Enable Bit"]
    #[inline(always)]
    pub fn ctswken(&self) -> CTSWKEN_R {
        CTSWKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - Wake-up Counter These bits field indicate how many clock cycle selected by fPDS_CNT do the slave can get the 1st bit (start bit) when the device is wake-up from Power-down mode."]
    #[inline(always)]
    pub fn wakecnt(&self) -> WAKECNT_R {
        WAKECNT_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - Baud Rate Detection Interval This bit fields indicate how many clock cycle selected by TMCNTSRC (UUART_BRGEN \\[5\\]) does the slave calculates the baud rate in one bits. The order of the bus shall be 1 and 0 step by step (e.g. the input data pattern shall be 0x55). The user can read the value to know the current input baud rate of the bus whenever the ABRDETIF (UUART_PROTSTS\\[9\\]) is set. Note: This bit can be cleared to 0 by software writing '0' to the BRDETITV."]
    #[inline(always)]
    pub fn brdetitv(&self) -> BRDETITV_R {
        BRDETITV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Stick Parity Enable Bit Note: Refer to RS-485 Support section for detailed information."]
    #[inline(always)]
    pub fn sticken(&self) -> STICKEN_R {
        STICKEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmit Break Control Enable Bit Note: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UART Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&self) -> PROTEN_R {
        PROTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Bits This bit defines the number of stop bits in an UART frame."]
    #[inline(always)]
    pub fn stopb(&mut self) -> STOPB_W {
        STOPB_W { w: self }
    }
    #[doc = "Bit 1 - Parity Enable Bit This bit defines the parity bit is enabled in an UART frame."]
    #[inline(always)]
    pub fn parityen(&mut self) -> PARITYEN_W {
        PARITYEN_W { w: self }
    }
    #[doc = "Bit 2 - Even Parity Enable Bit Note: This bit has effect only when PARITYEN is set."]
    #[inline(always)]
    pub fn evenparity(&mut self) -> EVENPARITY_W {
        EVENPARITY_W { w: self }
    }
    #[doc = "Bit 3 - nRTS Auto-flow Control Enable Bit Note: This bit has effect only when the RTSAUDIREN is not set."]
    #[inline(always)]
    pub fn rtsautoen(&mut self) -> RTSAUTOEN_W {
        RTSAUTOEN_W { w: self }
    }
    #[doc = "Bit 4 - nCTS Auto-flow Control Enable Bit When nCTS auto-flow is enabled, the UART will send data to external device when nCTS input assert (UART will not send data to device if nCTS input is dis-asserted)."]
    #[inline(always)]
    pub fn ctsautoen(&mut self) -> CTSAUTOEN_W {
        CTSAUTOEN_W { w: self }
    }
    #[doc = "Bit 5 - nRTS Auto Direction Enable Bit When nRTS auto direction is enabled, if the transmitted bytes in the TX buffer is empty, the UART asserted nRTS signal automatically. Note 1: This bit is used for nRTS auto direction control for RS485. Note 2: This bit has effect only when the RTSAUTOEN is not set."]
    #[inline(always)]
    pub fn rtsaudiren(&mut self) -> RTSAUDIREN_W {
        RTSAUDIREN_W { w: self }
    }
    #[doc = "Bit 6 - Auto-baud Rate Detect Enable Bit Note: When the auto - baud rate detect operation finishes, hardware will clear this bit. The associated interrupt ABRDETIF (UUART_PROTST\\[9\\]) will be generated (If ARBIEN (UUART_PROTIEN \\[1\\]) is enabled)."]
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W {
        ABREN_W { w: self }
    }
    #[doc = "Bit 9 - Data Wake-up Mode Enable Bit"]
    #[inline(always)]
    pub fn datwken(&mut self) -> DATWKEN_W {
        DATWKEN_W { w: self }
    }
    #[doc = "Bit 10 - nCTS Wake-up Mode Enable Bit"]
    #[inline(always)]
    pub fn ctswken(&mut self) -> CTSWKEN_W {
        CTSWKEN_W { w: self }
    }
    #[doc = "Bits 11:14 - Wake-up Counter These bits field indicate how many clock cycle selected by fPDS_CNT do the slave can get the 1st bit (start bit) when the device is wake-up from Power-down mode."]
    #[inline(always)]
    pub fn wakecnt(&mut self) -> WAKECNT_W {
        WAKECNT_W { w: self }
    }
    #[doc = "Bits 16:24 - Baud Rate Detection Interval This bit fields indicate how many clock cycle selected by TMCNTSRC (UUART_BRGEN \\[5\\]) does the slave calculates the baud rate in one bits. The order of the bus shall be 1 and 0 step by step (e.g. the input data pattern shall be 0x55). The user can read the value to know the current input baud rate of the bus whenever the ABRDETIF (UUART_PROTSTS\\[9\\]) is set. Note: This bit can be cleared to 0 by software writing '0' to the BRDETITV."]
    #[inline(always)]
    pub fn brdetitv(&mut self) -> BRDETITV_W {
        BRDETITV_W { w: self }
    }
    #[doc = "Bit 26 - Stick Parity Enable Bit Note: Refer to RS-485 Support section for detailed information."]
    #[inline(always)]
    pub fn sticken(&mut self) -> STICKEN_W {
        STICKEN_W { w: self }
    }
    #[doc = "Bit 29 - Transmit Break Control Enable Bit Note: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcen(&mut self) -> BCEN_W {
        BCEN_W { w: self }
    }
    #[doc = "Bit 31 - UART Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&mut self) -> PROTEN_W {
        PROTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_protctl](index.html) module"]
pub struct UUART_PROTCTL_SPEC;
impl crate::RegisterSpec for UUART_PROTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_protctl::R](R) reader structure"]
impl crate::Readable for UUART_PROTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_protctl::W](W) writer structure"]
impl crate::Writable for UUART_PROTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_PROTCTL to value 0"]
impl crate::Resettable for UUART_PROTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
