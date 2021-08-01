#[doc = "Register `PDMA_TACTSTS` reader"]
pub struct R(crate::R<PDMA_TACTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TACTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TACTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TACTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF0_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF0_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF0` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF0_R(crate::FieldReader<bool, TXACTF0_A>);
impl TXACTF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF0_A {
        match self.bits {
            false => TXACTF0_A::_0,
            true => TXACTF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF0_A::_1
    }
}
impl core::ops::Deref for TXACTF0_R {
    type Target = crate::FieldReader<bool, TXACTF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF1_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF1_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF1` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF1_R(crate::FieldReader<bool, TXACTF1_A>);
impl TXACTF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF1_A {
        match self.bits {
            false => TXACTF1_A::_0,
            true => TXACTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF1_A::_1
    }
}
impl core::ops::Deref for TXACTF1_R {
    type Target = crate::FieldReader<bool, TXACTF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF2_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF2_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF2` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF2_R(crate::FieldReader<bool, TXACTF2_A>);
impl TXACTF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF2_A {
        match self.bits {
            false => TXACTF2_A::_0,
            true => TXACTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF2_A::_1
    }
}
impl core::ops::Deref for TXACTF2_R {
    type Target = crate::FieldReader<bool, TXACTF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF3_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF3_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF3` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF3_R(crate::FieldReader<bool, TXACTF3_A>);
impl TXACTF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF3_A {
        match self.bits {
            false => TXACTF3_A::_0,
            true => TXACTF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF3_A::_1
    }
}
impl core::ops::Deref for TXACTF3_R {
    type Target = crate::FieldReader<bool, TXACTF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF4_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF4_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF4` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF4_R(crate::FieldReader<bool, TXACTF4_A>);
impl TXACTF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF4_A {
        match self.bits {
            false => TXACTF4_A::_0,
            true => TXACTF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF4_A::_1
    }
}
impl core::ops::Deref for TXACTF4_R {
    type Target = crate::FieldReader<bool, TXACTF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF5_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF5_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF5` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF5_R(crate::FieldReader<bool, TXACTF5_A>);
impl TXACTF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF5_A {
        match self.bits {
            false => TXACTF5_A::_0,
            true => TXACTF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF5_A::_1
    }
}
impl core::ops::Deref for TXACTF5_R {
    type Target = crate::FieldReader<bool, TXACTF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF6_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF6_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF6` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF6_R(crate::FieldReader<bool, TXACTF6_A>);
impl TXACTF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF6_A {
        match self.bits {
            false => TXACTF6_A::_0,
            true => TXACTF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF6_A::_1
    }
}
impl core::ops::Deref for TXACTF6_R {
    type Target = crate::FieldReader<bool, TXACTF6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF7_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF7_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF7` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF7_R(crate::FieldReader<bool, TXACTF7_A>);
impl TXACTF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF7_A {
        match self.bits {
            false => TXACTF7_A::_0,
            true => TXACTF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF7_A::_1
    }
}
impl core::ops::Deref for TXACTF7_R {
    type Target = crate::FieldReader<bool, TXACTF7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACTF8_A {
    #[doc = "0: PDMA channel is finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTF8_A> for bool {
    #[inline(always)]
    fn from(variant: TXACTF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACTF8` reader - Transfer on Active Flag (Read Only)\nThis bit indicates which PDMA channel is in active."]
pub struct TXACTF8_R(crate::FieldReader<bool, TXACTF8_A>);
impl TXACTF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACTF8_A {
        match self.bits {
            false => TXACTF8_A::_0,
            true => TXACTF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTF8_A::_1
    }
}
impl core::ops::Deref for TXACTF8_R {
    type Target = crate::FieldReader<bool, TXACTF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf0(&self) -> TXACTF0_R {
        TXACTF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf1(&self) -> TXACTF1_R {
        TXACTF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf2(&self) -> TXACTF2_R {
        TXACTF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf3(&self) -> TXACTF3_R {
        TXACTF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf4(&self) -> TXACTF4_R {
        TXACTF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf5(&self) -> TXACTF5_R {
        TXACTF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf6(&self) -> TXACTF6_R {
        TXACTF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf7(&self) -> TXACTF7_R {
        TXACTF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer on Active Flag (Read Only) This bit indicates which PDMA channel is in active."]
    #[inline(always)]
    pub fn txactf8(&self) -> TXACTF8_R {
        TXACTF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "PDMA Transfer Active Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_tactsts](index.html) module"]
pub struct PDMA_TACTSTS_SPEC;
impl crate::RegisterSpec for PDMA_TACTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_tactsts::R](R) reader structure"]
impl crate::Readable for PDMA_TACTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDMA_TACTSTS to value 0"]
impl crate::Resettable for PDMA_TACTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
