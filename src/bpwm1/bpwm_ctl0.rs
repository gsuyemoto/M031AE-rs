#[doc = "Register `BPWM_CTL0` reader"]
pub struct R(crate::R<BPWM_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CTL0` writer"]
pub struct W(crate::W<BPWM_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CTL0_SPEC>;
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
impl From<crate::W<BPWM_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRLD0` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD0_R(crate::FieldReader<bool, bool>);
impl CTRLD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD0` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD0_W<'a> {
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
#[doc = "Field `CTRLD1` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD1_R(crate::FieldReader<bool, bool>);
impl CTRLD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD1` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD1_W<'a> {
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
#[doc = "Field `CTRLD2` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD2_R(crate::FieldReader<bool, bool>);
impl CTRLD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD2` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD2_W<'a> {
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
#[doc = "Field `CTRLD3` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD3_R(crate::FieldReader<bool, bool>);
impl CTRLD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD3` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD3_W<'a> {
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
#[doc = "Field `CTRLD4` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD4_R(crate::FieldReader<bool, bool>);
impl CTRLD4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD4` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD4_W<'a> {
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
#[doc = "Field `CTRLD5` reader - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD5_R(crate::FieldReader<bool, bool>);
impl CTRLD5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLD5` writer - Center Re-load\nEach bit n controls the corresponding BPWM channel n.\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLD5_W<'a> {
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN0_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN0` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN0_R(crate::FieldReader<bool, IMMLDEN0_A>);
impl IMMLDEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN0_A {
        match self.bits {
            false => IMMLDEN0_A::_0,
            true => IMMLDEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN0_A::_1
    }
}
impl core::ops::Deref for IMMLDEN0_R {
    type Target = crate::FieldReader<bool, IMMLDEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN0` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN0_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN0_A::_1)
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN1_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN1` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN1_R(crate::FieldReader<bool, IMMLDEN1_A>);
impl IMMLDEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN1_A {
        match self.bits {
            false => IMMLDEN1_A::_0,
            true => IMMLDEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN1_A::_1
    }
}
impl core::ops::Deref for IMMLDEN1_R {
    type Target = crate::FieldReader<bool, IMMLDEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN1` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN1_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN1_A::_1)
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN2_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN2` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN2_R(crate::FieldReader<bool, IMMLDEN2_A>);
impl IMMLDEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN2_A {
        match self.bits {
            false => IMMLDEN2_A::_0,
            true => IMMLDEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN2_A::_1
    }
}
impl core::ops::Deref for IMMLDEN2_R {
    type Target = crate::FieldReader<bool, IMMLDEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN2` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN2_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN2_A::_1)
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN3_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN3` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN3_R(crate::FieldReader<bool, IMMLDEN3_A>);
impl IMMLDEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN3_A {
        match self.bits {
            false => IMMLDEN3_A::_0,
            true => IMMLDEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN3_A::_1
    }
}
impl core::ops::Deref for IMMLDEN3_R {
    type Target = crate::FieldReader<bool, IMMLDEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN3` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN3_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN3_A::_1)
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN4_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN4_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN4` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN4_R(crate::FieldReader<bool, IMMLDEN4_A>);
impl IMMLDEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN4_A {
        match self.bits {
            false => IMMLDEN4_A::_0,
            true => IMMLDEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN4_A::_1
    }
}
impl core::ops::Deref for IMMLDEN4_R {
    type Target = crate::FieldReader<bool, IMMLDEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN4` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN4_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN4_A::_1)
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
#[doc = "Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMLDEN5_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDEN5_A> for bool {
    #[inline(always)]
    fn from(variant: IMMLDEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMLDEN5` reader - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN5_R(crate::FieldReader<bool, IMMLDEN5_A>);
impl IMMLDEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMLDEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMLDEN5_A {
        match self.bits {
            false => IMMLDEN5_A::_0,
            true => IMMLDEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDEN5_A::_1
    }
}
impl core::ops::Deref for IMMLDEN5_R {
    type Target = crate::FieldReader<bool, IMMLDEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDEN5` writer - Immediately Load Enable Bit(S)\nEach bit n controls the corresponding BPWM channel n.\nNote: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDEN5_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDEN5_A::_1)
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
#[doc = "ICE Debug Mode Counter Halt (Write Protect)\nIf counter halt is enabled, BPWM all counters will keep current value until exit ICE debug mode. \nNote: This bit is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGHALT_A {
    #[doc = "0: ICE debug mode counter halt Disabled"]
    _0 = 0,
    #[doc = "1: ICE debug mode counter halt Enabled"]
    _1 = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - ICE Debug Mode Counter Halt (Write Protect)\nIf counter halt is enabled, BPWM all counters will keep current value until exit ICE debug mode. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGHALT_R(crate::FieldReader<bool, DBGHALT_A>);
