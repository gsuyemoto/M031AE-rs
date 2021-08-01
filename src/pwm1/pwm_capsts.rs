#[doc = "Register `PWM_CAPSTS` reader"]
pub struct R(crate::R<PWM_CAPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CAPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CAPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRLIFOV0` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV0_R(crate::FieldReader<bool, bool>);
impl CRLIFOV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFOV1` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV1_R(crate::FieldReader<bool, bool>);
impl CRLIFOV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFOV2` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV2_R(crate::FieldReader<bool, bool>);
impl CRLIFOV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFOV3` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV3_R(crate::FieldReader<bool, bool>);
impl CRLIFOV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFOV4` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV4_R(crate::FieldReader<bool, bool>);
impl CRLIFOV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFOV5` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOV5_R(crate::FieldReader<bool, bool>);
impl CRLIFOV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIFOV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV0` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV0_R(crate::FieldReader<bool, bool>);
impl CFLIFOV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV1` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV1_R(crate::FieldReader<bool, bool>);
impl CFLIFOV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV2` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV2_R(crate::FieldReader<bool, bool>);
impl CFLIFOV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV3` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV3_R(crate::FieldReader<bool, bool>);
impl CFLIFOV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV4` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV4_R(crate::FieldReader<bool, bool>);
impl CFLIFOV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOV5` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1.\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOV5_R(crate::FieldReader<bool, bool>);
impl CFLIFOV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIFOV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov0(&self) -> CRLIFOV0_R {
        CRLIFOV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov1(&self) -> CRLIFOV1_R {
        CRLIFOV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov2(&self) -> CRLIFOV2_R {
        CRLIFOV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov3(&self) -> CRLIFOV3_R {
        CRLIFOV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov4(&self) -> CRLIFOV4_R {
        CRLIFOV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CRLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifov5(&self) -> CRLIFOV5_R {
        CRLIFOV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov0(&self) -> CFLIFOV0_R {
        CFLIFOV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov1(&self) -> CFLIFOV1_R {
        CFLIFOV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov2(&self) -> CFLIFOV2_R {
        CFLIFOV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov3(&self) -> CFLIFOV3_R {
        CFLIFOV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov4(&self) -> CFLIFOV4_R {
        CFLIFOV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CFLIF is 1. Note: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifov5(&self) -> CFLIFOV5_R {
        CFLIFOV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "PWM Capture Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capsts](index.html) module"]
pub struct PWM_CAPSTS_SPEC;
impl crate::RegisterSpec for PWM_CAPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capsts::R](R) reader structure"]
impl crate::Readable for PWM_CAPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_CAPSTS to value 0"]
impl crate::Resettable for PWM_CAPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
