#[doc = "Register `USBD_LPMATTR` reader"]
pub struct R(crate::R<USBD_LPMATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_LPMATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_LPMATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_LPMATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPMLINKSTS` reader - LPM Link State\nThese bits contain the bLinkState received with last ACK LPM Token"]
pub struct LPMLINKSTS_R(crate::FieldReader<u8, u8>);
impl LPMLINKSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMLINKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMLINKSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMBESL` reader - LPM Best Effort Service Latency\nThese bits contain the BESL value received with last ACK LPM Token"]
pub struct LPMBESL_R(crate::FieldReader<u8, u8>);
impl LPMBESL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMBESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMBESL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMRWAKUP` reader - LPM Remote Wakeup\nThis bit contains the bRemoteWake value received with last ACK LPM Token"]
pub struct LPMRWAKUP_R(crate::FieldReader<bool, bool>);
impl LPMRWAKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMRWAKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMRWAKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - LPM Link State These bits contain the bLinkState received with last ACK LPM Token"]
    #[inline(always)]
    pub fn lpmlinksts(&self) -> LPMLINKSTS_R {
        LPMLINKSTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LPM Best Effort Service Latency These bits contain the BESL value received with last ACK LPM Token"]
    #[inline(always)]
    pub fn lpmbesl(&self) -> LPMBESL_R {
        LPMBESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPM Remote Wakeup This bit contains the bRemoteWake value received with last ACK LPM Token"]
    #[inline(always)]
    pub fn lpmrwakup(&self) -> LPMRWAKUP_R {
        LPMRWAKUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "USB LPM Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_lpmattr](index.html) module"]
pub struct USBD_LPMATTR_SPEC;
impl crate::RegisterSpec for USBD_LPMATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_lpmattr::R](R) reader structure"]
impl crate::Readable for USBD_LPMATTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBD_LPMATTR to value 0"]
impl crate::Resettable for USBD_LPMATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
