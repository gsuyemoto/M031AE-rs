#[doc = "Register `UUART_PROTIEN` reader"]
pub struct R(crate::R<UUART_PROTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_PROTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_PROTIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_PROTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_PROTIEN` writer"]
pub struct W(crate::W<UUART_PROTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_PROTIEN_SPEC>;
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
impl From<crate::W<UUART_PROTIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_PROTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Auto-baud Rate Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRIEN_A {
    #[doc = "0: Auto-baud rate interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Auto-baud rate interrupt Enabled"]
    _1 = 1,
}
impl From<ABRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRIEN` reader - Auto-baud Rate Interrupt Enable Bit"]
pub struct ABRIEN_R(crate::FieldReader<bool, ABRIEN_A>);
impl ABRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRIEN_A {
        match self.bits {
            false => ABRIEN_A::_0,
            true => ABRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRIEN_A::_1
    }
}
impl core::ops::Deref for ABRIEN_R {
    type Target = crate::FieldReader<bool, ABRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRIEN` writer - Auto-baud Rate Interrupt Enable Bit"]
pub struct ABRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRIEN_A::_0)
    }
    #[doc = "Auto-baud rate interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRIEN_A::_1)
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
#[doc = "Receive Line Status Interrupt Enable Bit\nNote: UUART_PROTSTS\\[7:5\\]
indicates the current interrupt event for receive line status interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSIEN_A {
    #[doc = "0: Receive line status interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive line status interrupt Enabled"]
    _1 = 1,
}
impl From<RLSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLSIEN` reader - Receive Line Status Interrupt Enable Bit\nNote: UUART_PROTSTS\\[7:5\\]
indicates the current interrupt event for receive line status interrupt."]
pub struct RLSIEN_R(crate::FieldReader<bool, RLSIEN_A>);
impl RLSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSIEN_A {
        match self.bits {
            false => RLSIEN_A::_0,
            true => RLSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RLSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RLSIEN_A::_1
    }
}
impl core::ops::Deref for RLSIEN_R {
    type Target = crate::FieldReader<bool, RLSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLSIEN` writer - Receive Line Status Interrupt Enable Bit\nNote: UUART_PROTSTS\\[7:5\\]
indicates the current interrupt event for receive line status interrupt."]
pub struct RLSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive line status interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLSIEN_A::_0)
    }
    #[doc = "Receive line status interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLSIEN_A::_1)
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
impl R {
    #[doc = "Bit 1 - Auto-baud Rate Interrupt Enable Bit"]
    #[inline(always)]
    pub fn abrien(&self) -> ABRIEN_R {
        ABRIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable Bit Note: UUART_PROTSTS\\[7:5\\]
indicates the current interrupt event for receive line status interrupt."]
    #[inline(always)]
    pub fn rlsien(&self) -> RLSIEN_R {
        RLSIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Auto-baud Rate Interrupt Enable Bit"]
    #[inline(always)]
    pub fn abrien(&mut self) -> ABRIEN_W {
        ABRIEN_W { w: self }
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable Bit Note: UUART_PROTSTS\\[7:5\\]
indicates the current interrupt event for receive line status interrupt."]
    #[inline(always)]
    pub fn rlsien(&mut self) -> RLSIEN_W {
        RLSIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_protien](index.html) module"]
pub struct UUART_PROTIEN_SPEC;
impl crate::RegisterSpec for UUART_PROTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_protien::R](R) reader structure"]
impl crate::Readable for UUART_PROTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_protien::W](W) writer structure"]
impl crate::Writable for UUART_PROTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_PROTIEN to value 0"]
impl crate::Resettable for UUART_PROTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
