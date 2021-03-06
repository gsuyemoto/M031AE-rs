#[doc = "Register `PWM_RCAPDAT4` reader"]
pub struct R(crate::R<PWM_RCAPDAT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_RCAPDAT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_RCAPDAT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_RCAPDAT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCAPDAT` reader - PWM Rising Capture Data Register (Read Only)\nWhen rising capture condition happened, the PWM counter value will be saved in this register."]
pub struct RCAPDAT_R(crate::FieldReader<u16, u16>);
impl RCAPDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RCAPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCAPDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Rising Capture Data Register (Read Only) When rising capture condition happened, the PWM counter value will be saved in this register."]
    #[inline(always)]
    pub fn rcapdat(&self) -> RCAPDAT_R {
        RCAPDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM Rising Capture Data Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_rcapdat4](index.html) module"]
pub struct PWM_RCAPDAT4_SPEC;
impl crate::RegisterSpec for PWM_RCAPDAT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_rcapdat4::R](R) reader structure"]
impl crate::Readable for PWM_RCAPDAT4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_RCAPDAT4 to value 0"]
impl crate::Resettable for PWM_RCAPDAT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
