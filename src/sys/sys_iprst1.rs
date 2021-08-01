#[doc = "Register `SYS_IPRST1` reader"]
pub struct R(crate::R<SYS_IPRST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IPRST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_IPRST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_IPRST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IPRST1` writer"]
pub struct W(crate::W<SYS_IPRST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IPRST1_SPEC>;
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
impl From<crate::W<SYS_IPRST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_IPRST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIORST_A {
    #[doc = "0: GPIO controller normal operation"]
    _0 = 0,
    #[doc = "1: GPIO controller reset"]
    _1 = 1,
}
impl From<GPIORST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIORST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIORST` reader - GPIO Controller Reset"]
pub struct GPIORST_R(crate::FieldReader<bool, GPIORST_A>);
impl GPIORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIORST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIORST_A {
        match self.bits {
            false => GPIORST_A::_0,
            true => GPIORST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GPIORST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GPIORST_A::_1
    }
}
impl core::ops::Deref for GPIORST_R {
    type Target = crate::FieldReader<bool, GPIORST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIORST` writer - GPIO Controller Reset"]
pub struct GPIORST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIORST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GPIO controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPIORST_A::_0)
    }
    #[doc = "GPIO controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPIORST_A::_1)
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
#[doc = "Timer0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0RST_A {
    #[doc = "0: Timer0 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer0 controller reset"]
    _1 = 1,
}
impl From<TMR0RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0RST` reader - Timer0 Controller Reset"]
pub struct TMR0RST_R(crate::FieldReader<bool, TMR0RST_A>);
impl TMR0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0RST_A {
        match self.bits {
            false => TMR0RST_A::_0,
            true => TMR0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR0RST_A::_1
    }
}
impl core::ops::Deref for TMR0RST_R {
    type Target = crate::FieldReader<bool, TMR0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0RST` writer - Timer0 Controller Reset"]
pub struct TMR0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0RST_A::_0)
    }
    #[doc = "Timer0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1RST_A {
    #[doc = "0: Timer1 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer1 controller reset"]
    _1 = 1,
}
impl From<TMR1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1RST` reader - Timer1 Controller Reset"]
pub struct TMR1RST_R(crate::FieldReader<bool, TMR1RST_A>);
impl TMR1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1RST_A {
        match self.bits {
            false => TMR1RST_A::_0,
            true => TMR1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR1RST_A::_1
    }
}
impl core::ops::Deref for TMR1RST_R {
    type Target = crate::FieldReader<bool, TMR1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1RST` writer - Timer1 Controller Reset"]
pub struct TMR1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1RST_A::_0)
    }
    #[doc = "Timer1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1RST_A::_1)
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
#[doc = "Timer2 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2RST_A {
    #[doc = "0: Timer2 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer2 controller reset"]
    _1 = 1,
}
impl From<TMR2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2RST` reader - Timer2 Controller Reset"]
pub struct TMR2RST_R(crate::FieldReader<bool, TMR2RST_A>);
impl TMR2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2RST_A {
        match self.bits {
            false => TMR2RST_A::_0,
            true => TMR2RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR2RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR2RST_A::_1
    }
}
impl core::ops::Deref for TMR2RST_R {
    type Target = crate::FieldReader<bool, TMR2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2RST` writer - Timer2 Controller Reset"]
pub struct TMR2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer2 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2RST_A::_0)
    }
    #[doc = "Timer2 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2RST_A::_1)
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
#[doc = "Timer3 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3RST_A {
    #[doc = "0: Timer3 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer3 controller reset"]
    _1 = 1,
}
impl From<TMR3RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR3RST` reader - Timer3 Controller Reset"]
pub struct TMR3RST_R(crate::FieldReader<bool, TMR3RST_A>);
impl TMR3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR3RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3RST_A {
        match self.bits {
            false => TMR3RST_A::_0,
            true => TMR3RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR3RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR3RST_A::_1
    }
}
impl core::ops::Deref for TMR3RST_R {
    type Target = crate::FieldReader<bool, TMR3RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR3RST` writer - Timer3 Controller Reset"]
pub struct TMR3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer3 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3RST_A::_0)
    }
    #[doc = "Timer3 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Analog Comparator 0/1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP01RST_A {
    #[doc = "0: Analog Comparator 0/1 controller normal operation"]
    _0 = 0,
    #[doc = "1: Analog Comparator 0/1 controller reset"]
    _1 = 1,
}
impl From<ACMP01RST_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP01RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP01RST` reader - Analog Comparator 0/1 Controller Reset"]
pub struct ACMP01RST_R(crate::FieldReader<bool, ACMP01RST_A>);
impl ACMP01RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP01RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP01RST_A {
        match self.bits {
            false => ACMP01RST_A::_0,
            true => ACMP01RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMP01RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMP01RST_A::_1
    }
}
impl core::ops::Deref for ACMP01RST_R {
    type Target = crate::FieldReader<bool, ACMP01RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP01RST` writer - Analog Comparator 0/1 Controller Reset"]
pub struct ACMP01RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP01RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP01RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator 0/1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP01RST_A::_0)
    }
    #[doc = "Analog Comparator 0/1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP01RST_A::_1)
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
#[doc = "I2C0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0RST_A {
    #[doc = "0: I2C0 controller normal operation"]
    _0 = 0,
    #[doc = "1: I2C0 controller reset"]
    _1 = 1,
}
impl From<I2C0RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0RST` reader - I2C0 Controller Reset"]
pub struct I2C0RST_R(crate::FieldReader<bool, I2C0RST_A>);
impl I2C0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0RST_A {
        match self.bits {
            false => I2C0RST_A::_0,
            true => I2C0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C0RST_A::_1
    }
}
impl core::ops::Deref for I2C0RST_R {
    type Target = crate::FieldReader<bool, I2C0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0RST` writer - I2C0 Controller Reset"]
pub struct I2C0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0RST_A::_0)
    }
    #[doc = "I2C0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0RST_A::_1)
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
#[doc = "I2C1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1RST_A {
    #[doc = "0: I2C1 controller normal operation"]
    _0 = 0,
    #[doc = "1: I2C1 controller reset"]
    _1 = 1,
}
impl From<I2C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 Controller Reset"]
pub struct I2C1RST_R(crate::FieldReader<bool, I2C1RST_A>);
impl I2C1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1RST_A {
        match self.bits {
            false => I2C1RST_A::_0,
            true => I2C1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C1RST_A::_1
    }
}
impl core::ops::Deref for I2C1RST_R {
    type Target = crate::FieldReader<bool, I2C1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 Controller Reset"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1RST_A::_0)
    }
    #[doc = "I2C1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1RST_A::_1)
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
#[doc = "QSPI0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0RST_A {
    #[doc = "0: QSPI0 controller normal operation"]
    _0 = 0,
    #[doc = "1: QSPI0 controller reset"]
    _1 = 1,
}
impl From<QSPI0RST_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPI0RST` reader - QSPI0 Controller Reset"]
pub struct QSPI0RST_R(crate::FieldReader<bool, QSPI0RST_A>);
impl QSPI0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        QSPI0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI0RST_A {
        match self.bits {
            false => QSPI0RST_A::_0,
            true => QSPI0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == QSPI0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == QSPI0RST_A::_1
    }
}
impl core::ops::Deref for QSPI0RST_R {
    type Target = crate::FieldReader<bool, QSPI0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI0RST` writer - QSPI0 Controller Reset"]
pub struct QSPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPI0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QSPI0RST_A::_0)
    }
    #[doc = "QSPI0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QSPI0RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "SPI0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0RST_A {
    #[doc = "0: SPI0 controller normal operation"]
    _0 = 0,
    #[doc = "1: SPI0 controller reset"]
    _1 = 1,
}
impl From<SPI0RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0RST` reader - SPI0 Controller Reset"]
pub struct SPI0RST_R(crate::FieldReader<bool, SPI0RST_A>);
impl SPI0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0RST_A {
        match self.bits {
            false => SPI0RST_A::_0,
            true => SPI0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0RST_A::_1
    }
}
impl core::ops::Deref for SPI0RST_R {
    type Target = crate::FieldReader<bool, SPI0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0RST` writer - SPI0 Controller Reset"]
pub struct SPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0RST_A::_0)
    }
    #[doc = "SPI0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "UART0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RST_A {
    #[doc = "0: UART0 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART0 controller reset"]
    _1 = 1,
}
impl From<UART0RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0RST` reader - UART0 Controller Reset"]
pub struct UART0RST_R(crate::FieldReader<bool, UART0RST_A>);
impl UART0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0RST_A {
        match self.bits {
            false => UART0RST_A::_0,
            true => UART0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0RST_A::_1
    }
}
impl core::ops::Deref for UART0RST_R {
    type Target = crate::FieldReader<bool, UART0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0RST` writer - UART0 Controller Reset"]
pub struct UART0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0RST_A::_0)
    }
    #[doc = "UART0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "UART1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RST_A {
    #[doc = "0: UART1 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART1 controller reset"]
    _1 = 1,
}
impl From<UART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1RST` reader - UART1 Controller Reset"]
pub struct UART1RST_R(crate::FieldReader<bool, UART1RST_A>);
impl UART1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1RST_A {
        match self.bits {
            false => UART1RST_A::_0,
            true => UART1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART1RST_A::_1
    }
}
impl core::ops::Deref for UART1RST_R {
    type Target = crate::FieldReader<bool, UART1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1RST` writer - UART1 Controller Reset"]
pub struct UART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1RST_A::_0)
    }
    #[doc = "UART1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "UART2 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2RST_A {
    #[doc = "0: UART2 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART2 controller reset"]
    _1 = 1,
}
impl From<UART2RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2RST` reader - UART2 Controller Reset"]
pub struct UART2RST_R(crate::FieldReader<bool, UART2RST_A>);
impl UART2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2RST_A {
        match self.bits {
            false => UART2RST_A::_0,
            true => UART2RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART2RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART2RST_A::_1
    }
}
impl core::ops::Deref for UART2RST_R {
    type Target = crate::FieldReader<bool, UART2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2RST` writer - UART2 Controller Reset"]
pub struct UART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART2 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2RST_A::_0)
    }
    #[doc = "UART2 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "UART3 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3RST_A {
    #[doc = "0: UART3 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART3 controller reset"]
    _1 = 1,
}
impl From<UART3RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART3RST` reader - UART3 Controller Reset"]
pub struct UART3RST_R(crate::FieldReader<bool, UART3RST_A>);
impl UART3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART3RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3RST_A {
        match self.bits {
            false => UART3RST_A::_0,
            true => UART3RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART3RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART3RST_A::_1
    }
}
impl core::ops::Deref for UART3RST_R {
    type Target = crate::FieldReader<bool, UART3RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3RST` writer - UART3 Controller Reset"]
pub struct UART3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART3 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3RST_A::_0)
    }
    #[doc = "UART3 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "UART4 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4RST_A {
    #[doc = "0: UART4 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART4 controller reset"]
    _1 = 1,
}
impl From<UART4RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4RST` reader - UART4 Controller Reset"]
pub struct UART4RST_R(crate::FieldReader<bool, UART4RST_A>);
impl UART4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4RST_A {
        match self.bits {
            false => UART4RST_A::_0,
            true => UART4RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART4RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART4RST_A::_1
    }
}
impl core::ops::Deref for UART4RST_R {
    type Target = crate::FieldReader<bool, UART4RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4RST` writer - UART4 Controller Reset"]
pub struct UART4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART4 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART4RST_A::_0)
    }
    #[doc = "UART4 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART4RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "UART5 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART5RST_A {
    #[doc = "0: UART5 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART5 controller reset"]
    _1 = 1,
}
impl From<UART5RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART5RST` reader - UART5 Controller Reset"]
pub struct UART5RST_R(crate::FieldReader<bool, UART5RST_A>);
impl UART5RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5RST_A {
        match self.bits {
            false => UART5RST_A::_0,
            true => UART5RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART5RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART5RST_A::_1
    }
}
impl core::ops::Deref for UART5RST_R {
    type Target = crate::FieldReader<bool, UART5RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5RST` writer - UART5 Controller Reset"]
pub struct UART5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART5 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART5RST_A::_0)
    }
    #[doc = "UART5 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART5RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "UART6 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART6RST_A {
    #[doc = "0: UART6 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART6 controller reset"]
    _1 = 1,
}
impl From<UART6RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART6RST` reader - UART6 Controller Reset"]
pub struct UART6RST_R(crate::FieldReader<bool, UART6RST_A>);
impl UART6RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART6RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART6RST_A {
        match self.bits {
            false => UART6RST_A::_0,
            true => UART6RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART6RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART6RST_A::_1
    }
}
impl core::ops::Deref for UART6RST_R {
    type Target = crate::FieldReader<bool, UART6RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART6RST` writer - UART6 Controller Reset"]
pub struct UART6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART6RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART6 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART6RST_A::_0)
    }
    #[doc = "UART6 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART6RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "UART7 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART7RST_A {
    #[doc = "0: UART7 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART7 controller reset"]
    _1 = 1,
}
impl From<UART7RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART7RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART7RST` reader - UART7 Controller Reset"]
pub struct UART7RST_R(crate::FieldReader<bool, UART7RST_A>);
impl UART7RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART7RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART7RST_A {
        match self.bits {
            false => UART7RST_A::_0,
            true => UART7RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART7RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART7RST_A::_1
    }
}
impl core::ops::Deref for UART7RST_R {
    type Target = crate::FieldReader<bool, UART7RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7RST` writer - UART7 Controller Reset"]
pub struct UART7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART7 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART7RST_A::_0)
    }
    #[doc = "UART7 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART7RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "USBD Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDRST_A {
    #[doc = "0: USBD controller normal operation"]
    _0 = 0,
    #[doc = "1: USBD controller reset"]
    _1 = 1,
}
impl From<USBDRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBDRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDRST` reader - USBD Controller Reset"]
pub struct USBDRST_R(crate::FieldReader<bool, USBDRST_A>);
impl USBDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBDRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDRST_A {
        match self.bits {
            false => USBDRST_A::_0,
            true => USBDRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBDRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBDRST_A::_1
    }
}
impl core::ops::Deref for USBDRST_R {
    type Target = crate::FieldReader<bool, USBDRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDRST` writer - USBD Controller Reset"]
pub struct USBDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USBD controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDRST_A::_0)
    }
    #[doc = "USBD controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "ADC Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCRST_A {
    #[doc = "0: ADC controller normal operation"]
    _0 = 0,
    #[doc = "1: ADC controller reset"]
    _1 = 1,
}
impl From<ADCRST_A> for bool {
    #[inline(always)]
    fn from(variant: ADCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCRST` reader - ADC Controller Reset"]
pub struct ADCRST_R(crate::FieldReader<bool, ADCRST_A>);
impl ADCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCRST_A {
        match self.bits {
            false => ADCRST_A::_0,
            true => ADCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCRST_A::_1
    }
}
impl core::ops::Deref for ADCRST_R {
    type Target = crate::FieldReader<bool, ADCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCRST` writer - ADC Controller Reset"]
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCRST_A::_0)
    }
    #[doc = "ADC controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - GPIO Controller Reset"]
    #[inline(always)]
    pub fn gpiorst(&self) -> GPIORST_R {
        GPIORST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0 Controller Reset"]
    #[inline(always)]
    pub fn tmr0rst(&self) -> TMR0RST_R {
        TMR0RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer1 Controller Reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> TMR1RST_R {
        TMR1RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer2 Controller Reset"]
    #[inline(always)]
    pub fn tmr2rst(&self) -> TMR2RST_R {
        TMR2RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer3 Controller Reset"]
    #[inline(always)]
    pub fn tmr3rst(&self) -> TMR3RST_R {
        TMR3RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Controller Reset"]
    #[inline(always)]
    pub fn acmp01rst(&self) -> ACMP01RST_R {
        ACMP01RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 Controller Reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C1 Controller Reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - QSPI0 Controller Reset"]
    #[inline(always)]
    pub fn qspi0rst(&self) -> QSPI0RST_R {
        QSPI0RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI0 Controller Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Controller Reset"]
    #[inline(always)]
    pub fn uart0rst(&self) -> UART0RST_R {
        UART0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 Controller Reset"]
    #[inline(always)]
    pub fn uart1rst(&self) -> UART1RST_R {
        UART1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 Controller Reset"]
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART3 Controller Reset"]
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART4 Controller Reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - UART5 Controller Reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UART6 Controller Reset"]
    #[inline(always)]
    pub fn uart6rst(&self) -> UART6RST_R {
        UART6RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UART7 Controller Reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USBD Controller Reset"]
    #[inline(always)]
    pub fn usbdrst(&self) -> USBDRST_R {
        USBDRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC Controller Reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - GPIO Controller Reset"]
    #[inline(always)]
    pub fn gpiorst(&mut self) -> GPIORST_W {
        GPIORST_W { w: self }
    }
    #[doc = "Bit 2 - Timer0 Controller Reset"]
    #[inline(always)]
    pub fn tmr0rst(&mut self) -> TMR0RST_W {
        TMR0RST_W { w: self }
    }
    #[doc = "Bit 3 - Timer1 Controller Reset"]
    #[inline(always)]
    pub fn tmr1rst(&mut self) -> TMR1RST_W {
        TMR1RST_W { w: self }
    }
    #[doc = "Bit 4 - Timer2 Controller Reset"]
    #[inline(always)]
    pub fn tmr2rst(&mut self) -> TMR2RST_W {
        TMR2RST_W { w: self }
    }
    #[doc = "Bit 5 - Timer3 Controller Reset"]
    #[inline(always)]
    pub fn tmr3rst(&mut self) -> TMR3RST_W {
        TMR3RST_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Controller Reset"]
    #[inline(always)]
    pub fn acmp01rst(&mut self) -> ACMP01RST_W {
        ACMP01RST_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 Controller Reset"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2C0RST_W {
        I2C0RST_W { w: self }
    }
    #[doc = "Bit 9 - I2C1 Controller Reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 12 - QSPI0 Controller Reset"]
    #[inline(always)]
    pub fn qspi0rst(&mut self) -> QSPI0RST_W {
        QSPI0RST_W { w: self }
    }
    #[doc = "Bit 13 - SPI0 Controller Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Controller Reset"]
    #[inline(always)]
    pub fn uart0rst(&mut self) -> UART0RST_W {
        UART0RST_W { w: self }
    }
    #[doc = "Bit 17 - UART1 Controller Reset"]
    #[inline(always)]
    pub fn uart1rst(&mut self) -> UART1RST_W {
        UART1RST_W { w: self }
    }
    #[doc = "Bit 18 - UART2 Controller Reset"]
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W {
        UART2RST_W { w: self }
    }
    #[doc = "Bit 19 - UART3 Controller Reset"]
    #[inline(always)]
    pub fn uart3rst(&mut self) -> UART3RST_W {
        UART3RST_W { w: self }
    }
    #[doc = "Bit 20 - UART4 Controller Reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W {
        UART4RST_W { w: self }
    }
    #[doc = "Bit 21 - UART5 Controller Reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W {
        UART5RST_W { w: self }
    }
    #[doc = "Bit 22 - UART6 Controller Reset"]
    #[inline(always)]
    pub fn uart6rst(&mut self) -> UART6RST_W {
        UART6RST_W { w: self }
    }
    #[doc = "Bit 23 - UART7 Controller Reset"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W {
        UART7RST_W { w: self }
    }
    #[doc = "Bit 27 - USBD Controller Reset"]
    #[inline(always)]
    pub fn usbdrst(&mut self) -> USBDRST_W {
        USBDRST_W { w: self }
    }
    #[doc = "Bit 28 - ADC Controller Reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_iprst1](index.html) module"]
pub struct SYS_IPRST1_SPEC;
impl crate::RegisterSpec for SYS_IPRST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_iprst1::R](R) reader structure"]
impl crate::Readable for SYS_IPRST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_iprst1::W](W) writer structure"]
impl crate::Writable for SYS_IPRST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IPRST1 to value 0"]
impl crate::Resettable for SYS_IPRST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
