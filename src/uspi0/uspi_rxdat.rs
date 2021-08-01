#[doc = "Register `USPI_RXDAT` reader"]
pub struct R(crate::R<USPI_RXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_RXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_RXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_RXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDAT` reader - Received Data\nThis bit field monitors the received data which stored in receive data buffer."]
pub struct RXDAT_R(crate::FieldReader<u16, u16>);
impl RXDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Received Data This bit field monitors the received data which stored in receive data buffer."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "USCI Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_rxdat](index.html) module"]
pub struct USPI_RXDAT_SPEC;
impl crate::RegisterSpec for USPI_RXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_rxdat::R](R) reader structure"]
impl crate::Readable for USPI_RXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USPI_RXDAT to value 0"]
impl crate::Resettable for USPI_RXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
