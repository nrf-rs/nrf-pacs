#[doc = "Register `CONFIG0` reader"]
pub struct R(crate::R<CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG0` writer"]
pub struct W(crate::W<CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG0_SPEC>;
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
impl From<crate::W<CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable protection for region 0. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION0_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0` reader - Enable protection for region 0. Write '0' has no effect."]
pub struct REGION0_R(crate::FieldReader<bool, REGION0_A>);
impl REGION0_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0_A {
        match self.bits {
            false => REGION0_A::DISABLED,
            true => REGION0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION0_A::ENABLED
    }
}
impl core::ops::Deref for REGION0_R {
    type Target = crate::FieldReader<bool, REGION0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION0` writer - Enable protection for region 0. Write '0' has no effect."]
pub struct REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0_A::ENABLED)
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
#[doc = "Enable protection for region 1. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION1_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1` reader - Enable protection for region 1. Write '0' has no effect."]
pub struct REGION1_R(crate::FieldReader<bool, REGION1_A>);
impl REGION1_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1_A {
        match self.bits {
            false => REGION1_A::DISABLED,
            true => REGION1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION1_A::ENABLED
    }
}
impl core::ops::Deref for REGION1_R {
    type Target = crate::FieldReader<bool, REGION1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION1` writer - Enable protection for region 1. Write '0' has no effect."]
pub struct REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1_A::ENABLED)
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
#[doc = "Enable protection for region 2. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION2_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2` reader - Enable protection for region 2. Write '0' has no effect."]
pub struct REGION2_R(crate::FieldReader<bool, REGION2_A>);
impl REGION2_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2_A {
        match self.bits {
            false => REGION2_A::DISABLED,
            true => REGION2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION2_A::ENABLED
    }
}
impl core::ops::Deref for REGION2_R {
    type Target = crate::FieldReader<bool, REGION2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION2` writer - Enable protection for region 2. Write '0' has no effect."]
pub struct REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2_A::ENABLED)
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
#[doc = "Enable protection for region 3. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION3_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3` reader - Enable protection for region 3. Write '0' has no effect."]
pub struct REGION3_R(crate::FieldReader<bool, REGION3_A>);
impl REGION3_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3_A {
        match self.bits {
            false => REGION3_A::DISABLED,
            true => REGION3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION3_A::ENABLED
    }
}
impl core::ops::Deref for REGION3_R {
    type Target = crate::FieldReader<bool, REGION3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION3` writer - Enable protection for region 3. Write '0' has no effect."]
pub struct REGION3_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3_A::ENABLED)
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
#[doc = "Enable protection for region 4. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION4_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION4_A> for bool {
    #[inline(always)]
    fn from(variant: REGION4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION4` reader - Enable protection for region 4. Write '0' has no effect."]
pub struct REGION4_R(crate::FieldReader<bool, REGION4_A>);
impl REGION4_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION4_A {
        match self.bits {
            false => REGION4_A::DISABLED,
            true => REGION4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION4_A::ENABLED
    }
}
impl core::ops::Deref for REGION4_R {
    type Target = crate::FieldReader<bool, REGION4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION4` writer - Enable protection for region 4. Write '0' has no effect."]
pub struct REGION4_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION4_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION4_A::ENABLED)
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
#[doc = "Enable protection for region 5. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION5_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION5_A> for bool {
    #[inline(always)]
    fn from(variant: REGION5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION5` reader - Enable protection for region 5. Write '0' has no effect."]
pub struct REGION5_R(crate::FieldReader<bool, REGION5_A>);
impl REGION5_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION5_A {
        match self.bits {
            false => REGION5_A::DISABLED,
            true => REGION5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION5_A::ENABLED
    }
}
impl core::ops::Deref for REGION5_R {
    type Target = crate::FieldReader<bool, REGION5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION5` writer - Enable protection for region 5. Write '0' has no effect."]
pub struct REGION5_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION5_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION5_A::ENABLED)
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
#[doc = "Enable protection for region 6. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION6_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION6_A> for bool {
    #[inline(always)]
    fn from(variant: REGION6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION6` reader - Enable protection for region 6. Write '0' has no effect."]
pub struct REGION6_R(crate::FieldReader<bool, REGION6_A>);
impl REGION6_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION6_A {
        match self.bits {
            false => REGION6_A::DISABLED,
            true => REGION6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION6_A::ENABLED
    }
}
impl core::ops::Deref for REGION6_R {
    type Target = crate::FieldReader<bool, REGION6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION6` writer - Enable protection for region 6. Write '0' has no effect."]
pub struct REGION6_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION6_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION6_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable protection for region 7. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION7_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION7_A> for bool {
    #[inline(always)]
    fn from(variant: REGION7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION7` reader - Enable protection for region 7. Write '0' has no effect."]
pub struct REGION7_R(crate::FieldReader<bool, REGION7_A>);
impl REGION7_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION7_A {
        match self.bits {
            false => REGION7_A::DISABLED,
            true => REGION7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION7_A::ENABLED
    }
}
impl core::ops::Deref for REGION7_R {
    type Target = crate::FieldReader<bool, REGION7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION7` writer - Enable protection for region 7. Write '0' has no effect."]
pub struct REGION7_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION7_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION7_A::ENABLED)
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
#[doc = "Enable protection for region 8. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION8_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION8_A> for bool {
    #[inline(always)]
    fn from(variant: REGION8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION8` reader - Enable protection for region 8. Write '0' has no effect."]
pub struct REGION8_R(crate::FieldReader<bool, REGION8_A>);
impl REGION8_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION8_A {
        match self.bits {
            false => REGION8_A::DISABLED,
            true => REGION8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION8_A::ENABLED
    }
}
impl core::ops::Deref for REGION8_R {
    type Target = crate::FieldReader<bool, REGION8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION8` writer - Enable protection for region 8. Write '0' has no effect."]
pub struct REGION8_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION8_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION8_A::ENABLED)
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
#[doc = "Enable protection for region 9. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION9_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION9_A> for bool {
    #[inline(always)]
    fn from(variant: REGION9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION9` reader - Enable protection for region 9. Write '0' has no effect."]
pub struct REGION9_R(crate::FieldReader<bool, REGION9_A>);
impl REGION9_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION9_A {
        match self.bits {
            false => REGION9_A::DISABLED,
            true => REGION9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION9_A::ENABLED
    }
}
impl core::ops::Deref for REGION9_R {
    type Target = crate::FieldReader<bool, REGION9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION9` writer - Enable protection for region 9. Write '0' has no effect."]
pub struct REGION9_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION9_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION9_A::ENABLED)
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
#[doc = "Enable protection for region 10. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION10_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION10_A> for bool {
    #[inline(always)]
    fn from(variant: REGION10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION10` reader - Enable protection for region 10. Write '0' has no effect."]
pub struct REGION10_R(crate::FieldReader<bool, REGION10_A>);
impl REGION10_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION10_A {
        match self.bits {
            false => REGION10_A::DISABLED,
            true => REGION10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION10_A::ENABLED
    }
}
impl core::ops::Deref for REGION10_R {
    type Target = crate::FieldReader<bool, REGION10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION10` writer - Enable protection for region 10. Write '0' has no effect."]
pub struct REGION10_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION10_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable protection for region 11. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION11_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION11_A> for bool {
    #[inline(always)]
    fn from(variant: REGION11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION11` reader - Enable protection for region 11. Write '0' has no effect."]
pub struct REGION11_R(crate::FieldReader<bool, REGION11_A>);
impl REGION11_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION11_A {
        match self.bits {
            false => REGION11_A::DISABLED,
            true => REGION11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION11_A::ENABLED
    }
}
impl core::ops::Deref for REGION11_R {
    type Target = crate::FieldReader<bool, REGION11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION11` writer - Enable protection for region 11. Write '0' has no effect."]
pub struct REGION11_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION11_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION11_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable protection for region 12. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION12_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION12_A> for bool {
    #[inline(always)]
    fn from(variant: REGION12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION12` reader - Enable protection for region 12. Write '0' has no effect."]
pub struct REGION12_R(crate::FieldReader<bool, REGION12_A>);
impl REGION12_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION12_A {
        match self.bits {
            false => REGION12_A::DISABLED,
            true => REGION12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION12_A::ENABLED
    }
}
impl core::ops::Deref for REGION12_R {
    type Target = crate::FieldReader<bool, REGION12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION12` writer - Enable protection for region 12. Write '0' has no effect."]
pub struct REGION12_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION12_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION12_A::ENABLED)
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
#[doc = "Enable protection for region 13. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION13_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION13_A> for bool {
    #[inline(always)]
    fn from(variant: REGION13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION13` reader - Enable protection for region 13. Write '0' has no effect."]
pub struct REGION13_R(crate::FieldReader<bool, REGION13_A>);
impl REGION13_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION13_A {
        match self.bits {
            false => REGION13_A::DISABLED,
            true => REGION13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION13_A::ENABLED
    }
}
impl core::ops::Deref for REGION13_R {
    type Target = crate::FieldReader<bool, REGION13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION13` writer - Enable protection for region 13. Write '0' has no effect."]
pub struct REGION13_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION13_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION13_A::ENABLED)
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
#[doc = "Enable protection for region 14. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION14_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION14_A> for bool {
    #[inline(always)]
    fn from(variant: REGION14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION14` reader - Enable protection for region 14. Write '0' has no effect."]
pub struct REGION14_R(crate::FieldReader<bool, REGION14_A>);
impl REGION14_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION14_A {
        match self.bits {
            false => REGION14_A::DISABLED,
            true => REGION14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION14_A::ENABLED
    }
}
impl core::ops::Deref for REGION14_R {
    type Target = crate::FieldReader<bool, REGION14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION14` writer - Enable protection for region 14. Write '0' has no effect."]
pub struct REGION14_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION14_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION14_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Enable protection for region 15. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION15_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION15_A> for bool {
    #[inline(always)]
    fn from(variant: REGION15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION15` reader - Enable protection for region 15. Write '0' has no effect."]
pub struct REGION15_R(crate::FieldReader<bool, REGION15_A>);
impl REGION15_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION15_A {
        match self.bits {
            false => REGION15_A::DISABLED,
            true => REGION15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION15_A::ENABLED
    }
}
impl core::ops::Deref for REGION15_R {
    type Target = crate::FieldReader<bool, REGION15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION15` writer - Enable protection for region 15. Write '0' has no effect."]
pub struct REGION15_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION15_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION15_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable protection for region 16. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION16_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION16_A> for bool {
    #[inline(always)]
    fn from(variant: REGION16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION16` reader - Enable protection for region 16. Write '0' has no effect."]
pub struct REGION16_R(crate::FieldReader<bool, REGION16_A>);
impl REGION16_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION16_A {
        match self.bits {
            false => REGION16_A::DISABLED,
            true => REGION16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION16_A::ENABLED
    }
}
impl core::ops::Deref for REGION16_R {
    type Target = crate::FieldReader<bool, REGION16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION16` writer - Enable protection for region 16. Write '0' has no effect."]
pub struct REGION16_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION16_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION16_A::ENABLED)
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
#[doc = "Enable protection for region 17. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION17_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION17_A> for bool {
    #[inline(always)]
    fn from(variant: REGION17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION17` reader - Enable protection for region 17. Write '0' has no effect."]
pub struct REGION17_R(crate::FieldReader<bool, REGION17_A>);
impl REGION17_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION17_A {
        match self.bits {
            false => REGION17_A::DISABLED,
            true => REGION17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION17_A::ENABLED
    }
}
impl core::ops::Deref for REGION17_R {
    type Target = crate::FieldReader<bool, REGION17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION17` writer - Enable protection for region 17. Write '0' has no effect."]
pub struct REGION17_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION17_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION17_A::ENABLED)
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
#[doc = "Enable protection for region 18. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION18_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION18_A> for bool {
    #[inline(always)]
    fn from(variant: REGION18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION18` reader - Enable protection for region 18. Write '0' has no effect."]
pub struct REGION18_R(crate::FieldReader<bool, REGION18_A>);
impl REGION18_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION18_A {
        match self.bits {
            false => REGION18_A::DISABLED,
            true => REGION18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION18_A::ENABLED
    }
}
impl core::ops::Deref for REGION18_R {
    type Target = crate::FieldReader<bool, REGION18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION18` writer - Enable protection for region 18. Write '0' has no effect."]
pub struct REGION18_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION18_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION18_A::ENABLED)
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
#[doc = "Enable protection for region 19. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION19_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION19_A> for bool {
    #[inline(always)]
    fn from(variant: REGION19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION19` reader - Enable protection for region 19. Write '0' has no effect."]
pub struct REGION19_R(crate::FieldReader<bool, REGION19_A>);
impl REGION19_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION19_A {
        match self.bits {
            false => REGION19_A::DISABLED,
            true => REGION19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION19_A::ENABLED
    }
}
impl core::ops::Deref for REGION19_R {
    type Target = crate::FieldReader<bool, REGION19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION19` writer - Enable protection for region 19. Write '0' has no effect."]
pub struct REGION19_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION19_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION19_A::ENABLED)
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
#[doc = "Enable protection for region 20. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION20_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION20_A> for bool {
    #[inline(always)]
    fn from(variant: REGION20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION20` reader - Enable protection for region 20. Write '0' has no effect."]
pub struct REGION20_R(crate::FieldReader<bool, REGION20_A>);
impl REGION20_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION20_A {
        match self.bits {
            false => REGION20_A::DISABLED,
            true => REGION20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION20_A::ENABLED
    }
}
impl core::ops::Deref for REGION20_R {
    type Target = crate::FieldReader<bool, REGION20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION20` writer - Enable protection for region 20. Write '0' has no effect."]
pub struct REGION20_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION20_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION20_A::ENABLED)
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
#[doc = "Enable protection for region 21. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION21_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION21_A> for bool {
    #[inline(always)]
    fn from(variant: REGION21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION21` reader - Enable protection for region 21. Write '0' has no effect."]
pub struct REGION21_R(crate::FieldReader<bool, REGION21_A>);
impl REGION21_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION21_A {
        match self.bits {
            false => REGION21_A::DISABLED,
            true => REGION21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION21_A::ENABLED
    }
}
impl core::ops::Deref for REGION21_R {
    type Target = crate::FieldReader<bool, REGION21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION21` writer - Enable protection for region 21. Write '0' has no effect."]
pub struct REGION21_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION21_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION21_A::ENABLED)
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
#[doc = "Enable protection for region 22. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION22_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION22_A> for bool {
    #[inline(always)]
    fn from(variant: REGION22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION22` reader - Enable protection for region 22. Write '0' has no effect."]
pub struct REGION22_R(crate::FieldReader<bool, REGION22_A>);
impl REGION22_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION22_A {
        match self.bits {
            false => REGION22_A::DISABLED,
            true => REGION22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION22_A::ENABLED
    }
}
impl core::ops::Deref for REGION22_R {
    type Target = crate::FieldReader<bool, REGION22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION22` writer - Enable protection for region 22. Write '0' has no effect."]
pub struct REGION22_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION22_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION22_A::ENABLED)
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
#[doc = "Enable protection for region 23. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION23_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION23_A> for bool {
    #[inline(always)]
    fn from(variant: REGION23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION23` reader - Enable protection for region 23. Write '0' has no effect."]
pub struct REGION23_R(crate::FieldReader<bool, REGION23_A>);
impl REGION23_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION23_A {
        match self.bits {
            false => REGION23_A::DISABLED,
            true => REGION23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION23_A::ENABLED
    }
}
impl core::ops::Deref for REGION23_R {
    type Target = crate::FieldReader<bool, REGION23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION23` writer - Enable protection for region 23. Write '0' has no effect."]
pub struct REGION23_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION23_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION23_A::ENABLED)
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
#[doc = "Enable protection for region 24. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION24_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION24_A> for bool {
    #[inline(always)]
    fn from(variant: REGION24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION24` reader - Enable protection for region 24. Write '0' has no effect."]
pub struct REGION24_R(crate::FieldReader<bool, REGION24_A>);
impl REGION24_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION24_A {
        match self.bits {
            false => REGION24_A::DISABLED,
            true => REGION24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION24_A::ENABLED
    }
}
impl core::ops::Deref for REGION24_R {
    type Target = crate::FieldReader<bool, REGION24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION24` writer - Enable protection for region 24. Write '0' has no effect."]
pub struct REGION24_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION24_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION24_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable protection for region 25. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION25_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION25_A> for bool {
    #[inline(always)]
    fn from(variant: REGION25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION25` reader - Enable protection for region 25. Write '0' has no effect."]
pub struct REGION25_R(crate::FieldReader<bool, REGION25_A>);
impl REGION25_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION25_A {
        match self.bits {
            false => REGION25_A::DISABLED,
            true => REGION25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION25_A::ENABLED
    }
}
impl core::ops::Deref for REGION25_R {
    type Target = crate::FieldReader<bool, REGION25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION25` writer - Enable protection for region 25. Write '0' has no effect."]
pub struct REGION25_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION25_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION25_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Enable protection for region 26. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION26_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION26_A> for bool {
    #[inline(always)]
    fn from(variant: REGION26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION26` reader - Enable protection for region 26. Write '0' has no effect."]
pub struct REGION26_R(crate::FieldReader<bool, REGION26_A>);
impl REGION26_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION26_A {
        match self.bits {
            false => REGION26_A::DISABLED,
            true => REGION26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION26_A::ENABLED
    }
}
impl core::ops::Deref for REGION26_R {
    type Target = crate::FieldReader<bool, REGION26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION26` writer - Enable protection for region 26. Write '0' has no effect."]
pub struct REGION26_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION26_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION26_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Enable protection for region 27. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION27_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION27_A> for bool {
    #[inline(always)]
    fn from(variant: REGION27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION27` reader - Enable protection for region 27. Write '0' has no effect."]
pub struct REGION27_R(crate::FieldReader<bool, REGION27_A>);
impl REGION27_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION27_A {
        match self.bits {
            false => REGION27_A::DISABLED,
            true => REGION27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION27_A::ENABLED
    }
}
impl core::ops::Deref for REGION27_R {
    type Target = crate::FieldReader<bool, REGION27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION27` writer - Enable protection for region 27. Write '0' has no effect."]
pub struct REGION27_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION27_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION27_A::ENABLED)
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
#[doc = "Enable protection for region 28. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION28_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION28_A> for bool {
    #[inline(always)]
    fn from(variant: REGION28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION28` reader - Enable protection for region 28. Write '0' has no effect."]
pub struct REGION28_R(crate::FieldReader<bool, REGION28_A>);
impl REGION28_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION28_A {
        match self.bits {
            false => REGION28_A::DISABLED,
            true => REGION28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION28_A::ENABLED
    }
}
impl core::ops::Deref for REGION28_R {
    type Target = crate::FieldReader<bool, REGION28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION28` writer - Enable protection for region 28. Write '0' has no effect."]
pub struct REGION28_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION28_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION28_A::ENABLED)
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
#[doc = "Enable protection for region 29. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION29_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION29_A> for bool {
    #[inline(always)]
    fn from(variant: REGION29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION29` reader - Enable protection for region 29. Write '0' has no effect."]
pub struct REGION29_R(crate::FieldReader<bool, REGION29_A>);
impl REGION29_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION29_A {
        match self.bits {
            false => REGION29_A::DISABLED,
            true => REGION29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION29_A::ENABLED
    }
}
impl core::ops::Deref for REGION29_R {
    type Target = crate::FieldReader<bool, REGION29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION29` writer - Enable protection for region 29. Write '0' has no effect."]
pub struct REGION29_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION29_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION29_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable protection for region 30. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION30_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION30_A> for bool {
    #[inline(always)]
    fn from(variant: REGION30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION30` reader - Enable protection for region 30. Write '0' has no effect."]
pub struct REGION30_R(crate::FieldReader<bool, REGION30_A>);
impl REGION30_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION30_A {
        match self.bits {
            false => REGION30_A::DISABLED,
            true => REGION30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION30_A::ENABLED
    }
}
impl core::ops::Deref for REGION30_R {
    type Target = crate::FieldReader<bool, REGION30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION30` writer - Enable protection for region 30. Write '0' has no effect."]
pub struct REGION30_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION30_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION30_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Enable protection for region 31. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION31_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION31_A> for bool {
    #[inline(always)]
    fn from(variant: REGION31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION31` reader - Enable protection for region 31. Write '0' has no effect."]
pub struct REGION31_R(crate::FieldReader<bool, REGION31_A>);
impl REGION31_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGION31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION31_A {
        match self.bits {
            false => REGION31_A::DISABLED,
            true => REGION31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION31_A::ENABLED
    }
}
impl core::ops::Deref for REGION31_R {
    type Target = crate::FieldReader<bool, REGION31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION31` writer - Enable protection for region 31. Write '0' has no effect."]
pub struct REGION31_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION31_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION31_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&self) -> REGION0_R {
        REGION0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&self) -> REGION1_R {
        REGION1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&self) -> REGION2_R {
        REGION2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&self) -> REGION3_R {
        REGION3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&self) -> REGION4_R {
        REGION4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&self) -> REGION5_R {
        REGION5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&self) -> REGION6_R {
        REGION6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&self) -> REGION7_R {
        REGION7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&self) -> REGION8_R {
        REGION8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&self) -> REGION9_R {
        REGION9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&self) -> REGION10_R {
        REGION10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&self) -> REGION11_R {
        REGION11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&self) -> REGION12_R {
        REGION12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&self) -> REGION13_R {
        REGION13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&self) -> REGION14_R {
        REGION14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&self) -> REGION15_R {
        REGION15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&self) -> REGION16_R {
        REGION16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&self) -> REGION17_R {
        REGION17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&self) -> REGION18_R {
        REGION18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&self) -> REGION19_R {
        REGION19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&self) -> REGION20_R {
        REGION20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&self) -> REGION21_R {
        REGION21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&self) -> REGION22_R {
        REGION22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&self) -> REGION23_R {
        REGION23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&self) -> REGION24_R {
        REGION24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&self) -> REGION25_R {
        REGION25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&self) -> REGION26_R {
        REGION26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&self) -> REGION27_R {
        REGION27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&self) -> REGION28_R {
        REGION28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&self) -> REGION29_R {
        REGION29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&self) -> REGION30_R {
        REGION30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&self) -> REGION31_R {
        REGION31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&mut self) -> REGION0_W {
        REGION0_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&mut self) -> REGION1_W {
        REGION1_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&mut self) -> REGION2_W {
        REGION2_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&mut self) -> REGION3_W {
        REGION3_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&mut self) -> REGION4_W {
        REGION4_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&mut self) -> REGION5_W {
        REGION5_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&mut self) -> REGION6_W {
        REGION6_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&mut self) -> REGION7_W {
        REGION7_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&mut self) -> REGION8_W {
        REGION8_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&mut self) -> REGION9_W {
        REGION9_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&mut self) -> REGION10_W {
        REGION10_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&mut self) -> REGION11_W {
        REGION11_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&mut self) -> REGION12_W {
        REGION12_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&mut self) -> REGION13_W {
        REGION13_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&mut self) -> REGION14_W {
        REGION14_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&mut self) -> REGION15_W {
        REGION15_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&mut self) -> REGION16_W {
        REGION16_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&mut self) -> REGION17_W {
        REGION17_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&mut self) -> REGION18_W {
        REGION18_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&mut self) -> REGION19_W {
        REGION19_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&mut self) -> REGION20_W {
        REGION20_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&mut self) -> REGION21_W {
        REGION21_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&mut self) -> REGION22_W {
        REGION22_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&mut self) -> REGION23_W {
        REGION23_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&mut self) -> REGION24_W {
        REGION24_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&mut self) -> REGION25_W {
        REGION25_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&mut self) -> REGION26_W {
        REGION26_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&mut self) -> REGION27_W {
        REGION27_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&mut self) -> REGION28_W {
        REGION28_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&mut self) -> REGION29_W {
        REGION29_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&mut self) -> REGION30_W {
        REGION30_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&mut self) -> REGION31_W {
        REGION31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config0](index.html) module"]
pub struct CONFIG0_SPEC;
impl crate::RegisterSpec for CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config0::R](R) reader structure"]
impl crate::Readable for CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config0::W](W) writer structure"]
impl crate::Writable for CONFIG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG0 to value 0"]
impl crate::Resettable for CONFIG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
