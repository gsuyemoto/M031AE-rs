#[doc = "Register `UI2C_ADMAT` reader"]
pub struct R(crate::R<UI2C_ADMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_ADMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_ADMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_ADMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_ADMAT` writer"]
pub struct W(crate::W<UI2C_ADMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_ADMAT_SPEC>;
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
impl From<crate::W<UI2C_ADMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_ADMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADMAT0` reader - USCI Address 0 Match Status Register\nWhen address 0 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
pub struct ADMAT0_R(crate::FieldReader<bool, bool>);
impl ADMAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADMAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADMAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMAT0` writer - USCI Address 0 Match Status Register\nWhen address 0 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
pub struct ADMAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAT0_W<'a> {
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
#[doc = "Field `ADMAT1` reader - USCI Address 1 Match Status Register\nWhen address 1 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
pub struct ADMAT1_R(crate::FieldReader<bool, bool>);
impl ADMAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADMAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADMAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMAT1` writer - USCI Address 1 Match Status Register\nWhen address 1 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
pub struct ADMAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAT1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USCI Address 0 Match Status Register When address 0 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn admat0(&self) -> ADMAT0_R {
        ADMAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Address 1 Match Status Register When address 1 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn admat1(&self) -> ADMAT1_R {
        ADMAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Address 0 Match Status Register When address 0 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn admat0(&mut self) -> ADMAT0_W {
        ADMAT0_W { w: self }
    }
    #[doc = "Bit 1 - USCI Address 1 Match Status Register When address 1 is matched, hardware will inform which address used. This bit will set to 1, and software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn admat1(&mut self) -> ADMAT1_W {
        ADMAT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Match Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_admat](index.html) module"]
pub struct UI2C_ADMAT_SPEC;
impl crate::RegisterSpec for UI2C_ADMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_admat::R](R) reader structure"]
impl crate::Readable for UI2C_ADMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_admat::W](W) writer structure"]
impl crate::Writable for UI2C_ADMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_ADMAT to value 0"]
impl crate::Resettable for UI2C_ADMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
