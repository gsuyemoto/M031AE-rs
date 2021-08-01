#[doc = "Register `UI2C_ADDRMSK1` reader"]
pub struct R(crate::R<UI2C_ADDRMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_ADDRMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_ADDRMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_ADDRMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_ADDRMSK1` writer"]
pub struct W(crate::W<UI2C_ADDRMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_ADDRMSK1_SPEC>;
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
impl From<crate::W<UI2C_ADDRMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_ADDRMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USCI Device Address Mask\nUSCI support multiple address recognition with two address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADDRMSK_A {
    #[doc = "0: Mask Disabled (the received corresponding register bit should be exact the same as address register.)"]
    _0 = 0,
    #[doc = "1: Mask Enabled (the received corresponding address bit is don't care.)"]
    _1 = 1,
}
impl From<ADDRMSK_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDRMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDRMSK` reader - USCI Device Address Mask\nUSCI support multiple address recognition with two address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
pub struct ADDRMSK_R(crate::FieldReader<u16, ADDRMSK_A>);
impl ADDRMSK_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDRMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDRMSK_A> {
        match self.bits {
            0 => Some(ADDRMSK_A::_0),
            1 => Some(ADDRMSK_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDRMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDRMSK_A::_1
    }
}
impl core::ops::Deref for ADDRMSK_R {
    type Target = crate::FieldReader<u16, ADDRMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMSK` writer - USCI Device Address Mask\nUSCI support multiple address recognition with two address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
pub struct ADDRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRMSK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mask Disabled (the received corresponding register bit should be exact the same as address register.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDRMSK_A::_0)
    }
    #[doc = "Mask Enabled (the received corresponding address bit is don't care.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDRMSK_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - USCI Device Address Mask USCI support multiple address recognition with two address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn addrmsk(&self) -> ADDRMSK_R {
        ADDRMSK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - USCI Device Address Mask USCI support multiple address recognition with two address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn addrmsk(&mut self) -> ADDRMSK_W {
        ADDRMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Device Address Mask Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_addrmsk1](index.html) module"]
pub struct UI2C_ADDRMSK1_SPEC;
impl crate::RegisterSpec for UI2C_ADDRMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_addrmsk1::R](R) reader structure"]
impl crate::Readable for UI2C_ADDRMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_addrmsk1::W](W) writer structure"]
impl crate::Writable for UI2C_ADDRMSK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_ADDRMSK1 to value 0"]
impl crate::Resettable for UI2C_ADDRMSK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
