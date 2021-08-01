#[doc = "Register `BPWM_PBUF` reader"]
pub struct R(crate::R<BPWM_PBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_PBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_PBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_PBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PBUF` reader - BPWM Period Buffer (Read Only)\nUsed as PERIOD active register."]
pub struct PBUF_R(crate::FieldReader<u16, u16>);
impl PBUF_R {
    pub(crate) fn new(bits: u16) -> Self {
        PBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - BPWM Period Buffer (Read Only) Used as PERIOD active register."]
    #[inline(always)]
    pub fn pbuf(&self) -> PBUF_R {
        PBUF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BPWM PERIOD Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_pbuf](index.html) module"]
pub struct BPWM_PBUF_SPEC;
impl crate::RegisterSpec for BPWM_PBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_pbuf::R](R) reader structure"]
impl crate::Readable for BPWM_PBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BPWM_PBUF to value 0"]
impl crate::Resettable for BPWM_PBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
