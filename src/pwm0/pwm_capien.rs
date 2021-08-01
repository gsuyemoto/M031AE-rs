#[doc = "Register `PWM_CAPIEN` reader"]
pub struct R(crate::R<PWM_CAPIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CAPIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CAPIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CAPIEN` writer"]
pub struct W(crate::W<PWM_CAPIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CAPIEN_SPEC>;
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
impl From<crate::W<PWM_CAPIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CAPIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN0_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN0` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN0_R(crate::FieldReader<bool, CAPRIEN0_A>);
impl CAPRIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN0_A {
        match self.bits {
            false => CAPRIEN0_A::_0,
            true => CAPRIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN0_A::_1
    }
}
impl core::ops::Deref for CAPRIEN0_R {
    type Target = crate::FieldReader<bool, CAPRIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN0` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN0_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN0_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN1_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN1` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN1_R(crate::FieldReader<bool, CAPRIEN1_A>);
impl CAPRIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN1_A {
        match self.bits {
            false => CAPRIEN1_A::_0,
            true => CAPRIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN1_A::_1
    }
}
impl core::ops::Deref for CAPRIEN1_R {
    type Target = crate::FieldReader<bool, CAPRIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN1` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN1_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN1_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN2_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN2` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN2_R(crate::FieldReader<bool, CAPRIEN2_A>);
impl CAPRIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN2_A {
        match self.bits {
            false => CAPRIEN2_A::_0,
            true => CAPRIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN2_A::_1
    }
}
impl core::ops::Deref for CAPRIEN2_R {
    type Target = crate::FieldReader<bool, CAPRIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN2` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN2_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN2_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN3_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN3` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN3_R(crate::FieldReader<bool, CAPRIEN3_A>);
impl CAPRIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN3_A {
        match self.bits {
            false => CAPRIEN3_A::_0,
            true => CAPRIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN3_A::_1
    }
}
impl core::ops::Deref for CAPRIEN3_R {
    type Target = crate::FieldReader<bool, CAPRIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN3` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN3_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN3_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN4_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN4` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN4_R(crate::FieldReader<bool, CAPRIEN4_A>);
impl CAPRIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN4_A {
        match self.bits {
            false => CAPRIEN4_A::_0,
            true => CAPRIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN4_A::_1
    }
}
impl core::ops::Deref for CAPRIEN4_R {
    type Target = crate::FieldReader<bool, CAPRIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN4` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN4_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN4_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIEN5_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIEN5` reader - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN5_R(crate::FieldReader<bool, CAPRIEN5_A>);
impl CAPRIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIEN5_A {
        match self.bits {
            false => CAPRIEN5_A::_0,
            true => CAPRIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIEN5_A::_1
    }
}
impl core::ops::Deref for CAPRIEN5_R {
    type Target = crate::FieldReader<bool, CAPRIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIEN5` writer - PWM Capture Rising Latch Interrupt Enable Bits"]
pub struct CAPRIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIEN5_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIEN5_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN0_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN0` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN0_R(crate::FieldReader<bool, CAPFIEN0_A>);
impl CAPFIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN0_A {
        match self.bits {
            false => CAPFIEN0_A::_0,
            true => CAPFIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN0_A::_1
    }
}
impl core::ops::Deref for CAPFIEN0_R {
    type Target = crate::FieldReader<bool, CAPFIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN0` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN0_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN0_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN1_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN1` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN1_R(crate::FieldReader<bool, CAPFIEN1_A>);
impl CAPFIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN1_A {
        match self.bits {
            false => CAPFIEN1_A::_0,
            true => CAPFIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN1_A::_1
    }
}
impl core::ops::Deref for CAPFIEN1_R {
    type Target = crate::FieldReader<bool, CAPFIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN1` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN1_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN1_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN2_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN2` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN2_R(crate::FieldReader<bool, CAPFIEN2_A>);
impl CAPFIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN2_A {
        match self.bits {
            false => CAPFIEN2_A::_0,
            true => CAPFIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN2_A::_1
    }
}
impl core::ops::Deref for CAPFIEN2_R {
    type Target = crate::FieldReader<bool, CAPFIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN2` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN2_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN2_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN3_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN3` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN3_R(crate::FieldReader<bool, CAPFIEN3_A>);
impl CAPFIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN3_A {
        match self.bits {
            false => CAPFIEN3_A::_0,
            true => CAPFIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN3_A::_1
    }
}
impl core::ops::Deref for CAPFIEN3_R {
    type Target = crate::FieldReader<bool, CAPFIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN3` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN3_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN3_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN4_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN4` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN4_R(crate::FieldReader<bool, CAPFIEN4_A>);
impl CAPFIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN4_A {
        match self.bits {
            false => CAPFIEN4_A::_0,
            true => CAPFIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN4_A::_1
    }
}
impl core::ops::Deref for CAPFIEN4_R {
    type Target = crate::FieldReader<bool, CAPFIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN4` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN4_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN4_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIEN5_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIEN5` reader - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN5_R(crate::FieldReader<bool, CAPFIEN5_A>);
impl CAPFIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIEN5_A {
        match self.bits {
            false => CAPFIEN5_A::_0,
            true => CAPFIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIEN5_A::_1
    }
}
impl core::ops::Deref for CAPFIEN5_R {
    type Target = crate::FieldReader<bool, CAPFIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIEN5` writer - PWM Capture Falling Latch Interrupt Enable Bits"]
pub struct CAPFIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIEN5_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIEN5_A::_1)
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
impl R {
    #[doc = "Bit 0 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien0(&self) -> CAPRIEN0_R {
        CAPRIEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien1(&self) -> CAPRIEN1_R {
        CAPRIEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien2(&self) -> CAPRIEN2_R {
        CAPRIEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien3(&self) -> CAPRIEN3_R {
        CAPRIEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien4(&self) -> CAPRIEN4_R {
        CAPRIEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien5(&self) -> CAPRIEN5_R {
        CAPRIEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien0(&self) -> CAPFIEN0_R {
        CAPFIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien1(&self) -> CAPFIEN1_R {
        CAPFIEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien2(&self) -> CAPFIEN2_R {
        CAPFIEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien3(&self) -> CAPFIEN3_R {
        CAPFIEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien4(&self) -> CAPFIEN4_R {
        CAPFIEN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien5(&self) -> CAPFIEN5_R {
        CAPFIEN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien0(&mut self) -> CAPRIEN0_W {
        CAPRIEN0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien1(&mut self) -> CAPRIEN1_W {
        CAPRIEN1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien2(&mut self) -> CAPRIEN2_W {
        CAPRIEN2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien3(&mut self) -> CAPRIEN3_W {
        CAPRIEN3_W { w: self }
    }
    #[doc = "Bit 4 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien4(&mut self) -> CAPRIEN4_W {
        CAPRIEN4_W { w: self }
    }
    #[doc = "Bit 5 - PWM Capture Rising Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn caprien5(&mut self) -> CAPRIEN5_W {
        CAPRIEN5_W { w: self }
    }
    #[doc = "Bit 8 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien0(&mut self) -> CAPFIEN0_W {
        CAPFIEN0_W { w: self }
    }
    #[doc = "Bit 9 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien1(&mut self) -> CAPFIEN1_W {
        CAPFIEN1_W { w: self }
    }
    #[doc = "Bit 10 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien2(&mut self) -> CAPFIEN2_W {
        CAPFIEN2_W { w: self }
    }
    #[doc = "Bit 11 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien3(&mut self) -> CAPFIEN3_W {
        CAPFIEN3_W { w: self }
    }
    #[doc = "Bit 12 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien4(&mut self) -> CAPFIEN4_W {
        CAPFIEN4_W { w: self }
    }
    #[doc = "Bit 13 - PWM Capture Falling Latch Interrupt Enable Bits"]
    #[inline(always)]
    pub fn capfien5(&mut self) -> CAPFIEN5_W {
        CAPFIEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Capture Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capien](index.html) module"]
pub struct PWM_CAPIEN_SPEC;
impl crate::RegisterSpec for PWM_CAPIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capien::R](R) reader structure"]
impl crate::Readable for PWM_CAPIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_capien::W](W) writer structure"]
impl crate::Writable for PWM_CAPIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CAPIEN to value 0"]
impl crate::Resettable for PWM_CAPIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
