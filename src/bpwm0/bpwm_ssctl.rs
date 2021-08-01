#[doc = "Register `BPWM_SSCTL` reader"]
pub struct R(crate::R<BPWM_SSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_SSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_SSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_SSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_SSCTL` writer"]
pub struct W(crate::W<BPWM_SSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_SSCTL_SPEC>;
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
impl From<crate::W<BPWM_SSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_SSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Synchronous Start Function 0 Enable Bit\nWhen synchronous start function is enabled, the BPWM_CH0 counter enable bit (CNTEN0) can be enabled by writing BPWM synchronous start trigger bit (CNTSEN).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEN0_A {
    #[doc = "0: BPWM synchronous start function Disabled"]
    _0 = 0,
    #[doc = "1: BPWM synchronous start function Enabled"]
    _1 = 1,
}
impl From<SSEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SSEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEN0` reader - BPWM Synchronous Start Function 0 Enable Bit\nWhen synchronous start function is enabled, the BPWM_CH0 counter enable bit (CNTEN0) can be enabled by writing BPWM synchronous start trigger bit (CNTSEN)."]
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
#[doc = "Field `SSEN0` writer - BPWM Synchronous Start Function 0 Enable Bit\nWhen synchronous start function is enabled, the BPWM_CH0 counter enable bit (CNTEN0) can be enabled by writing BPWM synchronous start trigger bit (CNTSEN)."]
pub struct SSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM synchronous start function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSEN0_A::_0)
    }
    #[doc = "BPWM synchronous start function Enabled"]
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
#[doc = "BPWM Synchronous Start Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSRC_A {
    #[doc = "0: Synchronous start source come from PWM0"]
    _0 = 0,
    #[doc = "1: Synchronous start source come from PWM1"]
    _1 = 1,
    #[doc = "2: Synchronous start source come from BPWM0"]
    _2 = 2,
    #[doc = "3: Synchronous start source come from BPWM1"]
    _3 = 3,
}
impl From<SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSRC` reader - BPWM Synchronous Start Source Select"]
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
#[doc = "Field `SSRC` writer - BPWM Synchronous Start Source Select"]
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
    #[doc = "Synchronous start source come from BPWM0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SSRC_A::_2)
    }
    #[doc = "Synchronous start source come from BPWM1"]
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
    #[doc = "Bit 0 - BPWM Synchronous Start Function 0 Enable Bit When synchronous start function is enabled, the BPWM_CH0 counter enable bit (CNTEN0) can be enabled by writing BPWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen0(&self) -> SSEN0_R {
        SSEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - BPWM Synchronous Start Source Select"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Synchronous Start Function 0 Enable Bit When synchronous start function is enabled, the BPWM_CH0 counter enable bit (CNTEN0) can be enabled by writing BPWM synchronous start trigger bit (CNTSEN)."]
    #[inline(always)]
    pub fn ssen0(&mut self) -> SSEN0_W {
        SSEN0_W { w: self }
    }
    #[doc = "Bits 8:9 - BPWM Synchronous Start Source Select"]
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
#[doc = "BPWM Synchronous Start Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_ssctl](index.html) module"]
pub struct BPWM_SSCTL_SPEC;
impl crate::RegisterSpec for BPWM_SSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_ssctl::R](R) reader structure"]
impl crate::Readable for BPWM_SSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_ssctl::W](W) writer structure"]
impl crate::Writable for BPWM_SSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_SSCTL to value 0"]
impl crate::Resettable for BPWM_SSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
