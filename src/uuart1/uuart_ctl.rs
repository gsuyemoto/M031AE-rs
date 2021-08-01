#[doc = "Register `UUART_CTL` reader"]
pub struct R(crate::R<UUART_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_CTL` writer"]
pub struct W(crate::W<UUART_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_CTL_SPEC>;
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
impl From<crate::W<UUART_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function Mode\nThis bit field selects the protocol for this USCI controller. Selecting a protocol that is not available or a reserved combination disables the USCI. When switching between two protocols, the USCI has to be disabled before selecting a new protocol. Simultaneously, the USCI will be reset when user write 000 to FUNMODE.\nNote: Other bit combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNMODE_A {
    #[doc = "0: The USCI is disabled. All protocol related state machines are set to idle state"]
    _0 = 0,
    #[doc = "1: The SPI protocol is selected"]
    _1 = 1,
    #[doc = "2: The UART protocol is selected"]
    _2 = 2,
    #[doc = "4: The I2C protocol is selected"]
    _4 = 4,
}
impl From<FUNMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNMODE` reader - Function Mode\nThis bit field selects the protocol for this USCI controller. Selecting a protocol that is not available or a reserved combination disables the USCI. When switching between two protocols, the USCI has to be disabled before selecting a new protocol. Simultaneously, the USCI will be reset when user write 000 to FUNMODE.\nNote: Other bit combinations are reserved."]
pub struct FUNMODE_R(crate::FieldReader<u8, FUNMODE_A>);
impl FUNMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNMODE_A> {
        match self.bits {
            0 => Some(FUNMODE_A::_0),
            1 => Some(FUNMODE_A::_1),
            2 => Some(FUNMODE_A::_2),
            4 => Some(FUNMODE_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FUNMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FUNMODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FUNMODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == FUNMODE_A::_4
    }
}
impl core::ops::Deref for FUNMODE_R {
    type Target = crate::FieldReader<u8, FUNMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNMODE` writer - Function Mode\nThis bit field selects the protocol for this USCI controller. Selecting a protocol that is not available or a reserved combination disables the USCI. When switching between two protocols, the USCI has to be disabled before selecting a new protocol. Simultaneously, the USCI will be reset when user write 000 to FUNMODE.\nNote: Other bit combinations are reserved."]
pub struct FUNMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The USCI is disabled. All protocol related state machines are set to idle state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUNMODE_A::_0)
    }
    #[doc = "The SPI protocol is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUNMODE_A::_1)
    }
    #[doc = "The UART protocol is selected"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FUNMODE_A::_2)
    }
    #[doc = "The I2C protocol is selected"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FUNMODE_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Mode This bit field selects the protocol for this USCI controller. Selecting a protocol that is not available or a reserved combination disables the USCI. When switching between two protocols, the USCI has to be disabled before selecting a new protocol. Simultaneously, the USCI will be reset when user write 000 to FUNMODE. Note: Other bit combinations are reserved."]
    #[inline(always)]
    pub fn funmode(&self) -> FUNMODE_R {
        FUNMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Mode This bit field selects the protocol for this USCI controller. Selecting a protocol that is not available or a reserved combination disables the USCI. When switching between two protocols, the USCI has to be disabled before selecting a new protocol. Simultaneously, the USCI will be reset when user write 000 to FUNMODE. Note: Other bit combinations are reserved."]
    #[inline(always)]
    pub fn funmode(&mut self) -> FUNMODE_W {
        FUNMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_ctl](index.html) module"]
pub struct UUART_CTL_SPEC;
impl crate::RegisterSpec for UUART_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_ctl::R](R) reader structure"]
impl crate::Readable for UUART_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_ctl::W](W) writer structure"]
impl crate::Writable for UUART_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_CTL to value 0"]
impl crate::Resettable for UUART_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
