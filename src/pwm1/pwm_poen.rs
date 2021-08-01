#[doc = "Register `PWM_POEN` reader"]
pub struct R(crate::R<PWM_POEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_POEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_POEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_POEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_POEN` writer"]
pub struct W(crate::W<PWM_POEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_POEN_SPEC>;
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
impl From<crate::W<PWM_POEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_POEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN0_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN0_A> for bool {
    #[inline(always)]
    fn from(variant: POEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN0` reader - PWM Pin Output Enable Bits"]
pub struct POEN0_R(crate::FieldReader<bool, POEN0_A>);
impl POEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN0_A {
        match self.bits {
            false => POEN0_A::_0,
            true => POEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN0_A::_1
    }
}
impl core::ops::Deref for POEN0_R {
    type Target = crate::FieldReader<bool, POEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN0` writer - PWM Pin Output Enable Bits"]
pub struct POEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN0_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN0_A::_1)
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
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN1_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN1_A> for bool {
    #[inline(always)]
    fn from(variant: POEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN1` reader - PWM Pin Output Enable Bits"]
pub struct POEN1_R(crate::FieldReader<bool, POEN1_A>);
impl POEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN1_A {
        match self.bits {
            false => POEN1_A::_0,
            true => POEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN1_A::_1
    }
}
impl core::ops::Deref for POEN1_R {
    type Target = crate::FieldReader<bool, POEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN1` writer - PWM Pin Output Enable Bits"]
pub struct POEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN1_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN1_A::_1)
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
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN2_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN2_A> for bool {
    #[inline(always)]
    fn from(variant: POEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN2` reader - PWM Pin Output Enable Bits"]
pub struct POEN2_R(crate::FieldReader<bool, POEN2_A>);
impl POEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN2_A {
        match self.bits {
            false => POEN2_A::_0,
            true => POEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN2_A::_1
    }
}
impl core::ops::Deref for POEN2_R {
    type Target = crate::FieldReader<bool, POEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN2` writer - PWM Pin Output Enable Bits"]
pub struct POEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN2_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN2_A::_1)
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
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN3_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN3_A> for bool {
    #[inline(always)]
    fn from(variant: POEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN3` reader - PWM Pin Output Enable Bits"]
pub struct POEN3_R(crate::FieldReader<bool, POEN3_A>);
impl POEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN3_A {
        match self.bits {
            false => POEN3_A::_0,
            true => POEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN3_A::_1
    }
}
impl core::ops::Deref for POEN3_R {
    type Target = crate::FieldReader<bool, POEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN3` writer - PWM Pin Output Enable Bits"]
pub struct POEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN3_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN3_A::_1)
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
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN4_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN4_A> for bool {
    #[inline(always)]
    fn from(variant: POEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN4` reader - PWM Pin Output Enable Bits"]
pub struct POEN4_R(crate::FieldReader<bool, POEN4_A>);
impl POEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN4_A {
        match self.bits {
            false => POEN4_A::_0,
            true => POEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN4_A::_1
    }
}
impl core::ops::Deref for POEN4_R {
    type Target = crate::FieldReader<bool, POEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN4` writer - PWM Pin Output Enable Bits"]
pub struct POEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN4_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN4_A::_1)
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
#[doc = "PWM Pin Output Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN5_A {
    #[doc = "0: PWM pin at tri-state"]
    _0 = 0,
    #[doc = "1: PWM pin in output mode"]
    _1 = 1,
}
impl From<POEN5_A> for bool {
    #[inline(always)]
    fn from(variant: POEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN5` reader - PWM Pin Output Enable Bits"]
pub struct POEN5_R(crate::FieldReader<bool, POEN5_A>);
impl POEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        POEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN5_A {
        match self.bits {
            false => POEN5_A::_0,
            true => POEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POEN5_A::_1
    }
}
impl core::ops::Deref for POEN5_R {
    type Target = crate::FieldReader<bool, POEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN5` writer - PWM Pin Output Enable Bits"]
pub struct POEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM pin at tri-state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN5_A::_0)
    }
    #[doc = "PWM pin in output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN5_A::_1)
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
    #[doc = "Bit 0 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen0(&self) -> POEN0_R {
        POEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen1(&self) -> POEN1_R {
        POEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen2(&self) -> POEN2_R {
        POEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen3(&self) -> POEN3_R {
        POEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen4(&self) -> POEN4_R {
        POEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen5(&self) -> POEN5_R {
        POEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen0(&mut self) -> POEN0_W {
        POEN0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen1(&mut self) -> POEN1_W {
        POEN1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen2(&mut self) -> POEN2_W {
        POEN2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen3(&mut self) -> POEN3_W {
        POEN3_W { w: self }
    }
    #[doc = "Bit 4 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen4(&mut self) -> POEN4_W {
        POEN4_W { w: self }
    }
    #[doc = "Bit 5 - PWM Pin Output Enable Bits"]
    #[inline(always)]
    pub fn poen5(&mut self) -> POEN5_W {
        POEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_poen](index.html) module"]
pub struct PWM_POEN_SPEC;
impl crate::RegisterSpec for PWM_POEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_poen::R](R) reader structure"]
impl crate::Readable for PWM_POEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_poen::W](W) writer structure"]
impl crate::Writable for PWM_POEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_POEN to value 0"]
impl crate::Resettable for PWM_POEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
