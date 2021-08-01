#[doc = "Register `I2C_STATUS1` reader"]
pub struct R(crate::R<I2C_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_STATUS1` writer"]
pub struct W(crate::W<I2C_STATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_STATUS1_SPEC>;
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
impl From<crate::W<I2C_STATUS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_STATUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "On Bus Busy (Read Only)\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONBUSY_A {
    #[doc = "0: The bus is IDLE (both SCLK and SDA High)"]
    _0 = 0,
    #[doc = "1: The bus is busy"]
    _1 = 1,
}
impl From<ONBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ONBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONBUSY` reader - On Bus Busy (Read Only)\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected."]
pub struct ONBUSY_R(crate::FieldReader<bool, ONBUSY_A>);
impl ONBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONBUSY_A {
        match self.bits {
            false => ONBUSY_A::_0,
            true => ONBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ONBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ONBUSY_A::_1
    }
}
impl core::ops::Deref for ONBUSY_R {
    type Target = crate::FieldReader<bool, ONBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - On Bus Busy (Read Only) Indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected."]
    #[inline(always)]
    pub fn onbusy(&self) -> ONBUSY_R {
        ONBUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status1](index.html) module"]
pub struct I2C_STATUS1_SPEC;
impl crate::RegisterSpec for I2C_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_status1::R](R) reader structure"]
impl crate::Readable for I2C_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_status1::W](W) writer structure"]
impl crate::Writable for I2C_STATUS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_STATUS1 to value 0"]
impl crate::Resettable for I2C_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
