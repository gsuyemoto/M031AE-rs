#[doc = "Register `PDMA_PRISET` reader"]
pub struct R(crate::R<PDMA_PRISET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_PRISET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_PRISET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_PRISET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_PRISET` writer"]
pub struct W(crate::W<PDMA_PRISET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_PRISET_SPEC>;
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
impl From<crate::W<PDMA_PRISET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_PRISET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET0_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET0_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET0` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET0_R(crate::FieldReader<bool, FPRISET0_A>);
impl FPRISET0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET0_A {
        match self.bits {
            false => FPRISET0_A::_0,
            true => FPRISET0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET0_A::_1
    }
}
impl core::ops::Deref for FPRISET0_R {
    type Target = crate::FieldReader<bool, FPRISET0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET0` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET0_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET0_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET1_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET1_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET1` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET1_R(crate::FieldReader<bool, FPRISET1_A>);
impl FPRISET1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET1_A {
        match self.bits {
            false => FPRISET1_A::_0,
            true => FPRISET1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET1_A::_1
    }
}
impl core::ops::Deref for FPRISET1_R {
    type Target = crate::FieldReader<bool, FPRISET1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET1` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET1_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET1_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET2_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET2_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET2` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET2_R(crate::FieldReader<bool, FPRISET2_A>);
impl FPRISET2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET2_A {
        match self.bits {
            false => FPRISET2_A::_0,
            true => FPRISET2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET2_A::_1
    }
}
impl core::ops::Deref for FPRISET2_R {
    type Target = crate::FieldReader<bool, FPRISET2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET2` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET2_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET2_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET3_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET3_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET3` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET3_R(crate::FieldReader<bool, FPRISET3_A>);
impl FPRISET3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET3_A {
        match self.bits {
            false => FPRISET3_A::_0,
            true => FPRISET3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET3_A::_1
    }
}
impl core::ops::Deref for FPRISET3_R {
    type Target = crate::FieldReader<bool, FPRISET3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET3` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET3_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET3_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET4_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET4_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET4` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET4_R(crate::FieldReader<bool, FPRISET4_A>);
impl FPRISET4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET4_A {
        match self.bits {
            false => FPRISET4_A::_0,
            true => FPRISET4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET4_A::_1
    }
}
impl core::ops::Deref for FPRISET4_R {
    type Target = crate::FieldReader<bool, FPRISET4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET4` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET4_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET4_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET5_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET5_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET5` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET5_R(crate::FieldReader<bool, FPRISET5_A>);
impl FPRISET5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET5_A {
        match self.bits {
            false => FPRISET5_A::_0,
            true => FPRISET5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET5_A::_1
    }
}
impl core::ops::Deref for FPRISET5_R {
    type Target = crate::FieldReader<bool, FPRISET5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET5` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET5_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET5_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET6_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET6_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET6` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET6_R(crate::FieldReader<bool, FPRISET6_A>);
impl FPRISET6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET6_A {
        match self.bits {
            false => FPRISET6_A::_0,
            true => FPRISET6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET6_A::_1
    }
}
impl core::ops::Deref for FPRISET6_R {
    type Target = crate::FieldReader<bool, FPRISET6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET6` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET6_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET6_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET7_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET7_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET7` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET7_R(crate::FieldReader<bool, FPRISET7_A>);
impl FPRISET7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET7_A {
        match self.bits {
            false => FPRISET7_A::_0,
            true => FPRISET7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET7_A::_1
    }
}
impl core::ops::Deref for FPRISET7_R {
    type Target = crate::FieldReader<bool, FPRISET7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET7` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET7_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET7_A::_1)
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
#[doc = "PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRISET8_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISET8_A> for bool {
    #[inline(always)]
    fn from(variant: FPRISET8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRISET8` reader - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET8_R(crate::FieldReader<bool, FPRISET8_A>);
impl FPRISET8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRISET8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRISET8_A {
        match self.bits {
            false => FPRISET8_A::_0,
            true => FPRISET8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISET8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISET8_A::_1
    }
}
impl core::ops::Deref for FPRISET8_R {
    type Target = crate::FieldReader<bool, FPRISET8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISET8` writer - PDMA Fixed Priority Setting\nSet this bit to 1 to enable fixed priority level.\nWrite Operation:\nNote: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
pub struct FPRISET8_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISET8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISET8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISET8_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISET8_A::_1)
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
    #[doc = "Bit 0 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset0(&self) -> FPRISET0_R {
        FPRISET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset1(&self) -> FPRISET1_R {
        FPRISET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset2(&self) -> FPRISET2_R {
        FPRISET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset3(&self) -> FPRISET3_R {
        FPRISET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset4(&self) -> FPRISET4_R {
        FPRISET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset5(&self) -> FPRISET5_R {
        FPRISET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset6(&self) -> FPRISET6_R {
        FPRISET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset7(&self) -> FPRISET7_R {
        FPRISET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset8(&self) -> FPRISET8_R {
        FPRISET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset0(&mut self) -> FPRISET0_W {
        FPRISET0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset1(&mut self) -> FPRISET1_W {
        FPRISET1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset2(&mut self) -> FPRISET2_W {
        FPRISET2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset3(&mut self) -> FPRISET3_W {
        FPRISET3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset4(&mut self) -> FPRISET4_W {
        FPRISET4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset5(&mut self) -> FPRISET5_W {
        FPRISET5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset6(&mut self) -> FPRISET6_W {
        FPRISET6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset7(&mut self) -> FPRISET7_W {
        FPRISET7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Fixed Priority Setting Set this bit to 1 to enable fixed priority level. Write Operation: Note: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register."]
    #[inline(always)]
    pub fn fpriset8(&mut self) -> FPRISET8_W {
        FPRISET8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Fixed Priority Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_priset](index.html) module"]
pub struct PDMA_PRISET_SPEC;
impl crate::RegisterSpec for PDMA_PRISET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_priset::R](R) reader structure"]
impl crate::Readable for PDMA_PRISET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_priset::W](W) writer structure"]
impl crate::Writable for PDMA_PRISET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_PRISET to value 0"]
impl crate::Resettable for PDMA_PRISET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
