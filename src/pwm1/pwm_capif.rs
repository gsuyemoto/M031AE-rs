#[doc = "Register `PWM_CAPIF` reader"]
pub struct R(crate::R<PWM_CAPIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CAPIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CAPIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CAPIF` writer"]
pub struct W(crate::W<PWM_CAPIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CAPIF_SPEC>;
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
impl From<crate::W<PWM_CAPIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CAPIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF0_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF0` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF0_R(crate::FieldReader<bool, CRLIF0_A>);
impl CRLIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF0_A {
        match self.bits {
            false => CRLIF0_A::_0,
            true => CRLIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF0_A::_1
    }
}
impl core::ops::Deref for CRLIF0_R {
    type Target = crate::FieldReader<bool, CRLIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF0` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF0_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF0_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF1_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF1` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF1_R(crate::FieldReader<bool, CRLIF1_A>);
impl CRLIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF1_A {
        match self.bits {
            false => CRLIF1_A::_0,
            true => CRLIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF1_A::_1
    }
}
impl core::ops::Deref for CRLIF1_R {
    type Target = crate::FieldReader<bool, CRLIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF1` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF1_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF1_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF2_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF2_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF2` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF2_R(crate::FieldReader<bool, CRLIF2_A>);
impl CRLIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF2_A {
        match self.bits {
            false => CRLIF2_A::_0,
            true => CRLIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF2_A::_1
    }
}
impl core::ops::Deref for CRLIF2_R {
    type Target = crate::FieldReader<bool, CRLIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF2` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF2_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF2_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF3_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF3_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF3` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF3_R(crate::FieldReader<bool, CRLIF3_A>);
impl CRLIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF3_A {
        match self.bits {
            false => CRLIF3_A::_0,
            true => CRLIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF3_A::_1
    }
}
impl core::ops::Deref for CRLIF3_R {
    type Target = crate::FieldReader<bool, CRLIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF3` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF3_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF3_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF4_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF4_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF4` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF4_R(crate::FieldReader<bool, CRLIF4_A>);
impl CRLIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF4_A {
        match self.bits {
            false => CRLIF4_A::_0,
            true => CRLIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF4_A::_1
    }
}
impl core::ops::Deref for CRLIF4_R {
    type Target = crate::FieldReader<bool, CRLIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF4` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF4_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF4_A::_1)
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
#[doc = "PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLIF5_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIF5_A> for bool {
    #[inline(always)]
    fn from(variant: CRLIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRLIF5` reader - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF5_R(crate::FieldReader<bool, CRLIF5_A>);
impl CRLIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRLIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRLIF5_A {
        match self.bits {
            false => CRLIF5_A::_0,
            true => CRLIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIF5_A::_1
    }
}
impl core::ops::Deref for CRLIF5_R {
    type Target = crate::FieldReader<bool, CRLIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIF5` writer - PWM Capture Rising Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CRLIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIF5_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIF5_A::_1)
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
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF0_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF0` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF0_R(crate::FieldReader<bool, CFLIF0_A>);
impl CFLIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF0_A {
        match self.bits {
            false => CFLIF0_A::_0,
            true => CFLIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF0_A::_1
    }
}
impl core::ops::Deref for CFLIF0_R {
    type Target = crate::FieldReader<bool, CFLIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF0` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF0_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF1_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF1` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF1_R(crate::FieldReader<bool, CFLIF1_A>);
impl CFLIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF1_A {
        match self.bits {
            false => CFLIF1_A::_0,
            true => CFLIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF1_A::_1
    }
}
impl core::ops::Deref for CFLIF1_R {
    type Target = crate::FieldReader<bool, CFLIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF1` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF1_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF2_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF2_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF2` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF2_R(crate::FieldReader<bool, CFLIF2_A>);
impl CFLIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF2_A {
        match self.bits {
            false => CFLIF2_A::_0,
            true => CFLIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF2_A::_1
    }
}
impl core::ops::Deref for CFLIF2_R {
    type Target = crate::FieldReader<bool, CFLIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF2` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF2_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF3_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF3_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF3` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF3_R(crate::FieldReader<bool, CFLIF3_A>);
impl CFLIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF3_A {
        match self.bits {
            false => CFLIF3_A::_0,
            true => CFLIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF3_A::_1
    }
}
impl core::ops::Deref for CFLIF3_R {
    type Target = crate::FieldReader<bool, CFLIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF3` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF3_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF4_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF4_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF4` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF4_R(crate::FieldReader<bool, CFLIF4_A>);
impl CFLIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF4_A {
        match self.bits {
            false => CFLIF4_A::_0,
            true => CFLIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF4_A::_1
    }
}
impl core::ops::Deref for CFLIF4_R {
    type Target = crate::FieldReader<bool, CFLIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF4` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF4_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFLIF5_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIF5_A> for bool {
    #[inline(always)]
    fn from(variant: CFLIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLIF5` reader - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF5_R(crate::FieldReader<bool, CFLIF5_A>);
impl CFLIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFLIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFLIF5_A {
        match self.bits {
            false => CFLIF5_A::_0,
            true => CFLIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIF5_A::_1
    }
}
impl core::ops::Deref for CFLIF5_R {
    type Target = crate::FieldReader<bool, CFLIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIF5` writer - PWM Capture Falling Latch Interrupt Flag\nNote 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data.\nNote 2: This bit is cleared by writing 1 to it."]
pub struct CFLIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIF5_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIF5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif0(&self) -> CRLIF0_R {
        CRLIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif1(&self) -> CRLIF1_R {
        CRLIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif2(&self) -> CRLIF2_R {
        CRLIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif3(&self) -> CRLIF3_R {
        CRLIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif4(&self) -> CRLIF4_R {
        CRLIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif5(&self) -> CRLIF5_R {
        CRLIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif0(&self) -> CFLIF0_R {
        CFLIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif1(&self) -> CFLIF1_R {
        CFLIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif2(&self) -> CFLIF2_R {
        CFLIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif3(&self) -> CFLIF3_R {
        CFLIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif4(&self) -> CFLIF4_R {
        CFLIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif5(&self) -> CFLIF5_R {
        CFLIF5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif0(&mut self) -> CRLIF0_W {
        CRLIF0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif1(&mut self) -> CRLIF1_W {
        CRLIF1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif2(&mut self) -> CRLIF2_W {
        CRLIF2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif3(&mut self) -> CRLIF3_W {
        CRLIF3_W { w: self }
    }
    #[doc = "Bit 4 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif4(&mut self) -> CRLIF4_W {
        CRLIF4_W { w: self }
    }
    #[doc = "Bit 5 - PWM Capture Rising Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CRLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn crlif5(&mut self) -> CRLIF5_W {
        CRLIF5_W { w: self }
    }
    #[doc = "Bit 8 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif0(&mut self) -> CFLIF0_W {
        CFLIF0_W { w: self }
    }
    #[doc = "Bit 9 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif1(&mut self) -> CFLIF1_W {
        CFLIF1_W { w: self }
    }
    #[doc = "Bit 10 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif2(&mut self) -> CFLIF2_W {
        CFLIF2_W { w: self }
    }
    #[doc = "Bit 11 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif3(&mut self) -> CFLIF3_W {
        CFLIF3_W { w: self }
    }
    #[doc = "Bit 12 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif4(&mut self) -> CFLIF4_W {
        CFLIF4_W { w: self }
    }
    #[doc = "Bit 13 - PWM Capture Falling Latch Interrupt Flag Note 1: When Capture with PDMA operating, CAPIF corresponding channel CFLIF will be cleared by hardware after PDMA transfer data. Note 2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn cflif5(&mut self) -> CFLIF5_W {
        CFLIF5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Capture Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capif](index.html) module"]
pub struct PWM_CAPIF_SPEC;
impl crate::RegisterSpec for PWM_CAPIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capif::R](R) reader structure"]
impl crate::Readable for PWM_CAPIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_capif::W](W) writer structure"]
impl crate::Writable for PWM_CAPIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CAPIF to value 0"]
impl crate::Resettable for PWM_CAPIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
