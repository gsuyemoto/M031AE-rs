#[doc = "Register `SYS_MODCTL` reader"]
pub struct R(crate::R<SYS_MODCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_MODCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_MODCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_MODCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_MODCTL` writer"]
pub struct W(crate::W<SYS_MODCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_MODCTL_SPEC>;
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
impl From<crate::W<SYS_MODCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_MODCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Modulation Function Enable Bit\nThis bit enables modulation funcion by modulating with PWM0 channel output and USCI0(USCI0_DAT0) or UART0(UART0_TXD) output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEN_A {
    #[doc = "0: Modulation Function Disabled"]
    _0 = 0,
    #[doc = "1: Modulation Function Enabled"]
    _1 = 1,
}
impl From<MODEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEN` reader - Modulation Function Enable Bit\nThis bit enables modulation funcion by modulating with PWM0 channel output and USCI0(USCI0_DAT0) or UART0(UART0_TXD) output."]
pub struct MODEN_R(crate::FieldReader<bool, MODEN_A>);
impl MODEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEN_A {
        match self.bits {
            false => MODEN_A::_0,
            true => MODEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODEN_A::_1
    }
}
impl core::ops::Deref for MODEN_R {
    type Target = crate::FieldReader<bool, MODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODEN` writer - Modulation Function Enable Bit\nThis bit enables modulation funcion by modulating with PWM0 channel output and USCI0(USCI0_DAT0) or UART0(UART0_TXD) output."]
pub struct MODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Modulation Function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODEN_A::_0)
    }
    #[doc = "Modulation Function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODEN_A::_1)
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
#[doc = "Field `MODH` reader - Modulation at Data High\nSelect modulation pulse(PWM0) at high or low of UART0_TXD or USCI0_DAT0\n0: Modulation pulse at UART0_TXD low or USCI0_DAT0 low.\n1: Modulation pulse at UART0_TXD high or USCI0_DAT0 high."]
pub struct MODH_R(crate::FieldReader<bool, bool>);
impl MODH_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODH` writer - Modulation at Data High\nSelect modulation pulse(PWM0) at high or low of UART0_TXD or USCI0_DAT0\n0: Modulation pulse at UART0_TXD low or USCI0_DAT0 low.\n1: Modulation pulse at UART0_TXD high or USCI0_DAT0 high."]
pub struct MODH_W<'a> {
    w: &'a mut W,
}
impl<'a> MODH_W<'a> {
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
#[doc = "Field `MODPWMSEL` reader - PWM0 Channel Select for Modulation\nSelect the PWM0 channel to modulate with the UART0_TXD or USCI0_DAT0.\n0000: PWM0 Channel 0 modulate with UART0_TXD.\n0001: PWM0 Channel 1 modulate with UART0_TXD.\n0010: PWM0 Channel 2 modulate with UART0_TXD.\n0011: PWM0 Channel 3 modulete with UART0_TXD.\n0100: PWM0 Channel 4 modulete with UART0_TXD.\n0101: PWM0 Channel 5 modulete with UART0_TXD.\n0110: Reserved.\n0111: Reserved.\n1000: PWM0 Channel 0 modulate with USCI0_DAT0.\n1001: PWM0 Channel 1 modulate with USCI0_DAT0.\n1010: PWM0 Channel 2 modulate with USCI0_DAT0.\n1011: PWM0 Channel 3 modulete with USCI0_DAT0.\n1100: PWM0 Channel 4 modulete with USCI0_DAT0.\n1101: PWM0 Channel 5 modulete with USCI0_DAT0.\n1110: Reserved.\n1111: Reserved.\nNote: This bis is valid while MODEN (SYS_MODCTL\\[0\\]) is set to 1."]
pub struct MODPWMSEL_R(crate::FieldReader<u8, u8>);
impl MODPWMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODPWMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODPWMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODPWMSEL` writer - PWM0 Channel Select for Modulation\nSelect the PWM0 channel to modulate with the UART0_TXD or USCI0_DAT0.\n0000: PWM0 Channel 0 modulate with UART0_TXD.\n0001: PWM0 Channel 1 modulate with UART0_TXD.\n0010: PWM0 Channel 2 modulate with UART0_TXD.\n0011: PWM0 Channel 3 modulete with UART0_TXD.\n0100: PWM0 Channel 4 modulete with UART0_TXD.\n0101: PWM0 Channel 5 modulete with UART0_TXD.\n0110: Reserved.\n0111: Reserved.\n1000: PWM0 Channel 0 modulate with USCI0_DAT0.\n1001: PWM0 Channel 1 modulate with USCI0_DAT0.\n1010: PWM0 Channel 2 modulate with USCI0_DAT0.\n1011: PWM0 Channel 3 modulete with USCI0_DAT0.\n1100: PWM0 Channel 4 modulete with USCI0_DAT0.\n1101: PWM0 Channel 5 modulete with USCI0_DAT0.\n1110: Reserved.\n1111: Reserved.\nNote: This bis is valid while MODEN (SYS_MODCTL\\[0\\]) is set to 1."]
pub struct MODPWMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODPWMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Modulation Function Enable Bit This bit enables modulation funcion by modulating with PWM0 channel output and USCI0(USCI0_DAT0) or UART0(UART0_TXD) output."]
    #[inline(always)]
    pub fn moden(&self) -> MODEN_R {
        MODEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Modulation at Data High Select modulation pulse(PWM0) at high or low of UART0_TXD or USCI0_DAT0 0: Modulation pulse at UART0_TXD low or USCI0_DAT0 low. 1: Modulation pulse at UART0_TXD high or USCI0_DAT0 high."]
    #[inline(always)]
    pub fn modh(&self) -> MODH_R {
        MODH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - PWM0 Channel Select for Modulation Select the PWM0 channel to modulate with the UART0_TXD or USCI0_DAT0. 0000: PWM0 Channel 0 modulate with UART0_TXD. 0001: PWM0 Channel 1 modulate with UART0_TXD. 0010: PWM0 Channel 2 modulate with UART0_TXD. 0011: PWM0 Channel 3 modulete with UART0_TXD. 0100: PWM0 Channel 4 modulete with UART0_TXD. 0101: PWM0 Channel 5 modulete with UART0_TXD. 0110: Reserved. 0111: Reserved. 1000: PWM0 Channel 0 modulate with USCI0_DAT0. 1001: PWM0 Channel 1 modulate with USCI0_DAT0. 1010: PWM0 Channel 2 modulate with USCI0_DAT0. 1011: PWM0 Channel 3 modulete with USCI0_DAT0. 1100: PWM0 Channel 4 modulete with USCI0_DAT0. 1101: PWM0 Channel 5 modulete with USCI0_DAT0. 1110: Reserved. 1111: Reserved. Note: This bis is valid while MODEN (SYS_MODCTL\\[0\\]) is set to 1."]
    #[inline(always)]
    pub fn modpwmsel(&self) -> MODPWMSEL_R {
        MODPWMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation Function Enable Bit This bit enables modulation funcion by modulating with PWM0 channel output and USCI0(USCI0_DAT0) or UART0(UART0_TXD) output."]
    #[inline(always)]
    pub fn moden(&mut self) -> MODEN_W {
        MODEN_W { w: self }
    }
    #[doc = "Bit 1 - Modulation at Data High Select modulation pulse(PWM0) at high or low of UART0_TXD or USCI0_DAT0 0: Modulation pulse at UART0_TXD low or USCI0_DAT0 low. 1: Modulation pulse at UART0_TXD high or USCI0_DAT0 high."]
    #[inline(always)]
    pub fn modh(&mut self) -> MODH_W {
        MODH_W { w: self }
    }
    #[doc = "Bits 4:7 - PWM0 Channel Select for Modulation Select the PWM0 channel to modulate with the UART0_TXD or USCI0_DAT0. 0000: PWM0 Channel 0 modulate with UART0_TXD. 0001: PWM0 Channel 1 modulate with UART0_TXD. 0010: PWM0 Channel 2 modulate with UART0_TXD. 0011: PWM0 Channel 3 modulete with UART0_TXD. 0100: PWM0 Channel 4 modulete with UART0_TXD. 0101: PWM0 Channel 5 modulete with UART0_TXD. 0110: Reserved. 0111: Reserved. 1000: PWM0 Channel 0 modulate with USCI0_DAT0. 1001: PWM0 Channel 1 modulate with USCI0_DAT0. 1010: PWM0 Channel 2 modulate with USCI0_DAT0. 1011: PWM0 Channel 3 modulete with USCI0_DAT0. 1100: PWM0 Channel 4 modulete with USCI0_DAT0. 1101: PWM0 Channel 5 modulete with USCI0_DAT0. 1110: Reserved. 1111: Reserved. Note: This bis is valid while MODEN (SYS_MODCTL\\[0\\]) is set to 1."]
    #[inline(always)]
    pub fn modpwmsel(&mut self) -> MODPWMSEL_W {
        MODPWMSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modulation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_modctl](index.html) module"]
pub struct SYS_MODCTL_SPEC;
impl crate::RegisterSpec for SYS_MODCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_modctl::R](R) reader structure"]
impl crate::Readable for SYS_MODCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_modctl::W](W) writer structure"]
impl crate::Writable for SYS_MODCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_MODCTL to value 0"]
impl crate::Resettable for SYS_MODCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
