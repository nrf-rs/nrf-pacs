#[doc = "Register `CONFIG2` reader"]
pub struct R(crate::R<CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG2` writer"]
pub struct W(crate::W<CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG2_SPEC>;
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
impl From<crate::W<CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable protection for region 64. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION64_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION64_A> for bool {
    #[inline(always)]
    fn from(variant: REGION64_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION64` reader - Enable protection for region 64. Write '0' has no effect."]
pub struct REGION64_R(crate::FieldReader<bool, REGION64_A>);
impl REGION64_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION64_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION64_A {
        match self.bits {
            false => REGION64_A::DISABLED,
            true => REGION64_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION64_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION64_A::ENABLED
    }
}
impl core::ops::Deref for REGION64_R {
    type Target = crate::FieldReader<bool, REGION64_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION64` writer - Enable protection for region 64. Write '0' has no effect."]
pub struct REGION64_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION64_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION64_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION64_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION64_A::ENABLED)
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
#[doc = "Enable protection for region 65. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION65_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION65_A> for bool {
    #[inline(always)]
    fn from(variant: REGION65_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION65` reader - Enable protection for region 65. Write '0' has no effect."]
pub struct REGION65_R(crate::FieldReader<bool, REGION65_A>);
impl REGION65_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION65_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION65_A {
        match self.bits {
            false => REGION65_A::DISABLED,
            true => REGION65_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION65_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION65_A::ENABLED
    }
}
impl core::ops::Deref for REGION65_R {
    type Target = crate::FieldReader<bool, REGION65_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION65` writer - Enable protection for region 65. Write '0' has no effect."]
pub struct REGION65_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION65_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION65_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION65_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION65_A::ENABLED)
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
#[doc = "Enable protection for region 66. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION66_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION66_A> for bool {
    #[inline(always)]
    fn from(variant: REGION66_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION66` reader - Enable protection for region 66. Write '0' has no effect."]
pub struct REGION66_R(crate::FieldReader<bool, REGION66_A>);
impl REGION66_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION66_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION66_A {
        match self.bits {
            false => REGION66_A::DISABLED,
            true => REGION66_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION66_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION66_A::ENABLED
    }
}
impl core::ops::Deref for REGION66_R {
    type Target = crate::FieldReader<bool, REGION66_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION66` writer - Enable protection for region 66. Write '0' has no effect."]
pub struct REGION66_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION66_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION66_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION66_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION66_A::ENABLED)
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
#[doc = "Enable protection for region 67. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION67_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION67_A> for bool {
    #[inline(always)]
    fn from(variant: REGION67_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION67` reader - Enable protection for region 67. Write '0' has no effect."]
pub struct REGION67_R(crate::FieldReader<bool, REGION67_A>);
impl REGION67_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION67_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION67_A {
        match self.bits {
            false => REGION67_A::DISABLED,
            true => REGION67_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION67_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION67_A::ENABLED
    }
}
impl core::ops::Deref for REGION67_R {
    type Target = crate::FieldReader<bool, REGION67_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION67` writer - Enable protection for region 67. Write '0' has no effect."]
pub struct REGION67_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION67_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION67_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION67_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION67_A::ENABLED)
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
#[doc = "Enable protection for region 68. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION68_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION68_A> for bool {
    #[inline(always)]
    fn from(variant: REGION68_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION68` reader - Enable protection for region 68. Write '0' has no effect."]
pub struct REGION68_R(crate::FieldReader<bool, REGION68_A>);
impl REGION68_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION68_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION68_A {
        match self.bits {
            false => REGION68_A::DISABLED,
            true => REGION68_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION68_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION68_A::ENABLED
    }
}
impl core::ops::Deref for REGION68_R {
    type Target = crate::FieldReader<bool, REGION68_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION68` writer - Enable protection for region 68. Write '0' has no effect."]
pub struct REGION68_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION68_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION68_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION68_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION68_A::ENABLED)
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
#[doc = "Enable protection for region 69. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION69_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION69_A> for bool {
    #[inline(always)]
    fn from(variant: REGION69_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION69` reader - Enable protection for region 69. Write '0' has no effect."]
pub struct REGION69_R(crate::FieldReader<bool, REGION69_A>);
impl REGION69_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION69_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION69_A {
        match self.bits {
            false => REGION69_A::DISABLED,
            true => REGION69_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION69_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION69_A::ENABLED
    }
}
impl core::ops::Deref for REGION69_R {
    type Target = crate::FieldReader<bool, REGION69_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION69` writer - Enable protection for region 69. Write '0' has no effect."]
pub struct REGION69_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION69_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION69_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION69_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION69_A::ENABLED)
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
#[doc = "Enable protection for region 70. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION70_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION70_A> for bool {
    #[inline(always)]
    fn from(variant: REGION70_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION70` reader - Enable protection for region 70. Write '0' has no effect."]
pub struct REGION70_R(crate::FieldReader<bool, REGION70_A>);
impl REGION70_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION70_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION70_A {
        match self.bits {
            false => REGION70_A::DISABLED,
            true => REGION70_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION70_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION70_A::ENABLED
    }
}
impl core::ops::Deref for REGION70_R {
    type Target = crate::FieldReader<bool, REGION70_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION70` writer - Enable protection for region 70. Write '0' has no effect."]
pub struct REGION70_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION70_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION70_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION70_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION70_A::ENABLED)
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
#[doc = "Enable protection for region 71. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION71_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION71_A> for bool {
    #[inline(always)]
    fn from(variant: REGION71_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION71` reader - Enable protection for region 71. Write '0' has no effect."]
pub struct REGION71_R(crate::FieldReader<bool, REGION71_A>);
impl REGION71_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION71_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION71_A {
        match self.bits {
            false => REGION71_A::DISABLED,
            true => REGION71_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION71_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION71_A::ENABLED
    }
}
impl core::ops::Deref for REGION71_R {
    type Target = crate::FieldReader<bool, REGION71_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION71` writer - Enable protection for region 71. Write '0' has no effect."]
pub struct REGION71_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION71_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION71_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION71_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION71_A::ENABLED)
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
#[doc = "Enable protection for region 72. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION72_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION72_A> for bool {
    #[inline(always)]
    fn from(variant: REGION72_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION72` reader - Enable protection for region 72. Write '0' has no effect."]
pub struct REGION72_R(crate::FieldReader<bool, REGION72_A>);
impl REGION72_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION72_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION72_A {
        match self.bits {
            false => REGION72_A::DISABLED,
            true => REGION72_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION72_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION72_A::ENABLED
    }
}
impl core::ops::Deref for REGION72_R {
    type Target = crate::FieldReader<bool, REGION72_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION72` writer - Enable protection for region 72. Write '0' has no effect."]
pub struct REGION72_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION72_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION72_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION72_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION72_A::ENABLED)
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
#[doc = "Enable protection for region 73. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION73_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION73_A> for bool {
    #[inline(always)]
    fn from(variant: REGION73_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION73` reader - Enable protection for region 73. Write '0' has no effect."]
pub struct REGION73_R(crate::FieldReader<bool, REGION73_A>);
impl REGION73_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION73_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION73_A {
        match self.bits {
            false => REGION73_A::DISABLED,
            true => REGION73_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION73_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION73_A::ENABLED
    }
}
impl core::ops::Deref for REGION73_R {
    type Target = crate::FieldReader<bool, REGION73_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION73` writer - Enable protection for region 73. Write '0' has no effect."]
pub struct REGION73_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION73_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION73_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION73_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION73_A::ENABLED)
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
#[doc = "Enable protection for region 74. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION74_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION74_A> for bool {
    #[inline(always)]
    fn from(variant: REGION74_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION74` reader - Enable protection for region 74. Write '0' has no effect."]
pub struct REGION74_R(crate::FieldReader<bool, REGION74_A>);
impl REGION74_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION74_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION74_A {
        match self.bits {
            false => REGION74_A::DISABLED,
            true => REGION74_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION74_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION74_A::ENABLED
    }
}
impl core::ops::Deref for REGION74_R {
    type Target = crate::FieldReader<bool, REGION74_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION74` writer - Enable protection for region 74. Write '0' has no effect."]
pub struct REGION74_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION74_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION74_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION74_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION74_A::ENABLED)
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
#[doc = "Enable protection for region 75. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION75_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION75_A> for bool {
    #[inline(always)]
    fn from(variant: REGION75_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION75` reader - Enable protection for region 75. Write '0' has no effect."]
pub struct REGION75_R(crate::FieldReader<bool, REGION75_A>);
impl REGION75_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION75_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION75_A {
        match self.bits {
            false => REGION75_A::DISABLED,
            true => REGION75_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION75_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION75_A::ENABLED
    }
}
impl core::ops::Deref for REGION75_R {
    type Target = crate::FieldReader<bool, REGION75_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION75` writer - Enable protection for region 75. Write '0' has no effect."]
pub struct REGION75_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION75_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION75_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION75_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION75_A::ENABLED)
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
#[doc = "Enable protection for region 76. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION76_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION76_A> for bool {
    #[inline(always)]
    fn from(variant: REGION76_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION76` reader - Enable protection for region 76. Write '0' has no effect."]
pub struct REGION76_R(crate::FieldReader<bool, REGION76_A>);
impl REGION76_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION76_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION76_A {
        match self.bits {
            false => REGION76_A::DISABLED,
            true => REGION76_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION76_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION76_A::ENABLED
    }
}
impl core::ops::Deref for REGION76_R {
    type Target = crate::FieldReader<bool, REGION76_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION76` writer - Enable protection for region 76. Write '0' has no effect."]
pub struct REGION76_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION76_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION76_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION76_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION76_A::ENABLED)
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
#[doc = "Enable protection for region 77. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION77_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION77_A> for bool {
    #[inline(always)]
    fn from(variant: REGION77_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION77` reader - Enable protection for region 77. Write '0' has no effect."]
pub struct REGION77_R(crate::FieldReader<bool, REGION77_A>);
impl REGION77_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION77_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION77_A {
        match self.bits {
            false => REGION77_A::DISABLED,
            true => REGION77_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION77_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION77_A::ENABLED
    }
}
impl core::ops::Deref for REGION77_R {
    type Target = crate::FieldReader<bool, REGION77_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION77` writer - Enable protection for region 77. Write '0' has no effect."]
pub struct REGION77_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION77_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION77_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION77_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION77_A::ENABLED)
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
#[doc = "Enable protection for region 78. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION78_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION78_A> for bool {
    #[inline(always)]
    fn from(variant: REGION78_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION78` reader - Enable protection for region 78. Write '0' has no effect."]
pub struct REGION78_R(crate::FieldReader<bool, REGION78_A>);
impl REGION78_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION78_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION78_A {
        match self.bits {
            false => REGION78_A::DISABLED,
            true => REGION78_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION78_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION78_A::ENABLED
    }
}
impl core::ops::Deref for REGION78_R {
    type Target = crate::FieldReader<bool, REGION78_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION78` writer - Enable protection for region 78. Write '0' has no effect."]
pub struct REGION78_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION78_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION78_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION78_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION78_A::ENABLED)
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
#[doc = "Enable protection for region 79. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION79_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION79_A> for bool {
    #[inline(always)]
    fn from(variant: REGION79_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION79` reader - Enable protection for region 79. Write '0' has no effect."]
pub struct REGION79_R(crate::FieldReader<bool, REGION79_A>);
impl REGION79_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION79_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION79_A {
        match self.bits {
            false => REGION79_A::DISABLED,
            true => REGION79_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION79_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION79_A::ENABLED
    }
}
impl core::ops::Deref for REGION79_R {
    type Target = crate::FieldReader<bool, REGION79_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION79` writer - Enable protection for region 79. Write '0' has no effect."]
pub struct REGION79_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION79_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION79_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION79_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION79_A::ENABLED)
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
#[doc = "Enable protection for region 80. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION80_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION80_A> for bool {
    #[inline(always)]
    fn from(variant: REGION80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION80` reader - Enable protection for region 80. Write '0' has no effect."]
pub struct REGION80_R(crate::FieldReader<bool, REGION80_A>);
impl REGION80_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION80_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION80_A {
        match self.bits {
            false => REGION80_A::DISABLED,
            true => REGION80_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION80_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION80_A::ENABLED
    }
}
impl core::ops::Deref for REGION80_R {
    type Target = crate::FieldReader<bool, REGION80_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION80` writer - Enable protection for region 80. Write '0' has no effect."]
pub struct REGION80_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION80_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION80_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION80_A::ENABLED)
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
#[doc = "Enable protection for region 81. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION81_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION81_A> for bool {
    #[inline(always)]
    fn from(variant: REGION81_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION81` reader - Enable protection for region 81. Write '0' has no effect."]
pub struct REGION81_R(crate::FieldReader<bool, REGION81_A>);
impl REGION81_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION81_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION81_A {
        match self.bits {
            false => REGION81_A::DISABLED,
            true => REGION81_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION81_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION81_A::ENABLED
    }
}
impl core::ops::Deref for REGION81_R {
    type Target = crate::FieldReader<bool, REGION81_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION81` writer - Enable protection for region 81. Write '0' has no effect."]
pub struct REGION81_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION81_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION81_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION81_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION81_A::ENABLED)
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
#[doc = "Enable protection for region 82. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION82_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION82_A> for bool {
    #[inline(always)]
    fn from(variant: REGION82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION82` reader - Enable protection for region 82. Write '0' has no effect."]
pub struct REGION82_R(crate::FieldReader<bool, REGION82_A>);
impl REGION82_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION82_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION82_A {
        match self.bits {
            false => REGION82_A::DISABLED,
            true => REGION82_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION82_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION82_A::ENABLED
    }
}
impl core::ops::Deref for REGION82_R {
    type Target = crate::FieldReader<bool, REGION82_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION82` writer - Enable protection for region 82. Write '0' has no effect."]
pub struct REGION82_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION82_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION82_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION82_A::ENABLED)
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
#[doc = "Enable protection for region 83. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION83_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION83_A> for bool {
    #[inline(always)]
    fn from(variant: REGION83_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION83` reader - Enable protection for region 83. Write '0' has no effect."]
pub struct REGION83_R(crate::FieldReader<bool, REGION83_A>);
impl REGION83_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION83_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION83_A {
        match self.bits {
            false => REGION83_A::DISABLED,
            true => REGION83_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION83_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION83_A::ENABLED
    }
}
impl core::ops::Deref for REGION83_R {
    type Target = crate::FieldReader<bool, REGION83_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION83` writer - Enable protection for region 83. Write '0' has no effect."]
pub struct REGION83_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION83_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION83_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION83_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION83_A::ENABLED)
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
#[doc = "Enable protection for region 84. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION84_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION84_A> for bool {
    #[inline(always)]
    fn from(variant: REGION84_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION84` reader - Enable protection for region 84. Write '0' has no effect."]
pub struct REGION84_R(crate::FieldReader<bool, REGION84_A>);
impl REGION84_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION84_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION84_A {
        match self.bits {
            false => REGION84_A::DISABLED,
            true => REGION84_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION84_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION84_A::ENABLED
    }
}
impl core::ops::Deref for REGION84_R {
    type Target = crate::FieldReader<bool, REGION84_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION84` writer - Enable protection for region 84. Write '0' has no effect."]
pub struct REGION84_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION84_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION84_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION84_A::ENABLED)
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
#[doc = "Enable protection for region 85. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION85_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION85_A> for bool {
    #[inline(always)]
    fn from(variant: REGION85_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION85` reader - Enable protection for region 85. Write '0' has no effect."]
pub struct REGION85_R(crate::FieldReader<bool, REGION85_A>);
impl REGION85_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION85_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION85_A {
        match self.bits {
            false => REGION85_A::DISABLED,
            true => REGION85_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION85_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION85_A::ENABLED
    }
}
impl core::ops::Deref for REGION85_R {
    type Target = crate::FieldReader<bool, REGION85_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION85` writer - Enable protection for region 85. Write '0' has no effect."]
pub struct REGION85_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION85_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION85_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION85_A::ENABLED)
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
#[doc = "Enable protection for region 86. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION86_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION86_A> for bool {
    #[inline(always)]
    fn from(variant: REGION86_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION86` reader - Enable protection for region 86. Write '0' has no effect."]
pub struct REGION86_R(crate::FieldReader<bool, REGION86_A>);
impl REGION86_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION86_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION86_A {
        match self.bits {
            false => REGION86_A::DISABLED,
            true => REGION86_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION86_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION86_A::ENABLED
    }
}
impl core::ops::Deref for REGION86_R {
    type Target = crate::FieldReader<bool, REGION86_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION86` writer - Enable protection for region 86. Write '0' has no effect."]
pub struct REGION86_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION86_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION86_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION86_A::ENABLED)
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
#[doc = "Enable protection for region 87. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION87_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION87_A> for bool {
    #[inline(always)]
    fn from(variant: REGION87_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION87` reader - Enable protection for region 87. Write '0' has no effect."]
pub struct REGION87_R(crate::FieldReader<bool, REGION87_A>);
impl REGION87_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION87_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION87_A {
        match self.bits {
            false => REGION87_A::DISABLED,
            true => REGION87_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION87_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION87_A::ENABLED
    }
}
impl core::ops::Deref for REGION87_R {
    type Target = crate::FieldReader<bool, REGION87_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION87` writer - Enable protection for region 87. Write '0' has no effect."]
pub struct REGION87_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION87_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION87_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION87_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION87_A::ENABLED)
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
#[doc = "Enable protection for region 88. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION88_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION88_A> for bool {
    #[inline(always)]
    fn from(variant: REGION88_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION88` reader - Enable protection for region 88. Write '0' has no effect."]
pub struct REGION88_R(crate::FieldReader<bool, REGION88_A>);
impl REGION88_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION88_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION88_A {
        match self.bits {
            false => REGION88_A::DISABLED,
            true => REGION88_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION88_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION88_A::ENABLED
    }
}
impl core::ops::Deref for REGION88_R {
    type Target = crate::FieldReader<bool, REGION88_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION88` writer - Enable protection for region 88. Write '0' has no effect."]
pub struct REGION88_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION88_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION88_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION88_A::ENABLED)
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
#[doc = "Enable protection for region 89. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION89_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION89_A> for bool {
    #[inline(always)]
    fn from(variant: REGION89_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION89` reader - Enable protection for region 89. Write '0' has no effect."]
pub struct REGION89_R(crate::FieldReader<bool, REGION89_A>);
impl REGION89_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION89_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION89_A {
        match self.bits {
            false => REGION89_A::DISABLED,
            true => REGION89_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION89_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION89_A::ENABLED
    }
}
impl core::ops::Deref for REGION89_R {
    type Target = crate::FieldReader<bool, REGION89_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION89` writer - Enable protection for region 89. Write '0' has no effect."]
pub struct REGION89_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION89_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION89_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION89_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION89_A::ENABLED)
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
#[doc = "Enable protection for region 90. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION90_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION90_A> for bool {
    #[inline(always)]
    fn from(variant: REGION90_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION90` reader - Enable protection for region 90. Write '0' has no effect."]
pub struct REGION90_R(crate::FieldReader<bool, REGION90_A>);
impl REGION90_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION90_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION90_A {
        match self.bits {
            false => REGION90_A::DISABLED,
            true => REGION90_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION90_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION90_A::ENABLED
    }
}
impl core::ops::Deref for REGION90_R {
    type Target = crate::FieldReader<bool, REGION90_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION90` writer - Enable protection for region 90. Write '0' has no effect."]
pub struct REGION90_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION90_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION90_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION90_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION90_A::ENABLED)
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
#[doc = "Enable protection for region 91. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION91_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION91_A> for bool {
    #[inline(always)]
    fn from(variant: REGION91_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION91` reader - Enable protection for region 91. Write '0' has no effect."]
pub struct REGION91_R(crate::FieldReader<bool, REGION91_A>);
impl REGION91_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION91_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION91_A {
        match self.bits {
            false => REGION91_A::DISABLED,
            true => REGION91_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION91_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION91_A::ENABLED
    }
}
impl core::ops::Deref for REGION91_R {
    type Target = crate::FieldReader<bool, REGION91_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION91` writer - Enable protection for region 91. Write '0' has no effect."]
pub struct REGION91_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION91_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION91_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION91_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION91_A::ENABLED)
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
#[doc = "Enable protection for region 92. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION92_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION92_A> for bool {
    #[inline(always)]
    fn from(variant: REGION92_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION92` reader - Enable protection for region 92. Write '0' has no effect."]
pub struct REGION92_R(crate::FieldReader<bool, REGION92_A>);
impl REGION92_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION92_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION92_A {
        match self.bits {
            false => REGION92_A::DISABLED,
            true => REGION92_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION92_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION92_A::ENABLED
    }
}
impl core::ops::Deref for REGION92_R {
    type Target = crate::FieldReader<bool, REGION92_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION92` writer - Enable protection for region 92. Write '0' has no effect."]
pub struct REGION92_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION92_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION92_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION92_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION92_A::ENABLED)
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
#[doc = "Enable protection for region 93. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION93_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION93_A> for bool {
    #[inline(always)]
    fn from(variant: REGION93_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION93` reader - Enable protection for region 93. Write '0' has no effect."]
pub struct REGION93_R(crate::FieldReader<bool, REGION93_A>);
impl REGION93_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION93_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION93_A {
        match self.bits {
            false => REGION93_A::DISABLED,
            true => REGION93_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION93_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION93_A::ENABLED
    }
}
impl core::ops::Deref for REGION93_R {
    type Target = crate::FieldReader<bool, REGION93_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION93` writer - Enable protection for region 93. Write '0' has no effect."]
pub struct REGION93_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION93_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION93_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION93_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION93_A::ENABLED)
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
#[doc = "Enable protection for region 94. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION94_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION94_A> for bool {
    #[inline(always)]
    fn from(variant: REGION94_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION94` reader - Enable protection for region 94. Write '0' has no effect."]
pub struct REGION94_R(crate::FieldReader<bool, REGION94_A>);
impl REGION94_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION94_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION94_A {
        match self.bits {
            false => REGION94_A::DISABLED,
            true => REGION94_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION94_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION94_A::ENABLED
    }
}
impl core::ops::Deref for REGION94_R {
    type Target = crate::FieldReader<bool, REGION94_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION94` writer - Enable protection for region 94. Write '0' has no effect."]
pub struct REGION94_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION94_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION94_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION94_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION94_A::ENABLED)
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
#[doc = "Enable protection for region 95. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION95_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION95_A> for bool {
    #[inline(always)]
    fn from(variant: REGION95_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION95` reader - Enable protection for region 95. Write '0' has no effect."]
pub struct REGION95_R(crate::FieldReader<bool, REGION95_A>);
impl REGION95_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION95_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION95_A {
        match self.bits {
            false => REGION95_A::DISABLED,
            true => REGION95_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION95_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION95_A::ENABLED
    }
}
impl core::ops::Deref for REGION95_R {
    type Target = crate::FieldReader<bool, REGION95_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION95` writer - Enable protection for region 95. Write '0' has no effect."]
pub struct REGION95_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION95_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION95_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION95_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION95_A::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&self) -> REGION64_R {
        REGION64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&self) -> REGION65_R {
        REGION65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&self) -> REGION66_R {
        REGION66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&self) -> REGION67_R {
        REGION67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&self) -> REGION68_R {
        REGION68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&self) -> REGION69_R {
        REGION69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&self) -> REGION70_R {
        REGION70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&self) -> REGION71_R {
        REGION71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&self) -> REGION72_R {
        REGION72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&self) -> REGION73_R {
        REGION73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&self) -> REGION74_R {
        REGION74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&self) -> REGION75_R {
        REGION75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&self) -> REGION76_R {
        REGION76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&self) -> REGION77_R {
        REGION77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&self) -> REGION78_R {
        REGION78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&self) -> REGION79_R {
        REGION79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&self) -> REGION80_R {
        REGION80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&self) -> REGION81_R {
        REGION81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&self) -> REGION82_R {
        REGION82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&self) -> REGION83_R {
        REGION83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&self) -> REGION84_R {
        REGION84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&self) -> REGION85_R {
        REGION85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&self) -> REGION86_R {
        REGION86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&self) -> REGION87_R {
        REGION87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&self) -> REGION88_R {
        REGION88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&self) -> REGION89_R {
        REGION89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&self) -> REGION90_R {
        REGION90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&self) -> REGION91_R {
        REGION91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&self) -> REGION92_R {
        REGION92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&self) -> REGION93_R {
        REGION93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&self) -> REGION94_R {
        REGION94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&self) -> REGION95_R {
        REGION95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&mut self) -> REGION64_W {
        REGION64_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&mut self) -> REGION65_W {
        REGION65_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&mut self) -> REGION66_W {
        REGION66_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&mut self) -> REGION67_W {
        REGION67_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&mut self) -> REGION68_W {
        REGION68_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&mut self) -> REGION69_W {
        REGION69_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&mut self) -> REGION70_W {
        REGION70_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&mut self) -> REGION71_W {
        REGION71_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&mut self) -> REGION72_W {
        REGION72_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&mut self) -> REGION73_W {
        REGION73_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&mut self) -> REGION74_W {
        REGION74_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&mut self) -> REGION75_W {
        REGION75_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&mut self) -> REGION76_W {
        REGION76_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&mut self) -> REGION77_W {
        REGION77_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&mut self) -> REGION78_W {
        REGION78_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&mut self) -> REGION79_W {
        REGION79_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&mut self) -> REGION80_W {
        REGION80_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&mut self) -> REGION81_W {
        REGION81_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&mut self) -> REGION82_W {
        REGION82_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&mut self) -> REGION83_W {
        REGION83_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&mut self) -> REGION84_W {
        REGION84_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&mut self) -> REGION85_W {
        REGION85_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&mut self) -> REGION86_W {
        REGION86_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&mut self) -> REGION87_W {
        REGION87_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&mut self) -> REGION88_W {
        REGION88_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&mut self) -> REGION89_W {
        REGION89_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&mut self) -> REGION90_W {
        REGION90_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&mut self) -> REGION91_W {
        REGION91_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&mut self) -> REGION92_W {
        REGION92_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&mut self) -> REGION93_W {
        REGION93_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&mut self) -> REGION94_W {
        REGION94_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&mut self) -> REGION95_W {
        REGION95_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config2](index.html) module"]
pub struct CONFIG2_SPEC;
impl crate::RegisterSpec for CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config2::R](R) reader structure"]
impl crate::Readable for CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config2::W](W) writer structure"]
impl crate::Writable for CONFIG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for CONFIG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
