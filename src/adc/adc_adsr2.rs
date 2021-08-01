#[doc = "Register `ADC_ADSR2` reader"]
pub struct R(crate::R<ADC_ADSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVERRUN` reader - Overrun Flag (Read Only)\nOVERRUN\\[29, 15:0\\]
are the mirror of the OVERRUN bit in ADDR29\\[16\\], ADDR15\\[16\\]
~ ADDR0\\[16\\]. The other bits are reserved. \nNote: When ADC is in burst mode and the FIFO is overrun, OVERRUN\\[29, 15:0\\]
will be set to 1."]
pub struct OVERRUN_R(crate::FieldReader<u32, u32>);
impl OVERRUN_R {
    pub(crate) fn new(bits: u32) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Overrun Flag (Read Only) OVERRUN\\[29, 15:0\\]
are the mirror of the OVERRUN bit in ADDR29\\[16\\], ADDR15\\[16\\]
~ ADDR0\\[16\\]. The other bits are reserved. Note: When ADC is in burst mode and the FIFO is overrun, OVERRUN\\[29, 15:0\\]
will be set to 1."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ADC Status Register2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adsr2](index.html) module"]
pub struct ADC_ADSR2_SPEC;
impl crate::RegisterSpec for ADC_ADSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adsr2::R](R) reader structure"]
impl crate::Readable for ADC_ADSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_ADSR2 to value 0"]
impl crate::Resettable for ADC_ADSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
