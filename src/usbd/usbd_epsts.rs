#[doc = "Register `USBD_EPSTS` reader"]
pub struct R(crate::R<USBD_EPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_EPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBD_EPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBD_EPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Overrun\nIt indicates that the received data is over the maximum payload number or not.\nif received data is over the maximum payload number, the extra data will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OV_A {
    #[doc = "0: No overrun"]
    _0 = 0,
    #[doc = "1: Out Data is more than the Max Payload in MXPLD register or the Setup Data is more than 8 Bytes"]
    _1 = 1,
}
impl From<OV_A> for bool {
    #[inline(always)]
    fn from(variant: OV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV` reader - Overrun\nIt indicates that the received data is over the maximum payload number or not.\nif received data is over the maximum payload number, the extra data will be ignored."]
pub struct OV_R(crate::FieldReader<bool, OV_A>);
impl OV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OV_A {
        match self.bits {
            false => OV_A::_0,
            true => OV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OV_A::_1
    }
}
impl core::ops::Deref for OV_R {
    type Target = crate::FieldReader<bool, OV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Overrun It indicates that the received data is over the maximum payload number or not. if received data is over the maximum payload number, the extra data will be ignored."]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "USB Device Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_epsts](index.html) module"]
pub struct USBD_EPSTS_SPEC;
impl crate::RegisterSpec for USBD_EPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_epsts::R](R) reader structure"]
impl crate::Readable for USBD_EPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBD_EPSTS to value 0"]
impl crate::Resettable for USBD_EPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
