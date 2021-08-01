#[doc = "Register `BPWM_RCAPDAT3` reader"]
pub struct R(crate::R<BPWM_RCAPDAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_RCAPDAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_RCAPDAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_RCAPDAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCAPDAT` reader - BPWM Rising Capture Data (Read Only)\nWhen rising capture condition happened, the BPWM counter value will be saved in this register."]
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
    #[doc = "Bits 0:15 - BPWM Rising Capture Data (Read Only) When rising capture condition happened, the BPWM counter value will be saved in this register."]
    #[inline(always)]
    pub fn rcapdat(&self) -> RCAPDAT_R {
        RCAPDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BPWM Rising Capture Data Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_rcapdat3](index.html) module"]
pub struct BPWM_RCAPDAT3_SPEC;
impl crate::RegisterSpec for BPWM_RCAPDAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_rcapdat3::R](R) reader structure"]
impl crate::Readable for BPWM_RCAPDAT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BPWM_RCAPDAT3 to value 0"]
impl crate::Resettable for BPWM_RCAPDAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
