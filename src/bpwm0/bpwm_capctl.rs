#[doc = "Register `BPWM_CAPCTL` reader"]
pub struct R(crate::R<BPWM_CAPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CAPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CAPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CAPCTL` writer"]
pub struct W(crate::W<BPWM_CAPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CAPCTL_SPEC>;
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
impl From<crate::W<BPWM_CAPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CAPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN0_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN0` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN0_R(crate::FieldReader<bool, CAPEN0_A>);
impl CAPEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN0_A {
        match self.bits {
            false => CAPEN0_A::_0,
            true => CAPEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN0_A::_1
    }
}
impl core::ops::Deref for CAPEN0_R {
    type Target = crate::FieldReader<bool, CAPEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN0` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN0_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN0_A::_1)
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
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN1_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN1` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN1_R(crate::FieldReader<bool, CAPEN1_A>);
impl CAPEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN1_A {
        match self.bits {
            false => CAPEN1_A::_0,
            true => CAPEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN1_A::_1
    }
}
impl core::ops::Deref for CAPEN1_R {
    type Target = crate::FieldReader<bool, CAPEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN1` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN1_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN1_A::_1)
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
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN2_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN2` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN2_R(crate::FieldReader<bool, CAPEN2_A>);
impl CAPEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN2_A {
        match self.bits {
            false => CAPEN2_A::_0,
            true => CAPEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN2_A::_1
    }
}
impl core::ops::Deref for CAPEN2_R {
    type Target = crate::FieldReader<bool, CAPEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN2` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN2_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN2_A::_1)
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
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN3_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN3` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN3_R(crate::FieldReader<bool, CAPEN3_A>);
impl CAPEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN3_A {
        match self.bits {
            false => CAPEN3_A::_0,
            true => CAPEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN3_A::_1
    }
}
impl core::ops::Deref for CAPEN3_R {
    type Target = crate::FieldReader<bool, CAPEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN3` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN3_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN3_A::_1)
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
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN4_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN4` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN4_R(crate::FieldReader<bool, CAPEN4_A>);
impl CAPEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN4_A {
        match self.bits {
            false => CAPEN4_A::_0,
            true => CAPEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN4_A::_1
    }
}
impl core::ops::Deref for CAPEN4_R {
    type Target = crate::FieldReader<bool, CAPEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN4` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN4_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN4_A::_1)
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
#[doc = "Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN5_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN5` reader - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN5_R(crate::FieldReader<bool, CAPEN5_A>);
impl CAPEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN5_A {
        match self.bits {
            false => CAPEN5_A::_0,
            true => CAPEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN5_A::_1
    }
}
impl core::ops::Deref for CAPEN5_R {
    type Target = crate::FieldReader<bool, CAPEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN5` writer - Capture Function Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN5_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the BPWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN5_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV0_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV0` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV0_R(crate::FieldReader<bool, CAPINV0_A>);
impl CAPINV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV0_A {
        match self.bits {
            false => CAPINV0_A::_0,
            true => CAPINV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV0_A::_1
    }
}
impl core::ops::Deref for CAPINV0_R {
    type Target = crate::FieldReader<bool, CAPINV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV0` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV0_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV0_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV1_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV1` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV1_R(crate::FieldReader<bool, CAPINV1_A>);
impl CAPINV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV1_A {
        match self.bits {
            false => CAPINV1_A::_0,
            true => CAPINV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV1_A::_1
    }
}
impl core::ops::Deref for CAPINV1_R {
    type Target = crate::FieldReader<bool, CAPINV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV1` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV1_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV1_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV2_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV2` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV2_R(crate::FieldReader<bool, CAPINV2_A>);
impl CAPINV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV2_A {
        match self.bits {
            false => CAPINV2_A::_0,
            true => CAPINV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV2_A::_1
    }
}
impl core::ops::Deref for CAPINV2_R {
    type Target = crate::FieldReader<bool, CAPINV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV2` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV2_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV2_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV3_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV3` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV3_R(crate::FieldReader<bool, CAPINV3_A>);
impl CAPINV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV3_A {
        match self.bits {
            false => CAPINV3_A::_0,
            true => CAPINV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV3_A::_1
    }
}
impl core::ops::Deref for CAPINV3_R {
    type Target = crate::FieldReader<bool, CAPINV3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV3` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV3_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV3_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV4_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV4` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV4_R(crate::FieldReader<bool, CAPINV4_A>);
impl CAPINV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV4_A {
        match self.bits {
            false => CAPINV4_A::_0,
            true => CAPINV4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV4_A::_1
    }
}
impl core::ops::Deref for CAPINV4_R {
    type Target = crate::FieldReader<bool, CAPINV4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV4` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV4_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV4_A::_1)
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
#[doc = "Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPINV5_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINV5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPINV5` reader - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV5_R(crate::FieldReader<bool, CAPINV5_A>);
impl CAPINV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPINV5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPINV5_A {
        match self.bits {
            false => CAPINV5_A::_0,
            true => CAPINV5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINV5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINV5_A::_1
    }
}
impl core::ops::Deref for CAPINV5_R {
    type Target = crate::FieldReader<bool, CAPINV5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINV5` writer - Capture Inverter Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPINV5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINV5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINV5_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINV5_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN0_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN0` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN0_R(crate::FieldReader<bool, RCRLDEN0_A>);
impl RCRLDEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN0_A {
        match self.bits {
            false => RCRLDEN0_A::_0,
            true => RCRLDEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN0_A::_1
    }
}
impl core::ops::Deref for RCRLDEN0_R {
    type Target = crate::FieldReader<bool, RCRLDEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN0` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN0_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN0_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN1_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN1` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN1_R(crate::FieldReader<bool, RCRLDEN1_A>);
impl RCRLDEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN1_A {
        match self.bits {
            false => RCRLDEN1_A::_0,
            true => RCRLDEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN1_A::_1
    }
}
impl core::ops::Deref for RCRLDEN1_R {
    type Target = crate::FieldReader<bool, RCRLDEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN1` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN1_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN1_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN2_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN2` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN2_R(crate::FieldReader<bool, RCRLDEN2_A>);
impl RCRLDEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN2_A {
        match self.bits {
            false => RCRLDEN2_A::_0,
            true => RCRLDEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN2_A::_1
    }
}
impl core::ops::Deref for RCRLDEN2_R {
    type Target = crate::FieldReader<bool, RCRLDEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN2` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN2_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN2_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN3_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN3_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN3` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN3_R(crate::FieldReader<bool, RCRLDEN3_A>);
impl RCRLDEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN3_A {
        match self.bits {
            false => RCRLDEN3_A::_0,
            true => RCRLDEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN3_A::_1
    }
}
impl core::ops::Deref for RCRLDEN3_R {
    type Target = crate::FieldReader<bool, RCRLDEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN3` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN3_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN3_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN4_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN4_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN4` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN4_R(crate::FieldReader<bool, RCRLDEN4_A>);
impl RCRLDEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN4_A {
        match self.bits {
            false => RCRLDEN4_A::_0,
            true => RCRLDEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN4_A::_1
    }
}
impl core::ops::Deref for RCRLDEN4_R {
    type Target = crate::FieldReader<bool, RCRLDEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN4` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN4_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN4_A::_1)
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
#[doc = "Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRLDEN5_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDEN5_A> for bool {
    #[inline(always)]
    fn from(variant: RCRLDEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRLDEN5` reader - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN5_R(crate::FieldReader<bool, RCRLDEN5_A>);
impl RCRLDEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRLDEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRLDEN5_A {
        match self.bits {
            false => RCRLDEN5_A::_0,
            true => RCRLDEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDEN5_A::_1
    }
}
impl core::ops::Deref for RCRLDEN5_R {
    type Target = crate::FieldReader<bool, RCRLDEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDEN5` writer - Rising Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct RCRLDEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDEN5_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDEN5_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN0_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN0` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN0_R(crate::FieldReader<bool, FCRLDEN0_A>);
impl FCRLDEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN0_A {
        match self.bits {
            false => FCRLDEN0_A::_0,
            true => FCRLDEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN0_A::_1
    }
}
impl core::ops::Deref for FCRLDEN0_R {
    type Target = crate::FieldReader<bool, FCRLDEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN0` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN0_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN0_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN1_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN1` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN1_R(crate::FieldReader<bool, FCRLDEN1_A>);
impl FCRLDEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN1_A {
        match self.bits {
            false => FCRLDEN1_A::_0,
            true => FCRLDEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN1_A::_1
    }
}
impl core::ops::Deref for FCRLDEN1_R {
    type Target = crate::FieldReader<bool, FCRLDEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN1` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN1_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN1_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN2_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN2` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN2_R(crate::FieldReader<bool, FCRLDEN2_A>);
impl FCRLDEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN2_A {
        match self.bits {
            false => FCRLDEN2_A::_0,
            true => FCRLDEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN2_A::_1
    }
}
impl core::ops::Deref for FCRLDEN2_R {
    type Target = crate::FieldReader<bool, FCRLDEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN2` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN2_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN2_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN3_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN3` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN3_R(crate::FieldReader<bool, FCRLDEN3_A>);
impl FCRLDEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN3_A {
        match self.bits {
            false => FCRLDEN3_A::_0,
            true => FCRLDEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN3_A::_1
    }
}
impl core::ops::Deref for FCRLDEN3_R {
    type Target = crate::FieldReader<bool, FCRLDEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN3` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN3_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN3_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN4_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN4_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN4` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN4_R(crate::FieldReader<bool, FCRLDEN4_A>);
impl FCRLDEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN4_A {
        match self.bits {
            false => FCRLDEN4_A::_0,
            true => FCRLDEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN4_A::_1
    }
}
impl core::ops::Deref for FCRLDEN4_R {
    type Target = crate::FieldReader<bool, FCRLDEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN4` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN4_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN4_A::_1)
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
#[doc = "Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRLDEN5_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDEN5_A> for bool {
    #[inline(always)]
    fn from(variant: FCRLDEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCRLDEN5` reader - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN5_R(crate::FieldReader<bool, FCRLDEN5_A>);
impl FCRLDEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRLDEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRLDEN5_A {
        match self.bits {
            false => FCRLDEN5_A::_0,
            true => FCRLDEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDEN5_A::_1
    }
}
impl core::ops::Deref for FCRLDEN5_R {
    type Target = crate::FieldReader<bool, FCRLDEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDEN5` writer - Falling Capture Reload Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct FCRLDEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDEN5_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDEN5_A::_1)
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
    #[doc = "Bit 0 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen4(&self) -> CAPEN4_R {
        CAPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen5(&self) -> CAPEN5_R {
        CAPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv0(&self) -> CAPINV0_R {
        CAPINV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv1(&self) -> CAPINV1_R {
        CAPINV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv2(&self) -> CAPINV2_R {
        CAPINV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv3(&self) -> CAPINV3_R {
        CAPINV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv4(&self) -> CAPINV4_R {
        CAPINV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv5(&self) -> CAPINV5_R {
        CAPINV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden0(&self) -> RCRLDEN0_R {
        RCRLDEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden1(&self) -> RCRLDEN1_R {
        RCRLDEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden2(&self) -> RCRLDEN2_R {
        RCRLDEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden3(&self) -> RCRLDEN3_R {
        RCRLDEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden4(&self) -> RCRLDEN4_R {
        RCRLDEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden5(&self) -> RCRLDEN5_R {
        RCRLDEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden0(&self) -> FCRLDEN0_R {
        FCRLDEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden1(&self) -> FCRLDEN1_R {
        FCRLDEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden2(&self) -> FCRLDEN2_R {
        FCRLDEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden3(&self) -> FCRLDEN3_R {
        FCRLDEN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden4(&self) -> FCRLDEN4_R {
        FCRLDEN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden5(&self) -> FCRLDEN5_R {
        FCRLDEN5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen0(&mut self) -> CAPEN0_W {
        CAPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen1(&mut self) -> CAPEN1_W {
        CAPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen2(&mut self) -> CAPEN2_W {
        CAPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen3(&mut self) -> CAPEN3_W {
        CAPEN3_W { w: self }
    }
    #[doc = "Bit 4 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen4(&mut self) -> CAPEN4_W {
        CAPEN4_W { w: self }
    }
    #[doc = "Bit 5 - Capture Function Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capen5(&mut self) -> CAPEN5_W {
        CAPEN5_W { w: self }
    }
    #[doc = "Bit 8 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv0(&mut self) -> CAPINV0_W {
        CAPINV0_W { w: self }
    }
    #[doc = "Bit 9 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv1(&mut self) -> CAPINV1_W {
        CAPINV1_W { w: self }
    }
    #[doc = "Bit 10 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv2(&mut self) -> CAPINV2_W {
        CAPINV2_W { w: self }
    }
    #[doc = "Bit 11 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv3(&mut self) -> CAPINV3_W {
        CAPINV3_W { w: self }
    }
    #[doc = "Bit 12 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv4(&mut self) -> CAPINV4_W {
        CAPINV4_W { w: self }
    }
    #[doc = "Bit 13 - Capture Inverter Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capinv5(&mut self) -> CAPINV5_W {
        CAPINV5_W { w: self }
    }
    #[doc = "Bit 16 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden0(&mut self) -> RCRLDEN0_W {
        RCRLDEN0_W { w: self }
    }
    #[doc = "Bit 17 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden1(&mut self) -> RCRLDEN1_W {
        RCRLDEN1_W { w: self }
    }
    #[doc = "Bit 18 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden2(&mut self) -> RCRLDEN2_W {
        RCRLDEN2_W { w: self }
    }
    #[doc = "Bit 19 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden3(&mut self) -> RCRLDEN3_W {
        RCRLDEN3_W { w: self }
    }
    #[doc = "Bit 20 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden4(&mut self) -> RCRLDEN4_W {
        RCRLDEN4_W { w: self }
    }
    #[doc = "Bit 21 - Rising Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn rcrlden5(&mut self) -> RCRLDEN5_W {
        RCRLDEN5_W { w: self }
    }
    #[doc = "Bit 24 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden0(&mut self) -> FCRLDEN0_W {
        FCRLDEN0_W { w: self }
    }
    #[doc = "Bit 25 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden1(&mut self) -> FCRLDEN1_W {
        FCRLDEN1_W { w: self }
    }
    #[doc = "Bit 26 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden2(&mut self) -> FCRLDEN2_W {
        FCRLDEN2_W { w: self }
    }
    #[doc = "Bit 27 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden3(&mut self) -> FCRLDEN3_W {
        FCRLDEN3_W { w: self }
    }
    #[doc = "Bit 28 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden4(&mut self) -> FCRLDEN4_W {
        FCRLDEN4_W { w: self }
    }
    #[doc = "Bit 29 - Falling Capture Reload Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn fcrlden5(&mut self) -> FCRLDEN5_W {
        FCRLDEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_capctl](index.html) module"]
pub struct BPWM_CAPCTL_SPEC;
impl crate::RegisterSpec for BPWM_CAPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_capctl::R](R) reader structure"]
impl crate::Readable for BPWM_CAPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_capctl::W](W) writer structure"]
impl crate::Writable for BPWM_CAPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CAPCTL to value 0"]
impl crate::Resettable for BPWM_CAPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
