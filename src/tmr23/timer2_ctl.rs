#[doc = "Register `TIMER2_CTL` reader"]
pub struct R(crate::R<TIMER2_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_CTL` writer"]
pub struct W(crate::W<TIMER2_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_CTL_SPEC>;
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
impl From<crate::W<TIMER2_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - Prescale Counter\nNote: Updating prescale counter value will reset internal 8-bit prescale counter and 24-bit up counter value."]
pub struct PSC_R(crate::FieldReader<u8, u8>);
impl PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - Prescale Counter\nNote: Updating prescale counter value will reset internal 8-bit prescale counter and 24-bit up counter value."]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Trigger PDMA Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PDMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPDMA_A {
    #[doc = "0: Timer interrupt trigger PDMA Disabled"]
    _0 = 0,
    #[doc = "1: Timer interrupt trigger PDMA Enabled"]
    _1 = 1,
}
impl From<TRGPDMA_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPDMA` reader - Trigger PDMA Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PDMA."]
pub struct TRGPDMA_R(crate::FieldReader<bool, TRGPDMA_A>);
impl TRGPDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGPDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPDMA_A {
        match self.bits {
            false => TRGPDMA_A::_0,
            true => TRGPDMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGPDMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGPDMA_A::_1
    }
}
impl core::ops::Deref for TRGPDMA_R {
    type Target = crate::FieldReader<bool, TRGPDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGPDMA` writer - Trigger PDMA Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PDMA."]
pub struct TRGPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGPDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGPDMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer interrupt trigger PDMA Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPDMA_A::_0)
    }
    #[doc = "Timer interrupt trigger PDMA Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPDMA_A::_1)
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
#[doc = "Trigger BPWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger BPWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGBPWM_A {
    #[doc = "0: Timer interrupt trigger BPWM Disabled"]
    _0 = 0,
    #[doc = "1: Timer interrupt trigger BPWM Enabled"]
    _1 = 1,
}
impl From<TRGBPWM_A> for bool {
    #[inline(always)]
    fn from(variant: TRGBPWM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGBPWM` reader - Trigger BPWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger BPWM."]
pub struct TRGBPWM_R(crate::FieldReader<bool, TRGBPWM_A>);
impl TRGBPWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGBPWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGBPWM_A {
        match self.bits {
            false => TRGBPWM_A::_0,
            true => TRGBPWM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGBPWM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGBPWM_A::_1
    }
}
impl core::ops::Deref for TRGBPWM_R {
    type Target = crate::FieldReader<bool, TRGBPWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGBPWM` writer - Trigger BPWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger BPWM."]
pub struct TRGBPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGBPWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGBPWM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer interrupt trigger BPWM Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGBPWM_A::_0)
    }
    #[doc = "Timer interrupt trigger BPWM Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGBPWM_A::_1)
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
#[doc = "Inter-timer Trigger Mode Enable Bit\nSetting this bit will enable the inter-timer trigger capture function.\nThe Timer0/2 will be in event counter mode and counting with external clock source or event.Also, Timer1/3 will be in trigger-counting mode of capture function.\nNote: For Timer1/3, this bit is ignored and the read back value is always 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTRGEN_A {
    #[doc = "0: Inter-Timer Trigger mode Disabled"]
    _0 = 0,
    #[doc = "1: Inter-Timer Trigger mode Enabled"]
    _1 = 1,
}
impl From<INTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTRGEN` reader - Inter-timer Trigger Mode Enable Bit\nSetting this bit will enable the inter-timer trigger capture function.\nThe Timer0/2 will be in event counter mode and counting with external clock source or event.Also, Timer1/3 will be in trigger-counting mode of capture function.\nNote: For Timer1/3, this bit is ignored and the read back value is always 0."]
pub struct INTRGEN_R(crate::FieldReader<bool, INTRGEN_A>);
impl INTRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTRGEN_A {
        match self.bits {
            false => INTRGEN_A::_0,
            true => INTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTRGEN_A::_1
    }
}
impl core::ops::Deref for INTRGEN_R {
    type Target = crate::FieldReader<bool, INTRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTRGEN` writer - Inter-timer Trigger Mode Enable Bit\nSetting this bit will enable the inter-timer trigger capture function.\nThe Timer0/2 will be in event counter mode and counting with external clock source or event.Also, Timer1/3 will be in trigger-counting mode of capture function.\nNote: For Timer1/3, this bit is ignored and the read back value is always 0."]
pub struct INTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTRGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inter-Timer Trigger mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTRGEN_A::_0)
    }
    #[doc = "Inter-Timer Trigger mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTRGEN_A::_1)
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
#[doc = "Capture Pin Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPSRC_A {
    #[doc = "0: Capture Function source is from TMx_EXT (x= 0~3) pin"]
    _0 = 0,
    #[doc = "1: Capture Function source is from internal ACMP output signal or LIRC. User can set INTERCAPSEL (TIMERx_EXTCTL\\[10:8\\]) to decide which internal ACMP output signal or LIRC as timer capture source"]
    _1 = 1,
}
impl From<CAPSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CAPSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPSRC` reader - Capture Pin Source Selection"]
pub struct CAPSRC_R(crate::FieldReader<bool, CAPSRC_A>);
impl CAPSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPSRC_A {
        match self.bits {
            false => CAPSRC_A::_0,
            true => CAPSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPSRC_A::_1
    }
}
impl core::ops::Deref for CAPSRC_R {
    type Target = crate::FieldReader<bool, CAPSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPSRC` writer - Capture Pin Source Selection"]
pub struct CAPSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture Function source is from TMx_EXT (x= 0~3) pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPSRC_A::_0)
    }
    #[doc = "Capture Function source is from internal ACMP output signal or LIRC. User can set INTERCAPSEL (TIMERx_EXTCTL\\[10:8\\]) to decide which internal ACMP output signal or LIRC as timer capture source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPSRC_A::_1)
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
#[doc = "Trigger Source Select Bit\nThis bit is used to select trigger source is from Timer time-out interrupt signal or capture interrupt signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSSEL_A {
    #[doc = "0: Timer time-out interrupt signal is used to trigger PWM, BPWM, ADC and PDMA"]
    _0 = 0,
    #[doc = "1: Capture interrupt signal is used to trigger PWM, BPWM, ADC and PDMA"]
    _1 = 1,
}
impl From<TRGSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSSEL` reader - Trigger Source Select Bit\nThis bit is used to select trigger source is from Timer time-out interrupt signal or capture interrupt signal."]
pub struct TRGSSEL_R(crate::FieldReader<bool, TRGSSEL_A>);
impl TRGSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSSEL_A {
        match self.bits {
            false => TRGSSEL_A::_0,
            true => TRGSSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSSEL_A::_1
    }
}
impl core::ops::Deref for TRGSSEL_R {
    type Target = crate::FieldReader<bool, TRGSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSSEL` writer - Trigger Source Select Bit\nThis bit is used to select trigger source is from Timer time-out interrupt signal or capture interrupt signal."]
pub struct TRGSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer time-out interrupt signal is used to trigger PWM, BPWM, ADC and PDMA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSSEL_A::_0)
    }
    #[doc = "Capture interrupt signal is used to trigger PWM, BPWM, ADC and PDMA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSSEL_A::_1)
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
#[doc = "Trigger PWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPWM_A {
    #[doc = "0: Timer interrupt trigger PWM Disabled"]
    _0 = 0,
    #[doc = "1: Timer interrupt trigger PWM Enabled"]
    _1 = 1,
}
impl From<TRGPWM_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPWM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPWM` reader - Trigger PWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PWM."]
pub struct TRGPWM_R(crate::FieldReader<bool, TRGPWM_A>);
impl TRGPWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGPWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPWM_A {
        match self.bits {
            false => TRGPWM_A::_0,
            true => TRGPWM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGPWM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGPWM_A::_1
    }
}
impl core::ops::Deref for TRGPWM_R {
    type Target = crate::FieldReader<bool, TRGPWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGPWM` writer - Trigger PWM Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PWM."]
pub struct TRGPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGPWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGPWM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer interrupt trigger PWM Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPWM_A::_0)
    }
    #[doc = "Timer interrupt trigger PWM Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPWM_A::_1)
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
#[doc = "Trigger ADC Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGADC_A {
    #[doc = "0: Timer interrupt trigger ADC Disabled"]
    _0 = 0,
    #[doc = "1: Timer interrupt trigger ADC Enabled"]
    _1 = 1,
}
impl From<TRGADC_A> for bool {
    #[inline(always)]
    fn from(variant: TRGADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGADC` reader - Trigger ADC Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger ADC."]
pub struct TRGADC_R(crate::FieldReader<bool, TRGADC_A>);
impl TRGADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGADC_A {
        match self.bits {
            false => TRGADC_A::_0,
            true => TRGADC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGADC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGADC_A::_1
    }
}
impl core::ops::Deref for TRGADC_R {
    type Target = crate::FieldReader<bool, TRGADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGADC` writer - Trigger ADC Enable Bit\nIf this bit is set to 1, timer time-out interrupt or capture interrupt can trigger ADC."]
pub struct TRGADC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer interrupt trigger ADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGADC_A::_0)
    }
    #[doc = "Timer interrupt trigger ADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGADC_A::_1)
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
#[doc = "Toggle-output Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGLPINSEL_A {
    #[doc = "0: Toggle mode output to Tx (Timer Event Counter Pin)"]
    _0 = 0,
    #[doc = "1: Toggle mode output to Tx_EXT (Timer External Capture Pin)"]
    _1 = 1,
}
impl From<TGLPINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TGLPINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGLPINSEL` reader - Toggle-output Pin Select"]
pub struct TGLPINSEL_R(crate::FieldReader<bool, TGLPINSEL_A>);
impl TGLPINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGLPINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGLPINSEL_A {
        match self.bits {
            false => TGLPINSEL_A::_0,
            true => TGLPINSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TGLPINSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TGLPINSEL_A::_1
    }
}
impl core::ops::Deref for TGLPINSEL_R {
    type Target = crate::FieldReader<bool, TGLPINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGLPINSEL` writer - Toggle-output Pin Select"]
pub struct TGLPINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TGLPINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGLPINSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Toggle mode output to Tx (Timer Event Counter Pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TGLPINSEL_A::_0)
    }
    #[doc = "Toggle mode output to Tx_EXT (Timer External Capture Pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TGLPINSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Wake-up Function Enable Bit\nIf this bit is set to 1, while timer interrupt flag TIF (TIMERx_INTSTS\\[0\\]) is 1 and INTEN (TIMERx_CTL\\[29\\]) is enabled, the timer interrupt signal will generate a wake-up trigger event to CPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: Wake-up function Disabled if timer interrupt signal generated"]
    _0 = 0,
    #[doc = "1: Wake-up function Enabled if timer interrupt signal generated"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - Wake-up Function Enable Bit\nIf this bit is set to 1, while timer interrupt flag TIF (TIMERx_INTSTS\\[0\\]) is 1 and INTEN (TIMERx_CTL\\[29\\]) is enabled, the timer interrupt signal will generate a wake-up trigger event to CPU."]
pub struct WKEN_R(crate::FieldReader<bool, WKEN_A>);
impl WKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKEN_A {
        match self.bits {
            false => WKEN_A::_0,
            true => WKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKEN_A::_1
    }
}
impl core::ops::Deref for WKEN_R {
    type Target = crate::FieldReader<bool, WKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKEN` writer - Wake-up Function Enable Bit\nIf this bit is set to 1, while timer interrupt flag TIF (TIMERx_INTSTS\\[0\\]) is 1 and INTEN (TIMERx_CTL\\[29\\]) is enabled, the timer interrupt signal will generate a wake-up trigger event to CPU."]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake-up function Disabled if timer interrupt signal generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "Wake-up function Enabled if timer interrupt signal generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Event Counter Mode Enable Bit \nThis bit is for external counting pin function enabled. \nNote 2: When TMR0/TMR2 INTRGEN is set to 1, this bit is forced to 1. When INTRGEN is 1 and TMR1/TMR3 CAPIF (TIMERx_EINTSTS\\[0\\]) is 1, this bit is forced to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCNTEN_A {
    #[doc = "0: Event counter mode Disabled"]
    _0 = 0,
    #[doc = "1: Event counter mode Enabled"]
    _1 = 1,
}
impl From<EXTCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCNTEN` reader - Event Counter Mode Enable Bit \nThis bit is for external counting pin function enabled. \nNote 2: When TMR0/TMR2 INTRGEN is set to 1, this bit is forced to 1. When INTRGEN is 1 and TMR1/TMR3 CAPIF (TIMERx_EINTSTS\\[0\\]) is 1, this bit is forced to 0."]
pub struct EXTCNTEN_R(crate::FieldReader<bool, EXTCNTEN_A>);
impl EXTCNTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTCNTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCNTEN_A {
        match self.bits {
            false => EXTCNTEN_A::_0,
            true => EXTCNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EXTCNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EXTCNTEN_A::_1
    }
}
impl core::ops::Deref for EXTCNTEN_R {
    type Target = crate::FieldReader<bool, EXTCNTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTCNTEN` writer - Event Counter Mode Enable Bit \nThis bit is for external counting pin function enabled. \nNote 2: When TMR0/TMR2 INTRGEN is set to 1, this bit is forced to 1. When INTRGEN is 1 and TMR1/TMR3 CAPIF (TIMERx_EINTSTS\\[0\\]) is 1, this bit is forced to 0."]
pub struct EXTCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCNTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event counter mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTCNTEN_A::_0)
    }
    #[doc = "Event counter mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTCNTEN_A::_1)
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
#[doc = "Timer Active Status Bit (Read Only)\nThis bit indicates the 24-bit up counter status.\nNote: This bit may active when CNT 0 transition to CNT 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSTS_A {
    #[doc = "0: 24-bit up counter is not active"]
    _0 = 0,
    #[doc = "1: 24-bit up counter is active"]
    _1 = 1,
}
impl From<ACTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: ACTSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSTS` reader - Timer Active Status Bit (Read Only)\nThis bit indicates the 24-bit up counter status.\nNote: This bit may active when CNT 0 transition to CNT 1."]
pub struct ACTSTS_R(crate::FieldReader<bool, ACTSTS_A>);
impl ACTSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSTS_A {
        match self.bits {
            false => ACTSTS_A::_0,
            true => ACTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACTSTS_A::_1
    }
}
impl core::ops::Deref for ACTSTS_R {
    type Target = crate::FieldReader<bool, ACTSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timer Counter Reset Bit\nSetting this bit will reset the 24-bit up counter value CNT (TIMERx_CNT\\[23:0\\]) and also force CNTEN (TIMERx_CTL\\[30\\]) to 0 if ACTSTS (TIMERx_CTL\\[25\\]) is 1.\nNote: This bit will be auto cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCNT_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset internal 8-bit prescale counter, 24-bit up counter value and CNTEN bit"]
    _1 = 1,
}
impl From<RSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCNT` reader - Timer Counter Reset Bit\nSetting this bit will reset the 24-bit up counter value CNT (TIMERx_CNT\\[23:0\\]) and also force CNTEN (TIMERx_CTL\\[30\\]) to 0 if ACTSTS (TIMERx_CTL\\[25\\]) is 1.\nNote: This bit will be auto cleared."]
pub struct RSTCNT_R(crate::FieldReader<bool, RSTCNT_A>);
impl RSTCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCNT_A {
        match self.bits {
            false => RSTCNT_A::_0,
            true => RSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTCNT_A::_1
    }
}
impl core::ops::Deref for RSTCNT_R {
    type Target = crate::FieldReader<bool, RSTCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCNT` writer - Timer Counter Reset Bit\nSetting this bit will reset the 24-bit up counter value CNT (TIMERx_CNT\\[23:0\\]) and also force CNTEN (TIMERx_CTL\\[30\\]) to 0 if ACTSTS (TIMERx_CTL\\[25\\]) is 1.\nNote: This bit will be auto cleared."]
pub struct RSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTCNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTCNT_A::_0)
    }
    #[doc = "Reset internal 8-bit prescale counter, 24-bit up counter value and CNTEN bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTCNT_A::_1)
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
#[doc = "Timer Counting Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: The timer controller is operated in One-shot mode"]
    _0 = 0,
    #[doc = "1: The timer controller is operated in Periodic mode"]
    _1 = 1,
    #[doc = "2: The timer controller is operated in Toggle-output mode"]
    _2 = 2,
    #[doc = "3: The timer controller is operated in Continuous Counting mode"]
    _3 = 3,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMODE` reader - Timer Counting Mode Select"]
pub struct OPMODE_R(crate::FieldReader<u8, OPMODE_A>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPMODE_A {
        match self.bits {
            0 => OPMODE_A::_0,
            1 => OPMODE_A::_1,
            2 => OPMODE_A::_2,
            3 => OPMODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OPMODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == OPMODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == OPMODE_A::_3
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, OPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - Timer Counting Mode Select"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The timer controller is operated in One-shot mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPMODE_A::_0)
    }
    #[doc = "The timer controller is operated in Periodic mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPMODE_A::_1)
    }
    #[doc = "The timer controller is operated in Toggle-output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(OPMODE_A::_2)
    }
    #[doc = "The timer controller is operated in Continuous Counting mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(OPMODE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Timer Interrupt Enable Bit\nNote: If this bit is enabled, when the timer time-out interrupt flag TIF is set to 1, the timer interrupt signal is generated and inform to CPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: Timer time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Timer time-out interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Timer Interrupt Enable Bit\nNote: If this bit is enabled, when the timer time-out interrupt flag TIF is set to 1, the timer interrupt signal is generated and inform to CPU."]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::_0,
            true => INTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN_A::_1
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - Timer Interrupt Enable Bit\nNote: If this bit is enabled, when the timer time-out interrupt flag TIF is set to 1, the timer interrupt signal is generated and inform to CPU."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
    #[doc = "Timer time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
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
#[doc = "Timer Counting Enable Bit\nNote 3: Set enable/disable this bit needs 2 * TMR_CLK period to become active, user can read ACTSTS (TIMERx_CTL\\[25\\]) to check enable/disable command is completed or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN_A {
    #[doc = "0: Stops/Suspends counting"]
    _0 = 0,
    #[doc = "1: Starts counting"]
    _1 = 1,
}
impl From<CNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN` reader - Timer Counting Enable Bit\nNote 3: Set enable/disable this bit needs 2 * TMR_CLK period to become active, user can read ACTSTS (TIMERx_CTL\\[25\\]) to check enable/disable command is completed or not."]
pub struct CNTEN_R(crate::FieldReader<bool, CNTEN_A>);
impl CNTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN_A {
        match self.bits {
            false => CNTEN_A::_0,
            true => CNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN_A::_1
    }
}
impl core::ops::Deref for CNTEN_R {
    type Target = crate::FieldReader<bool, CNTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN` writer - Timer Counting Enable Bit\nNote 3: Set enable/disable this bit needs 2 * TMR_CLK period to become active, user can read ACTSTS (TIMERx_CTL\\[25\\]) to check enable/disable command is completed or not."]
pub struct CNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stops/Suspends counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN_A::_0)
    }
    #[doc = "Starts counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN_A::_1)
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
#[doc = "ICE Debug Mode Acknowledge Disable Bit (Write Protect)\nTIMER counter will keep going no matter CPU is held by ICE or not.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEDEBUG_A {
    #[doc = "0: ICE debug mode acknowledgement effects TIMER counting"]
    _0 = 0,
    #[doc = "1: ICE debug mode acknowledgement Disabled"]
    _1 = 1,
}
impl From<ICEDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: ICEDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEDEBUG` reader - ICE Debug Mode Acknowledge Disable Bit (Write Protect)\nTIMER counter will keep going no matter CPU is held by ICE or not.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ICEDEBUG_R(crate::FieldReader<bool, ICEDEBUG_A>);
impl ICEDEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEDEBUG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDEBUG_A {
        match self.bits {
            false => ICEDEBUG_A::_0,
            true => ICEDEBUG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICEDEBUG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICEDEBUG_A::_1
    }
}
impl core::ops::Deref for ICEDEBUG_R {
    type Target = crate::FieldReader<bool, ICEDEBUG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEDEBUG` writer - ICE Debug Mode Acknowledge Disable Bit (Write Protect)\nTIMER counter will keep going no matter CPU is held by ICE or not.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ICEDEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEDEBUG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEDEBUG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode acknowledgement effects TIMER counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_0)
    }
    #[doc = "ICE debug mode acknowledgement Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_1)
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
    #[doc = "Bits 0:7 - Prescale Counter Note: Updating prescale counter value will reset internal 8-bit prescale counter and 24-bit up counter value."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Trigger PDMA Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PDMA."]
    #[inline(always)]
    pub fn trgpdma(&self) -> TRGPDMA_R {
        TRGPDMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Trigger BPWM Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger BPWM."]
    #[inline(always)]
    pub fn trgbpwm(&self) -> TRGBPWM_R {
        TRGBPWM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Inter-timer Trigger Mode Enable Bit Setting this bit will enable the inter-timer trigger capture function. The Timer0/2 will be in event counter mode and counting with external clock source or event.Also, Timer1/3 will be in trigger-counting mode of capture function. Note: For Timer1/3, this bit is ignored and the read back value is always 0."]
    #[inline(always)]
    pub fn intrgen(&self) -> INTRGEN_R {
        INTRGEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Capture Pin Source Selection"]
    #[inline(always)]
    pub fn capsrc(&self) -> CAPSRC_R {
        CAPSRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Trigger Source Select Bit This bit is used to select trigger source is from Timer time-out interrupt signal or capture interrupt signal."]
    #[inline(always)]
    pub fn trgssel(&self) -> TRGSSEL_R {
        TRGSSEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Trigger PWM Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PWM."]
    #[inline(always)]
    pub fn trgpwm(&self) -> TRGPWM_R {
        TRGPWM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Trigger ADC Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger ADC."]
    #[inline(always)]
    pub fn trgadc(&self) -> TRGADC_R {
        TRGADC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Toggle-output Pin Select"]
    #[inline(always)]
    pub fn tglpinsel(&self) -> TGLPINSEL_R {
        TGLPINSEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Wake-up Function Enable Bit If this bit is set to 1, while timer interrupt flag TIF (TIMERx_INTSTS\\[0\\]) is 1 and INTEN (TIMERx_CTL\\[29\\]) is enabled, the timer interrupt signal will generate a wake-up trigger event to CPU."]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event Counter Mode Enable Bit This bit is for external counting pin function enabled. Note 2: When TMR0/TMR2 INTRGEN is set to 1, this bit is forced to 1. When INTRGEN is 1 and TMR1/TMR3 CAPIF (TIMERx_EINTSTS\\[0\\]) is 1, this bit is forced to 0."]
    #[inline(always)]
    pub fn extcnten(&self) -> EXTCNTEN_R {
        EXTCNTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer Active Status Bit (Read Only) This bit indicates the 24-bit up counter status. Note: This bit may active when CNT 0 transition to CNT 1."]
    #[inline(always)]
    pub fn actsts(&self) -> ACTSTS_R {
        ACTSTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer Counter Reset Bit Setting this bit will reset the 24-bit up counter value CNT (TIMERx_CNT\\[23:0\\]) and also force CNTEN (TIMERx_CTL\\[30\\]) to 0 if ACTSTS (TIMERx_CTL\\[25\\]) is 1. Note: This bit will be auto cleared."]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Timer Counting Mode Select"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Timer Interrupt Enable Bit Note: If this bit is enabled, when the timer time-out interrupt flag TIF is set to 1, the timer interrupt signal is generated and inform to CPU."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Timer Counting Enable Bit Note 3: Set enable/disable this bit needs 2 * TMR_CLK period to become active, user can read ACTSTS (TIMERx_CTL\\[25\\]) to check enable/disable command is completed or not."]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Bit (Write Protect) TIMER counter will keep going no matter CPU is held by ICE or not. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn icedebug(&self) -> ICEDEBUG_R {
        ICEDEBUG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescale Counter Note: Updating prescale counter value will reset internal 8-bit prescale counter and 24-bit up counter value."]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bit 8 - Trigger PDMA Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PDMA."]
    #[inline(always)]
    pub fn trgpdma(&mut self) -> TRGPDMA_W {
        TRGPDMA_W { w: self }
    }
    #[doc = "Bit 9 - Trigger BPWM Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger BPWM."]
    #[inline(always)]
    pub fn trgbpwm(&mut self) -> TRGBPWM_W {
        TRGBPWM_W { w: self }
    }
    #[doc = "Bit 10 - Inter-timer Trigger Mode Enable Bit Setting this bit will enable the inter-timer trigger capture function. The Timer0/2 will be in event counter mode and counting with external clock source or event.Also, Timer1/3 will be in trigger-counting mode of capture function. Note: For Timer1/3, this bit is ignored and the read back value is always 0."]
    #[inline(always)]
    pub fn intrgen(&mut self) -> INTRGEN_W {
        INTRGEN_W { w: self }
    }
    #[doc = "Bit 16 - Capture Pin Source Selection"]
    #[inline(always)]
    pub fn capsrc(&mut self) -> CAPSRC_W {
        CAPSRC_W { w: self }
    }
    #[doc = "Bit 18 - Trigger Source Select Bit This bit is used to select trigger source is from Timer time-out interrupt signal or capture interrupt signal."]
    #[inline(always)]
    pub fn trgssel(&mut self) -> TRGSSEL_W {
        TRGSSEL_W { w: self }
    }
    #[doc = "Bit 19 - Trigger PWM Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger PWM."]
    #[inline(always)]
    pub fn trgpwm(&mut self) -> TRGPWM_W {
        TRGPWM_W { w: self }
    }
    #[doc = "Bit 21 - Trigger ADC Enable Bit If this bit is set to 1, timer time-out interrupt or capture interrupt can trigger ADC."]
    #[inline(always)]
    pub fn trgadc(&mut self) -> TRGADC_W {
        TRGADC_W { w: self }
    }
    #[doc = "Bit 22 - Toggle-output Pin Select"]
    #[inline(always)]
    pub fn tglpinsel(&mut self) -> TGLPINSEL_W {
        TGLPINSEL_W { w: self }
    }
    #[doc = "Bit 23 - Wake-up Function Enable Bit If this bit is set to 1, while timer interrupt flag TIF (TIMERx_INTSTS\\[0\\]) is 1 and INTEN (TIMERx_CTL\\[29\\]) is enabled, the timer interrupt signal will generate a wake-up trigger event to CPU."]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Bit 24 - Event Counter Mode Enable Bit This bit is for external counting pin function enabled. Note 2: When TMR0/TMR2 INTRGEN is set to 1, this bit is forced to 1. When INTRGEN is 1 and TMR1/TMR3 CAPIF (TIMERx_EINTSTS\\[0\\]) is 1, this bit is forced to 0."]
    #[inline(always)]
    pub fn extcnten(&mut self) -> EXTCNTEN_W {
        EXTCNTEN_W { w: self }
    }
    #[doc = "Bit 26 - Timer Counter Reset Bit Setting this bit will reset the 24-bit up counter value CNT (TIMERx_CNT\\[23:0\\]) and also force CNTEN (TIMERx_CTL\\[30\\]) to 0 if ACTSTS (TIMERx_CTL\\[25\\]) is 1. Note: This bit will be auto cleared."]
    #[inline(always)]
    pub fn rstcnt(&mut self) -> RSTCNT_W {
        RSTCNT_W { w: self }
    }
    #[doc = "Bits 27:28 - Timer Counting Mode Select"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bit 29 - Timer Interrupt Enable Bit Note: If this bit is enabled, when the timer time-out interrupt flag TIF is set to 1, the timer interrupt signal is generated and inform to CPU."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 30 - Timer Counting Enable Bit Note 3: Set enable/disable this bit needs 2 * TMR_CLK period to become active, user can read ACTSTS (TIMERx_CTL\\[25\\]) to check enable/disable command is completed or not."]
    #[inline(always)]
    pub fn cnten(&mut self) -> CNTEN_W {
        CNTEN_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Bit (Write Protect) TIMER counter will keep going no matter CPU is held by ICE or not. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn icedebug(&mut self) -> ICEDEBUG_W {
        ICEDEBUG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_ctl](index.html) module"]
pub struct TIMER2_CTL_SPEC;
impl crate::RegisterSpec for TIMER2_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_ctl::R](R) reader structure"]
impl crate::Readable for TIMER2_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_ctl::W](W) writer structure"]
impl crate::Writable for TIMER2_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER2_CTL to value 0x05"]
impl crate::Resettable for TIMER2_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
