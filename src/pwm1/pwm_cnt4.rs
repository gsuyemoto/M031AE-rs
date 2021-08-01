#[doc = "Register `PWM_CNT4` reader"]
pub struct R(crate::R<PWM_CNT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CNT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CNT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - PWM Data Register (Read Only)\nUser can monitor CNTR to know the current value in 16-bit period counter."]
pub struct CNT_R(crate::FieldReader<u16, u16>);
impl CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Direction Indicator Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRF_A {
    #[doc = "0: Counter is counting down"]
    _0 = 0,
    #[doc = "1: Counter is counting up"]
    _1 = 1,
}
impl From<DIRF_A> for bool {
    #[inline(always)]
    fn from(variant: DIRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` reader - PWM Direction Indicator Flag (Read Only)"]
pub struct DIRF_R(crate::FieldReader<bool, DIRF_A>);
impl DIRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRF_A {
        match self.bits {
            false => DIRF_A::_0,
            true => DIRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIRF_A::_1
    }
}
impl core::ops::Deref for DIRF_R {
    type Target = crate::FieldReader<bool, DIRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Data Register (Read Only) User can monitor CNTR to know the current value in 16-bit period counter."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - PWM Direction Indicator Flag (Read Only)"]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "PWM Counter Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cnt4](index.html) module"]
pub struct PWM_CNT4_SPEC;
impl crate::RegisterSpec for PWM_CNT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cnt4::R](R) reader structure"]
impl crate::Readable for PWM_CNT4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_CNT4 to value 0"]
impl crate::Resettable for PWM_CNT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
