#[doc = "Register `UART_FUNCSEL` reader"]
pub struct R(crate::R<UART_FUNCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FUNCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FUNCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FUNCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_FUNCSEL` writer"]
pub struct W(crate::W<UART_FUNCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FUNCSEL_SPEC>;
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
impl From<crate::W<UART_FUNCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FUNCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNCSEL_A {
    #[doc = "0: UART function"]
    _0 = 0,
    #[doc = "2: IrDA function"]
    _2 = 2,
    #[doc = "3: RS-485 function"]
    _3 = 3,
    #[doc = "4: UART Single-wire function"]
    _4 = 4,
}
impl From<FUNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNCSEL` reader - Function Select"]
pub struct FUNCSEL_R(crate::FieldReader<u8, FUNCSEL_A>);
impl FUNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNCSEL_A> {
        match self.bits {
            0 => Some(FUNCSEL_A::_0),
            2 => Some(FUNCSEL_A::_2),
            3 => Some(FUNCSEL_A::_3),
            4 => Some(FUNCSEL_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FUNCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FUNCSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FUNCSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == FUNCSEL_A::_4
    }
}
impl core::ops::Deref for FUNCSEL_R {
    type Target = crate::FieldReader<u8, FUNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNCSEL` writer - Function Select"]
pub struct FUNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUNCSEL_A::_0)
    }
    #[doc = "IrDA function"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FUNCSEL_A::_2)
    }
    #[doc = "RS-485 function"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FUNCSEL_A::_3)
    }
    #[doc = "UART Single-wire function"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FUNCSEL_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "TX and RX Disable Bit\nSetting this bit can disable TX and RX.\nNote: The TX and RX will not disable immediately when this bit is set. The TX and RX compelet current task before disable TX and RX. When TX and RX disable, the TXRXACT (UART_FIFOSTS\\[31\\]) is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXDIS_A {
    #[doc = "0: TX and RX Enabled"]
    _0 = 0,
    #[doc = "1: TX and RX Disabled"]
    _1 = 1,
}
impl From<TXRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRXDIS` reader - TX and RX Disable Bit\nSetting this bit can disable TX and RX.\nNote: The TX and RX will not disable immediately when this bit is set. The TX and RX compelet current task before disable TX and RX. When TX and RX disable, the TXRXACT (UART_FIFOSTS\\[31\\]) is cleared."]
pub struct TXRXDIS_R(crate::FieldReader<bool, TXRXDIS_A>);
impl TXRXDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRXDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXDIS_A {
        match self.bits {
            false => TXRXDIS_A::_0,
            true => TXRXDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRXDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRXDIS_A::_1
    }
}
impl core::ops::Deref for TXRXDIS_R {
    type Target = crate::FieldReader<bool, TXRXDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRXDIS` writer - TX and RX Disable Bit\nSetting this bit can disable TX and RX.\nNote: The TX and RX will not disable immediately when this bit is set. The TX and RX compelet current task before disable TX and RX. When TX and RX disable, the TXRXACT (UART_FIFOSTS\\[31\\]) is cleared."]
pub struct TXRXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRXDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX and RX Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRXDIS_A::_0)
    }
    #[doc = "TX and RX Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRXDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select"]
    #[inline(always)]
    pub fn funcsel(&self) -> FUNCSEL_R {
        FUNCSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - TX and RX Disable Bit Setting this bit can disable TX and RX. Note: The TX and RX will not disable immediately when this bit is set. The TX and RX compelet current task before disable TX and RX. When TX and RX disable, the TXRXACT (UART_FIFOSTS\\[31\\]) is cleared."]
    #[inline(always)]
    pub fn txrxdis(&self) -> TXRXDIS_R {
        TXRXDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select"]
    #[inline(always)]
    pub fn funcsel(&mut self) -> FUNCSEL_W {
        FUNCSEL_W { w: self }
    }
    #[doc = "Bit 3 - TX and RX Disable Bit Setting this bit can disable TX and RX. Note: The TX and RX will not disable immediately when this bit is set. The TX and RX compelet current task before disable TX and RX. When TX and RX disable, the TXRXACT (UART_FIFOSTS\\[31\\]) is cleared."]
    #[inline(always)]
    pub fn txrxdis(&mut self) -> TXRXDIS_W {
        TXRXDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_funcsel](index.html) module"]
pub struct UART_FUNCSEL_SPEC;
impl crate::RegisterSpec for UART_FUNCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_funcsel::R](R) reader structure"]
impl crate::Readable for UART_FUNCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_funcsel::W](W) writer structure"]
impl crate::Writable for UART_FUNCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_FUNCSEL to value 0"]
impl crate::Resettable for UART_FUNCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
