#[doc = "Register `ADC_ADCALR` reader"]
pub struct R(crate::R<ADC_ADCALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADCALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADCALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADCALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADCALR` writer"]
pub struct W(crate::W<ADC_ADCALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADCALR_SPEC>;
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
impl From<crate::W<ADC_ADCALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADCALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Calibration Function Enable Bit\nNote: If chip is powered off, calibration function should be executed again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEN_A {
    #[doc = "0: Calibration function Disabled"]
    _0 = 0,
}
impl From<CALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALEN` reader - Calibration Function Enable Bit\nNote: If chip is powered off, calibration function should be executed again."]
pub struct CALEN_R(crate::FieldReader<bool, CALEN_A>);
impl CALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALEN_A> {
        match self.bits {
            false => Some(CALEN_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALEN_A::_0
    }
}
impl core::ops::Deref for CALEN_R {
    type Target = crate::FieldReader<bool, CALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALEN` writer - Calibration Function Enable Bit\nNote: If chip is powered off, calibration function should be executed again."]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALEN_A::_0)
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
#[doc = "Calibration Interrupt Enable Bit\nIf calibration function is enabled and the calibration finish, CALIF bit will be asserted, in the meanwhile, if CALIE bit is set to 1, a calibration interrupt request is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIE_A {
    #[doc = "0: Calibration function Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Calibration function Interrupt Enabled"]
    _1 = 1,
}
impl From<CALIE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIE` reader - Calibration Interrupt Enable Bit\nIf calibration function is enabled and the calibration finish, CALIF bit will be asserted, in the meanwhile, if CALIE bit is set to 1, a calibration interrupt request is generated."]
pub struct CALIE_R(crate::FieldReader<bool, CALIE_A>);
impl CALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIE_A {
        match self.bits {
            false => CALIE_A::_0,
            true => CALIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALIE_A::_1
    }
}
impl core::ops::Deref for CALIE_R {
    type Target = crate::FieldReader<bool, CALIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIE` writer - Calibration Interrupt Enable Bit\nIf calibration function is enabled and the calibration finish, CALIF bit will be asserted, in the meanwhile, if CALIE bit is set to 1, a calibration interrupt request is generated."]
pub struct CALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration function Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALIE_A::_0)
    }
    #[doc = "Calibration function Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALIE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Calibration Function Enable Bit Note: If chip is powered off, calibration function should be executed again."]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Calibration Interrupt Enable Bit If calibration function is enabled and the calibration finish, CALIF bit will be asserted, in the meanwhile, if CALIE bit is set to 1, a calibration interrupt request is generated."]
    #[inline(always)]
    pub fn calie(&self) -> CALIE_R {
        CALIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Function Enable Bit Note: If chip is powered off, calibration function should be executed again."]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    #[doc = "Bit 1 - Calibration Interrupt Enable Bit If calibration function is enabled and the calibration finish, CALIF bit will be asserted, in the meanwhile, if CALIE bit is set to 1, a calibration interrupt request is generated."]
    #[inline(always)]
    pub fn calie(&mut self) -> CALIE_W {
        CALIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adcalr](index.html) module"]
pub struct ADC_ADCALR_SPEC;
impl crate::RegisterSpec for ADC_ADCALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adcalr::R](R) reader structure"]
impl crate::Readable for ADC_ADCALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adcalr::W](W) writer structure"]
impl crate::Writable for ADC_ADCALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADCALR to value 0x5c"]
impl crate::Resettable for ADC_ADCALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5c
    }
}
