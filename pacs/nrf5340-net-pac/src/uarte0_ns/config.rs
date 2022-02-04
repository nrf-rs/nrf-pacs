#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hardware flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWFC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HWFC_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC` reader - Hardware flow control"]
pub struct HWFC_R(crate::FieldReader<bool, HWFC_A>);
impl HWFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWFC_A {
        match self.bits {
            false => HWFC_A::DISABLED,
            true => HWFC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HWFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HWFC_A::ENABLED
    }
}
impl core::ops::Deref for HWFC_R {
    type Target = crate::FieldReader<bool, HWFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWFC` writer - Hardware flow control"]
pub struct HWFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Exclude parity bit"]
    EXCLUDED = 0,
    #[doc = "7: Include even parity bit"]
    INCLUDED = 7,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PARITY` reader - Parity"]
pub struct PARITY_R(crate::FieldReader<u8, PARITY_A>);
impl PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITY_A> {
        match self.bits {
            0 => Some(PARITY_A::EXCLUDED),
            7 => Some(PARITY_A::INCLUDED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == PARITY_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == PARITY_A::INCLUDED
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<u8, PARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` writer - Parity"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(PARITY_A::EXCLUDED)
    }
    #[doc = "Include even parity bit"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(PARITY_A::INCLUDED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: One stop bit"]
    ONE = 0,
    #[doc = "1: Two stop bits"]
    TWO = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop bits"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::ONE,
            true => STOP_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == STOP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == STOP_A::TWO
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop bits"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STOP_A::ONE)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(STOP_A::TWO)
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
#[doc = "Even or odd parity type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYTYPE_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
}
impl From<PARITYTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYTYPE` reader - Even or odd parity type"]
pub struct PARITYTYPE_R(crate::FieldReader<bool, PARITYTYPE_A>);
impl PARITYTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITYTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYTYPE_A {
        match self.bits {
            false => PARITYTYPE_A::EVEN,
            true => PARITYTYPE_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == PARITYTYPE_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == PARITYTYPE_A::ODD
    }
}
impl core::ops::Deref for PARITYTYPE_R {
    type Target = crate::FieldReader<bool, PARITYTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITYTYPE` writer - Even or odd parity type"]
pub struct PARITYTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITYTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITYTYPE_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITYTYPE_A::ODD)
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
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&self) -> HWFC_R {
        HWFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    pub fn paritytype(&self) -> PARITYTYPE_R {
        PARITYTYPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&mut self) -> HWFC_W {
        HWFC_W { w: self }
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    pub fn paritytype(&mut self) -> PARITYTYPE_W {
        PARITYTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of parity and hardware flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
