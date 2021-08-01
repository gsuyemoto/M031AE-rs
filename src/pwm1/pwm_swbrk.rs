#[doc = "Register `PWM_SWBRK` writer"]
pub struct W(crate::W<PWM_SWBRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SWBRK_SPEC>;
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
impl From<crate::W<PWM_SWBRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SWBRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRKETRG0` writer - PWM Edge Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKETRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKETRG0_W<'a> {
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
#[doc = "Field `BRKETRG2` writer - PWM Edge Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKETRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKETRG2_W<'a> {
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
#[doc = "Field `BRKETRG4` writer - PWM Edge Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKETRG4_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKETRG4_W<'a> {
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
#[doc = "Field `BRKLTRG0` writer - PWM Level Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLTRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLTRG0_W<'a> {
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
#[doc = "Field `BRKLTRG2` writer - PWM Level Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLTRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLTRG2_W<'a> {
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
#[doc = "Field `BRKLTRG4` writer - PWM Level Brake Software Trigger (Write Only) (Write Protect)\nWrite 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. \nNote: This bit is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLTRG4_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLTRG4_W<'a> {
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
impl W {
    #[doc = "Bit 0 - PWM Edge Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brketrg0(&mut self) -> BRKETRG0_W {
        BRKETRG0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Edge Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brketrg2(&mut self) -> BRKETRG2_W {
        BRKETRG2_W { w: self }
    }
    #[doc = "Bit 2 - PWM Edge Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger Edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brketrg4(&mut self) -> BRKETRG4_W {
        BRKETRG4_W { w: self }
    }
    #[doc = "Bit 8 - PWM Level Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkltrg0(&mut self) -> BRKLTRG0_W {
        BRKLTRG0_W { w: self }
    }
    #[doc = "Bit 9 - PWM Level Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkltrg2(&mut self) -> BRKLTRG2_W {
        BRKLTRG2_W { w: self }
    }
    #[doc = "Bit 10 - PWM Level Brake Software Trigger (Write Only) (Write Protect) Write 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. Note: This bit is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkltrg4(&mut self) -> BRKLTRG4_W {
        BRKLTRG4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Software Brake Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_swbrk](index.html) module"]
pub struct PWM_SWBRK_SPEC;
impl crate::RegisterSpec for PWM_SWBRK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_swbrk::W](W) writer structure"]
impl crate::Writable for PWM_SWBRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SWBRK to value 0"]
impl crate::Resettable for PWM_SWBRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
