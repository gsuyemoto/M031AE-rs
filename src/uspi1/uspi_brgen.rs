#[doc = "Register `USPI_BRGEN` reader"]
pub struct R(crate::R<USPI_BRGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_BRGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_BRGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_BRGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_BRGEN` writer"]
pub struct W(crate::W<USPI_BRGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_BRGEN_SPEC>;
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
impl From<crate::W<USPI_BRGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_BRGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reference Clock Source Selection\nThis bit selects the source of reference clock (fREF_CLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCLKSEL_A {
    #[doc = "0: Peripheral device clock fPCLK"]
    _0 = 0,
    #[doc = "1: Reserved."]
    _1 = 1,
}
impl From<RCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCLKSEL` reader - Reference Clock Source Selection\nThis bit selects the source of reference clock (fREF_CLK)."]
pub struct RCLKSEL_R(crate::FieldReader<bool, RCLKSEL_A>);
impl RCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCLKSEL_A {
        match self.bits {
            false => RCLKSEL_A::_0,
            true => RCLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCLKSEL_A::_1
    }
}
impl core::ops::Deref for RCLKSEL_R {
    type Target = crate::FieldReader<bool, RCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCLKSEL` writer - Reference Clock Source Selection\nThis bit selects the source of reference clock (fREF_CLK)."]
pub struct RCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Peripheral device clock fPCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCLKSEL_A::_0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCLKSEL_A::_1)
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
#[doc = "Protocol Clock Source Selection\nThis bit selects the source of protocol clock (fPROT_CLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCLKSEL_A {
    #[doc = "0: Reference clock fREF_CLK"]
    _0 = 0,
    #[doc = "1: fREF_CLK2 (its frequency is half of fREF_CLK)"]
    _1 = 1,
}
impl From<PTCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PTCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCLKSEL` reader - Protocol Clock Source Selection\nThis bit selects the source of protocol clock (fPROT_CLK)."]
pub struct PTCLKSEL_R(crate::FieldReader<bool, PTCLKSEL_A>);
impl PTCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCLKSEL_A {
        match self.bits {
            false => PTCLKSEL_A::_0,
            true => PTCLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCLKSEL_A::_1
    }
}
impl core::ops::Deref for PTCLKSEL_R {
    type Target = crate::FieldReader<bool, PTCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCLKSEL` writer - Protocol Clock Source Selection\nThis bit selects the source of protocol clock (fPROT_CLK)."]
pub struct PTCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reference clock fREF_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCLKSEL_A::_0)
    }
    #[doc = "fREF_CLK2 (its frequency is half of fREF_CLK)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCLKSEL_A::_1)
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
#[doc = "Sample Clock Source Selection\nThis bit field used for the clock source selection of sample clock (fSAMP_CLK) for the protocol processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPCLKSEL_A {
    #[doc = "0: fDIV_CLK"]
    _0 = 0,
    #[doc = "1: fPROT_CLK"]
    _1 = 1,
    #[doc = "2: fSCLK"]
    _2 = 2,
    #[doc = "3: fREF_CLK"]
    _3 = 3,
}
impl From<SPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPCLKSEL` reader - Sample Clock Source Selection\nThis bit field used for the clock source selection of sample clock (fSAMP_CLK) for the protocol processor."]
pub struct SPCLKSEL_R(crate::FieldReader<u8, SPCLKSEL_A>);
impl SPCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCLKSEL_A {
        match self.bits {
            0 => SPCLKSEL_A::_0,
            1 => SPCLKSEL_A::_1,
            2 => SPCLKSEL_A::_2,
            3 => SPCLKSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SPCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SPCLKSEL_A::_3
    }
}
impl core::ops::Deref for SPCLKSEL_R {
    type Target = crate::FieldReader<u8, SPCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPCLKSEL` writer - Sample Clock Source Selection\nThis bit field used for the clock source selection of sample clock (fSAMP_CLK) for the protocol processor."]
pub struct SPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "fDIV_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_0)
    }
    #[doc = "fPROT_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_1)
    }
    #[doc = "fSCLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_2)
    }
    #[doc = "fREF_CLK"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Time Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMCNTEN_A {
    #[doc = "0: Time measurement counter Disabled"]
    _0 = 0,
    #[doc = "1: Time measurement counter Enabled"]
    _1 = 1,
}
impl From<TMCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCNTEN` reader - Time Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter."]
pub struct TMCNTEN_R(crate::FieldReader<bool, TMCNTEN_A>);
impl TMCNTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMCNTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMCNTEN_A {
        match self.bits {
            false => TMCNTEN_A::_0,
            true => TMCNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMCNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMCNTEN_A::_1
    }
}
impl core::ops::Deref for TMCNTEN_R {
    type Target = crate::FieldReader<bool, TMCNTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMCNTEN` writer - Time Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter."]
pub struct TMCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMCNTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time measurement counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMCNTEN_A::_0)
    }
    #[doc = "Time measurement counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMCNTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Time Measurement Counter Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMCNTSRC_A {
    #[doc = "0: Time measurement counter with fPROT_CLK"]
    _0 = 0,
    #[doc = "1: Time measurement counter with fDIV_CLK"]
    _1 = 1,
}
impl From<TMCNTSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TMCNTSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCNTSRC` reader - Time Measurement Counter Clock Source Selection"]
pub struct TMCNTSRC_R(crate::FieldReader<bool, TMCNTSRC_A>);
impl TMCNTSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMCNTSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMCNTSRC_A {
        match self.bits {
            false => TMCNTSRC_A::_0,
            true => TMCNTSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMCNTSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMCNTSRC_A::_1
    }
}
impl core::ops::Deref for TMCNTSRC_R {
    type Target = crate::FieldReader<bool, TMCNTSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMCNTSRC` writer - Time Measurement Counter Clock Source Selection"]
pub struct TMCNTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMCNTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMCNTSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time measurement counter with fPROT_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMCNTSRC_A::_0)
    }
    #[doc = "Time measurement counter with fDIV_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMCNTSRC_A::_1)
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
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub struct CLKDIV_R(crate::FieldReader<u16, u16>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reference Clock Source Selection This bit selects the source of reference clock (fREF_CLK)."]
    #[inline(always)]
    pub fn rclksel(&self) -> RCLKSEL_R {
        RCLKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protocol Clock Source Selection This bit selects the source of protocol clock (fPROT_CLK)."]
    #[inline(always)]
    pub fn ptclksel(&self) -> PTCLKSEL_R {
        PTCLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Sample Clock Source Selection This bit field used for the clock source selection of sample clock (fSAMP_CLK) for the protocol processor."]
    #[inline(always)]
    pub fn spclksel(&self) -> SPCLKSEL_R {
        SPCLKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Time Measurement Counter Enable Bit This bit enables the 10-bit timing measurement counter."]
    #[inline(always)]
    pub fn tmcnten(&self) -> TMCNTEN_R {
        TMCNTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Time Measurement Counter Clock Source Selection"]
    #[inline(always)]
    pub fn tmcntsrc(&self) -> TMCNTSRC_R {
        TMCNTSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Clock Source Selection This bit selects the source of reference clock (fREF_CLK)."]
    #[inline(always)]
    pub fn rclksel(&mut self) -> RCLKSEL_W {
        RCLKSEL_W { w: self }
    }
    #[doc = "Bit 1 - Protocol Clock Source Selection This bit selects the source of protocol clock (fPROT_CLK)."]
    #[inline(always)]
    pub fn ptclksel(&mut self) -> PTCLKSEL_W {
        PTCLKSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Sample Clock Source Selection This bit field used for the clock source selection of sample clock (fSAMP_CLK) for the protocol processor."]
    #[inline(always)]
    pub fn spclksel(&mut self) -> SPCLKSEL_W {
        SPCLKSEL_W { w: self }
    }
    #[doc = "Bit 4 - Time Measurement Counter Enable Bit This bit enables the 10-bit timing measurement counter."]
    #[inline(always)]
    pub fn tmcnten(&mut self) -> TMCNTEN_W {
        TMCNTEN_W { w: self }
    }
    #[doc = "Bit 5 - Time Measurement Counter Clock Source Selection"]
    #[inline(always)]
    pub fn tmcntsrc(&mut self) -> TMCNTSRC_W {
        TMCNTSRC_W { w: self }
    }
    #[doc = "Bits 16:25 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_brgen](index.html) module"]
pub struct USPI_BRGEN_SPEC;
impl crate::RegisterSpec for USPI_BRGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_brgen::R](R) reader structure"]
impl crate::Readable for USPI_BRGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_brgen::W](W) writer structure"]
impl crate::Writable for USPI_BRGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_BRGEN to value 0x3c00"]
impl crate::Resettable for USPI_BRGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c00
    }
}
