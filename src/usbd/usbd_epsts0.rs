#[doc = "Register `USBD_EPSTS0` reader"]
pub struct R(crate::R<USBD_EPSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_EPSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_EPSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_EPSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Endpoint 5 Status\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS5_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS5_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS5` reader - Endpoint 5 Status\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS5_R(crate::FieldReader<u8, EPSTS5_A>);
impl EPSTS5_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS5_A> {
        match self.bits {
            0 => Some(EPSTS5_A::_0),
            1 => Some(EPSTS5_A::_1),
            2 => Some(EPSTS5_A::_2),
            6 => Some(EPSTS5_A::_6),
            7 => Some(EPSTS5_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS5_A::_2
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS5_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS5_A::_7
    }
}
impl core::ops::Deref for EPSTS5_R {
    type Target = crate::FieldReader<u8, EPSTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 6 Status\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS6_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS6_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS6` reader - Endpoint 6 Status\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS6_R(crate::FieldReader<u8, EPSTS6_A>);
impl EPSTS6_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS6_A> {
        match self.bits {
            0 => Some(EPSTS6_A::_0),
            1 => Some(EPSTS6_A::_1),
            2 => Some(EPSTS6_A::_2),
            6 => Some(EPSTS6_A::_6),
            7 => Some(EPSTS6_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS6_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS6_A::_2
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS6_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS6_A::_7
    }
}
impl core::ops::Deref for EPSTS6_R {
    type Target = crate::FieldReader<u8, EPSTS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 7 Status\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS7_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS7_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS7` reader - Endpoint 7 Status\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS7_R(crate::FieldReader<u8, EPSTS7_A>);
impl EPSTS7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS7_A> {
        match self.bits {
            0 => Some(EPSTS7_A::_0),
            1 => Some(EPSTS7_A::_1),
            2 => Some(EPSTS7_A::_2),
            6 => Some(EPSTS7_A::_6),
            7 => Some(EPSTS7_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS7_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS7_A::_2
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS7_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS7_A::_7
    }
}
impl core::ops::Deref for EPSTS7_R {
    type Target = crate::FieldReader<u8, EPSTS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 20:23 - Endpoint 5 Status These bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts5(&self) -> EPSTS5_R {
        EPSTS5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Endpoint 6 Status These bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts6(&self) -> EPSTS6_R {
        EPSTS6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Endpoint 7 Status These bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts7(&self) -> EPSTS7_R {
        EPSTS7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "USB Device Endpoint Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_epsts0](index.html) module"]
pub struct USBD_EPSTS0_SPEC;
impl crate::RegisterSpec for USBD_EPSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_epsts0::R](R) reader structure"]
impl crate::Readable for USBD_EPSTS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBD_EPSTS0 to value 0"]
impl crate::Resettable for USBD_EPSTS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
