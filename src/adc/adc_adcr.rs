#[doc = "Register `ADC_ADCR` reader"]
pub struct R(crate::R<ADC_ADCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADCR` writer"]
pub struct W(crate::W<ADC_ADCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADCR_SPEC>;
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
impl From<crate::W<ADC_ADCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Enable Bit\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit to save power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_A {
    #[doc = "0: A/D converter Disabled"]
    _0 = 0,
    #[doc = "1: A/D converter Enabled"]
    _1 = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - A/D Converter Enable Bit\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit to save power consumption."]
pub struct ADEN_R(crate::FieldReader<bool, ADEN_A>);
impl ADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEN_A {
        match self.bits {
            false => ADEN_A::_0,
            true => ADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADEN_A::_1
    }
}
impl core::ops::Deref for ADEN_R {
    type Target = crate::FieldReader<bool, ADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEN` writer - A/D Converter Enable Bit\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit to save power consumption."]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A/D converter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEN_A::_0)
    }
    #[doc = "A/D converter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "A/D Interrupt Enable Bit\nA/D conversion end interrupt request is generated if ADIE bit is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIE_A {
    #[doc = "0: A/D interrupt function Disabled"]
    _0 = 0,
    #[doc = "1: A/D interrupt function Enabled"]
    _1 = 1,
}
impl From<ADIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIE` reader - A/D Interrupt Enable Bit\nA/D conversion end interrupt request is generated if ADIE bit is set to 1."]
pub struct ADIE_R(crate::FieldReader<bool, ADIE_A>);
impl ADIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIE_A {
        match self.bits {
            false => ADIE_A::_0,
            true => ADIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADIE_A::_1
    }
}
impl core::ops::Deref for ADIE_R {
    type Target = crate::FieldReader<bool, ADIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIE` writer - A/D Interrupt Enable Bit\nA/D conversion end interrupt request is generated if ADIE bit is set to 1."]
pub struct ADIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A/D interrupt function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIE_A::_0)
    }
    #[doc = "A/D interrupt function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIE_A::_1)
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
#[doc = "A/D Converter Operation Mode Control\nNote 1: When changing the operation mode, software should clear ADST bit first.\nNote 2: In Burst mode, the A/D result data is always at ADC Data Register 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADMD_A {
    #[doc = "0: Single conversion"]
    _0 = 0,
    #[doc = "1: Burst conversion"]
    _1 = 1,
    #[doc = "2: Single-cycle Scan"]
    _2 = 2,
    #[doc = "3: Continuous Scan"]
    _3 = 3,
}
impl From<ADMD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADMD` reader - A/D Converter Operation Mode Control\nNote 1: When changing the operation mode, software should clear ADST bit first.\nNote 2: In Burst mode, the A/D result data is always at ADC Data Register 0."]
pub struct ADMD_R(crate::FieldReader<u8, ADMD_A>);
impl ADMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMD_A {
        match self.bits {
            0 => ADMD_A::_0,
            1 => ADMD_A::_1,
            2 => ADMD_A::_2,
            3 => ADMD_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADMD_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ADMD_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ADMD_A::_3
    }
}
impl core::ops::Deref for ADMD_R {
    type Target = crate::FieldReader<u8, ADMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMD` writer - A/D Converter Operation Mode Control\nNote 1: When changing the operation mode, software should clear ADST bit first.\nNote 2: In Burst mode, the A/D result data is always at ADC Data Register 0."]
