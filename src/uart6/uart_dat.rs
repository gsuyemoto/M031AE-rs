#[doc = "Register `UART_DAT` reader"]
pub struct R(crate::R<UART_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DAT` writer"]
pub struct W(crate::W<UART_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DAT_SPEC>;
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
impl From<crate::W<UART_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Data Receive/Transmit Buffer\nWrite Operation:\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART controller will send out the data stored in transmitter FIFO top location through the UART_TXD.\nRead Operation:\nBy reading this register, the UART controller will return an 8-bit data received from receiver FIFO."]
pub struct DAT_R(crate::FieldReader<u8, u8>);
impl DAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT` writer - Data Receive/Transmit Buffer\nWrite Operation:\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART controller will send out the data stored in transmitter FIFO top location through the UART_TXD.\nRead Operation:\nBy reading this register, the UART controller will return an 8-bit data received from receiver FIFO."]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PARITY` reader - Parity Bit Receive/Transmit Buffer\nWrite Operation:\nBy writing to this bit, the parity bit will be stored in transmitter FIFO. If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set, the UART controller will send out this bit follow the DAT (UART_DAT\\[7:0\\]) through the UART_TXD.\nRead Operation:\nIf PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are enabled, the parity bit can be read by this bit.\nNote: This bit has effect only when PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set."]
pub struct PARITY_R(crate::FieldReader<bool, bool>);
impl PARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` writer - Parity Bit Receive/Transmit Buffer\nWrite Operation:\nBy writing to this bit, the parity bit will be stored in transmitter FIFO. If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set, the UART controller will send out this bit follow the DAT (UART_DAT\\[7:0\\]) through the UART_TXD.\nRead Operation:\nIf PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are enabled, the parity bit can be read by this bit.\nNote: This bit has effect only when PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set."]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Receive/Transmit Buffer Write Operation: By writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation: By reading this register, the UART controller will return an 8-bit data received from receiver FIFO."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Parity Bit Receive/Transmit Buffer Write Operation: By writing to this bit, the parity bit will be stored in transmitter FIFO. If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set, the UART controller will send out this bit follow the DAT (UART_DAT\\[7:0\\]) through the UART_TXD. Read Operation: If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are enabled, the parity bit can be read by this bit. Note: This bit has effect only when PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Receive/Transmit Buffer Write Operation: By writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation: By reading this register, the UART controller will return an 8-bit data received from receiver FIFO."]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
    #[doc = "Bit 8 - Parity Bit Receive/Transmit Buffer Write Operation: By writing to this bit, the parity bit will be stored in transmitter FIFO. If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set, the UART controller will send out this bit follow the DAT (UART_DAT\\[7:0\\]) through the UART_TXD. Read Operation: If PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are enabled, the parity bit can be read by this bit. Note: This bit has effect only when PBE (UART_LINE\\[3\\]) and PSS (UART_LINE\\[7\\]) are set."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Receive/Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dat](index.html) module"]
pub struct UART_DAT_SPEC;
impl crate::RegisterSpec for UART_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_dat::R](R) reader structure"]
impl crate::Readable for UART_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_dat::W](W) writer structure"]
impl crate::Writable for UART_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DAT to value 0"]
impl crate::Resettable for UART_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
