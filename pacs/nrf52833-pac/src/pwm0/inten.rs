#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Enable or disable interrupt for event STOPPED"]
pub struct STOPPED_R(crate::FieldReader<bool, STOPPED_A>);
impl STOPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPPED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STOPPED_A::ENABLED
    }
}
impl core::ops::Deref for STOPPED_R {
    type Target = crate::FieldReader<bool, STOPPED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPPED` writer - Enable or disable interrupt for event STOPPED"]
pub struct STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPPED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPPED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPPED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SEQSTARTED0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` reader - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub struct SEQSTARTED0_R(crate::FieldReader<bool, SEQSTARTED0_A>);
impl SEQSTARTED0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQSTARTED0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED0_A {
        match self.bits {
            false => SEQSTARTED0_A::DISABLED,
            true => SEQSTARTED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQSTARTED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQSTARTED0_A::ENABLED
    }
}
impl core::ops::Deref for SEQSTARTED0_R {
    type Target = crate::FieldReader<bool, SEQSTARTED0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQSTARTED0` writer - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub struct SEQSTARTED0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTARTED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQSTARTED0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQSTARTED0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQSTARTED0_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SEQSTARTED1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` reader - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub struct SEQSTARTED1_R(crate::FieldReader<bool, SEQSTARTED1_A>);
impl SEQSTARTED1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQSTARTED1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED1_A {
        match self.bits {
            false => SEQSTARTED1_A::DISABLED,
            true => SEQSTARTED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQSTARTED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQSTARTED1_A::ENABLED
    }
}
impl core::ops::Deref for SEQSTARTED1_R {
    type Target = crate::FieldReader<bool, SEQSTARTED1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQSTARTED1` writer - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub struct SEQSTARTED1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTARTED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQSTARTED1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQSTARTED1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQSTARTED1_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SEQEND0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` reader - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub struct SEQEND0_R(crate::FieldReader<bool, SEQEND0_A>);
impl SEQEND0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQEND0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_A {
        match self.bits {
            false => SEQEND0_A::DISABLED,
            true => SEQEND0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQEND0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQEND0_A::ENABLED
    }
}
impl core::ops::Deref for SEQEND0_R {
    type Target = crate::FieldReader<bool, SEQEND0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQEND0` writer - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub struct SEQEND0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND0_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SEQEND1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` reader - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub struct SEQEND1_R(crate::FieldReader<bool, SEQEND1_A>);
impl SEQEND1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQEND1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_A {
        match self.bits {
            false => SEQEND1_A::DISABLED,
            true => SEQEND1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQEND1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQEND1_A::ENABLED
    }
}
impl core::ops::Deref for SEQEND1_R {
    type Target = crate::FieldReader<bool, SEQEND1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQEND1` writer - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub struct SEQEND1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND1_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMPERIODEND_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PWMPERIODEND_A> for bool {
    #[inline(always)]
    fn from(variant: PWMPERIODEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Enable or disable interrupt for event PWMPERIODEND"]
pub struct PWMPERIODEND_R(crate::FieldReader<bool, PWMPERIODEND_A>);
impl PWMPERIODEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMPERIODEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMPERIODEND_A {
        match self.bits {
            false => PWMPERIODEND_A::DISABLED,
            true => PWMPERIODEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PWMPERIODEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWMPERIODEND_A::ENABLED
    }
}
impl core::ops::Deref for PWMPERIODEND_R {
    type Target = crate::FieldReader<bool, PWMPERIODEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMPERIODEND` writer - Enable or disable interrupt for event PWMPERIODEND"]
pub struct PWMPERIODEND_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMPERIODEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMPERIODEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMPERIODEND_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMPERIODEND_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<LOOPSDONE_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` reader - Enable or disable interrupt for event LOOPSDONE"]
pub struct LOOPSDONE_R(crate::FieldReader<bool, LOOPSDONE_A>);
impl LOOPSDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPSDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_A {
        match self.bits {
            false => LOOPSDONE_A::DISABLED,
            true => LOOPSDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOOPSDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOOPSDONE_A::ENABLED
    }
}
impl core::ops::Deref for LOOPSDONE_R {
    type Target = crate::FieldReader<bool, LOOPSDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPSDONE` writer - Enable or disable interrupt for event LOOPSDONE"]
pub struct LOOPSDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_A::ENABLED)
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
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> SEQSTARTED0_R {
        SEQSTARTED0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> SEQSTARTED1_R {
        SEQSTARTED1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> SEQEND0_R {
        SEQEND0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> SEQEND1_R {
        SEQEND1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PWMPERIODEND_R {
        PWMPERIODEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LOOPSDONE_R {
        LOOPSDONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&mut self) -> SEQSTARTED0_W {
        SEQSTARTED0_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&mut self) -> SEQSTARTED1_W {
        SEQSTARTED1_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&mut self) -> SEQEND0_W {
        SEQEND0_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&mut self) -> SEQEND1_W {
        SEQEND1_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&mut self) -> PWMPERIODEND_W {
        PWMPERIODEND_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&mut self) -> LOOPSDONE_W {
        LOOPSDONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
