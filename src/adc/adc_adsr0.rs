#[doc = "Register `ADC_ADSR0` reader"]
pub struct R(crate::R<ADC_ADSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADSR0` writer"]
pub struct W(crate::W<ADC_ADSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADSR0_SPEC>;
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
impl From<crate::W<ADC_ADSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADF` reader - A/D Conversion End Flag\nA status flag that indicates the end of A/D conversion. Software can write 1 to clear this bit.\nThe ADF bit is set to 1 at the following three conditions:\nWhen A/D conversion ends in Single mode.\nWhen A/D conversion ends on all specified channels in Single-cycle Scan mode and Continuous Scan mode.\nWhen more than or equal to 4 samples in FIFO in Burst mode."]
pub struct ADF_R(crate::FieldReader<bool, bool>);
impl ADF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADF` writer - A/D Conversion End Flag\nA status flag that indicates the end of A/D conversion. Software can write 1 to clear this bit.\nThe ADF bit is set to 1 at the following three conditions:\nWhen A/D conversion ends in Single mode.\nWhen A/D conversion ends on all specified channels in Single-cycle Scan mode and Continuous Scan mode.\nWhen more than or equal to 4 samples in FIFO in Burst mode."]
pub struct ADF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Compare Flag 0\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR0 register then this bit is set to 1. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPF0_A {
    #[doc = "0: Conversion result in ADDR does not meet ADCMPR0 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in ADDR meets ADCMPR0 setting"]
    _1 = 1,
}
impl From<CMPF0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPF0` reader - Compare Flag 0\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR0 register then this bit is set to 1. This bit is cleared by writing 1 to it."]
pub struct CMPF0_R(crate::FieldReader<bool, CMPF0_A>);
impl CMPF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPF0_A {
        match self.bits {
            false => CMPF0_A::_0,
            true => CMPF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPF0_A::_1
    }
}
impl core::ops::Deref for CMPF0_R {
    type Target = crate::FieldReader<bool, CMPF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPF0` writer - Compare Flag 0\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR0 register then this bit is set to 1. This bit is cleared by writing 1 to it."]
pub struct CMPF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in ADDR does not meet ADCMPR0 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPF0_A::_0)
    }
    #[doc = "Conversion result in ADDR meets ADCMPR0 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPF0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Compare Flag 1\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR1 register, this bit is set to 1; it is cleared by writing 1 to it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPF1_A {
    #[doc = "0: Conversion result in ADDR does not meet ADCMPR1 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in ADDR meets ADCMPR1 setting"]
    _1 = 1,
}
impl From<CMPF1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPF1` reader - Compare Flag 1\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR1 register, this bit is set to 1; it is cleared by writing 1 to it"]
pub struct CMPF1_R(crate::FieldReader<bool, CMPF1_A>);
impl CMPF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPF1_A {
        match self.bits {
            false => CMPF1_A::_0,
            true => CMPF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPF1_A::_1
    }
}
impl core::ops::Deref for CMPF1_R {
    type Target = crate::FieldReader<bool, CMPF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPF1` writer - Compare Flag 1\nWhen the A/D conversion result of the selected channel meets setting condition in ADCMPR1 register, this bit is set to 1; it is cleared by writing 1 to it"]
pub struct CMPF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in ADDR does not meet ADCMPR1 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPF1_A::_0)
    }
    #[doc = "Conversion result in ADDR meets ADCMPR1 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPF1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "BUSY/IDLE (Read Only)\nThis bit is a mirror of ADST bit in ADCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: A/D converter is in idle state"]
    _0 = 0,
    #[doc = "1: A/D converter is busy at conversion"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - BUSY/IDLE (Read Only)\nThis bit is a mirror of ADST bit in ADCR register."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALIDF` reader - Data Valid Flag (Read Only)\nIf any one of VALID (ADDRx\\[17\\]) is set, this flag will be set to 1.\nNote: When ADC is in burst mode and any conversion result is valid, this flag will be set to 1."]
pub struct VALIDF_R(crate::FieldReader<bool, bool>);
impl VALIDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALIDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALIDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUNF` reader - Overrun Flag (Read Only)\nIf any one of OVERRUN (ADDRx\\[16\\]) is set, this flag will be set to 1.\nNote: When ADC is in burst mode and the FIFO is overrun, this flag will be set to 1."]
pub struct OVERRUNF_R(crate::FieldReader<bool, bool>);
impl OVERRUNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL` reader - Current Conversion Channel (Read Only)"]
pub struct CHANNEL_R(crate::FieldReader<u8, u8>);
impl CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - A/D Conversion End Flag A status flag that indicates the end of A/D conversion. Software can write 1 to clear this bit. The ADF bit is set to 1 at the following three conditions: When A/D conversion ends in Single mode. When A/D conversion ends on all specified channels in Single-cycle Scan mode and Continuous Scan mode. When more than or equal to 4 samples in FIFO in Burst mode."]
    #[inline(always)]
    pub fn adf(&self) -> ADF_R {
        ADF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare Flag 0 When the A/D conversion result of the selected channel meets setting condition in ADCMPR0 register then this bit is set to 1. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cmpf0(&self) -> CMPF0_R {
        CMPF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Flag 1 When the A/D conversion result of the selected channel meets setting condition in ADCMPR1 register, this bit is set to 1; it is cleared by writing 1 to it"]
    #[inline(always)]
    pub fn cmpf1(&self) -> CMPF1_R {
        CMPF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BUSY/IDLE (Read Only) This bit is a mirror of ADST bit in ADCR register."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data Valid Flag (Read Only) If any one of VALID (ADDRx\\[17\\]) is set, this flag will be set to 1. Note: When ADC is in burst mode and any conversion result is valid, this flag will be set to 1."]
    #[inline(always)]
    pub fn validf(&self) -> VALIDF_R {
        VALIDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Overrun Flag (Read Only) If any one of OVERRUN (ADDRx\\[16\\]) is set, this flag will be set to 1. Note: When ADC is in burst mode and the FIFO is overrun, this flag will be set to 1."]
    #[inline(always)]
    pub fn overrunf(&self) -> OVERRUNF_R {
        OVERRUNF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - Current Conversion Channel (Read Only)"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion End Flag A status flag that indicates the end of A/D conversion. Software can write 1 to clear this bit. The ADF bit is set to 1 at the following three conditions: When A/D conversion ends in Single mode. When A/D conversion ends on all specified channels in Single-cycle Scan mode and Continuous Scan mode. When more than or equal to 4 samples in FIFO in Burst mode."]
    #[inline(always)]
    pub fn adf(&mut self) -> ADF_W {
        ADF_W { w: self }
    }
    #[doc = "Bit 1 - Compare Flag 0 When the A/D conversion result of the selected channel meets setting condition in ADCMPR0 register then this bit is set to 1. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cmpf0(&mut self) -> CMPF0_W {
        CMPF0_W { w: self }
    }
    #[doc = "Bit 2 - Compare Flag 1 When the A/D conversion result of the selected channel meets setting condition in ADCMPR1 register, this bit is set to 1; it is cleared by writing 1 to it"]
    #[inline(always)]
    pub fn cmpf1(&mut self) -> CMPF1_W {
        CMPF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Status Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adsr0](index.html) module"]
pub struct ADC_ADSR0_SPEC;
impl crate::RegisterSpec for ADC_ADSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adsr0::R](R) reader structure"]
impl crate::Readable for ADC_ADSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adsr0::W](W) writer structure"]
impl crate::Writable for ADC_ADSR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADSR0 to value 0"]
impl crate::Resettable for ADC_ADSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
