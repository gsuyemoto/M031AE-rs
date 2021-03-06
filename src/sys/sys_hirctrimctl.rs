#[doc = "Register `SYS_HIRCTRIMCTL` reader"]
pub struct R(crate::R<SYS_HIRCTRIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_HIRCTRIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_HIRCTRIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_HIRCTRIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_HIRCTRIMCTL` writer"]
pub struct W(crate::W<SYS_HIRCTRIMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_HIRCTRIMCTL_SPEC>;
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
impl From<crate::W<SYS_HIRCTRIMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_HIRCTRIMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trim Frequency Selection\nThis field indicates the target frequency of 48 MHz internal high speed RC oscillator (HIRC) auto trim.\nDuring auto trim operation, if clock error detected with CESTOPEN is set to 1 or trim retry limitation count reached, this field will be cleared to 00 automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQSEL_A {
    #[doc = "0: Disable HIRC auto trim function"]
    _0 = 0,
    #[doc = "1: Enable HIRC auto trim function and trim HIRC to 48 MHz"]
    _1 = 1,
    #[doc = "2: Reserved."]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQSEL` reader - Trim Frequency Selection\nThis field indicates the target frequency of 48 MHz internal high speed RC oscillator (HIRC) auto trim.\nDuring auto trim operation, if clock error detected with CESTOPEN is set to 1 or trim retry limitation count reached, this field will be cleared to 00 automatically."]
pub struct FREQSEL_R(crate::FieldReader<u8, FREQSEL_A>);
impl FREQSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FREQSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQSEL_A {
        match self.bits {
            0 => FREQSEL_A::_0,
            1 => FREQSEL_A::_1,
            2 => FREQSEL_A::_2,
            3 => FREQSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FREQSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FREQSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FREQSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FREQSEL_A::_3
    }
}
impl core::ops::Deref for FREQSEL_R {
    type Target = crate::FieldReader<u8, FREQSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQSEL` writer - Trim Frequency Selection\nThis field indicates the target frequency of 48 MHz internal high speed RC oscillator (HIRC) auto trim.\nDuring auto trim operation, if clock error detected with CESTOPEN is set to 1 or trim retry limitation count reached, this field will be cleared to 00 automatically."]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disable HIRC auto trim function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FREQSEL_A::_0)
    }
    #[doc = "Enable HIRC auto trim function and trim HIRC to 48 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Trim Calculation Loop Selection\nThis field defines that trim value calculation is based on how many reference clocks.\nNote: For example, if LOOPSEL is set as 00, auto trim circuit will calculate trim value based on the average frequency difference in 4 clocks of reference clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOOPSEL_A {
    #[doc = "0: Trim value calculation is based on average difference in 4 clocks of reference clock"]
    _0 = 0,
    #[doc = "1: Trim value calculation is based on average difference in 8 clocks of reference clock"]
    _1 = 1,
    #[doc = "2: Trim value calculation is based on average difference in 16 clocks of reference clock"]
    _2 = 2,
    #[doc = "3: Trim value calculation is based on average difference in 32 clocks of reference clock"]
    _3 = 3,
}
impl From<LOOPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOOPSEL` reader - Trim Calculation Loop Selection\nThis field defines that trim value calculation is based on how many reference clocks.\nNote: For example, if LOOPSEL is set as 00, auto trim circuit will calculate trim value based on the average frequency difference in 4 clocks of reference clock."]
pub struct LOOPSEL_R(crate::FieldReader<u8, LOOPSEL_A>);
impl LOOPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOOPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSEL_A {
        match self.bits {
            0 => LOOPSEL_A::_0,
            1 => LOOPSEL_A::_1,
            2 => LOOPSEL_A::_2,
            3 => LOOPSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOOPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOOPSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == LOOPSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == LOOPSEL_A::_3
    }
}
impl core::ops::Deref for LOOPSEL_R {
    type Target = crate::FieldReader<u8, LOOPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPSEL` writer - Trim Calculation Loop Selection\nThis field defines that trim value calculation is based on how many reference clocks.\nNote: For example, if LOOPSEL is set as 00, auto trim circuit will calculate trim value based on the average frequency difference in 4 clocks of reference clock."]
pub struct LOOPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trim value calculation is based on average difference in 4 clocks of reference clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPSEL_A::_0)
    }
    #[doc = "Trim value calculation is based on average difference in 8 clocks of reference clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPSEL_A::_1)
    }
    #[doc = "Trim value calculation is based on average difference in 16 clocks of reference clock"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(LOOPSEL_A::_2)
    }
    #[doc = "Trim value calculation is based on average difference in 32 clocks of reference clock"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(LOOPSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Trim Value Update Limitation Count\nThis field defines that how many times the auto trim circuit will try to update the HIRC trim value before the frequency of HIRC locked.\nOnce the HIRC locked, the internal trim value update counter will be reset.\nIf the trim value update counter reached this limitation value and frequency of HIRC still doesn't lock, the auto trim operation will be disabled and FREQSEL will be cleared to 00.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RETRYCNT_A {
    #[doc = "0: Trim retry count limitation is 64 loops"]
    _0 = 0,
    #[doc = "1: Trim retry count limitation is 128 loops"]
    _1 = 1,
    #[doc = "2: Trim retry count limitation is 256 loops"]
    _2 = 2,
    #[doc = "3: Trim retry count limitation is 512 loops"]
    _3 = 3,
}
impl From<RETRYCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: RETRYCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RETRYCNT` reader - Trim Value Update Limitation Count\nThis field defines that how many times the auto trim circuit will try to update the HIRC trim value before the frequency of HIRC locked.\nOnce the HIRC locked, the internal trim value update counter will be reset.\nIf the trim value update counter reached this limitation value and frequency of HIRC still doesn't lock, the auto trim operation will be disabled and FREQSEL will be cleared to 00."]
pub struct RETRYCNT_R(crate::FieldReader<u8, RETRYCNT_A>);
impl RETRYCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RETRYCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRYCNT_A {
        match self.bits {
            0 => RETRYCNT_A::_0,
            1 => RETRYCNT_A::_1,
            2 => RETRYCNT_A::_2,
            3 => RETRYCNT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RETRYCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RETRYCNT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RETRYCNT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RETRYCNT_A::_3
    }
}
impl core::ops::Deref for RETRYCNT_R {
    type Target = crate::FieldReader<u8, RETRYCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRYCNT` writer - Trim Value Update Limitation Count\nThis field defines that how many times the auto trim circuit will try to update the HIRC trim value before the frequency of HIRC locked.\nOnce the HIRC locked, the internal trim value update counter will be reset.\nIf the trim value update counter reached this limitation value and frequency of HIRC still doesn't lock, the auto trim operation will be disabled and FREQSEL will be cleared to 00."]
pub struct RETRYCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRYCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRYCNT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trim retry count limitation is 64 loops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RETRYCNT_A::_0)
    }
    #[doc = "Trim retry count limitation is 128 loops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RETRYCNT_A::_1)
    }
    #[doc = "Trim retry count limitation is 256 loops"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RETRYCNT_A::_2)
    }
    #[doc = "Trim retry count limitation is 512 loops"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RETRYCNT_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Clock Error Stop Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CESTOPEN_A {
    #[doc = "0: The trim operation is keep going if clock is inaccuracy"]
    _0 = 0,
    #[doc = "1: The trim operation is stopped if clock is inaccuracy"]
    _1 = 1,
}
impl From<CESTOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CESTOPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CESTOPEN` reader - Clock Error Stop Enable Bit"]
pub struct CESTOPEN_R(crate::FieldReader<bool, CESTOPEN_A>);
impl CESTOPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CESTOPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CESTOPEN_A {
        match self.bits {
            false => CESTOPEN_A::_0,
            true => CESTOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CESTOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CESTOPEN_A::_1
    }
}
impl core::ops::Deref for CESTOPEN_R {
    type Target = crate::FieldReader<bool, CESTOPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CESTOPEN` writer - Clock Error Stop Enable Bit"]
pub struct CESTOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CESTOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CESTOPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The trim operation is keep going if clock is inaccuracy"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CESTOPEN_A::_0)
    }
    #[doc = "The trim operation is stopped if clock is inaccuracy"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CESTOPEN_A::_1)
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
#[doc = "Boundary Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOUNDEN_A {
    #[doc = "0: Boundary function Disabled"]
    _0 = 0,
    #[doc = "1: Boundary function Enabled"]
    _1 = 1,
}
impl From<BOUNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BOUNDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOUNDEN` reader - Boundary Enable Bit"]
pub struct BOUNDEN_R(crate::FieldReader<bool, BOUNDEN_A>);
impl BOUNDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOUNDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOUNDEN_A {
        match self.bits {
            false => BOUNDEN_A::_0,
            true => BOUNDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BOUNDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BOUNDEN_A::_1
    }
}
impl core::ops::Deref for BOUNDEN_R {
    type Target = crate::FieldReader<bool, BOUNDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOUNDEN` writer - Boundary Enable Bit"]
pub struct BOUNDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOUNDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Boundary function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOUNDEN_A::_0)
    }
    #[doc = "Boundary function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOUNDEN_A::_1)
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
#[doc = "Reference Clock Selection\nNote 1: HIRC trim reference clock supports LXT or internal USB synchronous mode depending on the chip spec. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\nNote 2: If there is no reference clock (LXT or internal USB synchronous mode) when the rc_trim is enabled, CLKERIF (SYS_HIRCTRIMCTL\\[2\\]) will be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCKSEL_A {
    #[doc = "0: HIRC trim reference clock is from LXT (32.768 kHz)"]
    _0 = 0,
    #[doc = "1: HIRC trim reference clock is from internal USB synchronous mode"]
    _1 = 1,
}
impl From<REFCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFCKSEL` reader - Reference Clock Selection\nNote 1: HIRC trim reference clock supports LXT or internal USB synchronous mode depending on the chip spec. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\nNote 2: If there is no reference clock (LXT or internal USB synchronous mode) when the rc_trim is enabled, CLKERIF (SYS_HIRCTRIMCTL\\[2\\]) will be set to 1."]
pub struct REFCKSEL_R(crate::FieldReader<bool, REFCKSEL_A>);
impl REFCKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCKSEL_A {
        match self.bits {
            false => REFCKSEL_A::_0,
            true => REFCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REFCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REFCKSEL_A::_1
    }
}
impl core::ops::Deref for REFCKSEL_R {
    type Target = crate::FieldReader<bool, REFCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCKSEL` writer - Reference Clock Selection\nNote 1: HIRC trim reference clock supports LXT or internal USB synchronous mode depending on the chip spec. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\nNote 2: If there is no reference clock (LXT or internal USB synchronous mode) when the rc_trim is enabled, CLKERIF (SYS_HIRCTRIMCTL\\[2\\]) will be set to 1."]
pub struct REFCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HIRC trim reference clock is from LXT (32.768 kHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REFCKSEL_A::_0)
    }
    #[doc = "HIRC trim reference clock is from internal USB synchronous mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REFCKSEL_A::_1)
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
#[doc = "Field `BOUNDARY` reader - Boundary Selection\nFill the boundary range from 0x1 to 0x1F, 0x0 is reserved.\nNote: This field is effective only when the BOUNDEN(SYS_HIRCTRIMCTL\\[9\\]) is enable."]
pub struct BOUNDARY_R(crate::FieldReader<u8, u8>);
impl BOUNDARY_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOUNDARY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOUNDARY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOUNDARY` writer - Boundary Selection\nFill the boundary range from 0x1 to 0x1F, 0x0 is reserved.\nNote: This field is effective only when the BOUNDEN(SYS_HIRCTRIMCTL\\[9\\]) is enable."]
pub struct BOUNDARY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Trim Frequency Selection This field indicates the target frequency of 48 MHz internal high speed RC oscillator (HIRC) auto trim. During auto trim operation, if clock error detected with CESTOPEN is set to 1 or trim retry limitation count reached, this field will be cleared to 00 automatically."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Trim Calculation Loop Selection This field defines that trim value calculation is based on how many reference clocks. Note: For example, if LOOPSEL is set as 00, auto trim circuit will calculate trim value based on the average frequency difference in 4 clocks of reference clock."]
    #[inline(always)]
    pub fn loopsel(&self) -> LOOPSEL_R {
        LOOPSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Trim Value Update Limitation Count This field defines that how many times the auto trim circuit will try to update the HIRC trim value before the frequency of HIRC locked. Once the HIRC locked, the internal trim value update counter will be reset. If the trim value update counter reached this limitation value and frequency of HIRC still doesn't lock, the auto trim operation will be disabled and FREQSEL will be cleared to 00."]
    #[inline(always)]
    pub fn retrycnt(&self) -> RETRYCNT_R {
        RETRYCNT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Clock Error Stop Enable Bit"]
    #[inline(always)]
    pub fn cestopen(&self) -> CESTOPEN_R {
        CESTOPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Boundary Enable Bit"]
    #[inline(always)]
    pub fn bounden(&self) -> BOUNDEN_R {
        BOUNDEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reference Clock Selection Note 1: HIRC trim reference clock supports LXT or internal USB synchronous mode depending on the chip spec. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information. Note 2: If there is no reference clock (LXT or internal USB synchronous mode) when the rc_trim is enabled, CLKERIF (SYS_HIRCTRIMCTL\\[2\\]) will be set to 1."]
    #[inline(always)]
    pub fn refcksel(&self) -> REFCKSEL_R {
        REFCKSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Boundary Selection Fill the boundary range from 0x1 to 0x1F, 0x0 is reserved. Note: This field is effective only when the BOUNDEN(SYS_HIRCTRIMCTL\\[9\\]) is enable."]
    #[inline(always)]
    pub fn boundary(&self) -> BOUNDARY_R {
        BOUNDARY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim Frequency Selection This field indicates the target frequency of 48 MHz internal high speed RC oscillator (HIRC) auto trim. During auto trim operation, if clock error detected with CESTOPEN is set to 1 or trim retry limitation count reached, this field will be cleared to 00 automatically."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Trim Calculation Loop Selection This field defines that trim value calculation is based on how many reference clocks. Note: For example, if LOOPSEL is set as 00, auto trim circuit will calculate trim value based on the average frequency difference in 4 clocks of reference clock."]
    #[inline(always)]
    pub fn loopsel(&mut self) -> LOOPSEL_W {
        LOOPSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Trim Value Update Limitation Count This field defines that how many times the auto trim circuit will try to update the HIRC trim value before the frequency of HIRC locked. Once the HIRC locked, the internal trim value update counter will be reset. If the trim value update counter reached this limitation value and frequency of HIRC still doesn't lock, the auto trim operation will be disabled and FREQSEL will be cleared to 00."]
    #[inline(always)]
    pub fn retrycnt(&mut self) -> RETRYCNT_W {
        RETRYCNT_W { w: self }
    }
    #[doc = "Bit 8 - Clock Error Stop Enable Bit"]
    #[inline(always)]
    pub fn cestopen(&mut self) -> CESTOPEN_W {
        CESTOPEN_W { w: self }
    }
    #[doc = "Bit 9 - Boundary Enable Bit"]
    #[inline(always)]
    pub fn bounden(&mut self) -> BOUNDEN_W {
        BOUNDEN_W { w: self }
    }
    #[doc = "Bit 10 - Reference Clock Selection Note 1: HIRC trim reference clock supports LXT or internal USB synchronous mode depending on the chip spec. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information. Note 2: If there is no reference clock (LXT or internal USB synchronous mode) when the rc_trim is enabled, CLKERIF (SYS_HIRCTRIMCTL\\[2\\]) will be set to 1."]
    #[inline(always)]
    pub fn refcksel(&mut self) -> REFCKSEL_W {
        REFCKSEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Boundary Selection Fill the boundary range from 0x1 to 0x1F, 0x0 is reserved. Note: This field is effective only when the BOUNDEN(SYS_HIRCTRIMCTL\\[9\\]) is enable."]
    #[inline(always)]
    pub fn boundary(&mut self) -> BOUNDARY_W {
        BOUNDARY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIRC Trim Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_hirctrimctl](index.html) module"]
pub struct SYS_HIRCTRIMCTL_SPEC;
impl crate::RegisterSpec for SYS_HIRCTRIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_hirctrimctl::R](R) reader structure"]
impl crate::Readable for SYS_HIRCTRIMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_hirctrimctl::W](W) writer structure"]
impl crate::Writable for SYS_HIRCTRIMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_HIRCTRIMCTL to value 0x0008_0000"]
impl crate::Resettable for SYS_HIRCTRIMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0000
    }
}
