#[doc = "Register `I2C_BUSTCTL` reader"]
pub struct R(crate::R<I2C_BUSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_BUSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_BUSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_BUSTCTL` writer"]
pub struct W(crate::W<I2C_BUSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUSTCTL_SPEC>;
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
impl From<crate::W<I2C_BUSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_BUSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bus Time Out Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSTOEN_A {
    #[doc = "0: Bus clock low time-out detection Disabled"]
    _0 = 0,
    #[doc = "1: Bus clock low time-out detection Enabled (bus clock is low for more than TTime-out (in BIDLE=0) or high more than TTime-out(in BIDLE =1)"]
    _1 = 1,
}
impl From<BUSTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSTOEN` reader - Bus Time Out Enable Bit"]
pub struct BUSTOEN_R(crate::FieldReader<bool, BUSTOEN_A>);
impl BUSTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSTOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSTOEN_A {
        match self.bits {
            false => BUSTOEN_A::_0,
            true => BUSTOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSTOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSTOEN_A::_1
    }
}
impl core::ops::Deref for BUSTOEN_R {
    type Target = crate::FieldReader<bool, BUSTOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTOEN` writer - Bus Time Out Enable Bit"]
pub struct BUSTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSTOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock low time-out detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSTOEN_A::_0)
    }
    #[doc = "Bus clock low time-out detection Enabled (bus clock is low for more than TTime-out (in BIDLE=0) or high more than TTime-out(in BIDLE =1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSTOEN_A::_1)
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
#[doc = "Cumulative Clock Low Time Out Enable Bit\nFor Master, it calculates the period from START to ACK\nFor Slave, it calculates the period from START to STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKTOEN_A {
    #[doc = "0: Cumulative clock low time-out detection Disabled"]
    _0 = 0,
    #[doc = "1: Cumulative clock low time-out detection Enabled"]
    _1 = 1,
}
impl From<CLKTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKTOEN` reader - Cumulative Clock Low Time Out Enable Bit\nFor Master, it calculates the period from START to ACK\nFor Slave, it calculates the period from START to STOP"]
pub struct CLKTOEN_R(crate::FieldReader<bool, CLKTOEN_A>);
impl CLKTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKTOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKTOEN_A {
        match self.bits {
            false => CLKTOEN_A::_0,
            true => CLKTOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKTOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKTOEN_A::_1
    }
}
impl core::ops::Deref for CLKTOEN_R {
    type Target = crate::FieldReader<bool, CLKTOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKTOEN` writer - Cumulative Clock Low Time Out Enable Bit\nFor Master, it calculates the period from START to ACK\nFor Slave, it calculates the period from START to STOP"]
pub struct CLKTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKTOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKTOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cumulative clock low time-out detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKTOEN_A::_0)
    }
    #[doc = "Cumulative clock low time-out detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKTOEN_A::_1)
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
#[doc = "Time-out Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSTOIEN_A {
    #[doc = "0: SCLK low time-out interrupt Disabled.\\nBus IDLE time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: SCLK low time-out interrupt Enabled.\\nBus IDLE time-out interrupt Enabled"]
    _1 = 1,
}
impl From<BUSTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSTOIEN` reader - Time-out Interrupt Enable Bit"]
pub struct BUSTOIEN_R(crate::FieldReader<bool, BUSTOIEN_A>);
impl BUSTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSTOIEN_A {
        match self.bits {
            false => BUSTOIEN_A::_0,
            true => BUSTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSTOIEN_A::_1
    }
}
impl core::ops::Deref for BUSTOIEN_R {
    type Target = crate::FieldReader<bool, BUSTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTOIEN` writer - Time-out Interrupt Enable Bit"]
pub struct BUSTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCLK low time-out interrupt Disabled.\nBus IDLE time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSTOIEN_A::_0)
    }
    #[doc = "SCLK low time-out interrupt Enabled.\nBus IDLE time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSTOIEN_A::_1)
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
#[doc = "Extended Clock Time Out Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKTOIEN_A {
    #[doc = "0: Clock time out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Clock time out interrupt Enabled"]
    _1 = 1,
}
impl From<CLKTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKTOIEN` reader - Extended Clock Time Out Interrupt Enable Bit"]
pub struct CLKTOIEN_R(crate::FieldReader<bool, CLKTOIEN_A>);
impl CLKTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKTOIEN_A {
        match self.bits {
            false => CLKTOIEN_A::_0,
            true => CLKTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKTOIEN_A::_1
    }
}
impl core::ops::Deref for CLKTOIEN_R {
    type Target = crate::FieldReader<bool, CLKTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKTOIEN` writer - Extended Clock Time Out Interrupt Enable Bit"]
pub struct CLKTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock time out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKTOIEN_A::_0)
    }
    #[doc = "Clock time out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKTOIEN_A::_1)
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
#[doc = "Time Out Reset Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TORSTEN_A {
    #[doc = "0: I2C state machine reset Disabled"]
    _0 = 0,
    #[doc = "1: I2C state machine reset Enabled. (The clock and data bus will be released to high)"]
    _1 = 1,
}
impl From<TORSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TORSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TORSTEN` reader - Time Out Reset Enable Bit"]
pub struct TORSTEN_R(crate::FieldReader<bool, TORSTEN_A>);
impl TORSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TORSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TORSTEN_A {
        match self.bits {
            false => TORSTEN_A::_0,
            true => TORSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TORSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TORSTEN_A::_1
    }
}
impl core::ops::Deref for TORSTEN_R {
    type Target = crate::FieldReader<bool, TORSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TORSTEN` writer - Time Out Reset Enable Bit"]
pub struct TORSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TORSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TORSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C state machine reset Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TORSTEN_A::_0)
    }
    #[doc = "I2C state machine reset Enabled. (The clock and data bus will be released to high)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TORSTEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Bus Time Out Enable Bit"]
    #[inline(always)]
    pub fn bustoen(&self) -> BUSTOEN_R {
        BUSTOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cumulative Clock Low Time Out Enable Bit For Master, it calculates the period from START to ACK For Slave, it calculates the period from START to STOP"]
    #[inline(always)]
    pub fn clktoen(&self) -> CLKTOEN_R {
        CLKTOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn bustoien(&self) -> BUSTOIEN_R {
        BUSTOIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Extended Clock Time Out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn clktoien(&self) -> CLKTOIEN_R {
        CLKTOIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Out Reset Enable Bit"]
    #[inline(always)]
    pub fn torsten(&self) -> TORSTEN_R {
        TORSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Time Out Enable Bit"]
    #[inline(always)]
    pub fn bustoen(&mut self) -> BUSTOEN_W {
        BUSTOEN_W { w: self }
    }
    #[doc = "Bit 1 - Cumulative Clock Low Time Out Enable Bit For Master, it calculates the period from START to ACK For Slave, it calculates the period from START to STOP"]
    #[inline(always)]
    pub fn clktoen(&mut self) -> CLKTOEN_W {
        CLKTOEN_W { w: self }
    }
    #[doc = "Bit 2 - Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn bustoien(&mut self) -> BUSTOIEN_W {
        BUSTOIEN_W { w: self }
    }
    #[doc = "Bit 3 - Extended Clock Time Out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn clktoien(&mut self) -> CLKTOIEN_W {
        CLKTOIEN_W { w: self }
    }
    #[doc = "Bit 4 - Time Out Reset Enable Bit"]
    #[inline(always)]
    pub fn torsten(&mut self) -> TORSTEN_W {
        TORSTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Management Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bustctl](index.html) module"]
pub struct I2C_BUSTCTL_SPEC;
impl crate::RegisterSpec for I2C_BUSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_bustctl::R](R) reader structure"]
impl crate::Readable for I2C_BUSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_bustctl::W](W) writer structure"]
impl crate::Writable for I2C_BUSTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_BUSTCTL to value 0"]
impl crate::Resettable for I2C_BUSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
