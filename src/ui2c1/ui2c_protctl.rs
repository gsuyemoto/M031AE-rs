#[doc = "Register `UI2C_PROTCTL` reader"]
pub struct R(crate::R<UI2C_PROTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_PROTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_PROTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_PROTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_PROTCTL` writer"]
pub struct W(crate::W<UI2C_PROTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_PROTCTL_SPEC>;
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
impl From<crate::W<UI2C_PROTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_PROTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "General Call Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCFUNC_A {
    #[doc = "0: General Call Function Disabled"]
    _0 = 0,
    #[doc = "1: General Call Function Enabled"]
    _1 = 1,
}
impl From<GCFUNC_A> for bool {
    #[inline(always)]
    fn from(variant: GCFUNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCFUNC` reader - General Call Function"]
pub struct GCFUNC_R(crate::FieldReader<bool, GCFUNC_A>);
impl GCFUNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCFUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCFUNC_A {
        match self.bits {
            false => GCFUNC_A::_0,
            true => GCFUNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GCFUNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GCFUNC_A::_1
    }
}
impl core::ops::Deref for GCFUNC_R {
    type Target = crate::FieldReader<bool, GCFUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCFUNC` writer - General Call Function"]
pub struct GCFUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GCFUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCFUNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "General Call Function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCFUNC_A::_0)
    }
    #[doc = "General Call Function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCFUNC_A::_1)
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
#[doc = "Field `AA` reader - Assert Acknowledge Control"]
pub struct AA_R(crate::FieldReader<bool, bool>);
impl AA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AA` writer - Assert Acknowledge Control"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
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
#[doc = "Field `STO` reader - I2C STOP Control"]
pub struct STO_R(crate::FieldReader<bool, bool>);
impl STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STO` writer - I2C STOP Control"]
pub struct STO_W<'a> {
    w: &'a mut W,
}
impl<'a> STO_W<'a> {
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
#[doc = "Field `STA` reader - I2C START Control\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
pub struct STA_R(crate::FieldReader<bool, bool>);
impl STA_R {
    pub(crate) fn new(bits: bool) -> Self {
        STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STA` writer - I2C START Control\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
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
#[doc = "Address 10-bit Function Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR10EN_A {
    #[doc = "0: Address match 10 bit function Disabled"]
    _0 = 0,
    #[doc = "1: Address match 10 bit function Enabled"]
    _1 = 1,
}
impl From<ADDR10EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR10EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR10EN` reader - Address 10-bit Function Enable Bit"]
pub struct ADDR10EN_R(crate::FieldReader<bool, ADDR10EN_A>);
impl ADDR10EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDR10EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR10EN_A {
        match self.bits {
            false => ADDR10EN_A::_0,
            true => ADDR10EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDR10EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDR10EN_A::_1
    }
}
impl core::ops::Deref for ADDR10EN_R {
    type Target = crate::FieldReader<bool, ADDR10EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR10EN` writer - Address 10-bit Function Enable Bit"]
pub struct ADDR10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR10EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR10EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address match 10 bit function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDR10EN_A::_0)
    }
    #[doc = "Address match 10 bit function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDR10EN_A::_1)
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
#[doc = "I2C Protocol Trigger (Write Only)\nWhen a new state is present in the UI2C_PROTSTS register, if the related interrupt enable bits are set, the I2C interrupt is requested. It must write one by software to this bit after the related interrupt flags are set to 1 and the I2C protocol function will go ahead until the STOP is active or the PROTIEN is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTRG_AW {
    #[doc = "0: I2C's stretch disabled and the I2C protocol function will go ahead"]
    _0 = 0,
    #[doc = "1: I2C's stretch active"]
    _1 = 1,
}
impl From<PTRG_AW> for bool {
    #[inline(always)]
    fn from(variant: PTRG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTRG` writer - I2C Protocol Trigger (Write Only)\nWhen a new state is present in the UI2C_PROTSTS register, if the related interrupt enable bits are set, the I2C interrupt is requested. It must write one by software to this bit after the related interrupt flags are set to 1 and the I2C protocol function will go ahead until the STOP is active or the PROTIEN is disabled."]
pub struct PTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTRG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C's stretch disabled and the I2C protocol function will go ahead"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTRG_AW::_0)
    }
    #[doc = "I2C's stretch active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTRG_AW::_1)
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
#[doc = "SCL Output Enable Bit\nThis bit enables monitor pulling SCL to low. This monitor will pull SCL to low until it has had time to respond to an I2C interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLOUTEN_A {
    #[doc = "0: SCL output will be forced high due to open drain mechanism"]
    _0 = 0,
    #[doc = "1: I2C module may act as a slave peripheral just like in normal operation, the I2C holds the clock line low until it has had time to clear I2C interrupt"]
    _1 = 1,
}
impl From<SCLOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCLOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLOUTEN` reader - SCL Output Enable Bit\nThis bit enables monitor pulling SCL to low. This monitor will pull SCL to low until it has had time to respond to an I2C interrupt."]
pub struct SCLOUTEN_R(crate::FieldReader<bool, SCLOUTEN_A>);
impl SCLOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLOUTEN_A {
        match self.bits {
            false => SCLOUTEN_A::_0,
            true => SCLOUTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCLOUTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCLOUTEN_A::_1
    }
}
impl core::ops::Deref for SCLOUTEN_R {
    type Target = crate::FieldReader<bool, SCLOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLOUTEN` writer - SCL Output Enable Bit\nThis bit enables monitor pulling SCL to low. This monitor will pull SCL to low until it has had time to respond to an I2C interrupt."]
pub struct SCLOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCL output will be forced high due to open drain mechanism"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCLOUTEN_A::_0)
    }
    #[doc = "I2C module may act as a slave peripheral just like in normal operation, the I2C holds the clock line low until it has had time to clear I2C interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCLOUTEN_A::_1)
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
#[doc = "Monitor Mode Enable Bit\nThis bit enables monitor mode. In monitor mode the SDA output will be put in high impedance mode. This prevents the I2C module from outputting data of any kind (including ACK) onto the I2C data bus.\nNote: Depending on the state of the SCLOUTEN bit, the SCL output may be also forced high, preventing the module from having control over the I2C clock line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONEN_A {
    #[doc = "0: The monitor mode Disabled"]
    _0 = 0,
    #[doc = "1: The monitor mode Enabled"]
    _1 = 1,
}
impl From<MONEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONEN` reader - Monitor Mode Enable Bit\nThis bit enables monitor mode. In monitor mode the SDA output will be put in high impedance mode. This prevents the I2C module from outputting data of any kind (including ACK) onto the I2C data bus.\nNote: Depending on the state of the SCLOUTEN bit, the SCL output may be also forced high, preventing the module from having control over the I2C clock line."]
pub struct MONEN_R(crate::FieldReader<bool, MONEN_A>);
impl MONEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONEN_A {
        match self.bits {
            false => MONEN_A::_0,
            true => MONEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MONEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MONEN_A::_1
    }
}
impl core::ops::Deref for MONEN_R {
    type Target = crate::FieldReader<bool, MONEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONEN` writer - Monitor Mode Enable Bit\nThis bit enables monitor mode. In monitor mode the SDA output will be put in high impedance mode. This prevents the I2C module from outputting data of any kind (including ACK) onto the I2C data bus.\nNote: Depending on the state of the SCLOUTEN bit, the SCL output may be also forced high, preventing the module from having control over the I2C clock line."]
pub struct MONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The monitor mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONEN_A::_0)
    }
    #[doc = "The monitor mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONEN_A::_1)
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
#[doc = "Field `TOCNT` reader - Time-out Clock Cycle\nThis bit field indicates how many clock cycle selected by TMCNTSRC (UI2C_BRGEN \\[5\\]) when each interrupt flags are clear. The time-out is enable when TOCNT bigger than 0. \nNote: The TMCNTSRC (UI2C_BRGEN \\[5\\]) must be set zero on I2C mode."]
pub struct TOCNT_R(crate::FieldReader<u16, u16>);
impl TOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOCNT` writer - Time-out Clock Cycle\nThis bit field indicates how many clock cycle selected by TMCNTSRC (UI2C_BRGEN \\[5\\]) when each interrupt flags are clear. The time-out is enable when TOCNT bigger than 0. \nNote: The TMCNTSRC (UI2C_BRGEN \\[5\\]) must be set zero on I2C mode."]
pub struct TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "I2C Protocol Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTEN_A {
    #[doc = "0: I2C Protocol Disabled"]
    _0 = 0,
    #[doc = "1: I2C Protocol Enabled"]
    _1 = 1,
}
impl From<PROTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PROTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTEN` reader - I2C Protocol Enable Bit"]
pub struct PROTEN_R(crate::FieldReader<bool, PROTEN_A>);
impl PROTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTEN_A {
        match self.bits {
            false => PROTEN_A::_0,
            true => PROTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROTEN_A::_1
    }
}
impl core::ops::Deref for PROTEN_R {
    type Target = crate::FieldReader<bool, PROTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTEN` writer - I2C Protocol Enable Bit"]
pub struct PROTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C Protocol Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTEN_A::_0)
    }
    #[doc = "I2C Protocol Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTEN_A::_1)
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
    #[doc = "Bit 0 - General Call Function"]
    #[inline(always)]
    pub fn gcfunc(&self) -> GCFUNC_R {
        GCFUNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Assert Acknowledge Control"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C STOP Control"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C START Control Setting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address 10-bit Function Enable Bit"]
    #[inline(always)]
    pub fn addr10en(&self) -> ADDR10EN_R {
        ADDR10EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCL Output Enable Bit This bit enables monitor pulling SCL to low. This monitor will pull SCL to low until it has had time to respond to an I2C interrupt."]
    #[inline(always)]
    pub fn sclouten(&self) -> SCLOUTEN_R {
        SCLOUTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Monitor Mode Enable Bit This bit enables monitor mode. In monitor mode the SDA output will be put in high impedance mode. This prevents the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Note: Depending on the state of the SCLOUTEN bit, the SCL output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Time-out Clock Cycle This bit field indicates how many clock cycle selected by TMCNTSRC (UI2C_BRGEN \\[5\\]) when each interrupt flags are clear. The time-out is enable when TOCNT bigger than 0. Note: The TMCNTSRC (UI2C_BRGEN \\[5\\]) must be set zero on I2C mode."]
    #[inline(always)]
    pub fn tocnt(&self) -> TOCNT_R {
        TOCNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - I2C Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&self) -> PROTEN_R {
        PROTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Function"]
    #[inline(always)]
    pub fn gcfunc(&mut self) -> GCFUNC_W {
        GCFUNC_W { w: self }
    }
    #[doc = "Bit 1 - Assert Acknowledge Control"]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W { w: self }
    }
    #[doc = "Bit 2 - I2C STOP Control"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W { w: self }
    }
    #[doc = "Bit 3 - I2C START Control Setting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 4 - Address 10-bit Function Enable Bit"]
    #[inline(always)]
    pub fn addr10en(&mut self) -> ADDR10EN_W {
        ADDR10EN_W { w: self }
    }
    #[doc = "Bit 5 - I2C Protocol Trigger (Write Only) When a new state is present in the UI2C_PROTSTS register, if the related interrupt enable bits are set, the I2C interrupt is requested. It must write one by software to this bit after the related interrupt flags are set to 1 and the I2C protocol function will go ahead until the STOP is active or the PROTIEN is disabled."]
    #[inline(always)]
    pub fn ptrg(&mut self) -> PTRG_W {
        PTRG_W { w: self }
    }
    #[doc = "Bit 8 - SCL Output Enable Bit This bit enables monitor pulling SCL to low. This monitor will pull SCL to low until it has had time to respond to an I2C interrupt."]
    #[inline(always)]
    pub fn sclouten(&mut self) -> SCLOUTEN_W {
        SCLOUTEN_W { w: self }
    }
    #[doc = "Bit 9 - Monitor Mode Enable Bit This bit enables monitor mode. In monitor mode the SDA output will be put in high impedance mode. This prevents the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Note: Depending on the state of the SCLOUTEN bit, the SCL output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W {
        MONEN_W { w: self }
    }
    #[doc = "Bits 16:25 - Time-out Clock Cycle This bit field indicates how many clock cycle selected by TMCNTSRC (UI2C_BRGEN \\[5\\]) when each interrupt flags are clear. The time-out is enable when TOCNT bigger than 0. Note: The TMCNTSRC (UI2C_BRGEN \\[5\\]) must be set zero on I2C mode."]
    #[inline(always)]
    pub fn tocnt(&mut self) -> TOCNT_W {
        TOCNT_W { w: self }
    }
    #[doc = "Bit 31 - I2C Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&mut self) -> PROTEN_W {
        PROTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_protctl](index.html) module"]
pub struct UI2C_PROTCTL_SPEC;
impl crate::RegisterSpec for UI2C_PROTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_protctl::R](R) reader structure"]
impl crate::Readable for UI2C_PROTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_protctl::W](W) writer structure"]
impl crate::Writable for UI2C_PROTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_PROTCTL to value 0"]
impl crate::Resettable for UI2C_PROTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
