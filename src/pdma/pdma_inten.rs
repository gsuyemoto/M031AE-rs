#[doc = "Register `PDMA_INTEN` reader"]
pub struct R(crate::R<PDMA_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_INTEN` writer"]
pub struct W(crate::W<PDMA_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_INTEN_SPEC>;
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
impl From<crate::W<PDMA_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN0_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN0` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN0_R(crate::FieldReader<bool, INTEN0_A>);
impl INTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN0_A {
        match self.bits {
            false => INTEN0_A::_0,
            true => INTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN0_A::_1
    }
}
impl core::ops::Deref for INTEN0_R {
    type Target = crate::FieldReader<bool, INTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN0` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN0_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN0_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN1_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN1` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN1_R(crate::FieldReader<bool, INTEN1_A>);
impl INTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN1_A {
        match self.bits {
            false => INTEN1_A::_0,
            true => INTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN1_A::_1
    }
}
impl core::ops::Deref for INTEN1_R {
    type Target = crate::FieldReader<bool, INTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN1` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN1_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN1_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN2_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN2` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN2_R(crate::FieldReader<bool, INTEN2_A>);
impl INTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN2_A {
        match self.bits {
            false => INTEN2_A::_0,
            true => INTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN2_A::_1
    }
}
impl core::ops::Deref for INTEN2_R {
    type Target = crate::FieldReader<bool, INTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN2` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN2_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN2_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN3_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN3` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN3_R(crate::FieldReader<bool, INTEN3_A>);
impl INTEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN3_A {
        match self.bits {
            false => INTEN3_A::_0,
            true => INTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN3_A::_1
    }
}
impl core::ops::Deref for INTEN3_R {
    type Target = crate::FieldReader<bool, INTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN3` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN3_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN3_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN4_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN4` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN4_R(crate::FieldReader<bool, INTEN4_A>);
impl INTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN4_A {
        match self.bits {
            false => INTEN4_A::_0,
            true => INTEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN4_A::_1
    }
}
impl core::ops::Deref for INTEN4_R {
    type Target = crate::FieldReader<bool, INTEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN4` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN4_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN4_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN5_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN5` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN5_R(crate::FieldReader<bool, INTEN5_A>);
impl INTEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN5_A {
        match self.bits {
            false => INTEN5_A::_0,
            true => INTEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN5_A::_1
    }
}
impl core::ops::Deref for INTEN5_R {
    type Target = crate::FieldReader<bool, INTEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN5` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN5_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN5_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN6_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN6` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN6_R(crate::FieldReader<bool, INTEN6_A>);
impl INTEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN6_A {
        match self.bits {
            false => INTEN6_A::_0,
            true => INTEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN6_A::_1
    }
}
impl core::ops::Deref for INTEN6_R {
    type Target = crate::FieldReader<bool, INTEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN6` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN6_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN6_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN7_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN7` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN7_R(crate::FieldReader<bool, INTEN7_A>);
impl INTEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN7_A {
        match self.bits {
            false => INTEN7_A::_0,
            true => INTEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN7_A::_1
    }
}
impl core::ops::Deref for INTEN7_R {
    type Target = crate::FieldReader<bool, INTEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN7` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN7_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN7_A::_1)
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
#[doc = "PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN8_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN8_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN8` reader - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN8_R(crate::FieldReader<bool, INTEN8_A>);
impl INTEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN8_A {
        match self.bits {
            false => INTEN8_A::_0,
            true => INTEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN8_A::_1
    }
}
impl core::ops::Deref for INTEN8_R {
    type Target = crate::FieldReader<bool, INTEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN8` writer - PDMA Interrupt Enable Bits\nThis field is used to enable PDMA channel\\[n\\]
interrupt.\nNote: The interrupt flag is time-out, abort, transfer done and align."]
pub struct INTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN8_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN8_A::_1)
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
    #[doc = "Bit 0 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten0(&mut self) -> INTEN0_W {
        INTEN0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten1(&mut self) -> INTEN1_W {
        INTEN1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten2(&mut self) -> INTEN2_W {
        INTEN2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten3(&mut self) -> INTEN3_W {
        INTEN3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten4(&mut self) -> INTEN4_W {
        INTEN4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten5(&mut self) -> INTEN5_W {
        INTEN5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten6(&mut self) -> INTEN6_W {
        INTEN6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten7(&mut self) -> INTEN7_W {
        INTEN7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Interrupt Enable Bits This field is used to enable PDMA channel\\[n\\]
interrupt. Note: The interrupt flag is time-out, abort, transfer done and align."]
    #[inline(always)]
    pub fn inten8(&mut self) -> INTEN8_W {
        INTEN8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_inten](index.html) module"]
pub struct PDMA_INTEN_SPEC;
impl crate::RegisterSpec for PDMA_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_inten::R](R) reader structure"]
impl crate::Readable for PDMA_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_inten::W](W) writer structure"]
impl crate::Writable for PDMA_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_INTEN to value 0"]
impl crate::Resettable for PDMA_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
