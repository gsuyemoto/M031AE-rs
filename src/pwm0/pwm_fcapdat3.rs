#[doc = "Register `PWM_FCAPDAT3` reader"]
pub struct R(crate::R<PWM_FCAPDAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FCAPDAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FCAPDAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FCAPDAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FCAPDAT` reader - PWM Falling Capture Data Register (Read Only)\nWhen falling capture condition happened, the PWM counter value will be saved in this register."]
pub struct FCAPDAT_R(crate::FieldReader<u16, u16>);
impl FCAPDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        FCAPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAPDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Falling Capture Data Register (Read Only) When falling capture condition happened, the PWM counter value will be saved in this register."]
    #[inline(always)]
    pub fn fcapdat(&self) -> FCAPDAT_R {
        FCAPDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM Falling Capture Data Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fcapdat3](index.html) module"]
pub struct PWM_FCAPDAT3_SPEC;
impl crate::RegisterSpec for PWM_FCAPDAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fcapdat3::R](R) reader structure"]
impl crate::Readable for PWM_FCAPDAT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_FCAPDAT3 to value 0"]
impl crate::Resettable for PWM_FCAPDAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
