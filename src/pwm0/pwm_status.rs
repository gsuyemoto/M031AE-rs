#[doc = "Register `PWM_STATUS` reader"]
pub struct R(crate::R<PWM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_STATUS` writer"]
pub struct W(crate::W<PWM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_STATUS_SPEC>;
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
impl From<crate::W<PWM_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-base Counter 0 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAX0_A {
    #[doc = "0: indicates the time-base counter never reached its maximum value 0xFFFF"]
    _0 = 0,
    #[doc = "1: indicates the time-base counter reached its maximum value"]
    _1 = 1,
}
impl From<CNTMAX0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMAX0` reader - Time-base Counter 0 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX0_R(crate::FieldReader<bool, CNTMAX0_A>);
impl CNTMAX0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTMAX0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX0_A {
        match self.bits {
            false => CNTMAX0_A::_0,
            true => CNTMAX0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTMAX0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTMAX0_A::_1
    }
}
impl core::ops::Deref for CNTMAX0_R {
    type Target = crate::FieldReader<bool, CNTMAX0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMAX0` writer - Time-base Counter 0 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAX0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "indicates the time-base counter never reached its maximum value 0xFFFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAX0_A::_0)
    }
    #[doc = "indicates the time-base counter reached its maximum value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAX0_A::_1)
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
#[doc = "Time-base Counter 2 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAX2_A {
    #[doc = "0: indicates the time-base counter never reached its maximum value 0xFFFF"]
    _0 = 0,
    #[doc = "1: indicates the time-base counter reached its maximum value"]
    _1 = 1,
}
impl From<CNTMAX2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMAX2` reader - Time-base Counter 2 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX2_R(crate::FieldReader<bool, CNTMAX2_A>);
impl CNTMAX2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTMAX2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX2_A {
        match self.bits {
            false => CNTMAX2_A::_0,
            true => CNTMAX2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTMAX2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTMAX2_A::_1
    }
}
impl core::ops::Deref for CNTMAX2_R {
    type Target = crate::FieldReader<bool, CNTMAX2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMAX2` writer - Time-base Counter 2 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAX2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAX2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "indicates the time-base counter never reached its maximum value 0xFFFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAX2_A::_0)
    }
    #[doc = "indicates the time-base counter reached its maximum value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAX2_A::_1)
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
#[doc = "Time-base Counter 4 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAX4_A {
    #[doc = "0: The time-base counter never reached its maximum value 0xFFFF"]
    _0 = 0,
    #[doc = "1: The time-base counter reached its maximum value"]
    _1 = 1,
}
impl From<CNTMAX4_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMAX4` reader - Time-base Counter 4 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX4_R(crate::FieldReader<bool, CNTMAX4_A>);
impl CNTMAX4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTMAX4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX4_A {
        match self.bits {
            false => CNTMAX4_A::_0,
            true => CNTMAX4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTMAX4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTMAX4_A::_1
    }
}
impl core::ops::Deref for CNTMAX4_R {
    type Target = crate::FieldReader<bool, CNTMAX4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMAX4` writer - Time-base Counter 4 Equal to 0xFFFF Latched Flag\nNote: This bit can be cleared by software writing 1."]
pub struct CNTMAX4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAX4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAX4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The time-base counter never reached its maximum value 0xFFFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAX4_A::_0)
    }
    #[doc = "The time-base counter reached its maximum value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAX4_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG0_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG0` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG0_R(crate::FieldReader<bool, ADCTRG0_A>);
impl ADCTRG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG0_A {
        match self.bits {
            false => ADCTRG0_A::_0,
            true => ADCTRG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG0_A::_1
    }
}
impl core::ops::Deref for ADCTRG0_R {
    type Target = crate::FieldReader<bool, ADCTRG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG0` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG0_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG0_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG1_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG1` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG1_R(crate::FieldReader<bool, ADCTRG1_A>);
impl ADCTRG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG1_A {
        match self.bits {
            false => ADCTRG1_A::_0,
            true => ADCTRG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG1_A::_1
    }
}
impl core::ops::Deref for ADCTRG1_R {
    type Target = crate::FieldReader<bool, ADCTRG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG1` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG1_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG1_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG2_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG2` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG2_R(crate::FieldReader<bool, ADCTRG2_A>);
impl ADCTRG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG2_A {
        match self.bits {
            false => ADCTRG2_A::_0,
            true => ADCTRG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG2_A::_1
    }
}
impl core::ops::Deref for ADCTRG2_R {
    type Target = crate::FieldReader<bool, ADCTRG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG2` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG2_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG2_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG3_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG3` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG3_R(crate::FieldReader<bool, ADCTRG3_A>);
impl ADCTRG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG3_A {
        match self.bits {
            false => ADCTRG3_A::_0,
            true => ADCTRG3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG3_A::_1
    }
}
impl core::ops::Deref for ADCTRG3_R {
    type Target = crate::FieldReader<bool, ADCTRG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG3` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG3_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG3_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG4_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG4_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG4` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG4_R(crate::FieldReader<bool, ADCTRG4_A>);
impl ADCTRG4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG4_A {
        match self.bits {
            false => ADCTRG4_A::_0,
            true => ADCTRG4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG4_A::_1
    }
}
impl core::ops::Deref for ADCTRG4_R {
    type Target = crate::FieldReader<bool, ADCTRG4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG4` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG4_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG4_A::_1)
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
#[doc = "ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTRG5_A {
    #[doc = "0: Indicates no ADC start of conversion trigger event has occurred"]
    _0 = 0,
    #[doc = "1: An ADC start of conversion trigger event has occurred"]
    _1 = 1,
}
impl From<ADCTRG5_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTRG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTRG5` reader - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG5_R(crate::FieldReader<bool, ADCTRG5_A>);
impl ADCTRG5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCTRG5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRG5_A {
        match self.bits {
            false => ADCTRG5_A::_0,
            true => ADCTRG5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCTRG5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCTRG5_A::_1
    }
}
impl core::ops::Deref for ADCTRG5_R {
    type Target = crate::FieldReader<bool, ADCTRG5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTRG5` writer - ADC Start of Conversion Status\nNote: This bit can be cleared by software writing 1."]
pub struct ADCTRG5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRG5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates no ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCTRG5_A::_0)
    }
    #[doc = "An ADC start of conversion trigger event has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCTRG5_A::_1)
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
impl R {
    #[doc = "Bit 0 - Time-base Counter 0 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax0(&self) -> CNTMAX0_R {
        CNTMAX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-base Counter 2 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax2(&self) -> CNTMAX2_R {
        CNTMAX2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time-base Counter 4 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax4(&self) -> CNTMAX4_R {
        CNTMAX4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg0(&self) -> ADCTRG0_R {
        ADCTRG0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg1(&self) -> ADCTRG1_R {
        ADCTRG1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg2(&self) -> ADCTRG2_R {
        ADCTRG2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg3(&self) -> ADCTRG3_R {
        ADCTRG3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg4(&self) -> ADCTRG4_R {
        ADCTRG4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg5(&self) -> ADCTRG5_R {
        ADCTRG5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time-base Counter 0 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax0(&mut self) -> CNTMAX0_W {
        CNTMAX0_W { w: self }
    }
    #[doc = "Bit 2 - Time-base Counter 2 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax2(&mut self) -> CNTMAX2_W {
        CNTMAX2_W { w: self }
    }
    #[doc = "Bit 4 - Time-base Counter 4 Equal to 0xFFFF Latched Flag Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn cntmax4(&mut self) -> CNTMAX4_W {
        CNTMAX4_W { w: self }
    }
    #[doc = "Bit 16 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg0(&mut self) -> ADCTRG0_W {
        ADCTRG0_W { w: self }
    }
    #[doc = "Bit 17 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg1(&mut self) -> ADCTRG1_W {
        ADCTRG1_W { w: self }
    }
    #[doc = "Bit 18 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg2(&mut self) -> ADCTRG2_W {
        ADCTRG2_W { w: self }
    }
    #[doc = "Bit 19 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg3(&mut self) -> ADCTRG3_W {
        ADCTRG3_W { w: self }
    }
    #[doc = "Bit 20 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg4(&mut self) -> ADCTRG4_W {
        ADCTRG4_W { w: self }
    }
    #[doc = "Bit 21 - ADC Start of Conversion Status Note: This bit can be cleared by software writing 1."]
    #[inline(always)]
    pub fn adctrg5(&mut self) -> ADCTRG5_W {
        ADCTRG5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_status](index.html) module"]
pub struct PWM_STATUS_SPEC;
impl crate::RegisterSpec for PWM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_status::R](R) reader structure"]
impl crate::Readable for PWM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_status::W](W) writer structure"]
impl crate::Writable for PWM_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_STATUS to value 0"]
impl crate::Resettable for PWM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
