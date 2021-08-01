#[doc = "Register `PDMA_TDSTS` reader"]
pub struct R(crate::R<PDMA_TDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TDSTS` writer"]
pub struct W(crate::W<PDMA_TDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TDSTS_SPEC>;
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
impl From<crate::W<PDMA_TDSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_TDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF0_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF0` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF0_R(crate::FieldReader<bool, TDIF0_A>);
impl TDIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF0_A {
        match self.bits {
            false => TDIF0_A::_0,
            true => TDIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF0_A::_1
    }
}
impl core::ops::Deref for TDIF0_R {
    type Target = crate::FieldReader<bool, TDIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF0` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF0_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF0_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF1_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF1` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF1_R(crate::FieldReader<bool, TDIF1_A>);
impl TDIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF1_A {
        match self.bits {
            false => TDIF1_A::_0,
            true => TDIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF1_A::_1
    }
}
impl core::ops::Deref for TDIF1_R {
    type Target = crate::FieldReader<bool, TDIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF1` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF1_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF1_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF2_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF2` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF2_R(crate::FieldReader<bool, TDIF2_A>);
impl TDIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF2_A {
        match self.bits {
            false => TDIF2_A::_0,
            true => TDIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF2_A::_1
    }
}
impl core::ops::Deref for TDIF2_R {
    type Target = crate::FieldReader<bool, TDIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF2` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF2_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF2_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF3_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF3` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF3_R(crate::FieldReader<bool, TDIF3_A>);
impl TDIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF3_A {
        match self.bits {
            false => TDIF3_A::_0,
            true => TDIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF3_A::_1
    }
}
impl core::ops::Deref for TDIF3_R {
    type Target = crate::FieldReader<bool, TDIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF3` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF3_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF3_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF4_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF4` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF4_R(crate::FieldReader<bool, TDIF4_A>);
impl TDIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF4_A {
        match self.bits {
            false => TDIF4_A::_0,
            true => TDIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF4_A::_1
    }
}
impl core::ops::Deref for TDIF4_R {
    type Target = crate::FieldReader<bool, TDIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF4` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF4_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF4_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF5_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF5_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF5` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF5_R(crate::FieldReader<bool, TDIF5_A>);
impl TDIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF5_A {
        match self.bits {
            false => TDIF5_A::_0,
            true => TDIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF5_A::_1
    }
}
impl core::ops::Deref for TDIF5_R {
    type Target = crate::FieldReader<bool, TDIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF5` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF5_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF5_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF6_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF6_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF6` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF6_R(crate::FieldReader<bool, TDIF6_A>);
impl TDIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF6_A {
        match self.bits {
            false => TDIF6_A::_0,
            true => TDIF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF6_A::_1
    }
}
impl core::ops::Deref for TDIF6_R {
    type Target = crate::FieldReader<bool, TDIF6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF6` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF6_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF6_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF7_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF7` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF7_R(crate::FieldReader<bool, TDIF7_A>);
impl TDIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF7_A {
        match self.bits {
            false => TDIF7_A::_0,
            true => TDIF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF7_A::_1
    }
}
impl core::ops::Deref for TDIF7_R {
    type Target = crate::FieldReader<bool, TDIF7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF7` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF7_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF7_A::_1)
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
#[doc = "Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF8_A {
    #[doc = "0: PDMA channel transfer has not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF8_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF8` reader - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF8_R(crate::FieldReader<bool, TDIF8_A>);
impl TDIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF8_A {
        match self.bits {
            false => TDIF8_A::_0,
            true => TDIF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF8_A::_1
    }
}
impl core::ops::Deref for TDIF8_R {
    type Target = crate::FieldReader<bool, TDIF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIF8` writer - Transfer Done Flag\nThis bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
pub struct TDIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIF8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel transfer has not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIF8_A::_0)
    }
    #[doc = "PDMA channel has finished transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIF8_A::_1)
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
    #[doc = "Bit 0 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif0(&self) -> TDIF0_R {
        TDIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif1(&self) -> TDIF1_R {
        TDIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif2(&self) -> TDIF2_R {
        TDIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif3(&self) -> TDIF3_R {
        TDIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif4(&self) -> TDIF4_R {
        TDIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif5(&self) -> TDIF5_R {
        TDIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif6(&self) -> TDIF6_R {
        TDIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif7(&self) -> TDIF7_R {
        TDIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif8(&self) -> TDIF8_R {
        TDIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif0(&mut self) -> TDIF0_W {
        TDIF0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif1(&mut self) -> TDIF1_W {
        TDIF1_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif2(&mut self) -> TDIF2_W {
        TDIF2_W { w: self }
    }
    #[doc = "Bit 3 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif3(&mut self) -> TDIF3_W {
        TDIF3_W { w: self }
    }
    #[doc = "Bit 4 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif4(&mut self) -> TDIF4_W {
        TDIF4_W { w: self }
    }
    #[doc = "Bit 5 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif5(&mut self) -> TDIF5_W {
        TDIF5_W { w: self }
    }
    #[doc = "Bit 6 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif6(&mut self) -> TDIF6_W {
        TDIF6_W { w: self }
    }
    #[doc = "Bit 7 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif7(&mut self) -> TDIF7_W {
        TDIF7_W { w: self }
    }
    #[doc = "Bit 8 - Transfer Done Flag This bit indicates whether PDMA controller channel transfer has been finished or not, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn tdif8(&mut self) -> TDIF8_W {
        TDIF8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Channel Transfer Done Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_tdsts](index.html) module"]
pub struct PDMA_TDSTS_SPEC;
impl crate::RegisterSpec for PDMA_TDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_tdsts::R](R) reader structure"]
impl crate::Readable for PDMA_TDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_tdsts::W](W) writer structure"]
impl crate::Writable for PDMA_TDSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TDSTS to value 0"]
impl crate::Resettable for PDMA_TDSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
