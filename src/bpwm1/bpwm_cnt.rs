#[doc = "Register `BPWM_CNT` reader"]
pub struct R(crate::R<BPWM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - BPWM Data Register (Read Only)\nUser can monitor CNTR to know the current value in 16-bit period counter."]
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
#[doc = "BPWM Direction Indicator Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRF_A {
    #[doc = "0: Counter is Down count"]
    _0 = 0,
    #[doc = "1: Counter is UP count"]
    _1 = 1,
}
impl From<DIRF_A> for bool {
    #[inline(always)]
    fn from(variant: DIRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` reader - BPWM Direction Indicator Flag (Read Only)"]
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
    #[doc = "Bits 0:15 - BPWM Data Register (Read Only) User can monitor CNTR to know the current value in 16-bit period counter."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - BPWM Direction Indicator Flag (Read Only)"]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "BPWM Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_cnt](index.html) module"]
pub struct BPWM_CNT_SPEC;
impl crate::RegisterSpec for BPWM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_cnt::R](R) reader structure"]
impl crate::Readable for BPWM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BPWM_CNT to value 0"]
impl crate::Resettable for BPWM_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
