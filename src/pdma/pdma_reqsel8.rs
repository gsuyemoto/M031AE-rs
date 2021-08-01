#[doc = "Register `PDMA_REQSEL8` reader"]
pub struct R(crate::R<PDMA_REQSEL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_REQSEL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_REQSEL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_REQSEL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_REQSEL8` writer"]
pub struct W(crate::W<PDMA_REQSEL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_REQSEL8_SPEC>;
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
impl From<crate::W<PDMA_REQSEL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_REQSEL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQSRC8` reader - Channel 8 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC8_R(crate::FieldReader<u8, u8>);
impl REQSRC8_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC8` writer - Channel 8 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC8_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Channel 8 Request Source Selection This filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc8(&self) -> REQSRC8_R {
        REQSRC8_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel 8 Request Source Selection This filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc8(&mut self) -> REQSRC8_W {
        REQSRC8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Request Source Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_reqsel8](index.html) module"]
pub struct PDMA_REQSEL8_SPEC;
impl crate::RegisterSpec for PDMA_REQSEL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_reqsel8::R](R) reader structure"]
impl crate::Readable for PDMA_REQSEL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_reqsel8::W](W) writer structure"]
impl crate::Writable for PDMA_REQSEL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_REQSEL8 to value 0"]
impl crate::Resettable for PDMA_REQSEL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
