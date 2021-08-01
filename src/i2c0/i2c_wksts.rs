#[doc = "Register `I2C_WKSTS` reader"]
pub struct R(crate::R<I2C_WKSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_WKSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_WKSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_WKSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_WKSTS` writer"]
pub struct W(crate::W<I2C_WKSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_WKSTS_SPEC>;
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
impl From<crate::W<I2C_WKSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_WKSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKIF` reader - I2C Wake-up Flag\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKIF_R(crate::FieldReader<bool, bool>);
impl WKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKIF` writer - I2C Wake-up Flag\nWhen chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKIF_W<'a> {
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
#[doc = "Wakeup Address Frame Acknowledge Bit Done\nNote: This bit can't release WKIF. Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKAKDONE_A {
    #[doc = "0: The ACK bit cycle of address match frame isn't done"]
    _0 = 0,
    #[doc = "1: The ACK bit cycle of address match frame is done in power-down"]
    _1 = 1,
}
impl From<WKAKDONE_A> for bool {
    #[inline(always)]
    fn from(variant: WKAKDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKAKDONE` reader - Wakeup Address Frame Acknowledge Bit Done\nNote: This bit can't release WKIF. Software can write 1 to clear this bit."]
pub struct WKAKDONE_R(crate::FieldReader<bool, WKAKDONE_A>);
impl WKAKDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKAKDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKAKDONE_A {
        match self.bits {
            false => WKAKDONE_A::_0,
            true => WKAKDONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKAKDONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKAKDONE_A::_1
    }
}
impl core::ops::Deref for WKAKDONE_R {
    type Target = crate::FieldReader<bool, WKAKDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKAKDONE` writer - Wakeup Address Frame Acknowledge Bit Done\nNote: This bit can't release WKIF. Software can write 1 to clear this bit."]
pub struct WKAKDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKAKDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKAKDONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The ACK bit cycle of address match frame isn't done"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKAKDONE_A::_0)
    }
    #[doc = "The ACK bit cycle of address match frame is done in power-down"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKAKDONE_A::_1)
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
#[doc = "Read/Write Status Bit in Address Wakeup Frame (Read Only)\nNote: This bit will be cleared when software can write 1 to WKAKDONE (I2C_WKSTS\\[1\\]) bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRSTSWK_A {
    #[doc = "0: Write command be record on the address match wakeup frame"]
    _0 = 0,
    #[doc = "1: Read command be record on the address match wakeup frame"]
    _1 = 1,
}
impl From<WRSTSWK_A> for bool {
    #[inline(always)]
    fn from(variant: WRSTSWK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRSTSWK` reader - Read/Write Status Bit in Address Wakeup Frame (Read Only)\nNote: This bit will be cleared when software can write 1 to WKAKDONE (I2C_WKSTS\\[1\\]) bit."]
pub struct WRSTSWK_R(crate::FieldReader<bool, WRSTSWK_A>);
impl WRSTSWK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRSTSWK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRSTSWK_A {
        match self.bits {
            false => WRSTSWK_A::_0,
            true => WRSTSWK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WRSTSWK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WRSTSWK_A::_1
    }
}
impl core::ops::Deref for WRSTSWK_R {
    type Target = crate::FieldReader<bool, WRSTSWK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - I2C Wake-up Flag When chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkif(&self) -> WKIF_R {
        WKIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Address Frame Acknowledge Bit Done Note: This bit can't release WKIF. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkakdone(&self) -> WKAKDONE_R {
        WKAKDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read/Write Status Bit in Address Wakeup Frame (Read Only) Note: This bit will be cleared when software can write 1 to WKAKDONE (I2C_WKSTS\\[1\\]) bit."]
    #[inline(always)]
    pub fn wrstswk(&self) -> WRSTSWK_R {
        WRSTSWK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Wake-up Flag When chip is woken up from Power-down mode by I2C, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkif(&mut self) -> WKIF_W {
        WKIF_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Address Frame Acknowledge Bit Done Note: This bit can't release WKIF. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkakdone(&mut self) -> WKAKDONE_W {
        WKAKDONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Wake-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_wksts](index.html) module"]
pub struct I2C_WKSTS_SPEC;
impl crate::RegisterSpec for I2C_WKSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_wksts::R](R) reader structure"]
impl crate::Readable for I2C_WKSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_wksts::W](W) writer structure"]
impl crate::Writable for I2C_WKSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_WKSTS to value 0"]
impl crate::Resettable for I2C_WKSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
