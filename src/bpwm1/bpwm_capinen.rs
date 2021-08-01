#[doc = "Register `BPWM_CAPINEN` reader"]
pub struct R(crate::R<BPWM_CAPINEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CAPINEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CAPINEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CAPINEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CAPINEN` writer"]
pub struct W(crate::W<BPWM_CAPINEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CAPINEN_SPEC>;
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
impl From<crate::W<BPWM_CAPINEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CAPINEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN0_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN0` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN0_R(crate::FieldReader<bool, CAPINEN0_A>);
impl CAPINEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN0_A {
        match self.bits {
            false => CAPINEN0_A::_0,
            true => CAPINEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN0_A::_1
    }
}
impl core::ops::Deref for CAPINEN0_R {
    type Target = crate::FieldReader<bool, CAPINEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN0` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN0_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN0_A::_1)
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
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN1_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN1` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN1_R(crate::FieldReader<bool, CAPINEN1_A>);
impl CAPINEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN1_A {
        match self.bits {
            false => CAPINEN1_A::_0,
            true => CAPINEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN1_A::_1
    }
}
impl core::ops::Deref for CAPINEN1_R {
    type Target = crate::FieldReader<bool, CAPINEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN1` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN1_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN1_A::_1)
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
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN2_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN2` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN2_R(crate::FieldReader<bool, CAPINEN2_A>);
impl CAPINEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN2_A {
        match self.bits {
            false => CAPINEN2_A::_0,
            true => CAPINEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN2_A::_1
    }
}
impl core::ops::Deref for CAPINEN2_R {
    type Target = crate::FieldReader<bool, CAPINEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN2` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN2_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN2_A::_1)
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
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN3_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN3` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN3_R(crate::FieldReader<bool, CAPINEN3_A>);
impl CAPINEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN3_A {
        match self.bits {
            false => CAPINEN3_A::_0,
            true => CAPINEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN3_A::_1
    }
}
impl core::ops::Deref for CAPINEN3_R {
    type Target = crate::FieldReader<bool, CAPINEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN3` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN3_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN3_A::_1)
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
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN4_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN4` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN4_R(crate::FieldReader<bool, CAPINEN4_A>);
impl CAPINEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN4_A {
        match self.bits {
            false => CAPINEN4_A::_0,
            true => CAPINEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN4_A::_1
    }
}
impl core::ops::Deref for CAPINEN4_R {
    type Target = crate::FieldReader<bool, CAPINEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN4` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN4_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN4_A::_1)
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
#[doc = "Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINEN5_A {
    #[doc = "0: BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    _0 = 0,
    #[doc = "1: BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    _1 = 1,
}
impl From<CAPINEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINEN5` reader - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN5_R(crate::FieldReader<bool, CAPINEN5_A>);
impl CAPINEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINEN5_A {
        match self.bits {
            false => CAPINEN5_A::_0,
            true => CAPINEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINEN5_A::_1
    }
}
impl core::ops::Deref for CAPINEN5_R {
    type Target = crate::FieldReader<bool, CAPINEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINEN5` writer - Capture Input Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Channel capture input path Disabled. The input of BPWM channel capture function is always regarded as 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINEN5_A::_0)
    }
    #[doc = "BPWM Channel capture input path Enabled. The input of BPWM channel capture function comes from correlative multifunction pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINEN5_A::_1)
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
    #[doc = "Bit 0 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen0(&self) -> CAPINEN0_R {
        CAPINEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen1(&self) -> CAPINEN1_R {
        CAPINEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen2(&self) -> CAPINEN2_R {
        CAPINEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen3(&self) -> CAPINEN3_R {
        CAPINEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen4(&self) -> CAPINEN4_R {
        CAPINEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen5(&self) -> CAPINEN5_R {
        CAPINEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen0(&mut self) -> CAPINEN0_W {
        CAPINEN0_W { w: self }
    }
    #[doc = "Bit 1 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen1(&mut self) -> CAPINEN1_W {
        CAPINEN1_W { w: self }
    }
    #[doc = "Bit 2 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen2(&mut self) -> CAPINEN2_W {
        CAPINEN2_W { w: self }
    }
    #[doc = "Bit 3 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen3(&mut self) -> CAPINEN3_W {
        CAPINEN3_W { w: self }
    }
    #[doc = "Bit 4 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen4(&mut self) -> CAPINEN4_W {
        CAPINEN4_W { w: self }
    }
    #[doc = "Bit 5 - Capture Input Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinen5(&mut self) -> CAPINEN5_W {
        CAPINEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Capture Input Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_capinen](index.html) module"]
pub struct BPWM_CAPINEN_SPEC;
impl crate::RegisterSpec for BPWM_CAPINEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_capinen::R](R) reader structure"]
impl crate::Readable for BPWM_CAPINEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_capinen::W](W) writer structure"]
impl crate::Writable for BPWM_CAPINEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CAPINEN to value 0"]
impl crate::Resettable for BPWM_CAPINEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
