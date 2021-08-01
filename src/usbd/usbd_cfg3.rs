#[doc = "Register `USBD_CFG3` reader"]
pub struct R(crate::R<USBD_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_CFG3` writer"]
pub struct W(crate::W<USBD_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_CFG3_SPEC>;
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
impl From<crate::W<USBD_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBD_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNUM` reader - Endpoint Number\nThese bits are used to define the endpoint number of the current endpoint"]
pub struct EPNUM_R(crate::FieldReader<u8, u8>);
impl EPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNUM` writer - Endpoint Number\nThese bits are used to define the endpoint number of the current endpoint"]
pub struct EPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Isochronous Endpoint\nThis bit is used to set the endpoint as Isochronous endpoint, no handshake.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOCH_A {
    #[doc = "0: No Isochronous endpoint"]
    _0 = 0,
    #[doc = "1: Isochronous endpoint"]
    _1 = 1,
}
impl From<ISOCH_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOCH` reader - Isochronous Endpoint\nThis bit is used to set the endpoint as Isochronous endpoint, no handshake."]
pub struct ISOCH_R(crate::FieldReader<bool, ISOCH_A>);
impl ISOCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCH_A {
        match self.bits {
            false => ISOCH_A::_0,
            true => ISOCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISOCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISOCH_A::_1
    }
}
impl core::ops::Deref for ISOCH_R {
    type Target = crate::FieldReader<bool, ISOCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOCH` writer - Isochronous Endpoint\nThis bit is used to set the endpoint as Isochronous endpoint, no handshake."]
pub struct ISOCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Isochronous endpoint"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISOCH_A::_0)
    }
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISOCH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Endpoint STATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: Endpoint is Disabled"]
    _0 = 0,
    #[doc = "1: Out endpoint"]
    _1 = 1,
    #[doc = "2: IN endpoint"]
    _2 = 2,
    #[doc = "3: Undefined"]
    _3 = 3,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Endpoint STATE"]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            0 => STATE_A::_0,
            1 => STATE_A::_1,
            2 => STATE_A::_2,
            3 => STATE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STATE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == STATE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == STATE_A::_3
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` writer - Endpoint STATE"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Endpoint is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STATE_A::_0)
    }
    #[doc = "Out endpoint"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STATE_A::_1)
    }
    #[doc = "IN endpoint"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(STATE_A::_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(STATE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Data Sequence Synchronization\nIN Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following IN token transaction. Hardware will toggle automatically in IN token base on it.\nOUT Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following OUT token transaction. Hardware will toggle automatically OUT token base on this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSQSYNC_A {
    #[doc = "0: DATA0 PID"]
    _0 = 0,
    #[doc = "1: DATA1 PID"]
    _1 = 1,
}
impl From<DSQSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: DSQSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSQSYNC` reader - Data Sequence Synchronization\nIN Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following IN token transaction. Hardware will toggle automatically in IN token base on it.\nOUT Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following OUT token transaction. Hardware will toggle automatically OUT token base on this bit."]
pub struct DSQSYNC_R(crate::FieldReader<bool, DSQSYNC_A>);
impl DSQSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSQSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSQSYNC_A {
        match self.bits {
            false => DSQSYNC_A::_0,
            true => DSQSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DSQSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DSQSYNC_A::_1
    }
}
impl core::ops::Deref for DSQSYNC_R {
    type Target = crate::FieldReader<bool, DSQSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSQSYNC` writer - Data Sequence Synchronization\nIN Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following IN token transaction. Hardware will toggle automatically in IN token base on it.\nOUT Token Transaction:\nThis bit is used to specify the DATA0 or DATA1 PID in the following OUT token transaction. Hardware will toggle automatically OUT token base on this bit."]
pub struct DSQSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSQSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSQSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DATA0 PID"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSQSYNC_A::_0)
    }
    #[doc = "DATA1 PID"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSQSYNC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Clear STALL Response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTALL_A {
    #[doc = "0: Disable the device to clear the STALL handshake in setup stage"]
    _0 = 0,
    #[doc = "1: Clear the device to response STALL handshake in setup stage"]
    _1 = 1,
}
impl From<CSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: CSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTALL` reader - Clear STALL Response"]
pub struct CSTALL_R(crate::FieldReader<bool, CSTALL_A>);
impl CSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTALL_A {
        match self.bits {
            false => CSTALL_A::_0,
            true => CSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSTALL_A::_1
    }
}
impl core::ops::Deref for CSTALL_R {
    type Target = crate::FieldReader<bool, CSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTALL` writer - Clear STALL Response"]
pub struct CSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the device to clear the STALL handshake in setup stage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTALL_A::_0)
    }
    #[doc = "Clear the device to response STALL handshake in setup stage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTALL_A::_1)
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
impl R {
    #[doc = "Bits 0:3 - Endpoint Number These bits are used to define the endpoint number of the current endpoint"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Isochronous Endpoint This bit is used to set the endpoint as Isochronous endpoint, no handshake."]
    #[inline(always)]
    pub fn isoch(&self) -> ISOCH_R {
        ISOCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Endpoint STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data Sequence Synchronization IN Token Transaction: This bit is used to specify the DATA0 or DATA1 PID in the following IN token transaction. Hardware will toggle automatically in IN token base on it. OUT Token Transaction: This bit is used to specify the DATA0 or DATA1 PID in the following OUT token transaction. Hardware will toggle automatically OUT token base on this bit."]
    #[inline(always)]
    pub fn dsqsync(&self) -> DSQSYNC_R {
        DSQSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear STALL Response"]
    #[inline(always)]
    pub fn cstall(&self) -> CSTALL_R {
        CSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Number These bits are used to define the endpoint number of the current endpoint"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W { w: self }
    }
    #[doc = "Bit 4 - Isochronous Endpoint This bit is used to set the endpoint as Isochronous endpoint, no handshake."]
    #[inline(always)]
    pub fn isoch(&mut self) -> ISOCH_W {
        ISOCH_W { w: self }
    }
    #[doc = "Bits 5:6 - Endpoint STATE"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 7 - Data Sequence Synchronization IN Token Transaction: This bit is used to specify the DATA0 or DATA1 PID in the following IN token transaction. Hardware will toggle automatically in IN token base on it. OUT Token Transaction: This bit is used to specify the DATA0 or DATA1 PID in the following OUT token transaction. Hardware will toggle automatically OUT token base on this bit."]
    #[inline(always)]
    pub fn dsqsync(&mut self) -> DSQSYNC_W {
        DSQSYNC_W { w: self }
    }
    #[doc = "Bit 9 - Clear STALL Response"]
    #[inline(always)]
    pub fn cstall(&mut self) -> CSTALL_W {
        CSTALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint 3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_cfg3](index.html) module"]
pub struct USBD_CFG3_SPEC;
impl crate::RegisterSpec for USBD_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_cfg3::R](R) reader structure"]
impl crate::Readable for USBD_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_cfg3::W](W) writer structure"]
impl crate::Writable for USBD_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_CFG3 to value 0"]
impl crate::Resettable for USBD_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
