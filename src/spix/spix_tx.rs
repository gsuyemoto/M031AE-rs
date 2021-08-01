#[doc = "Register `SPIx_TX` writer"]
pub struct W(crate::W<SPIX_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIX_TX_SPEC>;
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
impl From<crate::W<SPIX_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIX_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` writer - Data Transmit Register\nThe data transmit registers pass through the transmitted data into the 4-level transmit FIFO buffers. The number of valid bits depends on the setting of DWIDTH (SPIx_CTL\\[12:8\\]) in SPI mode or WDWIDTH (SPIx_I2SCTL\\[5:4\\]) in I2S mode.\nIn SPI mode, if DWIDTH is set to 0x08, the bits TX\\[7:0\\]
will be transmitted. If DWIDTH is set to 0x00, the SPI controller will perform a 32-bit transfer.\nIn I2S mode, if WDWIDTH (SPIx_I2SCTL\\[5:4\\]) is set to 0x2, the data width of audio channel is 24-bit and corresponding to TX\\[23:0\\]. If WDWIDTH is set as 0x0, 0x1, or 0x3, all bits of this field are valid and referred to the data arrangement in I2S mode FIFO operation section\nNote: In Master mode, SPI controller will start to transfer the SPI bus clock after 1 APB clock and 6 peripheral clock cycles after user writes to this register."]
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
    #[doc = "Bits 0:31 - Data Transmit Register The data transmit registers pass through the transmitted data into the 4-level transmit FIFO buffers. The number of valid bits depends on the setting of DWIDTH (SPIx_CTL\\[12:8\\]) in SPI mode or WDWIDTH (SPIx_I2SCTL\\[5:4\\]) in I2S mode. In SPI mode, if DWIDTH is set to 0x08, the bits TX\\[7:0\\]
will be transmitted. If DWIDTH is set to 0x00, the SPI controller will perform a 32-bit transfer. In I2S mode, if WDWIDTH (SPIx_I2SCTL\\[5:4\\]) is set to 0x2, the data width of audio channel is 24-bit and corresponding to TX\\[23:0\\]. If WDWIDTH is set as 0x0, 0x1, or 0x3, all bits of this field are valid and referred to the data arrangement in I2S mode FIFO operation section Note: In Master mode, SPI controller will start to transfer the SPI bus clock after 1 APB clock and 6 peripheral clock cycles after user writes to this register."]
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
#[doc = "SPI Data Transmit Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spix_tx](index.html) module"]
pub struct SPIX_TX_SPEC;
impl crate::RegisterSpec for SPIX_TX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spix_tx::W](W) writer structure"]
impl crate::Writable for SPIX_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIx_TX to value 0"]
impl crate::Resettable for SPIX_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
