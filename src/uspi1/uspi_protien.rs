#[doc = "Register `USPI_PROTIEN` reader"]
pub struct R(crate::R<USPI_PROTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_PROTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_PROTIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_PROTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_PROTIEN` writer"]
pub struct W(crate::W<USPI_PROTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_PROTIEN_SPEC>;
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
impl From<crate::W<USPI_PROTIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_PROTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Select Inactive Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSINAIEN_A {
    #[doc = "0: Slave select inactive interrupt generation Disabled"]
    _0 = 0,
    #[doc = "1: Slave select inactive interrupt generation Enabled"]
    _1 = 1,
}
impl From<SSINAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSINAIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSINAIEN` reader - Slave Select Inactive Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to inactive."]
pub struct SSINAIEN_R(crate::FieldReader<bool, SSINAIEN_A>);
impl SSINAIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSINAIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSINAIEN_A {
        match self.bits {
            false => SSINAIEN_A::_0,
            true => SSINAIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSINAIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSINAIEN_A::_1
    }
}
impl core::ops::Deref for SSINAIEN_R {
    type Target = crate::FieldReader<bool, SSINAIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINAIEN` writer - Slave Select Inactive Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to inactive."]
pub struct SSINAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINAIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSINAIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select inactive interrupt generation Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSINAIEN_A::_0)
    }
    #[doc = "Slave select inactive interrupt generation Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSINAIEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Slave Select Active Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACTIEN_A {
    #[doc = "0: Slave select active interrupt generation Disabled"]
    _0 = 0,
    #[doc = "1: Slave select active interrupt generation Enabled"]
    _1 = 1,
}
impl From<SSACTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSACTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACTIEN` reader - Slave Select Active Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to active."]
pub struct SSACTIEN_R(crate::FieldReader<bool, SSACTIEN_A>);
impl SSACTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACTIEN_A {
        match self.bits {
            false => SSACTIEN_A::_0,
            true => SSACTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACTIEN_A::_1
    }
}
impl core::ops::Deref for SSACTIEN_R {
    type Target = crate::FieldReader<bool, SSACTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACTIEN` writer - Slave Select Active Interrupt Enable Bit\nThis bit enables/disables the generation of a slave select interrupt if the slave select changes to active."]
pub struct SSACTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select active interrupt generation Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACTIEN_A::_0)
    }
    #[doc = "Slave select active interrupt generation Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACTIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Slave Time-out Interrupt Enable Bit\nIn SPI protocol, this bit enables the interrupt generation in case of a Slave time-out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVTOIEN_A {
    #[doc = "0: The Slave time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The Slave time-out interrupt Enabled"]
    _1 = 1,
}
impl From<SLVTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVTOIEN` reader - Slave Time-out Interrupt Enable Bit\nIn SPI protocol, this bit enables the interrupt generation in case of a Slave time-out event."]
pub struct SLVTOIEN_R(crate::FieldReader<bool, SLVTOIEN_A>);
impl SLVTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVTOIEN_A {
        match self.bits {
            false => SLVTOIEN_A::_0,
            true => SLVTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVTOIEN_A::_1
    }
}
impl core::ops::Deref for SLVTOIEN_R {
    type Target = crate::FieldReader<bool, SLVTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOIEN` writer - Slave Time-out Interrupt Enable Bit\nIn SPI protocol, this bit enables the interrupt generation in case of a Slave time-out event."]
pub struct SLVTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Slave time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVTOIEN_A::_0)
    }
    #[doc = "The Slave time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVTOIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Slave Mode Bit Count Error Interrupt Enable Bit\nIf data transfer is terminated by slave time-out or slave select inactive event in Slave mode, so that the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]). Bit count error event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVBEIEN_A {
    #[doc = "0: The Slave mode bit count error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The Slave mode bit count error interrupt Enabled"]
    _1 = 1,
}
impl From<SLVBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVBEIEN` reader - Slave Mode Bit Count Error Interrupt Enable Bit\nIf data transfer is terminated by slave time-out or slave select inactive event in Slave mode, so that the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]). Bit count error event occurs."]
pub struct SLVBEIEN_R(crate::FieldReader<bool, SLVBEIEN_A>);
impl SLVBEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVBEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVBEIEN_A {
        match self.bits {
            false => SLVBEIEN_A::_0,
            true => SLVBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVBEIEN_A::_1
    }
}
impl core::ops::Deref for SLVBEIEN_R {
    type Target = crate::FieldReader<bool, SLVBEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVBEIEN` writer - Slave Mode Bit Count Error Interrupt Enable Bit\nIf data transfer is terminated by slave time-out or slave select inactive event in Slave mode, so that the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]). Bit count error event occurs."]
pub struct SLVBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVBEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Slave mode bit count error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVBEIEN_A::_0)
    }
    #[doc = "The Slave mode bit count error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVBEIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Select Inactive Interrupt Enable Bit This bit enables/disables the generation of a slave select interrupt if the slave select changes to inactive."]
    #[inline(always)]
    pub fn ssinaien(&self) -> SSINAIEN_R {
        SSINAIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Select Active Interrupt Enable Bit This bit enables/disables the generation of a slave select interrupt if the slave select changes to active."]
    #[inline(always)]
    pub fn ssactien(&self) -> SSACTIEN_R {
        SSACTIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Time-out Interrupt Enable Bit In SPI protocol, this bit enables the interrupt generation in case of a Slave time-out event."]
    #[inline(always)]
    pub fn slvtoien(&self) -> SLVTOIEN_R {
        SLVTOIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Mode Bit Count Error Interrupt Enable Bit If data transfer is terminated by slave time-out or slave select inactive event in Slave mode, so that the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]). Bit count error event occurs."]
    #[inline(always)]
    pub fn slvbeien(&self) -> SLVBEIEN_R {
        SLVBEIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Inactive Interrupt Enable Bit This bit enables/disables the generation of a slave select interrupt if the slave select changes to inactive."]
    #[inline(always)]
    pub fn ssinaien(&mut self) -> SSINAIEN_W {
        SSINAIEN_W { w: self }
    }
    #[doc = "Bit 1 - Slave Select Active Interrupt Enable Bit This bit enables/disables the generation of a slave select interrupt if the slave select changes to active."]
    #[inline(always)]
    pub fn ssactien(&mut self) -> SSACTIEN_W {
        SSACTIEN_W { w: self }
    }
    #[doc = "Bit 2 - Slave Time-out Interrupt Enable Bit In SPI protocol, this bit enables the interrupt generation in case of a Slave time-out event."]
    #[inline(always)]
    pub fn slvtoien(&mut self) -> SLVTOIEN_W {
        SLVTOIEN_W { w: self }
    }
    #[doc = "Bit 3 - Slave Mode Bit Count Error Interrupt Enable Bit If data transfer is terminated by slave time-out or slave select inactive event in Slave mode, so that the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]). Bit count error event occurs."]
    #[inline(always)]
    pub fn slvbeien(&mut self) -> SLVBEIEN_W {
        SLVBEIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_protien](index.html) module"]
pub struct USPI_PROTIEN_SPEC;
impl crate::RegisterSpec for USPI_PROTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_protien::R](R) reader structure"]
impl crate::Readable for USPI_PROTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_protien::W](W) writer structure"]
impl crate::Writable for USPI_PROTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_PROTIEN to value 0"]
impl crate::Resettable for USPI_PROTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
