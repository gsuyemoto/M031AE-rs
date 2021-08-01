#[doc = "Register `QSPIx_RX` reader"]
pub struct R(crate::R<QSPIX_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPIX_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPIX_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPIX_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX` reader - Data Receive Register (Read Only)\nThere are 8-level FIFO buffers in this controller. The data receive register holds the data received from QSPI data input pin. If the RXEMPTY (QSPIx_STATUS\\[8) is not set to 1, the receive FIFO buffers can be accessed through software by reading this register."]
pub struct RX_R(crate::FieldReader<u32, u32>);
impl RX_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Receive Register (Read Only) There are 8-level FIFO buffers in this controller. The data receive register holds the data received from QSPI data input pin. If the RXEMPTY (QSPIx_STATUS\\[8) is not set to 1, the receive FIFO buffers can be accessed through software by reading this register."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "QSPI Data Receive Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspix_rx](index.html) module"]
pub struct QSPIX_RX_SPEC;
impl crate::RegisterSpec for QSPIX_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspix_rx::R](R) reader structure"]
impl crate::Readable for QSPIX_RX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QSPIx_RX to value 0"]
impl crate::Resettable for QSPIX_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
