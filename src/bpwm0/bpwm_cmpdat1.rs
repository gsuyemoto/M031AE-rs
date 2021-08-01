#[doc = "Register `BPWM_CMPDAT1` reader"]
pub struct R(crate::R<BPWM_CMPDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CMPDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CMPDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CMPDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CMPDAT1` writer"]
pub struct W(crate::W<BPWM_CMPDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CMPDAT1_SPEC>;
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
impl From<crate::W<BPWM_CMPDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CMPDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPDAT` reader - BPWM Comparator Register\nCMPDAT use to compare with CNTR to generate BPWM waveform, interrupt and trigger ADC.\nIn independent mode, CMPDAT0~5 denote as 6 independent BPWM_CH0~5 compared point."]
pub struct CMPDAT_R(crate::FieldReader<u16, u16>);
impl CMPDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDAT` writer - BPWM Comparator Register\nCMPDAT use to compare with CNTR to generate BPWM waveform, interrupt and trigger ADC.\nIn independent mode, CMPDAT0~5 denote as 6 independent BPWM_CH0~5 compared point."]
pub struct CMPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BPWM Comparator Register CMPDAT use to compare with CNTR to generate BPWM waveform, interrupt and trigger ADC. In independent mode, CMPDAT0~5 denote as 6 independent BPWM_CH0~5 compared point."]
    #[inline(always)]
    pub fn cmpdat(&self) -> CMPDAT_R {
        CMPDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPWM Comparator Register CMPDAT use to compare with CNTR to generate BPWM waveform, interrupt and trigger ADC. In independent mode, CMPDAT0~5 denote as 6 independent BPWM_CH0~5 compared point."]
    #[inline(always)]
    pub fn cmpdat(&mut self) -> CMPDAT_W {
        CMPDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Comparator Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_cmpdat1](index.html) module"]
pub struct BPWM_CMPDAT1_SPEC;
impl crate::RegisterSpec for BPWM_CMPDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_cmpdat1::R](R) reader structure"]
impl crate::Readable for BPWM_CMPDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_cmpdat1::W](W) writer structure"]
impl crate::Writable for BPWM_CMPDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CMPDAT1 to value 0"]
impl crate::Resettable for BPWM_CMPDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
