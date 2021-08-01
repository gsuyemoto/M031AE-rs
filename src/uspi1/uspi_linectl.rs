#[doc = "Register `USPI_LINECTL` reader"]
pub struct R(crate::R<USPI_LINECTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_LINECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_LINECTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_LINECTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_LINECTL` writer"]
pub struct W(crate::W<USPI_LINECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_LINECTL_SPEC>;
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
impl From<crate::W<USPI_LINECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_LINECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LSB First Transmission Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSB_A {
    #[doc = "0: The MSB, which bit of transmit/receive data buffer depends on the setting of DWIDTH, is transmitted/received first"]
    _0 = 0,
    #[doc = "1: The LSB, the bit 0 of data buffer, will be transmitted/received first"]
    _1 = 1,
}
impl From<LSB_A> for bool {
    #[inline(always)]
    fn from(variant: LSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSB` reader - LSB First Transmission Selection"]
pub struct LSB_R(crate::FieldReader<bool, LSB_A>);
impl LSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSB_A {
        match self.bits {
            false => LSB_A::_0,
            true => LSB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LSB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LSB_A::_1
    }
}
impl core::ops::Deref for LSB_R {
    type Target = crate::FieldReader<bool, LSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSB` writer - LSB First Transmission Selection"]
pub struct LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> LSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MSB, which bit of transmit/receive data buffer depends on the setting of DWIDTH, is transmitted/received first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSB_A::_0)
    }
    #[doc = "The LSB, the bit 0 of data buffer, will be transmitted/received first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSB_A::_1)
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
#[doc = "Data Output Inverse Selection\nThis bit defines the relation between the internal shift data value and the output data signal of USCIx_DAT0/1 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATOINV_A {
    #[doc = "0: Data output values of USCIx_DAT0/1 pins are not inverted"]
    _0 = 0,
    #[doc = "1: Data output values of USCIx_DAT0/1 pins are inverted"]
    _1 = 1,
}
impl From<DATOINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATOINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATOINV` reader - Data Output Inverse Selection\nThis bit defines the relation between the internal shift data value and the output data signal of USCIx_DAT0/1 pins."]
pub struct DATOINV_R(crate::FieldReader<bool, DATOINV_A>);
impl DATOINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATOINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATOINV_A {
        match self.bits {
            false => DATOINV_A::_0,
            true => DATOINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATOINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATOINV_A::_1
    }
}
impl core::ops::Deref for DATOINV_R {
    type Target = crate::FieldReader<bool, DATOINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATOINV` writer - Data Output Inverse Selection\nThis bit defines the relation between the internal shift data value and the output data signal of USCIx_DAT0/1 pins."]
pub struct DATOINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATOINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATOINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data output values of USCIx_DAT0/1 pins are not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATOINV_A::_0)
    }
    #[doc = "Data output values of USCIx_DAT0/1 pins are inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATOINV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Control Signal Output Inverse Selection\nThis bit defines the relation between the internal control signal and the output control signal.\nNote: The control signal has different definitions in different protocol. In SPI protocol, the control signal means slave select signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLOINV_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The control signal will be inverted before its output"]
    _1 = 1,
}
impl From<CTLOINV_A> for bool {
    #[inline(always)]
    fn from(variant: CTLOINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLOINV` reader - Control Signal Output Inverse Selection\nThis bit defines the relation between the internal control signal and the output control signal.\nNote: The control signal has different definitions in different protocol. In SPI protocol, the control signal means slave select signal."]
pub struct CTLOINV_R(crate::FieldReader<bool, CTLOINV_A>);
impl CTLOINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLOINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLOINV_A {
        match self.bits {
            false => CTLOINV_A::_0,
            true => CTLOINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTLOINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTLOINV_A::_1
    }
}
impl core::ops::Deref for CTLOINV_R {
    type Target = crate::FieldReader<bool, CTLOINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLOINV` writer - Control Signal Output Inverse Selection\nThis bit defines the relation between the internal control signal and the output control signal.\nNote: The control signal has different definitions in different protocol. In SPI protocol, the control signal means slave select signal."]
pub struct CTLOINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLOINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLOINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTLOINV_A::_0)
    }
    #[doc = "The control signal will be inverted before its output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTLOINV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DWIDTH` reader - Word Length of Transmission\nThis bit field defines the data word length (amount of bits) for reception and transmission. The data word is always right-aligned in the data buffer. USCI support word length from 4 to 16 bits.\n0x0: The data word contains 16 bits located at bit positions \\[15:0\\].\n0x1: Reserved.\n0x2: Reserved.\n0x3: Reserved.\n0x4: The data word contains 4 bits located at bit positions \\[3:0\\].\n0x5: The data word contains 5 bits located at bit positions \\[4:0\\].\n...\n0xF: The data word contains 15 bits located at bit positions \\[14:0\\]."]
pub struct DWIDTH_R(crate::FieldReader<u8, u8>);
impl DWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWIDTH` writer - Word Length of Transmission\nThis bit field defines the data word length (amount of bits) for reception and transmission. The data word is always right-aligned in the data buffer. USCI support word length from 4 to 16 bits.\n0x0: The data word contains 16 bits located at bit positions \\[15:0\\].\n0x1: Reserved.\n0x2: Reserved.\n0x3: Reserved.\n0x4: The data word contains 4 bits located at bit positions \\[3:0\\].\n0x5: The data word contains 5 bits located at bit positions \\[4:0\\].\n...\n0xF: The data word contains 15 bits located at bit positions \\[14:0\\]."]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSB First Transmission Selection"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Output Inverse Selection This bit defines the relation between the internal shift data value and the output data signal of USCIx_DAT0/1 pins."]
    #[inline(always)]
    pub fn datoinv(&self) -> DATOINV_R {
        DATOINV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control Signal Output Inverse Selection This bit defines the relation between the internal control signal and the output control signal. Note: The control signal has different definitions in different protocol. In SPI protocol, the control signal means slave select signal."]
    #[inline(always)]
    pub fn ctloinv(&self) -> CTLOINV_R {
        CTLOINV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Word Length of Transmission This bit field defines the data word length (amount of bits) for reception and transmission. The data word is always right-aligned in the data buffer. USCI support word length from 4 to 16 bits. 0x0: The data word contains 16 bits located at bit positions \\[15:0\\]. 0x1: Reserved. 0x2: Reserved. 0x3: Reserved. 0x4: The data word contains 4 bits located at bit positions \\[3:0\\]. 0x5: The data word contains 5 bits located at bit positions \\[4:0\\]. ... 0xF: The data word contains 15 bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSB First Transmission Selection"]
    #[inline(always)]
    pub fn lsb(&mut self) -> LSB_W {
        LSB_W { w: self }
    }
    #[doc = "Bit 5 - Data Output Inverse Selection This bit defines the relation between the internal shift data value and the output data signal of USCIx_DAT0/1 pins."]
    #[inline(always)]
    pub fn datoinv(&mut self) -> DATOINV_W {
        DATOINV_W { w: self }
    }
    #[doc = "Bit 7 - Control Signal Output Inverse Selection This bit defines the relation between the internal control signal and the output control signal. Note: The control signal has different definitions in different protocol. In SPI protocol, the control signal means slave select signal."]
    #[inline(always)]
    pub fn ctloinv(&mut self) -> CTLOINV_W {
        CTLOINV_W { w: self }
    }
    #[doc = "Bits 8:11 - Word Length of Transmission This bit field defines the data word length (amount of bits) for reception and transmission. The data word is always right-aligned in the data buffer. USCI support word length from 4 to 16 bits. 0x0: The data word contains 16 bits located at bit positions \\[15:0\\]. 0x1: Reserved. 0x2: Reserved. 0x3: Reserved. 0x4: The data word contains 4 bits located at bit positions \\[3:0\\]. 0x5: The data word contains 5 bits located at bit positions \\[4:0\\]. ... 0xF: The data word contains 15 bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_linectl](index.html) module"]
pub struct USPI_LINECTL_SPEC;
impl crate::RegisterSpec for USPI_LINECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_linectl::R](R) reader structure"]
impl crate::Readable for USPI_LINECTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_linectl::W](W) writer structure"]
impl crate::Writable for USPI_LINECTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_LINECTL to value 0"]
impl crate::Resettable for USPI_LINECTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
