#[doc = "Register `BPWM_CAPSTS` reader"]
pub struct R(crate::R<BPWM_CAPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CAPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CAPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CAPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRIFOV0` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV0_R(crate::FieldReader<bool, bool>);
impl CRIFOV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRIFOV1` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV1_R(crate::FieldReader<bool, bool>);
impl CRIFOV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRIFOV2` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV2_R(crate::FieldReader<bool, bool>);
impl CRIFOV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRIFOV3` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV3_R(crate::FieldReader<bool, bool>);
impl CRIFOV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRIFOV4` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV4_R(crate::FieldReader<bool, bool>);
impl CRIFOV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRIFOV5` reader - Capture Rising Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPRIF."]
pub struct CRIFOV5_R(crate::FieldReader<bool, bool>);
impl CRIFOV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRIFOV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRIFOV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV0` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV0_R(crate::FieldReader<bool, bool>);
impl CFIFOV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV1` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV1_R(crate::FieldReader<bool, bool>);
impl CFIFOV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV2` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV2_R(crate::FieldReader<bool, bool>);
impl CFIFOV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV3` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV3_R(crate::FieldReader<bool, bool>);
impl CFIFOV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV4` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV4_R(crate::FieldReader<bool, bool>);
impl CFIFOV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFIFOV5` reader - Capture Falling Interrupt Flag Overrun Status (Read Only)\nThis flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n.\nNote: This bit will be cleared automatically when user clear corresponding CAPFIF."]
pub struct CFIFOV5_R(crate::FieldReader<bool, bool>);
impl CFIFOV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFIFOV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIFOV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov0(&self) -> CRIFOV0_R {
        CRIFOV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov1(&self) -> CRIFOV1_R {
        CRIFOV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov2(&self) -> CRIFOV2_R {
        CRIFOV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov3(&self) -> CRIFOV3_R {
        CRIFOV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov4(&self) -> CRIFOV4_R {
        CRIFOV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Rising Interrupt Flag Overrun Status (Read Only) This flag indicates if rising latch happened when the corresponding CAPRIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPRIF."]
    #[inline(always)]
    pub fn crifov5(&self) -> CRIFOV5_R {
        CRIFOV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov0(&self) -> CFIFOV0_R {
        CFIFOV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov1(&self) -> CFIFOV1_R {
        CFIFOV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov2(&self) -> CFIFOV2_R {
        CFIFOV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov3(&self) -> CFIFOV3_R {
        CFIFOV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov4(&self) -> CFIFOV4_R {
        CFIFOV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Capture Falling Interrupt Flag Overrun Status (Read Only) This flag indicates if falling latch happened when the corresponding CAPFIF is 1. Each bit n controls the corresponding BPWM channel n. Note: This bit will be cleared automatically when user clear corresponding CAPFIF."]
    #[inline(always)]
    pub fn cfifov5(&self) -> CFIFOV5_R {
        CFIFOV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "BPWM Capture Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_capsts](index.html) module"]
pub struct BPWM_CAPSTS_SPEC;
impl crate::RegisterSpec for BPWM_CAPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_capsts::R](R) reader structure"]
impl crate::Readable for BPWM_CAPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BPWM_CAPSTS to value 0"]
impl crate::Resettable for BPWM_CAPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
