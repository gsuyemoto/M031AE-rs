#[doc = "Register `UI2C_PROTSTS` reader"]
pub struct R(crate::R<UI2C_PROTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_PROTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_PROTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_PROTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_PROTSTS` writer"]
pub struct W(crate::W<UI2C_PROTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_PROTSTS_SPEC>;
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
impl From<crate::W<UI2C_PROTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_PROTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-out Interrupt Flag\nNote: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIF_A {
    #[doc = "0: A time-out interrupt status has not occurred"]
    _0 = 0,
    #[doc = "1: A time-out interrupt status has occurred"]
    _1 = 1,
}
impl From<TOIF_A> for bool {
    #[inline(always)]
    fn from(variant: TOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIF` reader - Time-out Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct TOIF_R(crate::FieldReader<bool, TOIF_A>);
impl TOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIF_A {
        match self.bits {
            false => TOIF_A::_0,
            true => TOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOIF_A::_1
    }
}
impl core::ops::Deref for TOIF_R {
    type Target = crate::FieldReader<bool, TOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIF` writer - Time-out Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct TOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A time-out interrupt status has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIF_A::_0)
    }
    #[doc = "A time-out interrupt status has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIF_A::_1)
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
#[doc = "On Bus Busy\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONBUSY_A {
    #[doc = "0: The bus is IDLE (both SCLK and SDA High)"]
    _0 = 0,
    #[doc = "1: The bus is busy"]
    _1 = 1,
}
impl From<ONBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ONBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONBUSY` reader - On Bus Busy\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
pub struct ONBUSY_R(crate::FieldReader<bool, ONBUSY_A>);
impl ONBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONBUSY_A {
        match self.bits {
            false => ONBUSY_A::_0,
            true => ONBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ONBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ONBUSY_A::_1
    }
}
impl core::ops::Deref for ONBUSY_R {
    type Target = crate::FieldReader<bool, ONBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONBUSY` writer - On Bus Busy\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
pub struct ONBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> ONBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONBUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bus is IDLE (both SCLK and SDA High)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONBUSY_A::_0)
    }
    #[doc = "The bus is busy"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONBUSY_A::_1)
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
#[doc = "Start Condition Received Interrupt Flag\nThis bit indicates that a start condition or repeated start condition has been detected on master mode. However, this bit also indicates that a repeated start condition has been detected on slave mode.\nNote: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARIF_A {
    #[doc = "0: A start condition has not yet been detected"]
    _0 = 0,
    #[doc = "1: A start condition has been detected"]
    _1 = 1,
}
impl From<STARIF_A> for bool {
    #[inline(always)]
    fn from(variant: STARIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARIF` reader - Start Condition Received Interrupt Flag\nThis bit indicates that a start condition or repeated start condition has been detected on master mode. However, this bit also indicates that a repeated start condition has been detected on slave mode.\nNote: It is cleared by software writing 1 into this bit"]
pub struct STARIF_R(crate::FieldReader<bool, STARIF_A>);
impl STARIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARIF_A {
        match self.bits {
            false => STARIF_A::_0,
            true => STARIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STARIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STARIF_A::_1
    }
}
impl core::ops::Deref for STARIF_R {
    type Target = crate::FieldReader<bool, STARIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARIF` writer - Start Condition Received Interrupt Flag\nThis bit indicates that a start condition or repeated start condition has been detected on master mode. However, this bit also indicates that a repeated start condition has been detected on slave mode.\nNote: It is cleared by software writing 1 into this bit"]
pub struct STARIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STARIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A start condition has not yet been detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARIF_A::_0)
    }
    #[doc = "A start condition has been detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARIF_A::_1)
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
#[doc = "Stop Condition Received Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STORIF_A {
    #[doc = "0: A stop condition has not yet been detected"]
    _0 = 0,
    #[doc = "1: A stop condition has been detected"]
    _1 = 1,
}
impl From<STORIF_A> for bool {
    #[inline(always)]
    fn from(variant: STORIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STORIF` reader - Stop Condition Received Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit"]
pub struct STORIF_R(crate::FieldReader<bool, STORIF_A>);
impl STORIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STORIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORIF_A {
        match self.bits {
            false => STORIF_A::_0,
            true => STORIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STORIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STORIF_A::_1
    }
}
impl core::ops::Deref for STORIF_R {
    type Target = crate::FieldReader<bool, STORIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STORIF` writer - Stop Condition Received Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit"]
pub struct STORIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STORIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STORIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A stop condition has not yet been detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STORIF_A::_0)
    }
    #[doc = "A stop condition has been detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STORIF_A::_1)
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
#[doc = "Non - Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIF_A {
    #[doc = "0: A non - acknowledge has not been received"]
    _0 = 0,
    #[doc = "1: A non - acknowledge has been received"]
    _1 = 1,
}
impl From<NACKIF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIF` reader - Non - Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct NACKIF_R(crate::FieldReader<bool, NACKIF_A>);
impl NACKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACKIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKIF_A {
        match self.bits {
            false => NACKIF_A::_0,
            true => NACKIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NACKIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NACKIF_A::_1
    }
}
impl core::ops::Deref for NACKIF_R {
    type Target = crate::FieldReader<bool, NACKIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKIF` writer - Non - Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct NACKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A non - acknowledge has not been received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKIF_A::_0)
    }
    #[doc = "A non - acknowledge has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKIF_A::_1)
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
#[doc = "Arbitration Lost Interrupt Flag\nNote: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBLOIF_A {
    #[doc = "0: An arbitration has not been lost"]
    _0 = 0,
    #[doc = "1: An arbitration has been lost"]
    _1 = 1,
}
impl From<ARBLOIF_A> for bool {
    #[inline(always)]
    fn from(variant: ARBLOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBLOIF` reader - Arbitration Lost Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct ARBLOIF_R(crate::FieldReader<bool, ARBLOIF_A>);
impl ARBLOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBLOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBLOIF_A {
        match self.bits {
            false => ARBLOIF_A::_0,
            true => ARBLOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ARBLOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ARBLOIF_A::_1
    }
}
impl core::ops::Deref for ARBLOIF_R {
    type Target = crate::FieldReader<bool, ARBLOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLOIF` writer - Arbitration Lost Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct ARBLOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBLOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An arbitration has not been lost"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBLOIF_A::_0)
    }
    #[doc = "An arbitration has been lost"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBLOIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Error Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit\nNote 2: This bit is set for slave mode, and user must write 1 into STO register to the defined 'not addressed' slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIF_A {
    #[doc = "0: An I2C error has not been detected"]
    _0 = 0,
    #[doc = "1: An I2C error has been detected"]
    _1 = 1,
}
impl From<ERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` reader - Error Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit\nNote 2: This bit is set for slave mode, and user must write 1 into STO register to the defined 'not addressed' slave mode."]
pub struct ERRIF_R(crate::FieldReader<bool, ERRIF_A>);
impl ERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIF_A {
        match self.bits {
            false => ERRIF_A::_0,
            true => ERRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERRIF_A::_1
    }
}
impl core::ops::Deref for ERRIF_R {
    type Target = crate::FieldReader<bool, ERRIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIF` writer - Error Interrupt Flag\nNote 1: It is cleared by software writing 1 into this bit\nNote 2: This bit is set for slave mode, and user must write 1 into STO register to the defined 'not addressed' slave mode."]
pub struct ERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An I2C error has not been detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRIF_A::_0)
    }
    #[doc = "An I2C error has been detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRIF_A::_1)
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
#[doc = "Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKIF_A {
    #[doc = "0: An acknowledge has not been received"]
    _0 = 0,
    #[doc = "1: An acknowledge has been received"]
    _1 = 1,
}
impl From<ACKIF_A> for bool {
    #[inline(always)]
    fn from(variant: ACKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKIF` reader - Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct ACKIF_R(crate::FieldReader<bool, ACKIF_A>);
impl ACKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKIF_A {
        match self.bits {
            false => ACKIF_A::_0,
            true => ACKIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACKIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACKIF_A::_1
    }
}
impl core::ops::Deref for ACKIF_R {
    type Target = crate::FieldReader<bool, ACKIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKIF` writer - Acknowledge Received Interrupt Flag\nNote: It is cleared by software writing 1 into this bit"]
pub struct ACKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An acknowledge has not been received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKIF_A::_0)
    }
    #[doc = "An acknowledge has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKIF_A::_1)
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
#[doc = "Slave Select Status\nThis bit indicates that this device has been selected as slave.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLASEL_A {
    #[doc = "0: The device is not selected as slave"]
    _0 = 0,
    #[doc = "1: The device is selected as slave"]
    _1 = 1,
}
impl From<SLASEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLASEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLASEL` reader - Slave Select Status\nThis bit indicates that this device has been selected as slave.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
pub struct SLASEL_R(crate::FieldReader<bool, SLASEL_A>);
impl SLASEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLASEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLASEL_A {
        match self.bits {
            false => SLASEL_A::_0,
            true => SLASEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLASEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLASEL_A::_1
    }
}
impl core::ops::Deref for SLASEL_R {
    type Target = crate::FieldReader<bool, SLASEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLASEL` writer - Slave Select Status\nThis bit indicates that this device has been selected as slave.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
pub struct SLASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLASEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The device is not selected as slave"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLASEL_A::_0)
    }
    #[doc = "The device is selected as slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLASEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Slave Read Request Status\nThis bit indicates that a slave read request has been detected.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAREAD_A {
    #[doc = "0: A slave R/W bit is 1 has not been detected"]
    _0 = 0,
    #[doc = "1: A slave R/W bit is 1 has been detected"]
    _1 = 1,
}
impl From<SLAREAD_A> for bool {
    #[inline(always)]
    fn from(variant: SLAREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAREAD` reader - Slave Read Request Status\nThis bit indicates that a slave read request has been detected.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
pub struct SLAREAD_R(crate::FieldReader<bool, SLAREAD_A>);
impl SLAREAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAREAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAREAD_A {
        match self.bits {
            false => SLAREAD_A::_0,
            true => SLAREAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLAREAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLAREAD_A::_1
    }
}
impl core::ops::Deref for SLAREAD_R {
    type Target = crate::FieldReader<bool, SLAREAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAREAD` writer - Slave Read Request Status\nThis bit indicates that a slave read request has been detected.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
pub struct SLAREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAREAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A slave R/W bit is 1 has not been detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLAREAD_A::_0)
    }
    #[doc = "A slave R/W bit is 1 has been detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLAREAD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Wake-up Address Frame Acknowledge Bit Done\nNote: This bit can't release when WKUPIF is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKAKDONE_A {
    #[doc = "0: The ACK bit cycle of address match frame isn't done"]
    _0 = 0,
    #[doc = "1: The ACK bit cycle of address match frame is done in power-down"]
    _1 = 1,
}
impl From<WKAKDONE_A> for bool {
    #[inline(always)]
    fn from(variant: WKAKDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKAKDONE` reader - Wake-up Address Frame Acknowledge Bit Done\nNote: This bit can't release when WKUPIF is set."]
pub struct WKAKDONE_R(crate::FieldReader<bool, WKAKDONE_A>);
impl WKAKDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKAKDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKAKDONE_A {
        match self.bits {
            false => WKAKDONE_A::_0,
            true => WKAKDONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKAKDONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKAKDONE_A::_1
    }
}
impl core::ops::Deref for WKAKDONE_R {
    type Target = crate::FieldReader<bool, WKAKDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKAKDONE` writer - Wake-up Address Frame Acknowledge Bit Done\nNote: This bit can't release when WKUPIF is set."]
pub struct WKAKDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKAKDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKAKDONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The ACK bit cycle of address match frame isn't done"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKAKDONE_A::_0)
    }
    #[doc = "The ACK bit cycle of address match frame is done in power-down"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKAKDONE_A::_1)
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
#[doc = "Read/Write Status Bit in Address Wake-up Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRSTSWK_A {
    #[doc = "0: Write command be record on the address match wake-up frame"]
    _0 = 0,
    #[doc = "1: Read command be record on the address match wake-up frame"]
    _1 = 1,
}
impl From<WRSTSWK_A> for bool {
    #[inline(always)]
    fn from(variant: WRSTSWK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRSTSWK` reader - Read/Write Status Bit in Address Wake-up Frame"]
pub struct WRSTSWK_R(crate::FieldReader<bool, WRSTSWK_A>);
impl WRSTSWK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRSTSWK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRSTSWK_A {
        match self.bits {
            false => WRSTSWK_A::_0,
            true => WRSTSWK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WRSTSWK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WRSTSWK_A::_1
    }
}
impl core::ops::Deref for WRSTSWK_R {
    type Target = crate::FieldReader<bool, WRSTSWK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRSTSWK` writer - Read/Write Status Bit in Address Wake-up Frame"]
pub struct WRSTSWK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSTSWK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRSTSWK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write command be record on the address match wake-up frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRSTSWK_A::_0)
    }
    #[doc = "Read command be record on the address match wake-up frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRSTSWK_A::_1)
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
#[doc = "Bus Hang-up\nThis bit indicates bus hang-up status. There is 4-bit counter count when SCL hold high and refer fSAMP_CLK. The hang-up counter will count to overflow and set this bit when SDA is low. The counter will be reset by falling edge of SCL signal.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSHANG_A {
    #[doc = "0: The bus is normal status for transmission"]
    _0 = 0,
    #[doc = "1: The bus is hang-up status for transmission"]
    _1 = 1,
}
impl From<BUSHANG_A> for bool {
    #[inline(always)]
    fn from(variant: BUSHANG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSHANG` reader - Bus Hang-up\nThis bit indicates bus hang-up status. There is 4-bit counter count when SCL hold high and refer fSAMP_CLK. The hang-up counter will count to overflow and set this bit when SDA is low. The counter will be reset by falling edge of SCL signal.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
pub struct BUSHANG_R(crate::FieldReader<bool, BUSHANG_A>);
impl BUSHANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSHANG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSHANG_A {
        match self.bits {
            false => BUSHANG_A::_0,
            true => BUSHANG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSHANG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSHANG_A::_1
    }
}
impl core::ops::Deref for BUSHANG_R {
    type Target = crate::FieldReader<bool, BUSHANG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSHANG` writer - Bus Hang-up\nThis bit indicates bus hang-up status. There is 4-bit counter count when SCL hold high and refer fSAMP_CLK. The hang-up counter will count to overflow and set this bit when SDA is low. The counter will be reset by falling edge of SCL signal.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
pub struct BUSHANG_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSHANG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSHANG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bus is normal status for transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSHANG_A::_0)
    }
    #[doc = "The bus is hang-up status for transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSHANG_A::_1)
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
#[doc = "Error Arbitration Lost\nThis bit indicates bus arbitration lost due to bigger noise which is can't be filtered by input processor. The I2C can send start condition when ERRARBLO is set. Thus this bit doesn't be cared on slave mode.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRARBLO_A {
    #[doc = "0: The bus is normal status for transmission"]
    _0 = 0,
    #[doc = "1: The bus is error arbitration lost status for transmission"]
    _1 = 1,
}
impl From<ERRARBLO_A> for bool {
    #[inline(always)]
    fn from(variant: ERRARBLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRARBLO` reader - Error Arbitration Lost\nThis bit indicates bus arbitration lost due to bigger noise which is can't be filtered by input processor. The I2C can send start condition when ERRARBLO is set. Thus this bit doesn't be cared on slave mode.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
pub struct ERRARBLO_R(crate::FieldReader<bool, ERRARBLO_A>);
impl ERRARBLO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRARBLO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRARBLO_A {
        match self.bits {
            false => ERRARBLO_A::_0,
            true => ERRARBLO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERRARBLO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERRARBLO_A::_1
    }
}
impl core::ops::Deref for ERRARBLO_R {
    type Target = crate::FieldReader<bool, ERRARBLO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRARBLO` writer - Error Arbitration Lost\nThis bit indicates bus arbitration lost due to bigger noise which is can't be filtered by input processor. The I2C can send start condition when ERRARBLO is set. Thus this bit doesn't be cared on slave mode.\nNote: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
pub struct ERRARBLO_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRARBLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRARBLO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bus is normal status for transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRARBLO_A::_0)
    }
    #[doc = "The bus is error arbitration lost status for transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRARBLO_A::_1)
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
impl R {
    #[doc = "Bit 5 - Time-out Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn toif(&self) -> TOIF_R {
        TOIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - On Bus Busy Indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
    #[inline(always)]
    pub fn onbusy(&self) -> ONBUSY_R {
        ONBUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start Condition Received Interrupt Flag This bit indicates that a start condition or repeated start condition has been detected on master mode. However, this bit also indicates that a repeated start condition has been detected on slave mode. Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn starif(&self) -> STARIF_R {
        STARIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stop Condition Received Interrupt Flag Note 1: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn storif(&self) -> STORIF_R {
        STORIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non - Acknowledge Received Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn nackif(&self) -> NACKIF_R {
        NACKIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn arbloif(&self) -> ARBLOIF_R {
        ARBLOIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Error Interrupt Flag Note 1: It is cleared by software writing 1 into this bit Note 2: This bit is set for slave mode, and user must write 1 into STO register to the defined 'not addressed' slave mode."]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Received Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn ackif(&self) -> ACKIF_R {
        ACKIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Slave Select Status This bit indicates that this device has been selected as slave. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
    #[inline(always)]
    pub fn slasel(&self) -> SLASEL_R {
        SLASEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Read Request Status This bit indicates that a slave read request has been detected. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
    #[inline(always)]
    pub fn slaread(&self) -> SLAREAD_R {
        SLAREAD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Wake-up Address Frame Acknowledge Bit Done Note: This bit can't release when WKUPIF is set."]
    #[inline(always)]
    pub fn wkakdone(&self) -> WKAKDONE_R {
        WKAKDONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Read/Write Status Bit in Address Wake-up Frame"]
    #[inline(always)]
    pub fn wrstswk(&self) -> WRSTSWK_R {
        WRSTSWK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bus Hang-up This bit indicates bus hang-up status. There is 4-bit counter count when SCL hold high and refer fSAMP_CLK. The hang-up counter will count to overflow and set this bit when SDA is low. The counter will be reset by falling edge of SCL signal. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
    #[inline(always)]
    pub fn bushang(&self) -> BUSHANG_R {
        BUSHANG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Error Arbitration Lost This bit indicates bus arbitration lost due to bigger noise which is can't be filtered by input processor. The I2C can send start condition when ERRARBLO is set. Thus this bit doesn't be cared on slave mode. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
    #[inline(always)]
    pub fn errarblo(&self) -> ERRARBLO_R {
        ERRARBLO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Time-out Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn toif(&mut self) -> TOIF_W {
        TOIF_W { w: self }
    }
    #[doc = "Bit 6 - On Bus Busy Indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
    #[inline(always)]
    pub fn onbusy(&mut self) -> ONBUSY_W {
        ONBUSY_W { w: self }
    }
    #[doc = "Bit 8 - Start Condition Received Interrupt Flag This bit indicates that a start condition or repeated start condition has been detected on master mode. However, this bit also indicates that a repeated start condition has been detected on slave mode. Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn starif(&mut self) -> STARIF_W {
        STARIF_W { w: self }
    }
    #[doc = "Bit 9 - Stop Condition Received Interrupt Flag Note 1: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn storif(&mut self) -> STORIF_W {
        STORIF_W { w: self }
    }
    #[doc = "Bit 10 - Non - Acknowledge Received Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn nackif(&mut self) -> NACKIF_W {
        NACKIF_W { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn arbloif(&mut self) -> ARBLOIF_W {
        ARBLOIF_W { w: self }
    }
    #[doc = "Bit 12 - Error Interrupt Flag Note 1: It is cleared by software writing 1 into this bit Note 2: This bit is set for slave mode, and user must write 1 into STO register to the defined 'not addressed' slave mode."]
    #[inline(always)]
    pub fn errif(&mut self) -> ERRIF_W {
        ERRIF_W { w: self }
    }
    #[doc = "Bit 13 - Acknowledge Received Interrupt Flag Note: It is cleared by software writing 1 into this bit"]
    #[inline(always)]
    pub fn ackif(&mut self) -> ACKIF_W {
        ACKIF_W { w: self }
    }
    #[doc = "Bit 14 - Slave Select Status This bit indicates that this device has been selected as slave. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
    #[inline(always)]
    pub fn slasel(&mut self) -> SLASEL_W {
        SLASEL_W { w: self }
    }
    #[doc = "Bit 15 - Slave Read Request Status This bit indicates that a slave read request has been detected. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware."]
    #[inline(always)]
    pub fn slaread(&mut self) -> SLAREAD_W {
        SLAREAD_W { w: self }
    }
    #[doc = "Bit 16 - Wake-up Address Frame Acknowledge Bit Done Note: This bit can't release when WKUPIF is set."]
    #[inline(always)]
    pub fn wkakdone(&mut self) -> WKAKDONE_W {
        WKAKDONE_W { w: self }
    }
    #[doc = "Bit 17 - Read/Write Status Bit in Address Wake-up Frame"]
    #[inline(always)]
    pub fn wrstswk(&mut self) -> WRSTSWK_W {
        WRSTSWK_W { w: self }
    }
    #[doc = "Bit 18 - Bus Hang-up This bit indicates bus hang-up status. There is 4-bit counter count when SCL hold high and refer fSAMP_CLK. The hang-up counter will count to overflow and set this bit when SDA is low. The counter will be reset by falling edge of SCL signal. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
    #[inline(always)]
    pub fn bushang(&mut self) -> BUSHANG_W {
        BUSHANG_W { w: self }
    }
    #[doc = "Bit 19 - Error Arbitration Lost This bit indicates bus arbitration lost due to bigger noise which is can't be filtered by input processor. The I2C can send start condition when ERRARBLO is set. Thus this bit doesn't be cared on slave mode. Note: This bit has no interrupt signal, and it will be cleared automatically by hardware when a START condition is present."]
    #[inline(always)]
    pub fn errarblo(&mut self) -> ERRARBLO_W {
        ERRARBLO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_protsts](index.html) module"]
pub struct UI2C_PROTSTS_SPEC;
impl crate::RegisterSpec for UI2C_PROTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_protsts::R](R) reader structure"]
impl crate::Readable for UI2C_PROTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_protsts::W](W) writer structure"]
impl crate::Writable for UI2C_PROTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_PROTSTS to value 0"]
impl crate::Resettable for UI2C_PROTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
