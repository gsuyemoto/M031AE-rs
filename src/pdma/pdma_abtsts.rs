#[doc = "Register `PDMA_ABTSTS` reader"]
pub struct R(crate::R<PDMA_ABTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_ABTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_ABTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_ABTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_ABTSTS` writer"]
pub struct W(crate::W<PDMA_ABTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_ABTSTS_SPEC>;
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
impl From<crate::W<PDMA_ABTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_ABTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF0_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF0` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF0_R(crate::FieldReader<bool, ABTIF0_A>);
impl ABTIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF0_A {
        match self.bits {
            false => ABTIF0_A::_0,
            true => ABTIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF0_A::_1
    }
}
impl core::ops::Deref for ABTIF0_R {
    type Target = crate::FieldReader<bool, ABTIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF0` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF0_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF0_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF1_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF1` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF1_R(crate::FieldReader<bool, ABTIF1_A>);
impl ABTIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF1_A {
        match self.bits {
            false => ABTIF1_A::_0,
            true => ABTIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF1_A::_1
    }
}
impl core::ops::Deref for ABTIF1_R {
    type Target = crate::FieldReader<bool, ABTIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF1` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF1_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF1_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF2_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF2_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF2` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF2_R(crate::FieldReader<bool, ABTIF2_A>);
impl ABTIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF2_A {
        match self.bits {
            false => ABTIF2_A::_0,
            true => ABTIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF2_A::_1
    }
}
impl core::ops::Deref for ABTIF2_R {
    type Target = crate::FieldReader<bool, ABTIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF2` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF2_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF2_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF3_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF3_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF3` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF3_R(crate::FieldReader<bool, ABTIF3_A>);
impl ABTIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF3_A {
        match self.bits {
            false => ABTIF3_A::_0,
            true => ABTIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF3_A::_1
    }
}
impl core::ops::Deref for ABTIF3_R {
    type Target = crate::FieldReader<bool, ABTIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF3` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF3_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF3_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF4_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF4` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF4_R(crate::FieldReader<bool, ABTIF4_A>);
impl ABTIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF4_A {
        match self.bits {
            false => ABTIF4_A::_0,
            true => ABTIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF4_A::_1
    }
}
impl core::ops::Deref for ABTIF4_R {
    type Target = crate::FieldReader<bool, ABTIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF4` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF4_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF4_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF5_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF5_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF5` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF5_R(crate::FieldReader<bool, ABTIF5_A>);
impl ABTIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF5_A {
        match self.bits {
            false => ABTIF5_A::_0,
            true => ABTIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF5_A::_1
    }
}
impl core::ops::Deref for ABTIF5_R {
    type Target = crate::FieldReader<bool, ABTIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF5` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF5_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF5_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF6_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF6_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF6` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF6_R(crate::FieldReader<bool, ABTIF6_A>);
impl ABTIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF6_A {
        match self.bits {
            false => ABTIF6_A::_0,
            true => ABTIF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF6_A::_1
    }
}
impl core::ops::Deref for ABTIF6_R {
    type Target = crate::FieldReader<bool, ABTIF6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF6` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF6_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF7_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF7` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF7_R(crate::FieldReader<bool, ABTIF7_A>);
impl ABTIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF7_A {
        match self.bits {
            false => ABTIF7_A::_0,
            true => ABTIF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF7_A::_1
    }
}
impl core::ops::Deref for ABTIF7_R {
    type Target = crate::FieldReader<bool, ABTIF7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF7` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF7_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF7_A::_1)
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
#[doc = "PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF8_A {
    #[doc = "0: No AHB bus ERROR response received when channel n transfer"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received when channel n transfer"]
    _1 = 1,
}
impl From<ABTIF8_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF8` reader - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF8_R(crate::FieldReader<bool, ABTIF8_A>);
impl ABTIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF8_A {
        match self.bits {
            false => ABTIF8_A::_0,
            true => ABTIF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF8_A::_1
    }
}
impl core::ops::Deref for ABTIF8_R {
    type Target = crate::FieldReader<bool, ABTIF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF8` writer - PDMA Read/Write Target Abort Interrupt Status Flag\nThis bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
pub struct ABTIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF8_A::_0)
    }
    #[doc = "AHB bus ERROR response received when channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF8_A::_1)
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
impl R {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif0(&self) -> ABTIF0_R {
        ABTIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif1(&self) -> ABTIF1_R {
        ABTIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif2(&self) -> ABTIF2_R {
        ABTIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif3(&self) -> ABTIF3_R {
        ABTIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif4(&self) -> ABTIF4_R {
        ABTIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif5(&self) -> ABTIF5_R {
        ABTIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif6(&self) -> ABTIF6_R {
        ABTIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif7(&self) -> ABTIF7_R {
        ABTIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif8(&self) -> ABTIF8_R {
        ABTIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif0(&mut self) -> ABTIF0_W {
        ABTIF0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif1(&mut self) -> ABTIF1_W {
        ABTIF1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif2(&mut self) -> ABTIF2_W {
        ABTIF2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif3(&mut self) -> ABTIF3_W {
        ABTIF3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif4(&mut self) -> ABTIF4_W {
        ABTIF4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif5(&mut self) -> ABTIF5_W {
        ABTIF5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif6(&mut self) -> ABTIF6_W {
        ABTIF6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif7(&mut self) -> ABTIF7_W {
        ABTIF7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Read/Write Target Abort Interrupt Status Flag This bit indicates which PDMA controller has target abort error; User can write 1 to clear these bits."]
    #[inline(always)]
    pub fn abtif8(&mut self) -> ABTIF8_W {
        ABTIF8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Channel Read/Write Target Abort Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_abtsts](index.html) module"]
pub struct PDMA_ABTSTS_SPEC;
impl crate::RegisterSpec for PDMA_ABTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_abtsts::R](R) reader structure"]
impl crate::Readable for PDMA_ABTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_abtsts::W](W) writer structure"]
impl crate::Writable for PDMA_ABTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_ABTSTS to value 0"]
impl crate::Resettable for PDMA_ABTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
