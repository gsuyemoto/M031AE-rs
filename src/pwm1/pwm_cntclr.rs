#[doc = "Register `PWM_CNTCLR` reader"]
pub struct R(crate::R<PWM_CNTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CNTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CNTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CNTCLR` writer"]
pub struct W(crate::W<PWM_CNTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CNTCLR_SPEC>;
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
impl From<crate::W<PWM_CNTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CNTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear PWM Counter Control Bit 0\nIt is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTCLR0_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 16-bit PWM counter to 0000H"]
    _1 = 1,
}
impl From<CNTCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTCLR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTCLR0` reader - Clear PWM Counter Control Bit 0\nIt is automatically cleared by hardware."]
pub struct CNTCLR0_R(crate::FieldReader<bool, CNTCLR0_A>);
impl CNTCLR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTCLR0_A {
        match self.bits {
            false => CNTCLR0_A::_0,
            true => CNTCLR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTCLR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTCLR0_A::_1
    }
}
impl core::ops::Deref for CNTCLR0_R {
    type Target = crate::FieldReader<bool, CNTCLR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR0` writer - Clear PWM Counter Control Bit 0\nIt is automatically cleared by hardware."]
pub struct CNTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTCLR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTCLR0_A::_0)
    }
    #[doc = "Clear 16-bit PWM counter to 0000H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTCLR0_A::_1)
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
#[doc = "Clear PWM Counter Control Bit 2\nIt is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTCLR2_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 16-bit PWM counter to 0000H"]
    _1 = 1,
}
impl From<CNTCLR2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTCLR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTCLR2` reader - Clear PWM Counter Control Bit 2\nIt is automatically cleared by hardware."]
pub struct CNTCLR2_R(crate::FieldReader<bool, CNTCLR2_A>);
impl CNTCLR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTCLR2_A {
        match self.bits {
            false => CNTCLR2_A::_0,
            true => CNTCLR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTCLR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTCLR2_A::_1
    }
}
impl core::ops::Deref for CNTCLR2_R {
    type Target = crate::FieldReader<bool, CNTCLR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR2` writer - Clear PWM Counter Control Bit 2\nIt is automatically cleared by hardware."]
pub struct CNTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTCLR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTCLR2_A::_0)
    }
    #[doc = "Clear 16-bit PWM counter to 0000H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTCLR2_A::_1)
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
#[doc = "Clear PWM Counter Control Bit 4\nIt is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTCLR4_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 16-bit PWM counter to 0000H"]
    _1 = 1,
}
impl From<CNTCLR4_A> for bool {
    #[inline(always)]
    fn from(variant: CNTCLR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTCLR4` reader - Clear PWM Counter Control Bit 4\nIt is automatically cleared by hardware."]
pub struct CNTCLR4_R(crate::FieldReader<bool, CNTCLR4_A>);
impl CNTCLR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTCLR4_A {
        match self.bits {
            false => CNTCLR4_A::_0,
            true => CNTCLR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTCLR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTCLR4_A::_1
    }
}
impl core::ops::Deref for CNTCLR4_R {
    type Target = crate::FieldReader<bool, CNTCLR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR4` writer - Clear PWM Counter Control Bit 4\nIt is automatically cleared by hardware."]
pub struct CNTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTCLR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTCLR4_A::_0)
    }
    #[doc = "Clear 16-bit PWM counter to 0000H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTCLR4_A::_1)
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
    #[doc = "Bit 0 - Clear PWM Counter Control Bit 0 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr0(&self) -> CNTCLR0_R {
        CNTCLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear PWM Counter Control Bit 2 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr2(&self) -> CNTCLR2_R {
        CNTCLR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear PWM Counter Control Bit 4 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr4(&self) -> CNTCLR4_R {
        CNTCLR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear PWM Counter Control Bit 0 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr0(&mut self) -> CNTCLR0_W {
        CNTCLR0_W { w: self }
    }
    #[doc = "Bit 2 - Clear PWM Counter Control Bit 2 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr2(&mut self) -> CNTCLR2_W {
        CNTCLR2_W { w: self }
    }
    #[doc = "Bit 4 - Clear PWM Counter Control Bit 4 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr4(&mut self) -> CNTCLR4_W {
        CNTCLR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clear Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cntclr](index.html) module"]
pub struct PWM_CNTCLR_SPEC;
impl crate::RegisterSpec for PWM_CNTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cntclr::R](R) reader structure"]
impl crate::Readable for PWM_CNTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cntclr::W](W) writer structure"]
impl crate::Writable for PWM_CNTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CNTCLR to value 0"]
impl crate::Resettable for PWM_CNTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
