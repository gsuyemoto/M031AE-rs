#[doc = "Register `ACMP_CALCTL` reader"]
pub struct R(crate::R<ACMP_CALCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP_CALCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMP_CALCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMP_CALCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMP_CALCTL` writer"]
pub struct W(crate::W<ACMP_CALCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMP_CALCTL_SPEC>;
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
impl From<crate::W<ACMP_CALCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMP_CALCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OP Amplifier 0 Calibration Trigger Bit\nNote 1: Before this bit is enabled,ACMPEN(ACMP_CTL0) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALTRG0_A {
    #[doc = "0: Calibration is stopped"]
    _0 = 0,
    #[doc = "1: Calibration is triggered"]
    _1 = 1,
}
impl From<CALTRG0_A> for bool {
    #[inline(always)]
    fn from(variant: CALTRG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALTRG0` reader - OP Amplifier 0 Calibration Trigger Bit\nNote 1: Before this bit is enabled,ACMPEN(ACMP_CTL0) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
pub struct CALTRG0_R(crate::FieldReader<bool, CALTRG0_A>);
impl CALTRG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALTRG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALTRG0_A {
        match self.bits {
            false => CALTRG0_A::_0,
            true => CALTRG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALTRG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALTRG0_A::_1
    }
}
impl core::ops::Deref for CALTRG0_R {
    type Target = crate::FieldReader<bool, CALTRG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALTRG0` writer - OP Amplifier 0 Calibration Trigger Bit\nNote 1: Before this bit is enabled,ACMPEN(ACMP_CTL0) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
pub struct CALTRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CALTRG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALTRG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALTRG0_A::_0)
    }
    #[doc = "Calibration is triggered"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALTRG0_A::_1)
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
#[doc = "OP Amplifier 1 Calibration Trigger Bit\nNote 1: Before this bit is enabled, ACMPEN(ACMP_CTL1) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software.\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALTRG1_A {
    #[doc = "0: Calibration is stopped"]
    _0 = 0,
    #[doc = "1: Calibration is triggered"]
    _1 = 1,
}
impl From<CALTRG1_A> for bool {
    #[inline(always)]
    fn from(variant: CALTRG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALTRG1` reader - OP Amplifier 1 Calibration Trigger Bit\nNote 1: Before this bit is enabled, ACMPEN(ACMP_CTL1) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software.\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
pub struct CALTRG1_R(crate::FieldReader<bool, CALTRG1_A>);
impl CALTRG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALTRG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALTRG1_A {
        match self.bits {
            false => CALTRG1_A::_0,
            true => CALTRG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALTRG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALTRG1_A::_1
    }
}
impl core::ops::Deref for CALTRG1_R {
    type Target = crate::FieldReader<bool, CALTRG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALTRG1` writer - OP Amplifier 1 Calibration Trigger Bit\nNote 1: Before this bit is enabled, ACMPEN(ACMP_CTL1) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance.\nNote 2: Hardware will auto clear this bit when next calibration is triggered by software.\nNote 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
pub struct CALTRG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CALTRG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALTRG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALTRG1_A::_0)
    }
    #[doc = "Calibration is triggered"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALTRG1_A::_1)
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
#[doc = "OPA0 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALRVS0_A {
    #[doc = "0: VREF is"]
    _0 = 0,
    #[doc = "1: VREF from high vcm to low vcm"]
    _1 = 1,
}
impl From<CALRVS0_A> for bool {
    #[inline(always)]
    fn from(variant: CALRVS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRVS0` reader - OPA0 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration"]
pub struct CALRVS0_R(crate::FieldReader<bool, CALRVS0_A>);
impl CALRVS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALRVS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALRVS0_A {
        match self.bits {
            false => CALRVS0_A::_0,
            true => CALRVS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALRVS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALRVS0_A::_1
    }
}
impl core::ops::Deref for CALRVS0_R {
    type Target = crate::FieldReader<bool, CALRVS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRVS0` writer - OPA0 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration"]
pub struct CALRVS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRVS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALRVS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREF is"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALRVS0_A::_0)
    }
    #[doc = "VREF from high vcm to low vcm"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALRVS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "OPA1 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALRVS1_A {
    #[doc = "0: VREF is"]
    _0 = 0,
    #[doc = "1: VREF from high vcm to low vcm"]
    _1 = 1,
}
impl From<CALRVS1_A> for bool {
    #[inline(always)]
    fn from(variant: CALRVS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRVS1` reader - OPA1 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration"]
pub struct CALRVS1_R(crate::FieldReader<bool, CALRVS1_A>);
impl CALRVS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALRVS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALRVS1_A {
        match self.bits {
            false => CALRVS1_A::_0,
            true => CALRVS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALRVS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALRVS1_A::_1
    }
}
impl core::ops::Deref for CALRVS1_R {
    type Target = crate::FieldReader<bool, CALRVS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRVS1` writer - OPA1 Calibration Reference Voltage Selection \nNote: CALRVS0 and CALRVS1 must be the same setting in calibration"]
pub struct CALRVS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRVS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALRVS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREF is"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALRVS1_A::_0)
    }
    #[doc = "VREF from high vcm to low vcm"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALRVS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OP Amplifier 0 Calibration Trigger Bit Note 1: Before this bit is enabled,ACMPEN(ACMP_CTL0) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance. Note 2: Hardware will auto clear this bit when next calibration is triggered by software Note 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
    #[inline(always)]
    pub fn caltrg0(&self) -> CALTRG0_R {
        CALTRG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OP Amplifier 1 Calibration Trigger Bit Note 1: Before this bit is enabled, ACMPEN(ACMP_CTL1) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance. Note 2: Hardware will auto clear this bit when next calibration is triggered by software. Note 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
    #[inline(always)]
    pub fn caltrg1(&self) -> CALTRG1_R {
        CALTRG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPA0 Calibration Reference Voltage Selection Note: CALRVS0 and CALRVS1 must be the same setting in calibration"]
    #[inline(always)]
    pub fn calrvs0(&self) -> CALRVS0_R {
        CALRVS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OPA1 Calibration Reference Voltage Selection Note: CALRVS0 and CALRVS1 must be the same setting in calibration"]
    #[inline(always)]
    pub fn calrvs1(&self) -> CALRVS1_R {
        CALRVS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OP Amplifier 0 Calibration Trigger Bit Note 1: Before this bit is enabled,ACMPEN(ACMP_CTL0) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance. Note 2: Hardware will auto clear this bit when next calibration is triggered by software Note 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
    #[inline(always)]
    pub fn caltrg0(&mut self) -> CALTRG0_W {
        CALTRG0_W { w: self }
    }
    #[doc = "Bit 1 - OP Amplifier 1 Calibration Trigger Bit Note 1: Before this bit is enabled, ACMPEN(ACMP_CTL1) should be set and the internal high speed RC oscillator (HIRC) should be enabled in advance. Note 2: Hardware will auto clear this bit when next calibration is triggered by software. Note 3: If user must trigger calibration twice or more times, the second trigger have to wait at least 300us after the previous calibration done"]
    #[inline(always)]
    pub fn caltrg1(&mut self) -> CALTRG1_W {
        CALTRG1_W { w: self }
    }
    #[doc = "Bit 16 - OPA0 Calibration Reference Voltage Selection Note: CALRVS0 and CALRVS1 must be the same setting in calibration"]
    #[inline(always)]
    pub fn calrvs0(&mut self) -> CALRVS0_W {
        CALRVS0_W { w: self }
    }
    #[doc = "Bit 17 - OPA1 Calibration Reference Voltage Selection Note: CALRVS0 and CALRVS1 must be the same setting in calibration"]
    #[inline(always)]
    pub fn calrvs1(&mut self) -> CALRVS1_W {
        CALRVS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp_calctl](index.html) module"]
pub struct ACMP_CALCTL_SPEC;
impl crate::RegisterSpec for ACMP_CALCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp_calctl::R](R) reader structure"]
impl crate::Readable for ACMP_CALCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmp_calctl::W](W) writer structure"]
impl crate::Writable for ACMP_CALCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMP_CALCTL to value 0xf0"]
impl crate::Resettable for ACMP_CALCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
