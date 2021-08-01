#[doc = "Register `PDMA_TRGSTS` reader"]
pub struct R(crate::R<PDMA_TRGSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TRGSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TRGSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TRGSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS0_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS0` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS0_R(crate::FieldReader<bool, REQSTS0_A>);
impl REQSTS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS0_A {
        match self.bits {
            false => REQSTS0_A::_0,
            true => REQSTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS0_A::_1
    }
}
impl core::ops::Deref for REQSTS0_R {
    type Target = crate::FieldReader<bool, REQSTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS1_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS1_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS1` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS1_R(crate::FieldReader<bool, REQSTS1_A>);
impl REQSTS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS1_A {
        match self.bits {
            false => REQSTS1_A::_0,
            true => REQSTS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS1_A::_1
    }
}
impl core::ops::Deref for REQSTS1_R {
    type Target = crate::FieldReader<bool, REQSTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS2_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS2_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS2` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS2_R(crate::FieldReader<bool, REQSTS2_A>);
impl REQSTS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS2_A {
        match self.bits {
            false => REQSTS2_A::_0,
            true => REQSTS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS2_A::_1
    }
}
impl core::ops::Deref for REQSTS2_R {
    type Target = crate::FieldReader<bool, REQSTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS3_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS3_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS3` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS3_R(crate::FieldReader<bool, REQSTS3_A>);
impl REQSTS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS3_A {
        match self.bits {
            false => REQSTS3_A::_0,
            true => REQSTS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS3_A::_1
    }
}
impl core::ops::Deref for REQSTS3_R {
    type Target = crate::FieldReader<bool, REQSTS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS4_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS4_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS4` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS4_R(crate::FieldReader<bool, REQSTS4_A>);
impl REQSTS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS4_A {
        match self.bits {
            false => REQSTS4_A::_0,
            true => REQSTS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS4_A::_1
    }
}
impl core::ops::Deref for REQSTS4_R {
    type Target = crate::FieldReader<bool, REQSTS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS5_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS5_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS5` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS5_R(crate::FieldReader<bool, REQSTS5_A>);
impl REQSTS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS5_A {
        match self.bits {
            false => REQSTS5_A::_0,
            true => REQSTS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS5_A::_1
    }
}
impl core::ops::Deref for REQSTS5_R {
    type Target = crate::FieldReader<bool, REQSTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS6_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS6_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS6` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS6_R(crate::FieldReader<bool, REQSTS6_A>);
impl REQSTS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS6_A {
        match self.bits {
            false => REQSTS6_A::_0,
            true => REQSTS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS6_A::_1
    }
}
impl core::ops::Deref for REQSTS6_R {
    type Target = crate::FieldReader<bool, REQSTS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS7_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS7_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS7` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS7_R(crate::FieldReader<bool, REQSTS7_A>);
impl REQSTS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS7_A {
        match self.bits {
            false => REQSTS7_A::_0,
            true => REQSTS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS7_A::_1
    }
}
impl core::ops::Deref for REQSTS7_R {
    type Target = crate::FieldReader<bool, REQSTS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQSTS8_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTS8_A> for bool {
    #[inline(always)]
    fn from(variant: REQSTS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQSTS8` reader - PDMA Channel Request Status (Read Only)\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \nNote: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
pub struct REQSTS8_R(crate::FieldReader<bool, REQSTS8_A>);
impl REQSTS8_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQSTS8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQSTS8_A {
        match self.bits {
            false => REQSTS8_A::_0,
            true => REQSTS8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTS8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTS8_A::_1
    }
}
impl core::ops::Deref for REQSTS8_R {
    type Target = crate::FieldReader<bool, REQSTS8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts0(&self) -> REQSTS0_R {
        REQSTS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts1(&self) -> REQSTS1_R {
        REQSTS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts2(&self) -> REQSTS2_R {
        REQSTS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts3(&self) -> REQSTS3_R {
        REQSTS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts4(&self) -> REQSTS4_R {
        REQSTS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts5(&self) -> REQSTS5_R {
        REQSTS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts6(&self) -> REQSTS6_R {
        REQSTS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts7(&self) -> REQSTS7_R {
        REQSTS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDMA Channel Request Status (Read Only) This flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. Note: If user pauses or resets each PDMA transfer by setting PDMA_PAUSE or PDMA_CHRST register respectively, this bit will be cleared automatically after finishing the current transfer."]
    #[inline(always)]
    pub fn reqsts8(&self) -> REQSTS8_R {
        REQSTS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "PDMA Channel Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_trgsts](index.html) module"]
pub struct PDMA_TRGSTS_SPEC;
impl crate::RegisterSpec for PDMA_TRGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_trgsts::R](R) reader structure"]
impl crate::Readable for PDMA_TRGSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDMA_TRGSTS to value 0"]
impl crate::Resettable for PDMA_TRGSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
