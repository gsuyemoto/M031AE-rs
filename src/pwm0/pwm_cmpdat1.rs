#[doc = "Register `PWM_CMPDAT1` reader"]
pub struct R(crate::R<PWM_CMPDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CMPDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CMPDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CMPDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CMPDAT1` writer"]
pub struct W(crate::W<PWM_CMPDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMPDAT1_SPEC>;
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
impl From<crate::W<PWM_CMPDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMPDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - PWM Comparator Register\nCMP is used to compare with CNTR to generate PWM waveform, interrupt and trigger ADC.\nIn independent mode, PWM_CMPDAT0~5 denote as 6 independent PWM_CH0~5 compared point.\nIn complementary mode, PWM_CMPDAT0, 2, 4 denote as first compared point, and PWM_CMPDAT1, 3, 5 denote as second compared point for the corresponding 3 complementary pairs PWM_CH0 and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
pub struct CMP_R(crate::FieldReader<u16, u16>);
impl CMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP` writer - PWM Comparator Register\nCMP is used to compare with CNTR to generate PWM waveform, interrupt and trigger ADC.\nIn independent mode, PWM_CMPDAT0~5 denote as 6 independent PWM_CH0~5 compared point.\nIn complementary mode, PWM_CMPDAT0, 2, 4 denote as first compared point, and PWM_CMPDAT1, 3, 5 denote as second compared point for the corresponding 3 complementary pairs PWM_CH0 and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
pub struct CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Comparator Register CMP is used to compare with CNTR to generate PWM waveform, interrupt and trigger ADC. In independent mode, PWM_CMPDAT0~5 denote as 6 independent PWM_CH0~5 compared point. In complementary mode, PWM_CMPDAT0, 2, 4 denote as first compared point, and PWM_CMPDAT1, 3, 5 denote as second compared point for the corresponding 3 complementary pairs PWM_CH0 and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Comparator Register CMP is used to compare with CNTR to generate PWM waveform, interrupt and trigger ADC. In independent mode, PWM_CMPDAT0~5 denote as 6 independent PWM_CH0~5 compared point. In complementary mode, PWM_CMPDAT0, 2, 4 denote as first compared point, and PWM_CMPDAT1, 3, 5 denote as second compared point for the corresponding 3 complementary pairs PWM_CH0 and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparator Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpdat1](index.html) module"]
pub struct PWM_CMPDAT1_SPEC;
impl crate::RegisterSpec for PWM_CMPDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cmpdat1::R](R) reader structure"]
impl crate::Readable for PWM_CMPDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmpdat1::W](W) writer structure"]
impl crate::Writable for PWM_CMPDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMPDAT1 to value 0"]
impl crate::Resettable for PWM_CMPDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
