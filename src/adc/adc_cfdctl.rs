#[doc = "Register `ADC_CFDCTL` reader"]
pub struct R(crate::R<ADC_CFDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFDCTL` writer"]
pub struct W(crate::W<ADC_CFDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFDCTL_SPEC>;
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
impl From<crate::W<ADC_CFDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Precharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECHEN_A {
    #[doc = "0: Channel precharge Disabled"]
    _0 = 0,
    #[doc = "1: Channel precharge Enabled"]
    _1 = 1,
}
impl From<PRECHEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRECHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRECHEN` reader - Precharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
pub struct PRECHEN_R(crate::FieldReader<bool, PRECHEN_A>);
impl PRECHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRECHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRECHEN_A {
        match self.bits {
            false => PRECHEN_A::_0,
            true => PRECHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRECHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRECHEN_A::_1
    }
}
impl core::ops::Deref for PRECHEN_R {
    type Target = crate::FieldReader<bool, PRECHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRECHEN` writer - Precharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
pub struct PRECHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRECHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel precharge Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRECHEN_A::_0)
    }
    #[doc = "Channel precharge Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRECHEN_A::_1)
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
#[doc = "Discharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCHEN_A {
    #[doc = "0: Channel discharge Disabled"]
    _0 = 0,
    #[doc = "1: Channel discharge Enabled"]
    _1 = 1,
}
impl From<DISCHEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCHEN` reader - Discharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
pub struct DISCHEN_R(crate::FieldReader<bool, DISCHEN_A>);
impl DISCHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCHEN_A {
        match self.bits {
            false => DISCHEN_A::_0,
            true => DISCHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DISCHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DISCHEN_A::_1
    }
}
impl core::ops::Deref for DISCHEN_R {
    type Target = crate::FieldReader<bool, DISCHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCHEN` writer - Discharge Enable\nNote: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
pub struct DISCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel discharge Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DISCHEN_A::_0)
    }
    #[doc = "Channel discharge Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DISCHEN_A::_1)
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
#[doc = "Floating Detect Channel Enable Bit\nNote: if FDETCHEN is enabled, internal channel is always turn on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDETCHEN_A {
    #[doc = "0: Floating Detect Channel Disabled"]
    _0 = 0,
    #[doc = "1: Floating Detect Channel Enabled"]
    _1 = 1,
}
impl From<FDETCHEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDETCHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDETCHEN` reader - Floating Detect Channel Enable Bit\nNote: if FDETCHEN is enabled, internal channel is always turn on."]
pub struct FDETCHEN_R(crate::FieldReader<bool, FDETCHEN_A>);
impl FDETCHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDETCHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDETCHEN_A {
        match self.bits {
            false => FDETCHEN_A::_0,
            true => FDETCHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FDETCHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FDETCHEN_A::_1
    }
}
impl core::ops::Deref for FDETCHEN_R {
    type Target = crate::FieldReader<bool, FDETCHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDETCHEN` writer - Floating Detect Channel Enable Bit\nNote: if FDETCHEN is enabled, internal channel is always turn on."]
pub struct FDETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDETCHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDETCHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Floating Detect Channel Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDETCHEN_A::_0)
    }
    #[doc = "Floating Detect Channel Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDETCHEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Precharge Enable Note: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
    #[inline(always)]
    pub fn prechen(&self) -> PRECHEN_R {
        PRECHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Discharge Enable Note: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
    #[inline(always)]
    pub fn dischen(&self) -> DISCHEN_R {
        DISCHEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Floating Detect Channel Enable Bit Note: if FDETCHEN is enabled, internal channel is always turn on."]
    #[inline(always)]
    pub fn fdetchen(&self) -> FDETCHEN_R {
        FDETCHEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Precharge Enable Note: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
    #[inline(always)]
    pub fn prechen(&mut self) -> PRECHEN_W {
        PRECHEN_W { w: self }
    }
    #[doc = "Bit 1 - Discharge Enable Note: Analog input voltage is 1/2 VREF when PRECHEN and DISCHEN are all enable."]
    #[inline(always)]
    pub fn dischen(&mut self) -> DISCHEN_W {
        DISCHEN_W { w: self }
    }
    #[doc = "Bit 8 - Floating Detect Channel Enable Bit Note: if FDETCHEN is enabled, internal channel is always turn on."]
    #[inline(always)]
    pub fn fdetchen(&mut self) -> FDETCHEN_W {
        FDETCHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Channel Floating Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfdctl](index.html) module"]
pub struct ADC_CFDCTL_SPEC;
impl crate::RegisterSpec for ADC_CFDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfdctl::R](R) reader structure"]
impl crate::Readable for ADC_CFDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfdctl::W](W) writer structure"]
impl crate::Writable for ADC_CFDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CFDCTL to value 0"]
impl crate::Resettable for ADC_CFDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
