#[doc = "Register `PWM_POLCTL` reader"]
pub struct R(crate::R<PWM_POLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_POLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_POLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_POLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_POLCTL` writer"]
pub struct W(crate::W<PWM_POLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_POLCTL_SPEC>;
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
impl From<crate::W<PWM_POLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_POLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV0_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV0_A> for bool {
    #[inline(always)]
    fn from(variant: PINV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV0` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV0_R(crate::FieldReader<bool, PINV0_A>);
impl PINV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV0_A {
        match self.bits {
            false => PINV0_A::_0,
            true => PINV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV0_A::_1
    }
}
impl core::ops::Deref for PINV0_R {
    type Target = crate::FieldReader<bool, PINV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV0` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV0_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV0_A::_1)
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
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV1_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV1_A> for bool {
    #[inline(always)]
    fn from(variant: PINV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV1` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV1_R(crate::FieldReader<bool, PINV1_A>);
impl PINV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV1_A {
        match self.bits {
            false => PINV1_A::_0,
            true => PINV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV1_A::_1
    }
}
impl core::ops::Deref for PINV1_R {
    type Target = crate::FieldReader<bool, PINV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV1` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV1_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV1_A::_1)
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
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV2_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV2_A> for bool {
    #[inline(always)]
    fn from(variant: PINV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV2` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV2_R(crate::FieldReader<bool, PINV2_A>);
impl PINV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV2_A {
        match self.bits {
            false => PINV2_A::_0,
            true => PINV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV2_A::_1
    }
}
impl core::ops::Deref for PINV2_R {
    type Target = crate::FieldReader<bool, PINV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV2` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV2_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV2_A::_1)
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
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV3_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV3_A> for bool {
    #[inline(always)]
    fn from(variant: PINV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV3` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV3_R(crate::FieldReader<bool, PINV3_A>);
impl PINV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV3_A {
        match self.bits {
            false => PINV3_A::_0,
            true => PINV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV3_A::_1
    }
}
impl core::ops::Deref for PINV3_R {
    type Target = crate::FieldReader<bool, PINV3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV3` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV3_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV3_A::_1)
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
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV4_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV4_A> for bool {
    #[inline(always)]
    fn from(variant: PINV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV4` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV4_R(crate::FieldReader<bool, PINV4_A>);
impl PINV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV4_A {
        match self.bits {
            false => PINV4_A::_0,
            true => PINV4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV4_A::_1
    }
}
impl core::ops::Deref for PINV4_R {
    type Target = crate::FieldReader<bool, PINV4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV4` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV4_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV4_A::_1)
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
#[doc = "PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINV5_A {
    #[doc = "0: PWM output polar inverse Disabled"]
    _0 = 0,
    #[doc = "1: PWM output polar inverse Enabled"]
    _1 = 1,
}
impl From<PINV5_A> for bool {
    #[inline(always)]
    fn from(variant: PINV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINV5` reader - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV5_R(crate::FieldReader<bool, PINV5_A>);
impl PINV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINV5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINV5_A {
        match self.bits {
            false => PINV5_A::_0,
            true => PINV5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINV5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINV5_A::_1
    }
}
impl core::ops::Deref for PINV5_R {
    type Target = crate::FieldReader<bool, PINV5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINV5` writer - PWM PIN Polar Inverse Control\nThe register controls polarity state of PWM output."]
pub struct PINV5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINV5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM output polar inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINV5_A::_0)
    }
    #[doc = "PWM output polar inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINV5_A::_1)
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
    #[doc = "Bit 0 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv0(&self) -> PINV0_R {
        PINV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv1(&self) -> PINV1_R {
        PINV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv2(&self) -> PINV2_R {
        PINV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv3(&self) -> PINV3_R {
        PINV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv4(&self) -> PINV4_R {
        PINV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv5(&self) -> PINV5_R {
        PINV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv0(&mut self) -> PINV0_W {
        PINV0_W { w: self }
    }
    #[doc = "Bit 1 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv1(&mut self) -> PINV1_W {
        PINV1_W { w: self }
    }
    #[doc = "Bit 2 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv2(&mut self) -> PINV2_W {
        PINV2_W { w: self }
    }
    #[doc = "Bit 3 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv3(&mut self) -> PINV3_W {
        PINV3_W { w: self }
    }
    #[doc = "Bit 4 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv4(&mut self) -> PINV4_W {
        PINV4_W { w: self }
    }
    #[doc = "Bit 5 - PWM PIN Polar Inverse Control The register controls polarity state of PWM output."]
    #[inline(always)]
    pub fn pinv5(&mut self) -> PINV5_W {
        PINV5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Pin Polar Inverse Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_polctl](index.html) module"]
pub struct PWM_POLCTL_SPEC;
impl crate::RegisterSpec for PWM_POLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_polctl::R](R) reader structure"]
impl crate::Readable for PWM_POLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_polctl::W](W) writer structure"]
impl crate::Writable for PWM_POLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_POLCTL to value 0"]
impl crate::Resettable for PWM_POLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
