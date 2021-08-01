#[doc = "Register `UI2C_PROTIEN` reader"]
pub struct R(crate::R<UI2C_PROTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_PROTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_PROTIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_PROTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_PROTIEN` writer"]
pub struct W(crate::W<UI2C_PROTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_PROTIEN_SPEC>;
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
impl From<crate::W<UI2C_PROTIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_PROTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-out Interrupt Enable Bit\nIn I2C protocol, this bit enables the interrupt generation in case of a time-out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIEN_A {
    #[doc = "0: The time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The time-out interrupt Enabled"]
    _1 = 1,
}
impl From<TOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIEN` reader - Time-out Interrupt Enable Bit\nIn I2C protocol, this bit enables the interrupt generation in case of a time-out event."]
pub struct TOIEN_R(crate::FieldReader<bool, TOIEN_A>);
impl TOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIEN_A {
        match self.bits {
            false => TOIEN_A::_0,
            true => TOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOIEN_A::_1
    }
}
impl core::ops::Deref for TOIEN_R {
    type Target = crate::FieldReader<bool, TOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIEN` writer - Time-out Interrupt Enable Bit\nIn I2C protocol, this bit enables the interrupt generation in case of a time-out event."]
pub struct TOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEN_A::_0)
    }
    #[doc = "The time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIEN_A::_1)
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
#[doc = "START Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a START condition is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARIEN_A {
    #[doc = "0: The start condition interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The start condition interrupt Enabled"]
    _1 = 1,
}
impl From<STARIEN_A> for bool {
    #[inline(always)]
    fn from(variant: STARIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARIEN` reader - START Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a START condition is detected."]
pub struct STARIEN_R(crate::FieldReader<bool, STARIEN_A>);
impl STARIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARIEN_A {
        match self.bits {
            false => STARIEN_A::_0,
            true => STARIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STARIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STARIEN_A::_1
    }
}
impl core::ops::Deref for STARIEN_R {
    type Target = crate::FieldReader<bool, STARIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARIEN` writer - START Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a START condition is detected."]
pub struct STARIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STARIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The start condition interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARIEN_A::_0)
    }
    #[doc = "The start condition interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARIEN_A::_1)
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
#[doc = "STOP Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a STOP condition is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STORIEN_A {
    #[doc = "0: The stop condition interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The stop condition interrupt Enabled"]
    _1 = 1,
}
impl From<STORIEN_A> for bool {
    #[inline(always)]
    fn from(variant: STORIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STORIEN` reader - STOP Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a STOP condition is detected."]
pub struct STORIEN_R(crate::FieldReader<bool, STORIEN_A>);
impl STORIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STORIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORIEN_A {
        match self.bits {
            false => STORIEN_A::_0,
            true => STORIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STORIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STORIEN_A::_1
    }
}
impl core::ops::Deref for STORIEN_R {
    type Target = crate::FieldReader<bool, STORIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STORIEN` writer - STOP Condition Received Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a STOP condition is detected."]
pub struct STORIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STORIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STORIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The stop condition interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STORIEN_A::_0)
    }
    #[doc = "The stop condition interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STORIEN_A::_1)
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
#[doc = "Non - Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a Non - acknowledge is detected by a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIEN_A {
    #[doc = "0: The non - acknowledge interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The non - acknowledge interrupt Enabled"]
    _1 = 1,
}
impl From<NACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIEN` reader - Non - Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a Non - acknowledge is detected by a master."]
pub struct NACKIEN_R(crate::FieldReader<bool, NACKIEN_A>);
impl NACKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKIEN_A {
        match self.bits {
            false => NACKIEN_A::_0,
            true => NACKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NACKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NACKIEN_A::_1
    }
}
impl core::ops::Deref for NACKIEN_R {
    type Target = crate::FieldReader<bool, NACKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKIEN` writer - Non - Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if a Non - acknowledge is detected by a master."]
pub struct NACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The non - acknowledge interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKIEN_A::_0)
    }
    #[doc = "The non - acknowledge interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKIEN_A::_1)
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
#[doc = "Arbitration Lost Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an arbitration lost event is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBLOIEN_A {
    #[doc = "0: The arbitration lost interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The arbitration lost interrupt Enabled"]
    _1 = 1,
}
impl From<ARBLOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARBLOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBLOIEN` reader - Arbitration Lost Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an arbitration lost event is detected."]
pub struct ARBLOIEN_R(crate::FieldReader<bool, ARBLOIEN_A>);
impl ARBLOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBLOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBLOIEN_A {
        match self.bits {
            false => ARBLOIEN_A::_0,
            true => ARBLOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ARBLOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ARBLOIEN_A::_1
    }
}
impl core::ops::Deref for ARBLOIEN_R {
    type Target = crate::FieldReader<bool, ARBLOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLOIEN` writer - Arbitration Lost Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an arbitration lost event is detected."]
pub struct ARBLOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBLOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The arbitration lost interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBLOIEN_A::_0)
    }
    #[doc = "The arbitration lost interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBLOIEN_A::_1)
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
#[doc = "Error Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an I2C error condition is detected (indicated by ERRIF (UI2C_PROTSTS \\[12\\])).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIEN_A {
    #[doc = "0: The error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The error interrupt Enabled"]
    _1 = 1,
}
impl From<ERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an I2C error condition is detected (indicated by ERRIF (UI2C_PROTSTS \\[12\\]))."]
pub struct ERRIEN_R(crate::FieldReader<bool, ERRIEN_A>);
impl ERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIEN_A {
        match self.bits {
            false => ERRIEN_A::_0,
            true => ERRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERRIEN_A::_1
    }
}
impl core::ops::Deref for ERRIEN_R {
    type Target = crate::FieldReader<bool, ERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIEN` writer - Error Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an I2C error condition is detected (indicated by ERRIF (UI2C_PROTSTS \\[12\\]))."]
pub struct ERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRIEN_A::_0)
    }
    #[doc = "The error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRIEN_A::_1)
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
#[doc = "Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an acknowledge is detected by a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKIEN_A {
    #[doc = "0: The acknowledge interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The acknowledge interrupt Enabled"]
    _1 = 1,
}
impl From<ACKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKIEN` reader - Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an acknowledge is detected by a master."]
pub struct ACKIEN_R(crate::FieldReader<bool, ACKIEN_A>);
impl ACKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKIEN_A {
        match self.bits {
            false => ACKIEN_A::_0,
            true => ACKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACKIEN_A::_1
    }
}
impl core::ops::Deref for ACKIEN_R {
    type Target = crate::FieldReader<bool, ACKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKIEN` writer - Acknowledge Interrupt Enable Bit\nThis bit enables the generation of a protocol interrupt if an acknowledge is detected by a master."]
pub struct ACKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The acknowledge interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKIEN_A::_0)
    }
    #[doc = "The acknowledge interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKIEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Time-out Interrupt Enable Bit In I2C protocol, this bit enables the interrupt generation in case of a time-out event."]
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - START Condition Received Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a START condition is detected."]
    #[inline(always)]
    pub fn starien(&self) -> STARIEN_R {
        STARIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - STOP Condition Received Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a STOP condition is detected."]
    #[inline(always)]
    pub fn storien(&self) -> STORIEN_R {
        STORIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non - Acknowledge Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a Non - acknowledge is detected by a master."]
    #[inline(always)]
    pub fn nackien(&self) -> NACKIEN_R {
        NACKIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an arbitration lost event is detected."]
    #[inline(always)]
    pub fn arbloien(&self) -> ARBLOIEN_R {
        ARBLOIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an I2C error condition is detected (indicated by ERRIF (UI2C_PROTSTS \\[12\\]))."]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an acknowledge is detected by a master."]
    #[inline(always)]
    pub fn ackien(&self) -> ACKIEN_R {
        ACKIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time-out Interrupt Enable Bit In I2C protocol, this bit enables the interrupt generation in case of a time-out event."]
    #[inline(always)]
    pub fn toien(&mut self) -> TOIEN_W {
        TOIEN_W { w: self }
    }
    #[doc = "Bit 1 - START Condition Received Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a START condition is detected."]
    #[inline(always)]
    pub fn starien(&mut self) -> STARIEN_W {
        STARIEN_W { w: self }
    }
    #[doc = "Bit 2 - STOP Condition Received Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a STOP condition is detected."]
    #[inline(always)]
    pub fn storien(&mut self) -> STORIEN_W {
        STORIEN_W { w: self }
    }
    #[doc = "Bit 3 - Non - Acknowledge Interrupt Enable Bit This bit enables the generation of a protocol interrupt if a Non - acknowledge is detected by a master."]
    #[inline(always)]
    pub fn nackien(&mut self) -> NACKIEN_W {
        NACKIEN_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration Lost Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an arbitration lost event is detected."]
    #[inline(always)]
    pub fn arbloien(&mut self) -> ARBLOIEN_W {
        ARBLOIEN_W { w: self }
    }
    #[doc = "Bit 5 - Error Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an I2C error condition is detected (indicated by ERRIF (UI2C_PROTSTS \\[12\\]))."]
    #[inline(always)]
    pub fn errien(&mut self) -> ERRIEN_W {
        ERRIEN_W { w: self }
    }
    #[doc = "Bit 6 - Acknowledge Interrupt Enable Bit This bit enables the generation of a protocol interrupt if an acknowledge is detected by a master."]
    #[inline(always)]
    pub fn ackien(&mut self) -> ACKIEN_W {
        ACKIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_protien](index.html) module"]
pub struct UI2C_PROTIEN_SPEC;
impl crate::RegisterSpec for UI2C_PROTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_protien::R](R) reader structure"]
impl crate::Readable for UI2C_PROTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_protien::W](W) writer structure"]
impl crate::Writable for UI2C_PROTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_PROTIEN to value 0"]
impl crate::Resettable for UI2C_PROTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
