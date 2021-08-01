#[doc = "Register `BPWM_CAPIF` reader"]
pub struct R(crate::R<BPWM_CAPIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CAPIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CAPIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CAPIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CAPIF` writer"]
pub struct W(crate::W<BPWM_CAPIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CAPIF_SPEC>;
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
impl From<crate::W<BPWM_CAPIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CAPIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF0_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF0` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF0_R(crate::FieldReader<bool, CAPRIF0_A>);
impl CAPRIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF0_A {
        match self.bits {
            false => CAPRIF0_A::_0,
            true => CAPRIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF0_A::_1
    }
}
impl core::ops::Deref for CAPRIF0_R {
    type Target = crate::FieldReader<bool, CAPRIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF0` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF0_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF0_A::_1)
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
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF1_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF1` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF1_R(crate::FieldReader<bool, CAPRIF1_A>);
impl CAPRIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF1_A {
        match self.bits {
            false => CAPRIF1_A::_0,
            true => CAPRIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF1_A::_1
    }
}
impl core::ops::Deref for CAPRIF1_R {
    type Target = crate::FieldReader<bool, CAPRIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF1` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF1_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF1_A::_1)
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
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF2_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF2` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF2_R(crate::FieldReader<bool, CAPRIF2_A>);
impl CAPRIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF2_A {
        match self.bits {
            false => CAPRIF2_A::_0,
            true => CAPRIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF2_A::_1
    }
}
impl core::ops::Deref for CAPRIF2_R {
    type Target = crate::FieldReader<bool, CAPRIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF2` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF2_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF2_A::_1)
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
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF3_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF3` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF3_R(crate::FieldReader<bool, CAPRIF3_A>);
impl CAPRIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF3_A {
        match self.bits {
            false => CAPRIF3_A::_0,
            true => CAPRIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF3_A::_1
    }
}
impl core::ops::Deref for CAPRIF3_R {
    type Target = crate::FieldReader<bool, CAPRIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF3` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF3_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF3_A::_1)
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
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF4_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF4` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF4_R(crate::FieldReader<bool, CAPRIF4_A>);
impl CAPRIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF4_A {
        match self.bits {
            false => CAPRIF4_A::_0,
            true => CAPRIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF4_A::_1
    }
}
impl core::ops::Deref for CAPRIF4_R {
    type Target = crate::FieldReader<bool, CAPRIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF4` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF4_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF4_A::_1)
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
#[doc = "BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPRIF5_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPRIF5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPRIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPRIF5` reader - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF5_R(crate::FieldReader<bool, CAPRIF5_A>);
impl CAPRIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPRIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPRIF5_A {
        match self.bits {
            false => CAPRIF5_A::_0,
            true => CAPRIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIF5_A::_1
    }
}
impl core::ops::Deref for CAPRIF5_R {
    type Target = crate::FieldReader<bool, CAPRIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIF5` writer - BPWM Capture Rising Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPRIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIF5_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIF5_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF0_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF0` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF0_R(crate::FieldReader<bool, CAPFIF0_A>);
impl CAPFIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF0_A {
        match self.bits {
            false => CAPFIF0_A::_0,
            true => CAPFIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF0_A::_1
    }
}
impl core::ops::Deref for CAPFIF0_R {
    type Target = crate::FieldReader<bool, CAPFIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF0` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF0_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF0_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF1_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF1` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF1_R(crate::FieldReader<bool, CAPFIF1_A>);
impl CAPFIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF1_A {
        match self.bits {
            false => CAPFIF1_A::_0,
            true => CAPFIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF1_A::_1
    }
}
impl core::ops::Deref for CAPFIF1_R {
    type Target = crate::FieldReader<bool, CAPFIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF1` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF1_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF1_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF2_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF2` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF2_R(crate::FieldReader<bool, CAPFIF2_A>);
impl CAPFIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF2_A {
        match self.bits {
            false => CAPFIF2_A::_0,
            true => CAPFIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF2_A::_1
    }
}
impl core::ops::Deref for CAPFIF2_R {
    type Target = crate::FieldReader<bool, CAPFIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF2` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF2_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF2_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF3_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF3` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF3_R(crate::FieldReader<bool, CAPFIF3_A>);
impl CAPFIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF3_A {
        match self.bits {
            false => CAPFIF3_A::_0,
            true => CAPFIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF3_A::_1
    }
}
impl core::ops::Deref for CAPFIF3_R {
    type Target = crate::FieldReader<bool, CAPFIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF3` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF3_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF3_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF4_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF4` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF4_R(crate::FieldReader<bool, CAPFIF4_A>);
impl CAPFIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF4_A {
        match self.bits {
            false => CAPFIF4_A::_0,
            true => CAPFIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF4_A::_1
    }
}
impl core::ops::Deref for CAPFIF4_R {
    type Target = crate::FieldReader<bool, CAPFIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF4` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF4_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF4_A::_1)
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
#[doc = "BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFIF5_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CAPFIF5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFIF5` reader - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF5_R(crate::FieldReader<bool, CAPFIF5_A>);
impl CAPFIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFIF5_A {
        match self.bits {
            false => CAPFIF5_A::_0,
            true => CAPFIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIF5_A::_1
    }
}
impl core::ops::Deref for CAPFIF5_R {
    type Target = crate::FieldReader<bool, CAPFIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIF5` writer - BPWM Capture Falling Latch Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nNote: This bit is cleared by writing 1 to it."]
pub struct CAPFIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIF5_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIF5_A::_1)
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
    #[doc = "Bit 0 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif0(&self) -> CAPRIF0_R {
        CAPRIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif1(&self) -> CAPRIF1_R {
        CAPRIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif2(&self) -> CAPRIF2_R {
        CAPRIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif3(&self) -> CAPRIF3_R {
        CAPRIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif4(&self) -> CAPRIF4_R {
        CAPRIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif5(&self) -> CAPRIF5_R {
        CAPRIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif0(&self) -> CAPFIF0_R {
        CAPFIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif1(&self) -> CAPFIF1_R {
        CAPFIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif2(&self) -> CAPFIF2_R {
        CAPFIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif3(&self) -> CAPFIF3_R {
        CAPFIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif4(&self) -> CAPFIF4_R {
        CAPFIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif5(&self) -> CAPFIF5_R {
        CAPFIF5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif0(&mut self) -> CAPRIF0_W {
        CAPRIF0_W { w: self }
    }
    #[doc = "Bit 1 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif1(&mut self) -> CAPRIF1_W {
        CAPRIF1_W { w: self }
    }
    #[doc = "Bit 2 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif2(&mut self) -> CAPRIF2_W {
        CAPRIF2_W { w: self }
    }
    #[doc = "Bit 3 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif3(&mut self) -> CAPRIF3_W {
        CAPRIF3_W { w: self }
    }
    #[doc = "Bit 4 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif4(&mut self) -> CAPRIF4_W {
        CAPRIF4_W { w: self }
    }
    #[doc = "Bit 5 - BPWM Capture Rising Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn caprif5(&mut self) -> CAPRIF5_W {
        CAPRIF5_W { w: self }
    }
    #[doc = "Bit 8 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif0(&mut self) -> CAPFIF0_W {
        CAPFIF0_W { w: self }
    }
    #[doc = "Bit 9 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif1(&mut self) -> CAPFIF1_W {
        CAPFIF1_W { w: self }
    }
    #[doc = "Bit 10 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif2(&mut self) -> CAPFIF2_W {
        CAPFIF2_W { w: self }
    }
    #[doc = "Bit 11 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif3(&mut self) -> CAPFIF3_W {
        CAPFIF3_W { w: self }
    }
    #[doc = "Bit 12 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif4(&mut self) -> CAPFIF4_W {
        CAPFIF4_W { w: self }
    }
    #[doc = "Bit 13 - BPWM Capture Falling Latch Interrupt Flag Each bit n controls the corresponding BPWM channel n. Note: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn capfif5(&mut self) -> CAPFIF5_W {
        CAPFIF5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Capture Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_capif](index.html) module"]
pub struct BPWM_CAPIF_SPEC;
impl crate::RegisterSpec for BPWM_CAPIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_capif::R](R) reader structure"]
impl crate::Readable for BPWM_CAPIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_capif::W](W) writer structure"]
impl crate::Writable for BPWM_CAPIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CAPIF to value 0"]
impl crate::Resettable for BPWM_CAPIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
