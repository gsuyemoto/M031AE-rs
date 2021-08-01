#[doc = "Register `QSPIx_TX` writer"]
pub struct W(crate::W<QSPIX_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPIX_TX_SPEC>;
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
impl From<crate::W<QSPIX_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPIX_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` writer - Data Transmit Register\nThe data transmit registers pass through the transmitted data into the 8-level transmit FIFO buffers. The number of valid bits depends on the setting of DWIDTH (QSPIx_CTL\\[12:8\\]).\nIf DWIDTH is set to 0x08, the bits TX\\[7:0\\]
will be transmitted. If DWIDTH is set to 0x00, the QSPI controller will perform a 32-bit transfer.\nNote: In Master mode, QSPI controller will start to transfer the QSPI bus clock after 1 APB clock and 6 peripheral clock cycles after user writes to this register."]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Transmit Register The data transmit registers pass through the transmitted data into the 8-level transmit FIFO buffers. The number of valid bits depends on the setting of DWIDTH (QSPIx_CTL\\[12:8\\]). If DWIDTH is set to 0x08, the bits TX\\[7:0\\]
will be transmitted. If DWIDTH is set to 0x00, the QSPI controller will perform a 32-bit transfer. Note: In Master mode, QSPI controller will start to transfer the QSPI bus clock after 1 APB clock and 6 peripheral clock cycles after user writes to this register."]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI Data Transmit Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspix_tx](index.html) module"]
pub struct QSPIX_TX_SPEC;
impl crate::RegisterSpec for QSPIX_TX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qspix_tx::W](W) writer structure"]
impl crate::Writable for QSPIX_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPIx_TX to value 0"]
impl crate::Resettable for QSPIX_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
