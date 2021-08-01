#[doc = "Register `ADC_ADCALSTSR` reader"]
pub struct R(crate::R<ADC_ADCALSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADCALSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADCALSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADCALSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADCALSTSR` writer"]
pub struct W(crate::W<ADC_ADCALSTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADCALSTSR_SPEC>;
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
impl From<crate::W<ADC_ADCALSTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADCALSTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIF` reader - Calibration Finish Interrupt Flag\nIf calibration is finished, this flag will be set to 1. It is cleared by writing 1 to it."]
pub struct CALIF_R(crate::FieldReader<bool, bool>);
impl CALIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIF` writer - Calibration Finish Interrupt Flag\nIf calibration is finished, this flag will be set to 1. It is cleared by writing 1 to it."]
pub struct CALIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Calibration Finish Interrupt Flag If calibration is finished, this flag will be set to 1. It is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn calif(&self) -> CALIF_R {
        CALIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Finish Interrupt Flag If calibration is finished, this flag will be set to 1. It is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn calif(&mut self) -> CALIF_W {
        CALIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adcalstsr](index.html) module"]
pub struct ADC_ADCALSTSR_SPEC;
impl crate::RegisterSpec for ADC_ADCALSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adcalstsr::R](R) reader structure"]
impl crate::Readable for ADC_ADCALSTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adcalstsr::W](W) writer structure"]
impl crate::Writable for ADC_ADCALSTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADCALSTSR to value 0"]
impl crate::Resettable for ADC_ADCALSTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
