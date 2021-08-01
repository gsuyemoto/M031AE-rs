#[doc = "Register `PDMA_DSCT4_DA` reader"]
pub struct R(crate::R<PDMA_DSCT4_DA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_DSCT4_DA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_DSCT4_DA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_DSCT4_DA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_DSCT4_DA` writer"]
pub struct W(crate::W<PDMA_DSCT4_DA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_DSCT4_DA_SPEC>;
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
impl From<crate::W<PDMA_DSCT4_DA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_DSCT4_DA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA` reader - PDMA Transfer Destination Address\nThis field indicates a 32-bit destination address of PDMA controller."]
pub struct DA_R(crate::FieldReader<u32, u32>);
impl DA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - PDMA Transfer Destination Address\nThis field indicates a 32-bit destination address of PDMA controller."]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PDMA Transfer Destination Address This field indicates a 32-bit destination address of PDMA controller."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDMA Transfer Destination Address This field indicates a 32-bit destination address of PDMA controller."]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Address Register of PDMA Channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_dsct4_da](index.html) module"]
pub struct PDMA_DSCT4_DA_SPEC;
impl crate::RegisterSpec for PDMA_DSCT4_DA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_dsct4_da::R](R) reader structure"]
impl crate::Readable for PDMA_DSCT4_DA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_dsct4_da::W](W) writer structure"]
impl crate::Writable for PDMA_DSCT4_DA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_DSCT4_DA to value 0"]
impl crate::Resettable for PDMA_DSCT4_DA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