pub struct ADMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADMD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single conversion"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADMD_A::_0)
    }
    #[doc = "Burst conversion"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADMD_A::_1)
    }
    #[doc = "Single-cycle Scan"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ADMD_A::_2)
    }
    #[doc = "Continuous Scan"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ADMD_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Hardware Trigger Source\nNote: Software should clear TRGEN bit and ADST bit to 0 before changing TRGS bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGS_A {
    #[doc = "0: A/D conversion is started by external STADC pin"]
    _0 = 0,
    #[doc = "1: Timer0 ~ Timer3 overflow pulse trigger"]
    _1 = 1,
    #[doc = "2: A/D conversion is started by BPWM trigger"]
    _2 = 2,
    #[doc = "3: A/D conversion is started by PWM trigger"]
    _3 = 3,
}
impl From<TRGS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGS` reader - Hardware Trigger Source\nNote: Software should clear TRGEN bit and ADST bit to 0 before changing TRGS bits."]
pub struct TRGS_R(crate::FieldReader<u8, TRGS_A>);
impl TRGS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGS_A {
        match self.bits {
            0 => TRGS_A::_0,
            1 => TRGS_A::_1,
            2 => TRGS_A::_2,
            3 => TRGS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGS_A::_3
    }
}
impl core::ops::Deref for TRGS_R {
    type Target = crate::FieldReader<u8, TRGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGS` writer - Hardware Trigger Source\nNote: Software should clear TRGEN bit and ADST bit to 0 before changing TRGS bits."]
pub struct TRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "A/D conversion is started by external STADC pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGS_A::_0)
    }
    #[doc = "Timer0 ~ Timer3 overflow pulse trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGS_A::_1)
    }
    #[doc = "A/D conversion is started by BPWM trigger"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGS_A::_2)
    }
    #[doc = "A/D conversion is started by PWM trigger"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGS_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "External Trigger Condition\nThese two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and at least 4 PCLKs for edge trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGCOND_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
    #[doc = "2: Falling edge"]
    _2 = 2,
    #[doc = "3: Rising edge"]
    _3 = 3,
}
impl From<TRGCOND_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGCOND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGCOND` reader - External Trigger Condition\nThese two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and at least 4 PCLKs for edge trigger."]
pub struct TRGCOND_R(crate::FieldReader<u8, TRGCOND_A>);
impl TRGCOND_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGCOND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGCOND_A {
        match self.bits {
            0 => TRGCOND_A::_0,
            1 => TRGCOND_A::_1,
            2 => TRGCOND_A::_2,
            3 => TRGCOND_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGCOND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGCOND_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGCOND_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGCOND_A::_3
    }
}
impl core::ops::Deref for TRGCOND_R {
    type Target = crate::FieldReader<u8, TRGCOND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGCOND` writer - External Trigger Condition\nThese two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and at least 4 PCLKs for edge trigger."]
