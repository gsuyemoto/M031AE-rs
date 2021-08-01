#[doc = "Register `ADC_ADSR1` reader"]
pub struct R(crate::R<ADC_ADSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - Data Valid Flag (Read Only)\nVALID\\[29, 15:0\\]
are the mirror of the VALID bits in ADDR29\\[17\\], ADDR15\\[17\\]~ ADDR0\\[17\\]. The other bits are reserved.\nNote: When ADC is in burst mode and any conversion result is valid, VALID\\[29, 15:0\\]
will be set to 1."]
pub struct VALID_R(crate::FieldReader<u32, u32>);
impl VALID_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Valid Flag (Read Only) VALID\\[29, 15:0\\]
are the mirror of the VALID bits in ADDR29\\[17\\], ADDR15\\[17\\]~ ADDR0\\[17\\]. The other bits are reserved. Note: When ADC is in burst mode and any conversion result is valid, VALID\\[29, 15:0\\]
will be set to 1."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ADC Status Register1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adsr1](index.html) module"]
pub struct ADC_ADSR1_SPEC;
impl crate::RegisterSpec for ADC_ADSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adsr1::R](R) reader structure"]
impl crate::Readable for ADC_ADSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_ADSR1 to value 0"]
impl crate::Resettable for ADC_ADSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
