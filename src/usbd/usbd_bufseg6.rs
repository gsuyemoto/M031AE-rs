#[doc = "Register `USBD_BUFSEG6` reader"]
pub struct R(crate::R<USBD_BUFSEG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_BUFSEG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_BUFSEG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_BUFSEG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_BUFSEG6` writer"]
pub struct W(crate::W<USBD_BUFSEG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_BUFSEG6_SPEC>;
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
impl From<crate::W<USBD_BUFSEG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBD_BUFSEG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFSEG` reader - Endpoint Buffer Segmentation\nIt is used to indicate the offset address for each endpoint with the USB SRAM starting address The effective starting address of the endpoint is\nUSBD_SRAM address + { BUFSEG, 3'b000}\nRefer to the section 6.22.5.7 for the endpoint SRAM structure and its description."]
pub struct BUFSEG_R(crate::FieldReader<u8, u8>);
impl BUFSEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUFSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFSEG` writer - Endpoint Buffer Segmentation\nIt is used to indicate the offset address for each endpoint with the USB SRAM starting address The effective starting address of the endpoint is\nUSBD_SRAM address + { BUFSEG, 3'b000}\nRefer to the section 6.22.5.7 for the endpoint SRAM structure and its description."]
pub struct BUFSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | ((value as u32 & 0x3f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - Endpoint Buffer Segmentation It is used to indicate the offset address for each endpoint with the USB SRAM starting address The effective starting address of the endpoint is USBD_SRAM address + { BUFSEG, 3'b000} Refer to the section 6.22.5.7 for the endpoint SRAM structure and its description."]
    #[inline(always)]
    pub fn bufseg(&self) -> BUFSEG_R {
        BUFSEG_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:8 - Endpoint Buffer Segmentation It is used to indicate the offset address for each endpoint with the USB SRAM starting address The effective starting address of the endpoint is USBD_SRAM address + { BUFSEG, 3'b000} Refer to the section 6.22.5.7 for the endpoint SRAM structure and its description."]
    #[inline(always)]
    pub fn bufseg(&mut self) -> BUFSEG_W {
        BUFSEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint 6 Buffer Segmentation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_bufseg6](index.html) module"]
pub struct USBD_BUFSEG6_SPEC;
impl crate::RegisterSpec for USBD_BUFSEG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_bufseg6::R](R) reader structure"]
impl crate::Readable for USBD_BUFSEG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_bufseg6::W](W) writer structure"]
impl crate::Writable for USBD_BUFSEG6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_BUFSEG6 to value 0"]
impl crate::Resettable for USBD_BUFSEG6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