pub struct TRGCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGCOND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGCOND_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGCOND_A::_0)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGCOND_A::_1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGCOND_A::_2)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGCOND_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "External Trigger Enable Bit\nEnable or disable triggering of A/D conversion by external STADC pin, PWM trigger, BPWM trigger and Timer trigger. If external trigger is enabled, the ADST bit can be set to 1 by the selected hardware trigger source.\nNote: The ADC external trigger function is only supported in Single-cycle Scan mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger Disabled"]
    _0 = 0,
    #[doc = "1: External trigger Enabled"]
    _1 = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN` reader - External Trigger Enable Bit\nEnable or disable triggering of A/D conversion by external STADC pin, PWM trigger, BPWM trigger and Timer trigger. If external trigger is enabled, the ADST bit can be set to 1 by the selected hardware trigger source.\nNote: The ADC external trigger function is only supported in Single-cycle Scan mode."]
pub struct TRGEN_R(crate::FieldReader<bool, TRGEN_A>);
impl TRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::_0,
            true => TRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN_A::_1
    }
}
impl core::ops::Deref for TRGEN_R {
    type Target = crate::FieldReader<bool, TRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN` writer - External Trigger Enable Bit\nEnable or disable triggering of A/D conversion by external STADC pin, PWM trigger, BPWM trigger and Timer trigger. If external trigger is enabled, the ADST bit can be set to 1 by the selected hardware trigger source.\nNote: The ADC external trigger function is only supported in Single-cycle Scan mode."]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN_A::_0)
    }
    #[doc = "External trigger Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "PDMA Transfer Enable Bit\nWhen A/D conversion is completed, the converted data is loaded into ADDR0~15, ADDR29. Software can enable this bit to generate a PDMA data transfer request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEN_A {
    #[doc = "0: PDMA data transfer Disabled"]
    _0 = 0,
    #[doc = "1: PDMA data transfer in ADDR0~15, ADDR29 Enabled"]
    _1 = 1,
}
impl From<PTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEN` reader - PDMA Transfer Enable Bit\nWhen A/D conversion is completed, the converted data is loaded into ADDR0~15, ADDR29. Software can enable this bit to generate a PDMA data transfer request."]
pub struct PTEN_R(crate::FieldReader<bool, PTEN_A>);
impl PTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEN_A {
        match self.bits {
            false => PTEN_A::_0,
            true => PTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEN_A::_1
    }
}
impl core::ops::Deref for PTEN_R {
    type Target = crate::FieldReader<bool, PTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEN` writer - PDMA Transfer Enable Bit\nWhen A/D conversion is completed, the converted data is loaded into ADDR0~15, ADDR29. Software can enable this bit to generate a PDMA data transfer request."]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA data transfer Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEN_A::_0)
    }
    #[doc = "PDMA data transfer in ADDR0~15, ADDR29 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Differential Input Mode Control\nNote: In Differential Input mode, only the even number of the two corresponding channels needs to be enabled in ADCHER register. The conversion result will be placed to the corresponding data register of the enabled channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFEN_A {
    #[doc = "0: Single-end analog input mode"]
    _0 = 0,
    #[doc = "1: Differential analog input mode"]
    _1 = 1,
}
impl From<DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFEN` reader - Differential Input Mode Control\nNote: In Differential Input mode, only the even number of the two corresponding channels needs to be enabled in ADCHER register. The conversion result will be placed to the corresponding data register of the enabled channel."]
pub struct DIFFEN_R(crate::FieldReader<bool, DIFFEN_A>);
impl DIFFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFEN_A {
        match self.bits {
            false => DIFFEN_A::_0,
            true => DIFFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIFFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIFFEN_A::_1
    }
}
impl core::ops::Deref for DIFFEN_R {
    type Target = crate::FieldReader<bool, DIFFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFEN` writer - Differential Input Mode Control\nNote: In Differential Input mode, only the even number of the two corresponding channels needs to be enabled in ADCHER register. The conversion result will be placed to the corresponding data register of the enabled channel."]
