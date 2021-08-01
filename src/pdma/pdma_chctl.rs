#[doc = "Register `PDMA_CHCTL` reader"]
pub struct R(crate::R<PDMA_CHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CHCTL` writer"]
pub struct W(crate::W<PDMA_CHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CHCTL_SPEC>;
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
impl From<crate::W<PDMA_CHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN0_R(crate::FieldReader<bool, CHEN0_A>);
impl CHEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::_0,
            true => CHEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN0_A::_1
    }
}
impl core::ops::Deref for CHEN0_R {
    type Target = crate::FieldReader<bool, CHEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN0` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN0_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN0_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN1` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN1_R(crate::FieldReader<bool, CHEN1_A>);
impl CHEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::_0,
            true => CHEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN1_A::_1
    }
}
impl core::ops::Deref for CHEN1_R {
    type Target = crate::FieldReader<bool, CHEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN1` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN1_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN1_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN2_R(crate::FieldReader<bool, CHEN2_A>);
impl CHEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::_0,
            true => CHEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN2_A::_1
    }
}
impl core::ops::Deref for CHEN2_R {
    type Target = crate::FieldReader<bool, CHEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN2` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN2_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN2_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN3` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN3_R(crate::FieldReader<bool, CHEN3_A>);
impl CHEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::_0,
            true => CHEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN3_A::_1
    }
}
impl core::ops::Deref for CHEN3_R {
    type Target = crate::FieldReader<bool, CHEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN3` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN3_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN3_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN4_R(crate::FieldReader<bool, CHEN4_A>);
impl CHEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::_0,
            true => CHEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN4_A::_1
    }
}
impl core::ops::Deref for CHEN4_R {
    type Target = crate::FieldReader<bool, CHEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN4` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN4_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN4_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN5` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN5_R(crate::FieldReader<bool, CHEN5_A>);
impl CHEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::_0,
            true => CHEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN5_A::_1
    }
}
impl core::ops::Deref for CHEN5_R {
    type Target = crate::FieldReader<bool, CHEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN5` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN5_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN5_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN6` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN6_R(crate::FieldReader<bool, CHEN6_A>);
impl CHEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::_0,
            true => CHEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN6_A::_1
    }
}
impl core::ops::Deref for CHEN6_R {
    type Target = crate::FieldReader<bool, CHEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN6` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN6_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN6_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN7` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN7_R(crate::FieldReader<bool, CHEN7_A>);
impl CHEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::_0,
            true => CHEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN7_A::_1
    }
}
impl core::ops::Deref for CHEN7_R {
    type Target = crate::FieldReader<bool, CHEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN7` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN7_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN7_A::_1)
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
#[doc = "PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN8_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHEN8_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN8` reader - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN8_R(crate::FieldReader<bool, CHEN8_A>);
impl CHEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN8_A {
        match self.bits {
            false => CHEN8_A::_0,
            true => CHEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN8_A::_1
    }
}
impl core::ops::Deref for CHEN8_R {
    type Target = crate::FieldReader<bool, CHEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN8` writer - PDMA Channel Enable Bits\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\nNote: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
pub struct CHEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN8_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN8_A::_1)
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
    #[doc = "Bit 0 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Channel Enable Bits Set this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled. Note: Setting the corresponding bit of PDMA_PAUSE or PDMA_CHRST register will also clear this bit."]
    #[inline(always)]
    pub fn chen8(&mut self) -> CHEN8_W {
        CHEN8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_chctl](index.html) module"]
pub struct PDMA_CHCTL_SPEC;
impl crate::RegisterSpec for PDMA_CHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_chctl::R](R) reader structure"]
impl crate::Readable for PDMA_CHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_chctl::W](W) writer structure"]
impl crate::Writable for PDMA_CHCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_CHCTL to value 0"]
impl crate::Resettable for PDMA_CHCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
