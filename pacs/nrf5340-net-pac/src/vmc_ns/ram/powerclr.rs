#[doc = "Register `POWERCLR` reader"]
pub struct R(crate::R<POWERCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWERCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWERCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWERCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWERCLR` writer"]
pub struct W(crate::W<POWERCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERCLR_SPEC>;
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
impl From<crate::W<POWERCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Keep RAM section S0 of RAM\\[n\\]
on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S0POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` reader - Keep RAM section S0 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S0POWER_R(crate::FieldReader<bool, S0POWER_A>);
impl S0POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0POWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S0POWER_A> {
        match self.bits {
            true => Some(S0POWER_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S0POWER_A::OFF
    }
}
impl core::ops::Deref for S0POWER_R {
    type Target = crate::FieldReader<bool, S0POWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S0POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S0POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0POWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_A::OFF)
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
#[doc = "Keep RAM section S1 of RAM\\[n\\]
on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S1POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` reader - Keep RAM section S1 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S1POWER_R(crate::FieldReader<bool, S1POWER_A>);
impl S1POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1POWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S1POWER_A> {
        match self.bits {
            true => Some(S1POWER_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S1POWER_A::OFF
    }
}
impl core::ops::Deref for S1POWER_R {
    type Target = crate::FieldReader<bool, S1POWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S1POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S1POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1POWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_A::OFF)
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
#[doc = "Keep RAM section S2 of RAM\\[n\\]
on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2POWER_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S2POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S2POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` reader - Keep RAM section S2 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S2POWER_R(crate::FieldReader<bool, S2POWER_A>);
impl S2POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2POWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S2POWER_A> {
        match self.bits {
            true => Some(S2POWER_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S2POWER_A::OFF
    }
}
impl core::ops::Deref for S2POWER_R {
    type Target = crate::FieldReader<bool, S2POWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S2POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S2POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S2POWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWER_A::OFF)
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
#[doc = "Keep RAM section S3 of RAM\\[n\\]
on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3POWER_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S3POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S3POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` reader - Keep RAM section S3 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S3POWER_R(crate::FieldReader<bool, S3POWER_A>);
impl S3POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3POWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S3POWER_A> {
        match self.bits {
            true => Some(S3POWER_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S3POWER_A::OFF
    }
}
impl core::ops::Deref for S3POWER_R {
    type Target = crate::FieldReader<bool, S3POWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 of RAM\\[n\\]
on or off in System ON mode"]
pub struct S3POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S3POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S3POWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWER_A::OFF)
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
#[doc = "Keep retention on RAM section S0 of RAM\\[n\\]
when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S0RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` reader - Keep retention on RAM section S0 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S0RETENTION_R(crate::FieldReader<bool, S0RETENTION_A>);
impl S0RETENTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0RETENTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S0RETENTION_A> {
        match self.bits {
            true => Some(S0RETENTION_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S0RETENTION_A::OFF
    }
}
impl core::ops::Deref for S0RETENTION_R {
    type Target = crate::FieldReader<bool, S0RETENTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S0RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S0RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0RETENTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_A::OFF)
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
#[doc = "Keep retention on RAM section S1 of RAM\\[n\\]
when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTION_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S1RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` reader - Keep retention on RAM section S1 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S1RETENTION_R(crate::FieldReader<bool, S1RETENTION_A>);
impl S1RETENTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1RETENTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S1RETENTION_A> {
        match self.bits {
            true => Some(S1RETENTION_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S1RETENTION_A::OFF
    }
}
impl core::ops::Deref for S1RETENTION_R {
    type Target = crate::FieldReader<bool, S1RETENTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S1RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S1RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1RETENTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_A::OFF)
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
#[doc = "Keep retention on RAM section S2 of RAM\\[n\\]
when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RETENTION_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S2RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S2RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` reader - Keep retention on RAM section S2 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S2RETENTION_R(crate::FieldReader<bool, S2RETENTION_A>);
impl S2RETENTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2RETENTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S2RETENTION_A> {
        match self.bits {
            true => Some(S2RETENTION_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S2RETENTION_A::OFF
    }
}
impl core::ops::Deref for S2RETENTION_R {
    type Target = crate::FieldReader<bool, S2RETENTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S2RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S2RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S2RETENTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTION_A::OFF)
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
#[doc = "Keep retention on RAM section S3 of RAM\\[n\\]
when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3RETENTION_A {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S3RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S3RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` reader - Keep retention on RAM section S3 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S3RETENTION_R(crate::FieldReader<bool, S3RETENTION_A>);
impl S3RETENTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3RETENTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S3RETENTION_A> {
        match self.bits {
            true => Some(S3RETENTION_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == S3RETENTION_A::OFF
    }
}
impl core::ops::Deref for S3RETENTION_R {
    type Target = crate::FieldReader<bool, S3RETENTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 of RAM\\[n\\]
when RAM section is switched off"]
pub struct S3RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S3RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S3RETENTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTION_A::OFF)
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
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&self) -> S0POWER_R {
        S0POWER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&self) -> S1POWER_R {
        S1POWER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s2power(&self) -> S2POWER_R {
        S2POWER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s3power(&self) -> S3POWER_R {
        S3POWER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0RETENTION_R {
        S0RETENTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1RETENTION_R {
        S1RETENTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s2retention(&self) -> S2RETENTION_R {
        S2RETENTION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s3retention(&self) -> S3RETENTION_R {
        S3RETENTION_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W {
        S0POWER_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W {
        S1POWER_W { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s2power(&mut self) -> S2POWER_W {
        S2POWER_W { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM\\[n\\]
on or off in System ON mode"]
    #[inline(always)]
    pub fn s3power(&mut self) -> S3POWER_W {
        S3POWER_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0RETENTION_W {
        S0RETENTION_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1RETENTION_W {
        S1RETENTION_W { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s2retention(&mut self) -> S2RETENTION_W {
        S2RETENTION_W { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM\\[n\\]
when RAM section is switched off"]
    #[inline(always)]
    pub fn s3retention(&mut self) -> S3RETENTION_W {
        S3RETENTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: RAM\\[n\\]
power control clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerclr](index.html) module"]
pub struct POWERCLR_SPEC;
impl crate::RegisterSpec for POWERCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerclr::R](R) reader structure"]
impl crate::Readable for POWERCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [powerclr::W](W) writer structure"]
impl crate::Writable for POWERCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERCLR to value 0xffff"]
impl crate::Resettable for POWERCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
