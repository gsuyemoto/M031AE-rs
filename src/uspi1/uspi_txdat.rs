#[doc = "Register `USPI_TXDAT` writer"]
pub struct W(crate::W<USPI_TXDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_TXDAT_SPEC>;
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
impl From<crate::W<USPI_TXDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_TXDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDAT` writer - Transmit Data\nSoftware can use this bit field to write 16-bit transmit data for transmission. In order to avoid overwriting the transmit data, user have to check TXEMPTY (USPI_BUFSTS\\[8\\]) status before writing transmit data into this bit field."]
pub struct TXDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Port Direction Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTDIR_AW {
    #[doc = "0: The data pin is configured as output mode"]
    _0 = 0,
    #[doc = "1: The data pin is configured as input mode"]
    _1 = 1,
}
impl From<PORTDIR_AW> for bool {
    #[inline(always)]
    fn from(variant: PORTDIR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTDIR` writer - Port Direction Control"]
pub struct PORTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTDIR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data pin is configured as output mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTDIR_AW::_0)
    }
    #[doc = "The data pin is configured as input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTDIR_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data Software can use this bit field to write 16-bit transmit data for transmission. In order to avoid overwriting the transmit data, user have to check TXEMPTY (USPI_BUFSTS\\[8\\]) status before writing transmit data into this bit field."]
    #[inline(always)]
    pub fn txdat(&mut self) -> TXDAT_W {
        TXDAT_W { w: self }
    }
    #[doc = "Bit 16 - Port Direction Control"]
    #[inline(always)]
    pub fn portdir(&mut self) -> PORTDIR_W {
        PORTDIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_txdat](index.html) module"]
pub struct USPI_TXDAT_SPEC;
impl crate::RegisterSpec for USPI_TXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uspi_txdat::W](W) writer structure"]
impl crate::Writable for USPI_TXDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_TXDAT to value 0"]
impl crate::Resettable for USPI_TXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
