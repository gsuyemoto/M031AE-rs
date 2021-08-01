#[doc = "Register `ADC_ADCHER` reader"]
pub struct R(crate::R<ADC_ADCHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADCHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADCHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADCHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADCHER` writer"]
pub struct W(crate::W<ADC_ADCHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADCHER_SPEC>;
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
impl From<crate::W<ADC_ADCHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADCHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog Input Channel Enable Control\nSet ADCHER\\[15:0\\]
bits to enable the corresponding analog input channel 15 ~ 0. If DIFFEN bit is set to 1, only the even number channel needs to be enabled.\nBesides, setting the ADCHER\\[29\\]
bit will enable internal channel for band-gap voltage. Other bits are reserved.\nNote: If the internal channel for band-gap voltage (CHEN\\[29\\]) is active, the maximum sampling rate will be 300k SPS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHEN_A {
    #[doc = "0: Channel Disabled"]
    _0 = 0,
    #[doc = "1: Channel Enabled"]
    _1 = 1,
}
impl From<CHEN_A> for u32 {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHEN` reader - Analog Input Channel Enable Control\nSet ADCHER\\[15:0\\]
bits to enable the corresponding analog input channel 15 ~ 0. If DIFFEN bit is set to 1, only the even number channel needs to be enabled.\nBesides, setting the ADCHER\\[29\\]
bit will enable internal channel for band-gap voltage. Other bits are reserved.\nNote: If the internal channel for band-gap voltage (CHEN\\[29\\]) is active, the maximum sampling rate will be 300k SPS."]
pub struct CHEN_R(crate::FieldReader<u32, CHEN_A>);
impl CHEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHEN_A> {
        match self.bits {
            0 => Some(CHEN_A::_0),
            1 => Some(CHEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN_A::_1
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<u32, CHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - Analog Input Channel Enable Control\nSet ADCHER\\[15:0\\]
bits to enable the corresponding analog input channel 15 ~ 0. If DIFFEN bit is set to 1, only the even number channel needs to be enabled.\nBesides, setting the ADCHER\\[29\\]
bit will enable internal channel for band-gap voltage. Other bits are reserved.\nNote: If the internal channel for band-gap voltage (CHEN\\[29\\]) is active, the maximum sampling rate will be 300k SPS."]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN_A::_0)
    }
    #[doc = "Channel Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Analog Input Channel Enable Control Set ADCHER\\[15:0\\]
bits to enable the corresponding analog input channel 15 ~ 0. If DIFFEN bit is set to 1, only the even number channel needs to be enabled. Besides, setting the ADCHER\\[29\\]
bit will enable internal channel for band-gap voltage. Other bits are reserved. Note: If the internal channel for band-gap voltage (CHEN\\[29\\]) is active, the maximum sampling rate will be 300k SPS."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Analog Input Channel Enable Control Set ADCHER\\[15:0\\]
bits to enable the corresponding analog input channel 15 ~ 0. If DIFFEN bit is set to 1, only the even number channel needs to be enabled. Besides, setting the ADCHER\\[29\\]
bit will enable internal channel for band-gap voltage. Other bits are reserved. Note: If the internal channel for band-gap voltage (CHEN\\[29\\]) is active, the maximum sampling rate will be 300k SPS."]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adcher](index.html) module"]
pub struct ADC_ADCHER_SPEC;
impl crate::RegisterSpec for ADC_ADCHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adcher::R](R) reader structure"]
impl crate::Readable for ADC_ADCHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adcher::W](W) writer structure"]
impl crate::Writable for ADC_ADCHER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADCHER to value 0"]
impl crate::Resettable for ADC_ADCHER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
