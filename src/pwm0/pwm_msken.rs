#[doc = "Register `PWM_MSKEN` reader"]
pub struct R(crate::R<PWM_MSKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_MSKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_MSKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_MSKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_MSKEN` writer"]
pub struct W(crate::W<PWM_MSKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_MSKEN_SPEC>;
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
impl From<crate::W<PWM_MSKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_MSKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN0_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN0_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN0` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN0_R(crate::FieldReader<bool, MSKEN0_A>);
impl MSKEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN0_A {
        match self.bits {
            false => MSKEN0_A::_0,
            true => MSKEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN0_A::_1
    }
}
impl core::ops::Deref for MSKEN0_R {
    type Target = crate::FieldReader<bool, MSKEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN0` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN0_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN0_A::_1)
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
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN1_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN1` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN1_R(crate::FieldReader<bool, MSKEN1_A>);
impl MSKEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN1_A {
        match self.bits {
            false => MSKEN1_A::_0,
            true => MSKEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN1_A::_1
    }
}
impl core::ops::Deref for MSKEN1_R {
    type Target = crate::FieldReader<bool, MSKEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN1` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN1_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN1_A::_1)
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
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN2_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN2` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN2_R(crate::FieldReader<bool, MSKEN2_A>);
impl MSKEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN2_A {
        match self.bits {
            false => MSKEN2_A::_0,
            true => MSKEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN2_A::_1
    }
}
impl core::ops::Deref for MSKEN2_R {
    type Target = crate::FieldReader<bool, MSKEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN2` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN2_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN2_A::_1)
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
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN3_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN3_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN3` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN3_R(crate::FieldReader<bool, MSKEN3_A>);
impl MSKEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN3_A {
        match self.bits {
            false => MSKEN3_A::_0,
            true => MSKEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN3_A::_1
    }
}
impl core::ops::Deref for MSKEN3_R {
    type Target = crate::FieldReader<bool, MSKEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN3` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN3_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN3_A::_1)
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
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN4_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN4_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN4` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN4_R(crate::FieldReader<bool, MSKEN4_A>);
impl MSKEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN4_A {
        match self.bits {
            false => MSKEN4_A::_0,
            true => MSKEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN4_A::_1
    }
}
impl core::ops::Deref for MSKEN4_R {
    type Target = crate::FieldReader<bool, MSKEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN4` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN4_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN4_A::_1)
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
#[doc = "PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKEN5_A {
    #[doc = "0: PWM output signal is non-masked"]
    _0 = 0,
    #[doc = "1: PWM output signal is masked and output MSKDATn data"]
    _1 = 1,
}
impl From<MSKEN5_A> for bool {
    #[inline(always)]
    fn from(variant: MSKEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKEN5` reader - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN5_R(crate::FieldReader<bool, MSKEN5_A>);
impl MSKEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKEN5_A {
        match self.bits {
            false => MSKEN5_A::_0,
            true => MSKEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKEN5_A::_1
    }
}
impl core::ops::Deref for MSKEN5_R {
    type Target = crate::FieldReader<bool, MSKEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKEN5` writer - PWM Mask Enable Bits\nThe PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
pub struct MSKEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output signal is non-masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKEN5_A::_0)
    }
    #[doc = "PWM output signal is masked and output MSKDATn data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKEN5_A::_1)
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
impl R {
    #[doc = "Bit 0 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken0(&self) -> MSKEN0_R {
        MSKEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken1(&self) -> MSKEN1_R {
        MSKEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken2(&self) -> MSKEN2_R {
        MSKEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken3(&self) -> MSKEN3_R {
        MSKEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken4(&self) -> MSKEN4_R {
        MSKEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken5(&self) -> MSKEN5_R {
        MSKEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken0(&mut self) -> MSKEN0_W {
        MSKEN0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken1(&mut self) -> MSKEN1_W {
        MSKEN1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken2(&mut self) -> MSKEN2_W {
        MSKEN2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken3(&mut self) -> MSKEN3_W {
        MSKEN3_W { w: self }
    }
    #[doc = "Bit 4 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken4(&mut self) -> MSKEN4_W {
        MSKEN4_W { w: self }
    }
    #[doc = "Bit 5 - PWM Mask Enable Bits The PWM output signal will be masked when this bit is enabled. The corresponding PWM channel n will output MSKDATn (PWM_MSK\\[5:0\\]) data."]
    #[inline(always)]
    pub fn msken5(&mut self) -> MSKEN5_W {
        MSKEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Mask Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_msken](index.html) module"]
pub struct PWM_MSKEN_SPEC;
impl crate::RegisterSpec for PWM_MSKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_msken::R](R) reader structure"]
impl crate::Readable for PWM_MSKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_msken::W](W) writer structure"]
impl crate::Writable for PWM_MSKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_MSKEN to value 0"]
impl crate::Resettable for PWM_MSKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
