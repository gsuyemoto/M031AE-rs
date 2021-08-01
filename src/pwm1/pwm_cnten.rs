#[doc = "Register `PWM_CNTEN` reader"]
pub struct R(crate::R<PWM_CNTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CNTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CNTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CNTEN` writer"]
pub struct W(crate::W<PWM_CNTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CNTEN_SPEC>;
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
impl From<crate::W<PWM_CNTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CNTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Counter Enable Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN0_A {
    #[doc = "0: PWM Counter and clock prescaler Stop Running"]
    _0 = 0,
    #[doc = "1: PWM Counter and clock prescaler Start Running"]
    _1 = 1,
}
impl From<CNTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN0` reader - PWM Counter Enable Bit 0"]
pub struct CNTEN0_R(crate::FieldReader<bool, CNTEN0_A>);
impl CNTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN0_A {
        match self.bits {
            false => CNTEN0_A::_0,
            true => CNTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN0_A::_1
    }
}
impl core::ops::Deref for CNTEN0_R {
    type Target = crate::FieldReader<bool, CNTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN0` writer - PWM Counter Enable Bit 0"]
pub struct CNTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM Counter and clock prescaler Stop Running"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN0_A::_0)
    }
    #[doc = "PWM Counter and clock prescaler Start Running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN0_A::_1)
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
#[doc = "PWM Counter Enable Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN2_A {
    #[doc = "0: PWM Counter and clock prescaler Stop Running"]
    _0 = 0,
    #[doc = "1: PWM Counter and clock prescaler Start Running"]
    _1 = 1,
}
impl From<CNTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN2` reader - PWM Counter Enable Bit 2"]
pub struct CNTEN2_R(crate::FieldReader<bool, CNTEN2_A>);
impl CNTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN2_A {
        match self.bits {
            false => CNTEN2_A::_0,
            true => CNTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN2_A::_1
    }
}
impl core::ops::Deref for CNTEN2_R {
    type Target = crate::FieldReader<bool, CNTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN2` writer - PWM Counter Enable Bit 2"]
pub struct CNTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM Counter and clock prescaler Stop Running"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN2_A::_0)
    }
    #[doc = "PWM Counter and clock prescaler Start Running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN2_A::_1)
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
#[doc = "PWM Counter Enable Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN4_A {
    #[doc = "0: PWM Counter and clock prescaler Stop Running"]
    _0 = 0,
    #[doc = "1: PWM Counter and clock prescaler Start Running"]
    _1 = 1,
}
impl From<CNTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN4` reader - PWM Counter Enable Bit 4"]
pub struct CNTEN4_R(crate::FieldReader<bool, CNTEN4_A>);
impl CNTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN4_A {
        match self.bits {
            false => CNTEN4_A::_0,
            true => CNTEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN4_A::_1
    }
}
impl core::ops::Deref for CNTEN4_R {
    type Target = crate::FieldReader<bool, CNTEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN4` writer - PWM Counter Enable Bit 4"]
pub struct CNTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM Counter and clock prescaler Stop Running"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN4_A::_0)
    }
    #[doc = "PWM Counter and clock prescaler Start Running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN4_A::_1)
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
impl R {
    #[doc = "Bit 0 - PWM Counter Enable Bit 0"]
    #[inline(always)]
    pub fn cnten0(&self) -> CNTEN0_R {
        CNTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Counter Enable Bit 2"]
    #[inline(always)]
    pub fn cnten2(&self) -> CNTEN2_R {
        CNTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Counter Enable Bit 4"]
    #[inline(always)]
    pub fn cnten4(&self) -> CNTEN4_R {
        CNTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Counter Enable Bit 0"]
    #[inline(always)]
    pub fn cnten0(&mut self) -> CNTEN0_W {
        CNTEN0_W { w: self }
    }
    #[doc = "Bit 2 - PWM Counter Enable Bit 2"]
    #[inline(always)]
    pub fn cnten2(&mut self) -> CNTEN2_W {
        CNTEN2_W { w: self }
    }
    #[doc = "Bit 4 - PWM Counter Enable Bit 4"]
    #[inline(always)]
    pub fn cnten4(&mut self) -> CNTEN4_W {
        CNTEN4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Counter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cnten](index.html) module"]
pub struct PWM_CNTEN_SPEC;
impl crate::RegisterSpec for PWM_CNTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cnten::R](R) reader structure"]
impl crate::Readable for PWM_CNTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cnten::W](W) writer structure"]
impl crate::Writable for PWM_CNTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CNTEN to value 0"]
impl crate::Resettable for PWM_CNTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
