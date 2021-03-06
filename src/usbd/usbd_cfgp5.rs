#[doc = "Register `USBD_CFGP5` reader"]
pub struct R(crate::R<USBD_CFGP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_CFGP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_CFGP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_CFGP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_CFGP5` writer"]
pub struct W(crate::W<USBD_CFGP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_CFGP5_SPEC>;
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
impl From<crate::W<USBD_CFGP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBD_CFGP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRRDY` reader - Clear Ready\nWhen the USBD_MXPLDx register is set by user, it means that the endpoint is ready to transmit or receive data. If the user wants to disable this transaction before the transaction start, users can set this bit to 1 to disable it and it is auto clear to 0.\nFor IN token, write '1' to clear the IN token had ready to transmit the data to USB.\nFor OUT token, write '1' to clear the OUT token had ready to receive the data from USB.\nThis bit is write 1 only and is always 0 when it is read back."]
pub struct CLRRDY_R(crate::FieldReader<bool, bool>);
impl CLRRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRRDY` writer - Clear Ready\nWhen the USBD_MXPLDx register is set by user, it means that the endpoint is ready to transmit or receive data. If the user wants to disable this transaction before the transaction start, users can set this bit to 1 to disable it and it is auto clear to 0.\nFor IN token, write '1' to clear the IN token had ready to transmit the data to USB.\nFor OUT token, write '1' to clear the OUT token had ready to receive the data from USB.\nThis bit is write 1 only and is always 0 when it is read back."]
pub struct CLRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRRDY_W<'a> {
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
#[doc = "Set STALL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTALL_A {
    #[doc = "0: Disable the device to response STALL"]
    _0 = 0,
    #[doc = "1: Set the device to respond STALL automatically"]
    _1 = 1,
}
impl From<SSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: SSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTALL` reader - Set STALL"]
pub struct SSTALL_R(crate::FieldReader<bool, SSTALL_A>);
impl SSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTALL_A {
        match self.bits {
            false => SSTALL_A::_0,
            true => SSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSTALL_A::_1
    }
}
impl core::ops::Deref for SSTALL_R {
    type Target = crate::FieldReader<bool, SSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTALL` writer - Set STALL"]
pub struct SSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the device to response STALL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTALL_A::_0)
    }
    #[doc = "Set the device to respond STALL automatically"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTALL_A::_1)
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
impl R {
    #[doc = "Bit 0 - Clear Ready When the USBD_MXPLDx register is set by user, it means that the endpoint is ready to transmit or receive data. If the user wants to disable this transaction before the transaction start, users can set this bit to 1 to disable it and it is auto clear to 0. For IN token, write '1' to clear the IN token had ready to transmit the data to USB. For OUT token, write '1' to clear the OUT token had ready to receive the data from USB. This bit is write 1 only and is always 0 when it is read back."]
    #[inline(always)]
    pub fn clrrdy(&self) -> CLRRDY_R {
        CLRRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set STALL"]
    #[inline(always)]
    pub fn sstall(&self) -> SSTALL_R {
        SSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Ready When the USBD_MXPLDx register is set by user, it means that the endpoint is ready to transmit or receive data. If the user wants to disable this transaction before the transaction start, users can set this bit to 1 to disable it and it is auto clear to 0. For IN token, write '1' to clear the IN token had ready to transmit the data to USB. For OUT token, write '1' to clear the OUT token had ready to receive the data from USB. This bit is write 1 only and is always 0 when it is read back."]
    #[inline(always)]
    pub fn clrrdy(&mut self) -> CLRRDY_W {
        CLRRDY_W { w: self }
    }
    #[doc = "Bit 1 - Set STALL"]
    #[inline(always)]
    pub fn sstall(&mut self) -> SSTALL_W {
        SSTALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint 5 Set Stall and Clear In/Out Ready Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_cfgp5](index.html) module"]
pub struct USBD_CFGP5_SPEC;
impl crate::RegisterSpec for USBD_CFGP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_cfgp5::R](R) reader structure"]
impl crate::Readable for USBD_CFGP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_cfgp5::W](W) writer structure"]
impl crate::Writable for USBD_CFGP5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_CFGP5 to value 0"]
impl crate::Resettable for USBD_CFGP5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