pub struct DIFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single-end analog input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFFEN_A::_0)
    }
    #[doc = "Differential analog input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "A/D Conversion Start or Calibration Start\nADST bit can be set to 1 from four sources: software, external pin STADC, PWM trigger and Timer trigger. ADST bit will be cleared to 0 by hardware automatically at the ends of Single mode, Single-cycle Scan mode and Calibration mode. In Continuous Scan mode and Burst mode, A/D conversion is continuously performed until software writes 0 to this bit or chip is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADST_A {
    #[doc = "0: Conversion stops and A/D converter enters idle state"]
    _0 = 0,
    #[doc = "1: Conversion starts or Calibration Start"]
    _1 = 1,
}
impl From<ADST_A> for bool {
    #[inline(always)]
    fn from(variant: ADST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADST` reader - A/D Conversion Start or Calibration Start\nADST bit can be set to 1 from four sources: software, external pin STADC, PWM trigger and Timer trigger. ADST bit will be cleared to 0 by hardware automatically at the ends of Single mode, Single-cycle Scan mode and Calibration mode. In Continuous Scan mode and Burst mode, A/D conversion is continuously performed until software writes 0 to this bit or chip is reset."]
pub struct ADST_R(crate::FieldReader<bool, ADST_A>);
impl ADST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADST_A {
        match self.bits {
            false => ADST_A::_0,
            true => ADST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADST_A::_1
    }
}
impl core::ops::Deref for ADST_R {
    type Target = crate::FieldReader<bool, ADST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADST` writer - A/D Conversion Start or Calibration Start\nADST bit can be set to 1 from four sources: software, external pin STADC, PWM trigger and Timer trigger. ADST bit will be cleared to 0 by hardware automatically at the ends of Single mode, Single-cycle Scan mode and Calibration mode. In Continuous Scan mode and Burst mode, A/D conversion is continuously performed until software writes 0 to this bit or chip is reset."]
pub struct ADST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion stops and A/D converter enters idle state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADST_A::_0)
    }
    #[doc = "Conversion starts or Calibration Start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RESET` reader - ADC RESET (Write Protect)\nIf user writes this bit, the ADC analog macro will reset. Calibration data in macro will be deleted, but registers in ADC controller will keep.\nNote: This bit is cleared by hardware."]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - ADC RESET (Write Protect)\nIf user writes this bit, the ADC analog macro will reset. Calibration data in macro will be deleted, but registers in ADC controller will keep.\nNote: This bit is cleared by hardware."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Differential Input Mode Output Format\nIf user enables differential input mode, the conversion result can be expressed with binary straight format (unsigned format) or 2's complement format (signed format).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMOF_A {
    #[doc = "0: A/D Conversion result will be filled in RSLT at ADDRx registers with unsigned format (straight binary format)"]
    _0 = 0,
    #[doc = "1: A/D Conversion result will be filled in RSLT at ADDRx registers with 2's complement format"]
    _1 = 1,
}
impl From<DMOF_A> for bool {
    #[inline(always)]
    fn from(variant: DMOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMOF` reader - Differential Input Mode Output Format\nIf user enables differential input mode, the conversion result can be expressed with binary straight format (unsigned format) or 2's complement format (signed format)."]
pub struct DMOF_R(crate::FieldReader<bool, DMOF_A>);
impl DMOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMOF_A {
        match self.bits {
            false => DMOF_A::_0,
            true => DMOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMOF_A::_1
    }
}
impl core::ops::Deref for DMOF_R {
    type Target = crate::FieldReader<bool, DMOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMOF` writer - Differential Input Mode Output Format\nIf user enables differential input mode, the conversion result can be expressed with binary straight format (unsigned format) or 2's complement format (signed format)."]
pub struct DMOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A/D Conversion result will be filled in RSLT at ADDRx registers with unsigned format (straight binary format)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMOF_A::_0)
    }
    #[doc = "A/D Conversion result will be filled in RSLT at ADDRx registers with 2's complement format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Enable Bit Note: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit to save power consumption."]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - A/D Interrupt Enable Bit A/D conversion end interrupt request is generated if ADIE bit is set to 1."]
    #[inline(always)]
    pub fn adie(&self) -> ADIE_R {
        ADIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - A/D Converter Operation Mode Control Note 1: When changing the operation mode, software should clear ADST bit first. Note 2: In Burst mode, the A/D result data is always at ADC Data Register 0."]
    #[inline(always)]
    pub fn admd(&self) -> ADMD_R {
        ADMD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Hardware Trigger Source Note: Software should clear TRGEN bit and ADST bit to 0 before changing TRGS bits."]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Trigger Condition These two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and at least 4 PCLKs for edge trigger."]
    #[inline(always)]
    pub fn trgcond(&self) -> TRGCOND_R {
        TRGCOND_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Trigger Enable Bit Enable or disable triggering of A/D conversion by external STADC pin, PWM trigger, BPWM trigger and Timer trigger. If external trigger is enabled, the ADST bit can be set to 1 by the selected hardware trigger source. Note: The ADC external trigger function is only supported in Single-cycle Scan mode."]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PDMA Transfer Enable Bit When A/D conversion is completed, the converted data is loaded into ADDR0~15, ADDR29. Software can enable this bit to generate a PDMA data transfer request."]
    #[inline(always)]
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Differential Input Mode Control Note: In Differential Input mode, only the even number of the two corresponding channels needs to be enabled in ADCHER register. The conversion result will be placed to the corresponding data register of the enabled channel."]
    #[inline(always)]
    pub fn diffen(&self) -> DIFFEN_R {
        DIFFEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - A/D Conversion Start or Calibration Start ADST bit can be set to 1 from four sources: software, external pin STADC, PWM trigger and Timer trigger. ADST bit will be cleared to 0 by hardware automatically at the ends of Single mode, Single-cycle Scan mode and Calibration mode. In Continuous Scan mode and Burst mode, A/D conversion is continuously performed until software writes 0 to this bit or chip is reset."]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC RESET (Write Protect) If user writes this bit, the ADC analog macro will reset. Calibration data in macro will be deleted, but registers in ADC controller will keep. Note: This bit is cleared by hardware."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Differential Input Mode Output Format If user enables differential input mode, the conversion result can be expressed with binary straight format (unsigned format) or 2's complement format (signed format)."]
    #[inline(always)]
    pub fn dmof(&self) -> DMOF_R {
        DMOF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Enable Bit Note: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit to save power consumption."]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Bit 1 - A/D Interrupt Enable Bit A/D conversion end interrupt request is generated if ADIE bit is set to 1."]
    #[inline(always)]
    pub fn adie(&mut self) -> ADIE_W {
        ADIE_W { w: self }
    }
    #[doc = "Bits 2:3 - A/D Converter Operation Mode Control Note 1: When changing the operation mode, software should clear ADST bit first. Note 2: In Burst mode, the A/D result data is always at ADC Data Register 0."]
    #[inline(always)]
    pub fn admd(&mut self) -> ADMD_W {
        ADMD_W { w: self }
    }
    #[doc = "Bits 4:5 - Hardware Trigger Source Note: Software should clear TRGEN bit and ADST bit to 0 before changing TRGS bits."]
    #[inline(always)]
    pub fn trgs(&mut self) -> TRGS_W {
        TRGS_W { w: self }
    }
    #[doc = "Bits 6:7 - External Trigger Condition These two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and at least 4 PCLKs for edge trigger."]
    #[inline(always)]
    pub fn trgcond(&mut self) -> TRGCOND_W {
        TRGCOND_W { w: self }
    }
    #[doc = "Bit 8 - External Trigger Enable Bit Enable or disable triggering of A/D conversion by external STADC pin, PWM trigger, BPWM trigger and Timer trigger. If external trigger is enabled, the ADST bit can be set to 1 by the selected hardware trigger source. Note: The ADC external trigger function is only supported in Single-cycle Scan mode."]
    #[inline(always)]
    pub fn trgen(&mut self) -> TRGEN_W {
        TRGEN_W { w: self }
    }
    #[doc = "Bit 9 - PDMA Transfer Enable Bit When A/D conversion is completed, the converted data is loaded into ADDR0~15, ADDR29. Software can enable this bit to generate a PDMA data transfer request."]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
    #[doc = "Bit 10 - Differential Input Mode Control Note: In Differential Input mode, only the even number of the two corresponding channels needs to be enabled in ADCHER register. The conversion result will be placed to the corresponding data register of the enabled channel."]
    #[inline(always)]
    pub fn diffen(&mut self) -> DIFFEN_W {
        DIFFEN_W { w: self }
    }
    #[doc = "Bit 11 - A/D Conversion Start or Calibration Start ADST bit can be set to 1 from four sources: software, external pin STADC, PWM trigger and Timer trigger. ADST bit will be cleared to 0 by hardware automatically at the ends of Single mode, Single-cycle Scan mode and Calibration mode. In Continuous Scan mode and Burst mode, A/D conversion is continuously performed until software writes 0 to this bit or chip is reset."]
    #[inline(always)]
    pub fn adst(&mut self) -> ADST_W {
        ADST_W { w: self }
    }
    #[doc = "Bit 12 - ADC RESET (Write Protect) If user writes this bit, the ADC analog macro will reset. Calibration data in macro will be deleted, but registers in ADC controller will keep. Note: This bit is cleared by hardware."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 31 - Differential Input Mode Output Format If user enables differential input mode, the conversion result can be expressed with binary straight format (unsigned format) or 2's complement format (signed format)."]
    #[inline(always)]
    pub fn dmof(&mut self) -> DMOF_W {
        DMOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adcr](index.html) module"]
pub struct ADC_ADCR_SPEC;
impl crate::RegisterSpec for ADC_ADCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adcr::R](R) reader structure"]
impl crate::Readable for ADC_ADCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adcr::W](W) writer structure"]
impl crate::Writable for ADC_ADCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADCR to value 0"]
impl crate::Resettable for ADC_ADCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
