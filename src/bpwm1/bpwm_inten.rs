#[doc = "Register `BPWM_INTEN` reader"]
pub struct R(crate::R<BPWM_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_INTEN` writer"]
pub struct W(crate::W<BPWM_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_INTEN_SPEC>;
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
impl From<crate::W<BPWM_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Zero Point Interrupt 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZIEN0_A {
    #[doc = "0: Zero point interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Zero point interrupt Enabled"]
    _1 = 1,
}
impl From<ZIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ZIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZIEN0` reader - BPWM Zero Point Interrupt 0 Enable Bit"]
pub struct ZIEN0_R(crate::FieldReader<bool, ZIEN0_A>);
impl ZIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZIEN0_A {
        match self.bits {
            false => ZIEN0_A::_0,
            true => ZIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZIEN0_A::_1
    }
}
impl core::ops::Deref for ZIEN0_R {
    type Target = crate::FieldReader<bool, ZIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIEN0` writer - BPWM Zero Point Interrupt 0 Enable Bit"]
pub struct ZIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Zero point interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZIEN0_A::_0)
    }
    #[doc = "Zero point interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZIEN0_A::_1)
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
#[doc = "BPWM Period Point Interrupt 0 Enable Bit\nNote: When up-down counter type period point means center point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIEN0_A {
    #[doc = "0: Period point interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Period point interrupt Enabled"]
    _1 = 1,
}
impl From<PIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIEN0` reader - BPWM Period Point Interrupt 0 Enable Bit\nNote: When up-down counter type period point means center point."]
pub struct PIEN0_R(crate::FieldReader<bool, PIEN0_A>);
impl PIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIEN0_A {
        match self.bits {
            false => PIEN0_A::_0,
            true => PIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIEN0_A::_1
    }
}
impl core::ops::Deref for PIEN0_R {
    type Target = crate::FieldReader<bool, PIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIEN0` writer - BPWM Period Point Interrupt 0 Enable Bit\nNote: When up-down counter type period point means center point."]
pub struct PIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Period point interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIEN0_A::_0)
    }
    #[doc = "Period point interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIEN0_A::_1)
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
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN0_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN0` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN0_R(crate::FieldReader<bool, CMPUIEN0_A>);
impl CMPUIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN0_A {
        match self.bits {
            false => CMPUIEN0_A::_0,
            true => CMPUIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN0_A::_1
    }
}
impl core::ops::Deref for CMPUIEN0_R {
    type Target = crate::FieldReader<bool, CMPUIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN0` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN0_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN1_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN1` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN1_R(crate::FieldReader<bool, CMPUIEN1_A>);
impl CMPUIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN1_A {
        match self.bits {
            false => CMPUIEN1_A::_0,
            true => CMPUIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN1_A::_1
    }
}
impl core::ops::Deref for CMPUIEN1_R {
    type Target = crate::FieldReader<bool, CMPUIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN1` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN1_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN2_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN2` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN2_R(crate::FieldReader<bool, CMPUIEN2_A>);
impl CMPUIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN2_A {
        match self.bits {
            false => CMPUIEN2_A::_0,
            true => CMPUIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN2_A::_1
    }
}
impl core::ops::Deref for CMPUIEN2_R {
    type Target = crate::FieldReader<bool, CMPUIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN2` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN2_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN3_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN3` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN3_R(crate::FieldReader<bool, CMPUIEN3_A>);
impl CMPUIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN3_A {
        match self.bits {
            false => CMPUIEN3_A::_0,
            true => CMPUIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN3_A::_1
    }
}
impl core::ops::Deref for CMPUIEN3_R {
    type Target = crate::FieldReader<bool, CMPUIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN3` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN3_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN4_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN4` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN4_R(crate::FieldReader<bool, CMPUIEN4_A>);
impl CMPUIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN4_A {
        match self.bits {
            false => CMPUIEN4_A::_0,
            true => CMPUIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN4_A::_1
    }
}
impl core::ops::Deref for CMPUIEN4_R {
    type Target = crate::FieldReader<bool, CMPUIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN4` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN4_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPUIEN5_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPUIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPUIEN5` reader - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN5_R(crate::FieldReader<bool, CMPUIEN5_A>);
impl CMPUIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUIEN5_A {
        match self.bits {
            false => CMPUIEN5_A::_0,
            true => CMPUIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIEN5_A::_1
    }
}
impl core::ops::Deref for CMPUIEN5_R {
    type Target = crate::FieldReader<bool, CMPUIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIEN5` writer - BPWM Compare Up Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPUIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIEN5_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIEN5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN0_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN0` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN0_R(crate::FieldReader<bool, CMPDIEN0_A>);
impl CMPDIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN0_A {
        match self.bits {
            false => CMPDIEN0_A::_0,
            true => CMPDIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN0_A::_1
    }
}
impl core::ops::Deref for CMPDIEN0_R {
    type Target = crate::FieldReader<bool, CMPDIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN0` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN0_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN1_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN1` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN1_R(crate::FieldReader<bool, CMPDIEN1_A>);
impl CMPDIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN1_A {
        match self.bits {
            false => CMPDIEN1_A::_0,
            true => CMPDIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN1_A::_1
    }
}
impl core::ops::Deref for CMPDIEN1_R {
    type Target = crate::FieldReader<bool, CMPDIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN1` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN1_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN2_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN2` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN2_R(crate::FieldReader<bool, CMPDIEN2_A>);
impl CMPDIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN2_A {
        match self.bits {
            false => CMPDIEN2_A::_0,
            true => CMPDIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN2_A::_1
    }
}
impl core::ops::Deref for CMPDIEN2_R {
    type Target = crate::FieldReader<bool, CMPDIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN2` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN2_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN3_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN3` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN3_R(crate::FieldReader<bool, CMPDIEN3_A>);
impl CMPDIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN3_A {
        match self.bits {
            false => CMPDIEN3_A::_0,
            true => CMPDIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN3_A::_1
    }
}
impl core::ops::Deref for CMPDIEN3_R {
    type Target = crate::FieldReader<bool, CMPDIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN3` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN3_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN4_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN4` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN4_R(crate::FieldReader<bool, CMPDIEN4_A>);
impl CMPDIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN4_A {
        match self.bits {
            false => CMPDIEN4_A::_0,
            true => CMPDIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN4_A::_1
    }
}
impl core::ops::Deref for CMPDIEN4_R {
    type Target = crate::FieldReader<bool, CMPDIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN4` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN4_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPDIEN5_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CMPDIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPDIEN5` reader - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN5_R(crate::FieldReader<bool, CMPDIEN5_A>);
impl CMPDIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDIEN5_A {
        match self.bits {
            false => CMPDIEN5_A::_0,
            true => CMPDIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIEN5_A::_1
    }
}
impl core::ops::Deref for CMPDIEN5_R {
    type Target = crate::FieldReader<bool, CMPDIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIEN5` writer - BPWM Compare Down Count Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CMPDIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIEN5_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIEN5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BPWM Zero Point Interrupt 0 Enable Bit"]
    #[inline(always)]
    pub fn zien0(&self) -> ZIEN0_R {
        ZIEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - BPWM Period Point Interrupt 0 Enable Bit Note: When up-down counter type period point means center point."]
    #[inline(always)]
    pub fn pien0(&self) -> PIEN0_R {
        PIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien0(&self) -> CMPUIEN0_R {
        CMPUIEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien1(&self) -> CMPUIEN1_R {
        CMPUIEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien2(&self) -> CMPUIEN2_R {
        CMPUIEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien3(&self) -> CMPUIEN3_R {
        CMPUIEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien4(&self) -> CMPUIEN4_R {
        CMPUIEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien5(&self) -> CMPUIEN5_R {
        CMPUIEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien0(&self) -> CMPDIEN0_R {
        CMPDIEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien1(&self) -> CMPDIEN1_R {
        CMPDIEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien2(&self) -> CMPDIEN2_R {
        CMPDIEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien3(&self) -> CMPDIEN3_R {
        CMPDIEN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien4(&self) -> CMPDIEN4_R {
        CMPDIEN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien5(&self) -> CMPDIEN5_R {
        CMPDIEN5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Zero Point Interrupt 0 Enable Bit"]
    #[inline(always)]
    pub fn zien0(&mut self) -> ZIEN0_W {
        ZIEN0_W { w: self }
    }
    #[doc = "Bit 8 - BPWM Period Point Interrupt 0 Enable Bit Note: When up-down counter type period point means center point."]
    #[inline(always)]
    pub fn pien0(&mut self) -> PIEN0_W {
        PIEN0_W { w: self }
    }
    #[doc = "Bit 16 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien0(&mut self) -> CMPUIEN0_W {
        CMPUIEN0_W { w: self }
    }
    #[doc = "Bit 17 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien1(&mut self) -> CMPUIEN1_W {
        CMPUIEN1_W { w: self }
    }
    #[doc = "Bit 18 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien2(&mut self) -> CMPUIEN2_W {
        CMPUIEN2_W { w: self }
    }
    #[doc = "Bit 19 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien3(&mut self) -> CMPUIEN3_W {
        CMPUIEN3_W { w: self }
    }
    #[doc = "Bit 20 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien4(&mut self) -> CMPUIEN4_W {
        CMPUIEN4_W { w: self }
    }
    #[doc = "Bit 21 - BPWM Compare Up Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpuien5(&mut self) -> CMPUIEN5_W {
        CMPUIEN5_W { w: self }
    }
    #[doc = "Bit 24 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien0(&mut self) -> CMPDIEN0_W {
        CMPDIEN0_W { w: self }
    }
    #[doc = "Bit 25 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien1(&mut self) -> CMPDIEN1_W {
        CMPDIEN1_W { w: self }
    }
    #[doc = "Bit 26 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien2(&mut self) -> CMPDIEN2_W {
        CMPDIEN2_W { w: self }
    }
    #[doc = "Bit 27 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien3(&mut self) -> CMPDIEN3_W {
        CMPDIEN3_W { w: self }
    }
    #[doc = "Bit 28 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien4(&mut self) -> CMPDIEN4_W {
        CMPDIEN4_W { w: self }
    }
    #[doc = "Bit 29 - BPWM Compare Down Count Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cmpdien5(&mut self) -> CMPDIEN5_W {
        CMPDIEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_inten](index.html) module"]
pub struct BPWM_INTEN_SPEC;
impl crate::RegisterSpec for BPWM_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_inten::R](R) reader structure"]
impl crate::Readable for BPWM_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_inten::W](W) writer structure"]
impl crate::Writable for BPWM_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_INTEN to value 0"]
impl crate::Resettable for BPWM_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
