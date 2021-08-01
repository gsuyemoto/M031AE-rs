#[doc = "Register `CLK_CLKSEL3` reader"]
pub struct R(crate::R<CLK_CLKSEL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CLKSEL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CLKSEL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL3` writer"]
pub struct W(crate::W<CLK_CLKSEL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL3_SPEC>;
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
impl From<crate::W<CLK_CLKSEL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CLKSEL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UART6 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART6SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK0"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART6SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART6SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART6SEL` reader - UART6 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART6SEL_R(crate::FieldReader<u8, UART6SEL_A>);
impl UART6SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART6SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART6SEL_A> {
        match self.bits {
            0 => Some(UART6SEL_A::_0),
            1 => Some(UART6SEL_A::_1),
            2 => Some(UART6SEL_A::_2),
            3 => Some(UART6SEL_A::_3),
            4 => Some(UART6SEL_A::_4),
            5 => Some(UART6SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART6SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART6SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART6SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART6SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART6SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART6SEL_A::_5
    }
}
impl core::ops::Deref for UART6SEL_R {
    type Target = crate::FieldReader<u8, UART6SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART6SEL` writer - UART6 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART6SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART6SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART6SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART6SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART6SEL_A::_3)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART6SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART6SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "UART7 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART7SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK1"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART7SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART7SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART7SEL` reader - UART7 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART7SEL_R(crate::FieldReader<u8, UART7SEL_A>);
impl UART7SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART7SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART7SEL_A> {
        match self.bits {
            0 => Some(UART7SEL_A::_0),
            1 => Some(UART7SEL_A::_1),
            2 => Some(UART7SEL_A::_2),
            3 => Some(UART7SEL_A::_3),
            4 => Some(UART7SEL_A::_4),
            5 => Some(UART7SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART7SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART7SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART7SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART7SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART7SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART7SEL_A::_5
    }
}
impl core::ops::Deref for UART7SEL_R {
    type Target = crate::FieldReader<u8, UART7SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7SEL` writer - UART7 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART7SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART7SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART7SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART7SEL_A::_3)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART7SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART7SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "UART4 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART4SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK0"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART4SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART4SEL` reader - UART4 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART4SEL_R(crate::FieldReader<u8, UART4SEL_A>);
impl UART4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART4SEL_A> {
        match self.bits {
            0 => Some(UART4SEL_A::_0),
            1 => Some(UART4SEL_A::_1),
            2 => Some(UART4SEL_A::_2),
            3 => Some(UART4SEL_A::_3),
            4 => Some(UART4SEL_A::_4),
            5 => Some(UART4SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART4SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART4SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART4SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART4SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART4SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART4SEL_A::_5
    }
}
impl core::ops::Deref for UART4SEL_R {
    type Target = crate::FieldReader<u8, UART4SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4SEL` writer - UART4 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART4SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART4SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART4SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART4SEL_A::_3)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART4SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART4SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "UART5 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART5SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK1"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART5SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART5SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART5SEL` reader - UART5 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART5SEL_R(crate::FieldReader<u8, UART5SEL_A>);
impl UART5SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART5SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART5SEL_A> {
        match self.bits {
            0 => Some(UART5SEL_A::_0),
            1 => Some(UART5SEL_A::_1),
            2 => Some(UART5SEL_A::_2),
            3 => Some(UART5SEL_A::_3),
            4 => Some(UART5SEL_A::_4),
            5 => Some(UART5SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART5SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART5SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART5SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART5SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART5SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART5SEL_A::_5
    }
}
impl core::ops::Deref for UART5SEL_R {
    type Target = crate::FieldReader<u8, UART5SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5SEL` writer - UART5 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART5SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART5SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART5SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART5SEL_A::_3)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART5SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART5SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "UART2 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART2SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK0"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART2SEL` reader - UART2 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART2SEL_R(crate::FieldReader<u8, UART2SEL_A>);
impl UART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART2SEL_A> {
        match self.bits {
            0 => Some(UART2SEL_A::_0),
            1 => Some(UART2SEL_A::_1),
            2 => Some(UART2SEL_A::_2),
            3 => Some(UART2SEL_A::_3),
            4 => Some(UART2SEL_A::_4),
            5 => Some(UART2SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART2SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART2SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART2SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART2SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART2SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART2SEL_A::_5
    }
}
impl core::ops::Deref for UART2SEL_R {
    type Target = crate::FieldReader<u8, UART2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2SEL` writer - UART2 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK0.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART2SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART2SEL_A::_3)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART2SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART2SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "UART3 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART3SEL_A {
    #[doc = "0: Clock source from external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from internal high speed RC oscillator (HIRC)"]
    _3 = 3,
    #[doc = "4: Clock source from PCLK1"]
    _4 = 4,
    #[doc = "5: Clock source from internal low speed RC oscillator (LIRC)"]
    _5 = 5,
}
impl From<UART3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART3SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART3SEL` reader - UART3 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART3SEL_R(crate::FieldReader<u8, UART3SEL_A>);
impl UART3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART3SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART3SEL_A> {
        match self.bits {
            0 => Some(UART3SEL_A::_0),
            1 => Some(UART3SEL_A::_1),
            2 => Some(UART3SEL_A::_2),
            3 => Some(UART3SEL_A::_3),
            4 => Some(UART3SEL_A::_4),
            5 => Some(UART3SEL_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART3SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART3SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UART3SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UART3SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == UART3SEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == UART3SEL_A::_5
    }
}
impl core::ops::Deref for UART3SEL_R {
    type Target = crate::FieldReader<u8, UART3SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3SEL` writer - UART3 Clock Source Selection\nNote: If PLL is not supported, clock source of selection '001' will be changed to PCLK1.\nNote: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct UART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3SEL_A::_1)
    }
    #[doc = "Clock source from external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UART3SEL_A::_2)
    }
    #[doc = "Clock source from internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UART3SEL_A::_3)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(UART3SEL_A::_4)
    }
    #[doc = "Clock source from internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(UART3SEL_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - UART6 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart6sel(&self) -> UART6SEL_R {
        UART6SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - UART7 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - UART4 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - UART5 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - UART2 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart2sel(&self) -> UART2SEL_R {
        UART2SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - UART3 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart3sel(&self) -> UART3SEL_R {
        UART3SEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - UART6 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart6sel(&mut self) -> UART6SEL_W {
        UART6SEL_W { w: self }
    }
    #[doc = "Bits 12:14 - UART7 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart7sel(&mut self) -> UART7SEL_W {
        UART7SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - UART4 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W {
        UART4SEL_W { w: self }
    }
    #[doc = "Bits 20:22 - UART5 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W {
        UART5SEL_W { w: self }
    }
    #[doc = "Bits 24:26 - UART2 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK0. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart2sel(&mut self) -> UART2SEL_W {
        UART2SEL_W { w: self }
    }
    #[doc = "Bits 28:30 - UART3 Clock Source Selection Note: If PLL is not supported, clock source of selection '001' will be changed to PCLK1. Note: If LXT or HXT is not supported, clock source of selection '000' or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn uart3sel(&mut self) -> UART3SEL_W {
        UART3SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel3](index.html) module"]
pub struct CLK_CLKSEL3_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel3::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel3::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL3 to value 0x4444_4400"]
impl crate::Resettable for CLK_CLKSEL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4400
    }
}
