#[doc = "Register `CLK_CLKSEL0` reader"]
pub struct R(crate::R<CLK_CLKSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CLKSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CLKSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL0` writer"]
pub struct W(crate::W<CLK_CLKSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL0_SPEC>;
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
impl From<crate::W<CLK_CLKSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CLKSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HCLK Clock Source Selection (Write Protect)\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by power on reset\nNote: If PLL is not supported, clock source of selection '010' will be changed to HIRC. \nNote: If LXT or HXT is not supported, clock source of selection '000'or '001' will be kept previous clock selection.\nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCLKSEL_A {
    #[doc = "0: Clock source from HXT"]
    _0 = 0,
    #[doc = "1: Clock source from LXT"]
    _1 = 1,
    #[doc = "2: Clock source from PLL"]
    _2 = 2,
    #[doc = "3: Clock source from LIRC"]
    _3 = 3,
    #[doc = "7: Clock source from HIRC"]
    _7 = 7,
}
impl From<HCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HCLKSEL` reader - HCLK Clock Source Selection (Write Protect)\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by power on reset\nNote: If PLL is not supported, clock source of selection '010' will be changed to HIRC. \nNote: If LXT or HXT is not supported, clock source of selection '000'or '001' will be kept previous clock selection.\nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct HCLKSEL_R(crate::FieldReader<u8, HCLKSEL_A>);
impl HCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HCLKSEL_A> {
        match self.bits {
            0 => Some(HCLKSEL_A::_0),
            1 => Some(HCLKSEL_A::_1),
            2 => Some(HCLKSEL_A::_2),
            3 => Some(HCLKSEL_A::_3),
            7 => Some(HCLKSEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HCLKSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == HCLKSEL_A::_7
    }
}
impl core::ops::Deref for HCLKSEL_R {
    type Target = crate::FieldReader<u8, HCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLKSEL` writer - HCLK Clock Source Selection (Write Protect)\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by power on reset\nNote: If PLL is not supported, clock source of selection '010' will be changed to HIRC. \nNote: If LXT or HXT is not supported, clock source of selection '000'or '001' will be kept previous clock selection.\nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct HCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from HXT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_0)
    }
    #[doc = "Clock source from LXT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_1)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_2)
    }
    #[doc = "Clock source from LIRC"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_3)
    }
    #[doc = "Clock source from HIRC"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Cortex-M0 SysTick Clock Source Selection (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: If LXT or HXT is not supported, clock source of selection '000', '001', or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STCLKSEL_A {
    #[doc = "0: Clock source from HXT"]
    _0 = 0,
    #[doc = "1: Clock source from LXT"]
    _1 = 1,
    #[doc = "2: Clock source from HXT/2"]
    _2 = 2,
    #[doc = "3: Clock source from HCLK/2"]
    _3 = 3,
    #[doc = "7: Clock source from HIRC/2"]
    _7 = 7,
}
impl From<STCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STCLKSEL` reader - Cortex-M0 SysTick Clock Source Selection (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: If LXT or HXT is not supported, clock source of selection '000', '001', or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct STCLKSEL_R(crate::FieldReader<u8, STCLKSEL_A>);
impl STCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        STCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STCLKSEL_A> {
        match self.bits {
            0 => Some(STCLKSEL_A::_0),
            1 => Some(STCLKSEL_A::_1),
            2 => Some(STCLKSEL_A::_2),
            3 => Some(STCLKSEL_A::_3),
            7 => Some(STCLKSEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == STCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == STCLKSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == STCLKSEL_A::_7
    }
}
impl core::ops::Deref for STCLKSEL_R {
    type Target = crate::FieldReader<u8, STCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STCLKSEL` writer - Cortex-M0 SysTick Clock Source Selection (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: If LXT or HXT is not supported, clock source of selection '000', '001', or '010' will be stopped. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct STCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from HXT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_0)
    }
    #[doc = "Clock source from LXT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_1)
    }
    #[doc = "Clock source from HXT/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_2)
    }
    #[doc = "Clock source from HCLK/2"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_3)
    }
    #[doc = "Clock source from HIRC/2"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "USB Device Clock Source Selection (Write Protect)\nThese bits are protected bit. It means programming this bit needs to write '59h', '16h', '88h' to address 0x4000_0100 to disable register protection. Refer to the register REGWRPROT at address GCR_BA+0x100.\nNote: If PLL is not supported, clock source of selection '1' will be changed to HIRC. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDSEL_A {
    #[doc = "0: Clock source from HIRC"]
    _0 = 0,
    #[doc = "1: Clock source from PLL divided"]
    _1 = 1,
}
impl From<USBDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDSEL` reader - USB Device Clock Source Selection (Write Protect)\nThese bits are protected bit. It means programming this bit needs to write '59h', '16h', '88h' to address 0x4000_0100 to disable register protection. Refer to the register REGWRPROT at address GCR_BA+0x100.\nNote: If PLL is not supported, clock source of selection '1' will be changed to HIRC. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct USBDSEL_R(crate::FieldReader<bool, USBDSEL_A>);
impl USBDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDSEL_A {
        match self.bits {
            false => USBDSEL_A::_0,
            true => USBDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBDSEL_A::_1
    }
}
impl core::ops::Deref for USBDSEL_R {
    type Target = crate::FieldReader<bool, USBDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDSEL` writer - USB Device Clock Source Selection (Write Protect)\nThese bits are protected bit. It means programming this bit needs to write '59h', '16h', '88h' to address 0x4000_0100 to disable register protection. Refer to the register REGWRPROT at address GCR_BA+0x100.\nNote: If PLL is not supported, clock source of selection '1' will be changed to HIRC. \nPlease refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
pub struct USBDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from HIRC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDSEL_A::_0)
    }
    #[doc = "Clock source from PLL divided"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HCLK Clock Source Selection (Write Protect) Before clock switching, the related clock sources (both pre-select and new-select) must be turned on. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note : reset by power on reset Note: If PLL is not supported, clock source of selection '010' will be changed to HIRC. Note: If LXT or HXT is not supported, clock source of selection '000'or '001' will be kept previous clock selection. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn hclksel(&self) -> HCLKSEL_R {
        HCLKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Cortex-M0 SysTick Clock Source Selection (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: If LXT or HXT is not supported, clock source of selection '000', '001', or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn stclksel(&self) -> STCLKSEL_R {
        STCLKSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 8 - USB Device Clock Source Selection (Write Protect) These bits are protected bit. It means programming this bit needs to write '59h', '16h', '88h' to address 0x4000_0100 to disable register protection. Refer to the register REGWRPROT at address GCR_BA+0x100. Note: If PLL is not supported, clock source of selection '1' will be changed to HIRC. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn usbdsel(&self) -> USBDSEL_R {
        USBDSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - HCLK Clock Source Selection (Write Protect) Before clock switching, the related clock sources (both pre-select and new-select) must be turned on. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note : reset by power on reset Note: If PLL is not supported, clock source of selection '010' will be changed to HIRC. Note: If LXT or HXT is not supported, clock source of selection '000'or '001' will be kept previous clock selection. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn hclksel(&mut self) -> HCLKSEL_W {
        HCLKSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Cortex-M0 SysTick Clock Source Selection (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: If LXT or HXT is not supported, clock source of selection '000', '001', or '010' will be stopped. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn stclksel(&mut self) -> STCLKSEL_W {
        STCLKSEL_W { w: self }
    }
    #[doc = "Bit 8 - USB Device Clock Source Selection (Write Protect) These bits are protected bit. It means programming this bit needs to write '59h', '16h', '88h' to address 0x4000_0100 to disable register protection. Refer to the register REGWRPROT at address GCR_BA+0x100. Note: If PLL is not supported, clock source of selection '1' will be changed to HIRC. Please refer to section 3.2 NuMicro M031/M032 Series Selection Guide for detailed information."]
    #[inline(always)]
    pub fn usbdsel(&mut self) -> USBDSEL_W {
        USBDSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel0](index.html) module"]
pub struct CLK_CLKSEL0_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel0::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel0::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL0 to value 0x3f"]
impl crate::Resettable for CLK_CLKSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
