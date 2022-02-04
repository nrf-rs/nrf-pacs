#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between event REPORTRDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_READCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` reader - Shortcut between event REPORTRDY and task READCLRACC"]
pub struct REPORTRDY_READCLRACC_R(crate::FieldReader<bool, REPORTRDY_READCLRACC_A>);
impl REPORTRDY_READCLRACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REPORTRDY_READCLRACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_READCLRACC_A {
        match self.bits {
            false => REPORTRDY_READCLRACC_A::DISABLED,
            true => REPORTRDY_READCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REPORTRDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REPORTRDY_READCLRACC_A::ENABLED
    }
}
impl core::ops::Deref for REPORTRDY_READCLRACC_R {
    type Target = crate::FieldReader<bool, REPORTRDY_READCLRACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` writer - Shortcut between event REPORTRDY and task READCLRACC"]
pub struct REPORTRDY_READCLRACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_READCLRACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_READCLRACC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::ENABLED)
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
#[doc = "Shortcut between event SAMPLERDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SAMPLERDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY_STOP` reader - Shortcut between event SAMPLERDY and task STOP"]
pub struct SAMPLERDY_STOP_R(crate::FieldReader<bool, SAMPLERDY_STOP_A>);
impl SAMPLERDY_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLERDY_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_STOP_A {
        match self.bits {
            false => SAMPLERDY_STOP_A::DISABLED,
            true => SAMPLERDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SAMPLERDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SAMPLERDY_STOP_A::ENABLED
    }
}
impl core::ops::Deref for SAMPLERDY_STOP_R {
    type Target = crate::FieldReader<bool, SAMPLERDY_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLERDY_STOP` writer - Shortcut between event SAMPLERDY and task STOP"]
pub struct SAMPLERDY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::ENABLED)
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
#[doc = "Shortcut between event REPORTRDY and task RDCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_RDCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_RDCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_RDCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` reader - Shortcut between event REPORTRDY and task RDCLRACC"]
pub struct REPORTRDY_RDCLRACC_R(crate::FieldReader<bool, REPORTRDY_RDCLRACC_A>);
impl REPORTRDY_RDCLRACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REPORTRDY_RDCLRACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_RDCLRACC_A {
        match self.bits {
            false => REPORTRDY_RDCLRACC_A::DISABLED,
            true => REPORTRDY_RDCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REPORTRDY_RDCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REPORTRDY_RDCLRACC_A::ENABLED
    }
}
impl core::ops::Deref for REPORTRDY_RDCLRACC_R {
    type Target = crate::FieldReader<bool, REPORTRDY_RDCLRACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` writer - Shortcut between event REPORTRDY and task RDCLRACC"]
pub struct REPORTRDY_RDCLRACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_RDCLRACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_RDCLRACC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_RDCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_RDCLRACC_A::ENABLED)
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
#[doc = "Shortcut between event REPORTRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_STOP` reader - Shortcut between event REPORTRDY and task STOP"]
pub struct REPORTRDY_STOP_R(crate::FieldReader<bool, REPORTRDY_STOP_A>);
impl REPORTRDY_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REPORTRDY_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_STOP_A {
        match self.bits {
            false => REPORTRDY_STOP_A::DISABLED,
            true => REPORTRDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REPORTRDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REPORTRDY_STOP_A::ENABLED
    }
}
impl core::ops::Deref for REPORTRDY_STOP_R {
    type Target = crate::FieldReader<bool, REPORTRDY_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPORTRDY_STOP` writer - Shortcut between event REPORTRDY and task STOP"]
pub struct REPORTRDY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_STOP_A::ENABLED)
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
#[doc = "Shortcut between event DBLRDY and task RDCLRDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLRDY_RDCLRDBL_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DBLRDY_RDCLRDBL_A> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_RDCLRDBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` reader - Shortcut between event DBLRDY and task RDCLRDBL"]
pub struct DBLRDY_RDCLRDBL_R(crate::FieldReader<bool, DBLRDY_RDCLRDBL_A>);
impl DBLRDY_RDCLRDBL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBLRDY_RDCLRDBL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLRDY_RDCLRDBL_A {
        match self.bits {
            false => DBLRDY_RDCLRDBL_A::DISABLED,
            true => DBLRDY_RDCLRDBL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBLRDY_RDCLRDBL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBLRDY_RDCLRDBL_A::ENABLED
    }
}
impl core::ops::Deref for DBLRDY_RDCLRDBL_R {
    type Target = crate::FieldReader<bool, DBLRDY_RDCLRDBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` writer - Shortcut between event DBLRDY and task RDCLRDBL"]
pub struct DBLRDY_RDCLRDBL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLRDY_RDCLRDBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLRDY_RDCLRDBL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLRDY_RDCLRDBL_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLRDY_RDCLRDBL_A::ENABLED)
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
#[doc = "Shortcut between event DBLRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLRDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DBLRDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY_STOP` reader - Shortcut between event DBLRDY and task STOP"]
pub struct DBLRDY_STOP_R(crate::FieldReader<bool, DBLRDY_STOP_A>);
impl DBLRDY_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBLRDY_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLRDY_STOP_A {
        match self.bits {
            false => DBLRDY_STOP_A::DISABLED,
            true => DBLRDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBLRDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBLRDY_STOP_A::ENABLED
    }
}
impl core::ops::Deref for DBLRDY_STOP_R {
    type Target = crate::FieldReader<bool, DBLRDY_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBLRDY_STOP` writer - Shortcut between event DBLRDY and task STOP"]
pub struct DBLRDY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLRDY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLRDY_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLRDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLRDY_STOP_A::ENABLED)
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
#[doc = "Shortcut between event SAMPLERDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_READCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SAMPLERDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` reader - Shortcut between event SAMPLERDY and task READCLRACC"]
pub struct SAMPLERDY_READCLRACC_R(crate::FieldReader<bool, SAMPLERDY_READCLRACC_A>);
impl SAMPLERDY_READCLRACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLERDY_READCLRACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_READCLRACC_A {
        match self.bits {
            false => SAMPLERDY_READCLRACC_A::DISABLED,
            true => SAMPLERDY_READCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SAMPLERDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SAMPLERDY_READCLRACC_A::ENABLED
    }
}
impl core::ops::Deref for SAMPLERDY_READCLRACC_R {
    type Target = crate::FieldReader<bool, SAMPLERDY_READCLRACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` writer - Shortcut between event SAMPLERDY and task READCLRACC"]
pub struct SAMPLERDY_READCLRACC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_READCLRACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_READCLRACC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_READCLRACC_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> REPORTRDY_READCLRACC_R {
        REPORTRDY_READCLRACC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SAMPLERDY_STOP_R {
        SAMPLERDY_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn reportrdy_rdclracc(&self) -> REPORTRDY_RDCLRACC_R {
        REPORTRDY_RDCLRACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn reportrdy_stop(&self) -> REPORTRDY_STOP_R {
        REPORTRDY_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn dblrdy_rdclrdbl(&self) -> DBLRDY_RDCLRDBL_R {
        DBLRDY_RDCLRDBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn dblrdy_stop(&self) -> DBLRDY_STOP_R {
        DBLRDY_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn samplerdy_readclracc(&self) -> SAMPLERDY_READCLRACC_R {
        SAMPLERDY_READCLRACC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn reportrdy_readclracc(&mut self) -> REPORTRDY_READCLRACC_W {
        REPORTRDY_READCLRACC_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn samplerdy_stop(&mut self) -> SAMPLERDY_STOP_W {
        SAMPLERDY_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn reportrdy_rdclracc(&mut self) -> REPORTRDY_RDCLRACC_W {
        REPORTRDY_RDCLRACC_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn reportrdy_stop(&mut self) -> REPORTRDY_STOP_W {
        REPORTRDY_STOP_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn dblrdy_rdclrdbl(&mut self) -> DBLRDY_RDCLRDBL_W {
        DBLRDY_RDCLRDBL_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn dblrdy_stop(&mut self) -> DBLRDY_STOP_W {
        DBLRDY_STOP_W { w: self }
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn samplerdy_readclracc(&mut self) -> SAMPLERDY_READCLRACC_W {
        SAMPLERDY_READCLRACC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
