#[doc = "Register `UUART_BRGEN` reader"]
pub struct R(crate::R<UUART_BRGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_BRGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_BRGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_BRGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_BRGEN` writer"]
pub struct W(crate::W<UUART_BRGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_BRGEN_SPEC>;
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
impl From<crate::W<UUART_BRGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_BRGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reference Clock Source Selection\nThis bit selects the source signal of reference clock (fREF_CLK).\n\nValue on reset: 0"]
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
#[doc = "Field `RCLKSEL` reader - Reference Clock Source Selection\nThis bit selects the source signal of reference clock (fREF_CLK)."]
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
#[doc = "Field `RCLKSEL` writer - Reference Clock Source Selection\nThis bit selects the source signal of reference clock (fREF_CLK)."]
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
#[doc = "Protocol Clock Source Selection\nThis bit selects the source signal of protocol clock (fPROT_CLK).\n\nValue on reset: 0"]
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
#[doc = "Field `PTCLKSEL` reader - Protocol Clock Source Selection\nThis bit selects the source signal of protocol clock (fPROT_CLK)."]
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
#[doc = "Field `PTCLKSEL` writer - Protocol Clock Source Selection\nThis bit selects the source signal of protocol clock (fPROT_CLK)."]
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
#[doc = "Sample Clock Source Selection\nThis bit field used for the clock source selection of a sample clock (fSAMP_CLK) for the protocol processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPCLKSEL_A {
    #[doc = "0: fSAMP_CLK = fDIV_CLK"]
    _0 = 0,
    #[doc = "1: fSAMP_CLK = fPROT_CLK"]
    _1 = 1,
    #[doc = "2: fSAMP_CLK = fSCLK"]
    _2 = 2,
    #[doc = "3: fSAMP_CLK = fREF_CLK"]
    _3 = 3,
}
impl From<SPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPCLKSEL` reader - Sample Clock Source Selection\nThis bit field used for the clock source selection of a sample clock (fSAMP_CLK) for the protocol processor."]
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
#[doc = "Field `SPCLKSEL` writer - Sample Clock Source Selection\nThis bit field used for the clock source selection of a sample clock (fSAMP_CLK) for the protocol processor."]
pub struct SPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "fSAMP_CLK = fDIV_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_0)
    }
    #[doc = "fSAMP_CLK = fPROT_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_1)
    }
    #[doc = "fSAMP_CLK = fSCLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SPCLKSEL_A::_2)
    }
    #[doc = "fSAMP_CLK = fREF_CLK"]
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
#[doc = "Timing Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMCNTEN_A {
    #[doc = "0: Timing measurement counter is Disabled"]
    _0 = 0,
    #[doc = "1: Timing measurement counter is Enabled"]
    _1 = 1,
}
impl From<TMCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCNTEN` reader - Timing Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter."]
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
#[doc = "Field `TMCNTEN` writer - Timing Measurement Counter Enable Bit\nThis bit enables the 10-bit timing measurement counter."]
pub struct TMCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMCNTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timing measurement counter is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMCNTEN_A::_0)
    }
    #[doc = "Timing measurement counter is Enabled"]
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
#[doc = "Timing Measurement Counter Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMCNTSRC_A {
    #[doc = "0: Timing measurement counter with fPROT_CLK"]
    _0 = 0,
    #[doc = "1: Timing measurement counter with fDIV_CLK"]
    _1 = 1,
}
impl From<TMCNTSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TMCNTSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCNTSRC` reader - Timing Measurement Counter Clock Source Selection"]
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
#[doc = "Field `TMCNTSRC` writer - Timing Measurement Counter Clock Source Selection"]
pub struct TMCNTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMCNTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMCNTSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timing measurement counter with fPROT_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMCNTSRC_A::_0)
    }
    #[doc = "Timing measurement counter with fDIV_CLK"]
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
#[doc = "Field `PDSCNT` reader - Pre-divider for Sample Counter"]
pub struct PDSCNT_R(crate::FieldReader<u8, u8>);
impl PDSCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDSCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDSCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDSCNT` writer - Pre-divider for Sample Counter"]
pub struct PDSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DSCNT` reader - Denominator for Sample Counter\nThis bit field defines the divide ratio of the sample clock fSAMP_CLK.\nNote: The maximum value of DSCNT is 0xF on UART mode and suggest to set over 4 to confirm the receiver data is sampled in right value."]
pub struct DSCNT_R(crate::FieldReader<u8, u8>);
impl DSCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSCNT` writer - Denominator for Sample Counter\nThis bit field defines the divide ratio of the sample clock fSAMP_CLK.\nNote: The maximum value of DSCNT is 0xF on UART mode and suggest to set over 4 to confirm the receiver data is sampled in right value."]
pub struct DSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - Clock Divider\nNote: In UART function, it can be updated by hardware in the 4th falling edge of the input data 0x55 when the auto baud rate function (ABREN(UUART_PROTCTL\\[6\\])) is enabled. The revised value is the average bit time between bit 5 and bit 6. The user can use revised CLKDIV and new BRDETITV (UUART_PROTCTL\\[24:16\\]) to calculate the precise baud rate."]
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
#[doc = "Field `CLKDIV` writer - Clock Divider\nNote: In UART function, it can be updated by hardware in the 4th falling edge of the input data 0x55 when the auto baud rate function (ABREN(UUART_PROTCTL\\[6\\])) is enabled. The revised value is the average bit time between bit 5 and bit 6. The user can use revised CLKDIV and new BRDETITV (UUART_PROTCTL\\[24:16\\]) to calculate the precise baud rate."]
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
    #[doc = "Bit 0 - Reference Clock Source Selection This bit selects the source signal of reference clock (fREF_CLK)."]
    #[inline(always)]
    pub fn rclksel(&self) -> RCLKSEL_R {
        RCLKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protocol Clock Source Selection This bit selects the source signal of protocol clock (fPROT_CLK)."]
    #[inline(always)]
    pub fn ptclksel(&self) -> PTCLKSEL_R {
        PTCLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Sample Clock Source Selection This bit field used for the clock source selection of a sample clock (fSAMP_CLK) for the protocol processor."]
    #[inline(always)]
    pub fn spclksel(&self) -> SPCLKSEL_R {
        SPCLKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Timing Measurement Counter Enable Bit This bit enables the 10-bit timing measurement counter."]
    #[inline(always)]
    pub fn tmcnten(&self) -> TMCNTEN_R {
        TMCNTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timing Measurement Counter Clock Source Selection"]
    #[inline(always)]
    pub fn tmcntsrc(&self) -> TMCNTSRC_R {
        TMCNTSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Pre-divider for Sample Counter"]
    #[inline(always)]
    pub fn pdscnt(&self) -> PDSCNT_R {
        PDSCNT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:14 - Denominator for Sample Counter This bit field defines the divide ratio of the sample clock fSAMP_CLK. Note: The maximum value of DSCNT is 0xF on UART mode and suggest to set over 4 to confirm the receiver data is sampled in right value."]
    #[inline(always)]
    pub fn dscnt(&self) -> DSCNT_R {
        DSCNT_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Clock Divider Note: In UART function, it can be updated by hardware in the 4th falling edge of the input data 0x55 when the auto baud rate function (ABREN(UUART_PROTCTL\\[6\\])) is enabled. The revised value is the average bit time between bit 5 and bit 6. The user can use revised CLKDIV and new BRDETITV (UUART_PROTCTL\\[24:16\\]) to calculate the precise baud rate."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Clock Source Selection This bit selects the source signal of reference clock (fREF_CLK)."]
    #[inline(always)]
    pub fn rclksel(&mut self) -> RCLKSEL_W {
        RCLKSEL_W { w: self }
    }
    #[doc = "Bit 1 - Protocol Clock Source Selection This bit selects the source signal of protocol clock (fPROT_CLK)."]
    #[inline(always)]
    pub fn ptclksel(&mut self) -> PTCLKSEL_W {
        PTCLKSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Sample Clock Source Selection This bit field used for the clock source selection of a sample clock (fSAMP_CLK) for the protocol processor."]
    #[inline(always)]
    pub fn spclksel(&mut self) -> SPCLKSEL_W {
        SPCLKSEL_W { w: self }
    }
    #[doc = "Bit 4 - Timing Measurement Counter Enable Bit This bit enables the 10-bit timing measurement counter."]
    #[inline(always)]
    pub fn tmcnten(&mut self) -> TMCNTEN_W {
        TMCNTEN_W { w: self }
    }
    #[doc = "Bit 5 - Timing Measurement Counter Clock Source Selection"]
    #[inline(always)]
    pub fn tmcntsrc(&mut self) -> TMCNTSRC_W {
        TMCNTSRC_W { w: self }
    }
    #[doc = "Bits 8:9 - Pre-divider for Sample Counter"]
    #[inline(always)]
    pub fn pdscnt(&mut self) -> PDSCNT_W {
        PDSCNT_W { w: self }
    }
    #[doc = "Bits 10:14 - Denominator for Sample Counter This bit field defines the divide ratio of the sample clock fSAMP_CLK. Note: The maximum value of DSCNT is 0xF on UART mode and suggest to set over 4 to confirm the receiver data is sampled in right value."]
    #[inline(always)]
    pub fn dscnt(&mut self) -> DSCNT_W {
        DSCNT_W { w: self }
    }
    #[doc = "Bits 16:25 - Clock Divider Note: In UART function, it can be updated by hardware in the 4th falling edge of the input data 0x55 when the auto baud rate function (ABREN(UUART_PROTCTL\\[6\\])) is enabled. The revised value is the average bit time between bit 5 and bit 6. The user can use revised CLKDIV and new BRDETITV (UUART_PROTCTL\\[24:16\\]) to calculate the precise baud rate."]
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
#[doc = "USCI Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_brgen](index.html) module"]
pub struct UUART_BRGEN_SPEC;
impl crate::RegisterSpec for UUART_BRGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_brgen::R](R) reader structure"]
impl crate::Readable for UUART_BRGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_brgen::W](W) writer structure"]
impl crate::Writable for UUART_BRGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_BRGEN to value 0x3c00"]
impl crate::Resettable for UUART_BRGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c00
    }
}