impl DBGHALT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGHALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::_0,
            true => DBGHALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGHALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGHALT_A::_1
    }
}
impl core::ops::Deref for DBGHALT_R {
    type Target = crate::FieldReader<bool, DBGHALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGHALT` writer - ICE Debug Mode Counter Halt (Write Protect)\nIf counter halt is enabled, BPWM all counters will keep current value until exit ICE debug mode. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGHALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGHALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode counter halt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGHALT_A::_0)
    }
    #[doc = "ICE debug mode counter halt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGHALT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "ICE Debug Mode Acknowledge Disable (Write Protect)\nBPWM pin will keep output no matter ICE debug mode acknowledged or not.\nNote: This bit is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGTRIOFF_A {
    #[doc = "0: ICE debug mode acknowledgement effects BPWM output"]
    _0 = 0,
    #[doc = "1: ICE debug mode acknowledgement Disabled"]
    _1 = 1,
}
impl From<DBGTRIOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DBGTRIOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGTRIOFF` reader - ICE Debug Mode Acknowledge Disable (Write Protect)\nBPWM pin will keep output no matter ICE debug mode acknowledged or not.\nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGTRIOFF_R(crate::FieldReader<bool, DBGTRIOFF_A>);
impl DBGTRIOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGTRIOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGTRIOFF_A {
        match self.bits {
            false => DBGTRIOFF_A::_0,
            true => DBGTRIOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGTRIOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGTRIOFF_A::_1
    }
}
impl core::ops::Deref for DBGTRIOFF_R {
    type Target = crate::FieldReader<bool, DBGTRIOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGTRIOFF` writer - ICE Debug Mode Acknowledge Disable (Write Protect)\nBPWM pin will keep output no matter ICE debug mode acknowledged or not.\nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGTRIOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGTRIOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGTRIOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode acknowledgement effects BPWM output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGTRIOFF_A::_0)
    }
    #[doc = "ICE debug mode acknowledgement Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGTRIOFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld0(&self) -> CTRLD0_R {
        CTRLD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld1(&self) -> CTRLD1_R {
        CTRLD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld2(&self) -> CTRLD2_R {
        CTRLD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld3(&self) -> CTRLD3_R {
        CTRLD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld4(&self) -> CTRLD4_R {
        CTRLD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld5(&self) -> CTRLD5_R {
        CTRLD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden0(&self) -> IMMLDEN0_R {
        IMMLDEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden1(&self) -> IMMLDEN1_R {
        IMMLDEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden2(&self) -> IMMLDEN2_R {
        IMMLDEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden3(&self) -> IMMLDEN3_R {
        IMMLDEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden4(&self) -> IMMLDEN4_R {
        IMMLDEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden5(&self) -> IMMLDEN5_R {
        IMMLDEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ICE Debug Mode Counter Halt (Write Protect) If counter halt is enabled, BPWM all counters will keep current value until exit ICE debug mode. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable (Write Protect) BPWM pin will keep output no matter ICE debug mode acknowledged or not. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbgtrioff(&self) -> DBGTRIOFF_R {
        DBGTRIOFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld0(&mut self) -> CTRLD0_W {
        CTRLD0_W { w: self }
    }
    #[doc = "Bit 1 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld1(&mut self) -> CTRLD1_W {
        CTRLD1_W { w: self }
    }
    #[doc = "Bit 2 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld2(&mut self) -> CTRLD2_W {
        CTRLD2_W { w: self }
    }
    #[doc = "Bit 3 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld3(&mut self) -> CTRLD3_W {
        CTRLD3_W { w: self }
    }
    #[doc = "Bit 4 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld4(&mut self) -> CTRLD4_W {
        CTRLD4_W { w: self }
    }
    #[doc = "Bit 5 - Center Re-load Each bit n controls the corresponding BPWM channel n. In up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrld5(&mut self) -> CTRLD5_W {
        CTRLD5_W { w: self }
    }
    #[doc = "Bit 16 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden0(&mut self) -> IMMLDEN0_W {
        IMMLDEN0_W { w: self }
    }
    #[doc = "Bit 17 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden1(&mut self) -> IMMLDEN1_W {
        IMMLDEN1_W { w: self }
    }
    #[doc = "Bit 18 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden2(&mut self) -> IMMLDEN2_W {
        IMMLDEN2_W { w: self }
    }
    #[doc = "Bit 19 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden3(&mut self) -> IMMLDEN3_W {
        IMMLDEN3_W { w: self }
    }
    #[doc = "Bit 20 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden4(&mut self) -> IMMLDEN4_W {
        IMMLDEN4_W { w: self }
    }
    #[doc = "Bit 21 - Immediately Load Enable Bit(S) Each bit n controls the corresponding BPWM channel n. Note: If IMMLDENn is Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immlden5(&mut self) -> IMMLDEN5_W {
        IMMLDEN5_W { w: self }
    }
    #[doc = "Bit 30 - ICE Debug Mode Counter Halt (Write Protect) If counter halt is enabled, BPWM all counters will keep current value until exit ICE debug mode. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable (Write Protect) BPWM pin will keep output no matter ICE debug mode acknowledged or not. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbgtrioff(&mut self) -> DBGTRIOFF_W {
        DBGTRIOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_ctl0](index.html) module"]
pub struct BPWM_CTL0_SPEC;
impl crate::RegisterSpec for BPWM_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_ctl0::R](R) reader structure"]
impl crate::Readable for BPWM_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_ctl0::W](W) writer structure"]
impl crate::Writable for BPWM_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CTL0 to value 0"]
impl crate::Resettable for BPWM_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
