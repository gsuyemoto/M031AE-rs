#[doc = "Register `ADC_ADPDMA` reader"]
pub struct R(crate::R<ADC_ADPDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADPDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADPDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADPDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURDAT` reader - ADC PDMA Current Transfer Data Register (Read Only)\nWhen PDMA transferring, read this register can monitor current PDMA transfer data.\nCurrent PDMA transfer data could be the content of ADDR0 ~ ADDR15, and ADDR29 registers."]
pub struct CURDAT_R(crate::FieldReader<u32, u32>);
impl CURDAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURDAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - ADC PDMA Current Transfer Data Register (Read Only) When PDMA transferring, read this register can monitor current PDMA transfer data. Current PDMA transfer data could be the content of ADDR0 ~ ADDR15, and ADDR29 registers."]
    #[inline(always)]
    pub fn curdat(&self) -> CURDAT_R {
        CURDAT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "ADC PDMA Current Transfer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adpdma](index.html) module"]
pub struct ADC_ADPDMA_SPEC;
impl crate::RegisterSpec for ADC_ADPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adpdma::R](R) reader structure"]
impl crate::Readable for ADC_ADPDMA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_ADPDMA to value 0"]
impl crate::Resettable for ADC_ADPDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
