#[doc = "Register `PWM_SSCTL` reader"]
pub struct R(crate::R<PWM_SSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SSCTL` writer"]
pub struct W(crate::W<PWM_SSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SSCTL_SPEC>;
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
impl From<crate::W<PWM_SSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Synchronous Start Function Enable Bit 0\nWhen synchronous start function is enabled, the PWM_CH0 counter enable bit (CNTEN0) can be enabled by writing PWM synchronous start trigger bit (CNTSEN).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEN0_A {
    #[doc = "0: PWM synchronous start function Disabled"]
    _0 = 0,
    #[doc = "1: PWM synchronous start function Enabled"]
    _1 = 1,
}
impl From<SSEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SSEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEN0` reader - PWM Synchronous Start Function Enable Bit 0\nWhen synchronous start function is enabled, the PWM_CH0 counter enable bit (CNTEN0) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN0_R(crate::FieldReader<bool, SSEN0_A>);
impl SSEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEN0_A {
        match self.bits {
            false => SSEN0_A::_0,
            true => SSEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSEN0_A::_1
    }
}
impl core::ops::Deref for SSEN0_R {
    type Target = crate::FieldReader<bool, SSEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEN0` writer - PWM Synchronous Start Function Enable Bit 0\nWhen synchronous start function is enabled, the PWM_CH0 counter enable bit (CNTEN0) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM synchronous start function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSEN0_A::_0)
    }
    #[doc = "PWM synchronous start function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSEN0_A::_1)
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
#[doc = "PWM Synchronous Start Function Enable Bit 2\nWhen synchronous start function is enabled, the PWM_CH2 counter enable bit (CNTEN2) can be enabled by writing PWM synchronous start trigger bit (CNTSEN).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEN2_A {
    #[doc = "0: PWM synchronous start function Disabled"]
    _0 = 0,
    #[doc = "1: PWM synchronous start function Enabled"]
    _1 = 1,
}
impl From<SSEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SSEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEN2` reader - PWM Synchronous Start Function Enable Bit 2\nWhen synchronous start function is enabled, the PWM_CH2 counter enable bit (CNTEN2) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN2_R(crate::FieldReader<bool, SSEN2_A>);
impl SSEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEN2_A {
        match self.bits {
            false => SSEN2_A::_0,
            true => SSEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSEN2_A::_1
    }
}
impl core::ops::Deref for SSEN2_R {
    type Target = crate::FieldReader<bool, SSEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEN2` writer - PWM Synchronous Start Function Enable Bit 2\nWhen synchronous start function is enabled, the PWM_CH2 counter enable bit (CNTEN2) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM synchronous start function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSEN2_A::_0)
    }
    #[doc = "PWM synchronous start function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSEN2_A::_1)
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
#[doc = "PWM Synchronous Start Function Enable Bit 4\nWhen synchronous start function is enabled, the PWM_CH4 counter enable bit (CNTEN4) can be enabled by writing PWM synchronous start trigger bit (CNTSEN).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEN4_A {
    #[doc = "0: PWM synchronous start function Disabled"]
    _0 = 0,
    #[doc = "1: PWM synchronous start function Enabled"]
    _1 = 1,
}
impl From<SSEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SSEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEN4` reader - PWM Synchronous Start Function Enable Bit 4\nWhen synchronous start function is enabled, the PWM_CH4 counter enable bit (CNTEN4) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN4_R(crate::FieldReader<bool, SSEN4_A>);
impl SSEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEN4_A {
        match self.bits {
            false => SSEN4_A::_0,
            true => SSEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSEN4_A::_1
    }
}
impl core::ops::Deref for SSEN4_R {
    type Target = crate::FieldReader<bool, SSEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEN4` writer - PWM Synchronous Start Function Enable Bit 4\nWhen synchronous start function is enabled, the PWM_CH4 counter enable bit (CNTEN4) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM synchronous start function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSEN4_A::_0)
    }
    #[doc = "PWM synchronous start function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSEN4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "PWM Synchronous Start Source Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSRC_A {
    #[doc = "0: Synchronous start source come from PWM0"]
    _0 = 0,
    #[doc = "1: Synchronous start source come from PWM1"]
    _1 = 1,
    #[doc = "2: Reserved."]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSRC` reader - PWM Synchronous Start Source Select Bits"]
pub struct SSRC_R(crate::FieldReader<u8, SSRC_A>);
impl SSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSRC_A {
        match self.bits {
            0 => SSRC_A::_0,
            1 => SSRC_A::_1,
            2 => SSRC_A::_2,
            3 => SSRC_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSRC_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SSRC_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SSRC_A::_3
    }
}
impl core::ops::Deref for SSRC_R {
    type Target = crate::FieldReader<u8, SSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSRC` writer - PWM Synchronous Start Source Select Bits"]
pub struct SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Synchronous start source come from PWM0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSRC_A::_0)
    }
    #[doc = "Synchronous start source come from PWM1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSRC_A::_1)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SSRC_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SSRC_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM Synchronous Start Function Enable Bit 0 When synchronous start function is enabled, the PWM_CH0 counter enable bit (CNTEN0) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen0(&self) -> SSEN0_R {
        SSEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Synchronous Start Function Enable Bit 2 When synchronous start function is enabled, the PWM_CH2 counter enable bit (CNTEN2) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen2(&self) -> SSEN2_R {
        SSEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Synchronous Start Function Enable Bit 4 When synchronous start function is enabled, the PWM_CH4 counter enable bit (CNTEN4) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen4(&self) -> SSEN4_R {
        SSEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - PWM Synchronous Start Source Select Bits"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Synchronous Start Function Enable Bit 0 When synchronous start function is enabled, the PWM_CH0 counter enable bit (CNTEN0) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen0(&mut self) -> SSEN0_W {
        SSEN0_W { w: self }
    }
    #[doc = "Bit 2 - PWM Synchronous Start Function Enable Bit 2 When synchronous start function is enabled, the PWM_CH2 counter enable bit (CNTEN2) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen2(&mut self) -> SSEN2_W {
        SSEN2_W { w: self }
    }
    #[doc = "Bit 4 - PWM Synchronous Start Function Enable Bit 4 When synchronous start function is enabled, the PWM_CH4 counter enable bit (CNTEN4) can be enabled by writing PWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen4(&mut self) -> SSEN4_W {
        SSEN4_W { w: self }
    }
    #[doc = "Bits 8:9 - PWM Synchronous Start Source Select Bits"]
    #[inline(always)]
    pub fn ssrc(&mut self) -> SSRC_W {
        SSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Synchronous Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ssctl](index.html) module"]
pub struct PWM_SSCTL_SPEC;
impl crate::RegisterSpec for PWM_SSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ssctl::R](R) reader structure"]
impl crate::Readable for PWM_SSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ssctl::W](W) writer structure"]
impl crate::Writable for PWM_SSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SSCTL to value 0"]
impl crate::Resettable for PWM_SSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
