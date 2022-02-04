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
#[doc = "Shortcut between event LASTTX and task STARTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTTX_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_STARTRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_STARTRX` reader - Shortcut between event LASTTX and task STARTRX"]
pub struct LASTTX_STARTRX_R(crate::FieldReader<bool, LASTTX_STARTRX_A>);
impl LASTTX_STARTRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTTX_STARTRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_STARTRX_A {
        match self.bits {
            false => LASTTX_STARTRX_A::DISABLED,
            true => LASTTX_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTTX_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTTX_STARTRX_A::ENABLED
    }
}
impl core::ops::Deref for LASTTX_STARTRX_R {
    type Target = crate::FieldReader<bool, LASTTX_STARTRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTTX_STARTRX` writer - Shortcut between event LASTTX and task STARTRX"]
pub struct LASTTX_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_STARTRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_STARTRX_A::ENABLED)
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
#[doc = "Shortcut between event LASTTX and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTTX_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_SUSPEND` reader - Shortcut between event LASTTX and task SUSPEND"]
pub struct LASTTX_SUSPEND_R(crate::FieldReader<bool, LASTTX_SUSPEND_A>);
impl LASTTX_SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTTX_SUSPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_SUSPEND_A {
        match self.bits {
            false => LASTTX_SUSPEND_A::DISABLED,
            true => LASTTX_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTTX_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTTX_SUSPEND_A::ENABLED
    }
}
impl core::ops::Deref for LASTTX_SUSPEND_R {
    type Target = crate::FieldReader<bool, LASTTX_SUSPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTTX_SUSPEND` writer - Shortcut between event LASTTX and task SUSPEND"]
pub struct LASTTX_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_SUSPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_SUSPEND_A::ENABLED)
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
#[doc = "Shortcut between event LASTTX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTTX_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LASTTX_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_STOP` reader - Shortcut between event LASTTX and task STOP"]
pub struct LASTTX_STOP_R(crate::FieldReader<bool, LASTTX_STOP_A>);
impl LASTTX_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTTX_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTTX_STOP_A {
        match self.bits {
            false => LASTTX_STOP_A::DISABLED,
            true => LASTTX_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTTX_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTTX_STOP_A::ENABLED
    }
}
impl core::ops::Deref for LASTTX_STOP_R {
    type Target = crate::FieldReader<bool, LASTTX_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTTX_STOP` writer - Shortcut between event LASTTX and task STOP"]
pub struct LASTTX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTTX_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTTX_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTTX_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTTX_STOP_A::ENABLED)
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
#[doc = "Shortcut between event LASTRX and task STARTTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STARTTX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTRX_STARTTX_A> for bool {
    #[inline(always)]
    fn from(variant: LASTRX_STARTTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_STARTTX` reader - Shortcut between event LASTRX and task STARTTX"]
pub struct LASTRX_STARTTX_R(crate::FieldReader<bool, LASTRX_STARTTX_A>);
impl LASTRX_STARTTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTRX_STARTTX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTRX_STARTTX_A {
        match self.bits {
            false => LASTRX_STARTTX_A::DISABLED,
            true => LASTRX_STARTTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTRX_STARTTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTRX_STARTTX_A::ENABLED
    }
}
impl core::ops::Deref for LASTRX_STARTTX_R {
    type Target = crate::FieldReader<bool, LASTRX_STARTTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTRX_STARTTX` writer - Shortcut between event LASTRX and task STARTTX"]
pub struct LASTRX_STARTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTRX_STARTTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTRX_STARTTX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTRX_STARTTX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTRX_STARTTX_A::ENABLED)
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
#[doc = "Shortcut between event LASTRX and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTRX_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: LASTRX_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_SUSPEND` reader - Shortcut between event LASTRX and task SUSPEND"]
pub struct LASTRX_SUSPEND_R(crate::FieldReader<bool, LASTRX_SUSPEND_A>);
impl LASTRX_SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTRX_SUSPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTRX_SUSPEND_A {
        match self.bits {
            false => LASTRX_SUSPEND_A::DISABLED,
            true => LASTRX_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTRX_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTRX_SUSPEND_A::ENABLED
    }
}
impl core::ops::Deref for LASTRX_SUSPEND_R {
    type Target = crate::FieldReader<bool, LASTRX_SUSPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTRX_SUSPEND` writer - Shortcut between event LASTRX and task SUSPEND"]
pub struct LASTRX_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTRX_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTRX_SUSPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTRX_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTRX_SUSPEND_A::ENABLED)
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
#[doc = "Shortcut between event LASTRX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LASTRX_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LASTRX_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_STOP` reader - Shortcut between event LASTRX and task STOP"]
pub struct LASTRX_STOP_R(crate::FieldReader<bool, LASTRX_STOP_A>);
impl LASTRX_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LASTRX_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LASTRX_STOP_A {
        match self.bits {
            false => LASTRX_STOP_A::DISABLED,
            true => LASTRX_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LASTRX_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LASTRX_STOP_A::ENABLED
    }
}
impl core::ops::Deref for LASTRX_STOP_R {
    type Target = crate::FieldReader<bool, LASTRX_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LASTRX_STOP` writer - Shortcut between event LASTRX and task STOP"]
pub struct LASTRX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTRX_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTRX_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LASTRX_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LASTRX_STOP_A::ENABLED)
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
impl R {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task STARTRX"]
    #[inline(always)]
    pub fn lasttx_startrx(&self) -> LASTTX_STARTRX_R {
        LASTTX_STARTRX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    pub fn lasttx_suspend(&self) -> LASTTX_SUSPEND_R {
        LASTTX_SUSPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    pub fn lasttx_stop(&self) -> LASTTX_STOP_R {
        LASTTX_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task STARTTX"]
    #[inline(always)]
    pub fn lastrx_starttx(&self) -> LASTRX_STARTTX_R {
        LASTRX_STARTTX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Shortcut between event LASTRX and task SUSPEND"]
    #[inline(always)]
    pub fn lastrx_suspend(&self) -> LASTRX_SUSPEND_R {
        LASTRX_SUSPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    pub fn lastrx_stop(&self) -> LASTRX_STOP_R {
        LASTRX_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task STARTRX"]
    #[inline(always)]
    pub fn lasttx_startrx(&mut self) -> LASTTX_STARTRX_W {
        LASTTX_STARTRX_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    pub fn lasttx_suspend(&mut self) -> LASTTX_SUSPEND_W {
        LASTTX_SUSPEND_W { w: self }
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    pub fn lasttx_stop(&mut self) -> LASTTX_STOP_W {
        LASTTX_STOP_W { w: self }
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task STARTTX"]
    #[inline(always)]
    pub fn lastrx_starttx(&mut self) -> LASTRX_STARTTX_W {
        LASTRX_STARTTX_W { w: self }
    }
    #[doc = "Bit 11 - Shortcut between event LASTRX and task SUSPEND"]
    #[inline(always)]
    pub fn lastrx_suspend(&mut self) -> LASTRX_SUSPEND_W {
        LASTRX_SUSPEND_W { w: self }
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    pub fn lastrx_stop(&mut self) -> LASTRX_STOP_W {
        LASTRX_STOP_W { w: self }
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
