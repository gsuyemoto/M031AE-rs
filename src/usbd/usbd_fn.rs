#[doc = "Register `USBD_FN` reader"]
pub struct R(crate::R<USBD_FN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_FN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_FN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_FN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FN` reader - Frame Number\nThese bits contain the 11-bits frame number in the last received SOF packet."]
pub struct FN_R(crate::FieldReader<u16, u16>);
impl FN_R {
    pub(crate) fn new(bits: u16) -> Self {
        FN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Number These bits contain the 11-bits frame number in the last received SOF packet."]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "USB Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_fn](index.html) module"]
pub struct USBD_FN_SPEC;
impl crate::RegisterSpec for USBD_FN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_fn::R](R) reader structure"]
impl crate::Readable for USBD_FN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBD_FN to value 0"]
impl crate::Resettable for USBD_FN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
