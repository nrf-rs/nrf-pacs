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
#[doc = "Shortcut between event SEQEND\\[0\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0_STOP` reader - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub struct SEQEND0_STOP_R(crate::FieldReader<bool, SEQEND0_STOP_A>);
impl SEQEND0_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQEND0_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_STOP_A {
        match self.bits {
            false => SEQEND0_STOP_A::DISABLED,
            true => SEQEND0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQEND0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQEND0_STOP_A::ENABLED
    }
}
impl core::ops::Deref for SEQEND0_STOP_R {
    type Target = crate::FieldReader<bool, SEQEND0_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQEND0_STOP` writer - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub struct SEQEND0_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND0_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND0_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::ENABLED)
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
#[doc = "Shortcut between event SEQEND\\[1\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1_STOP` reader - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub struct SEQEND1_STOP_R(crate::FieldReader<bool, SEQEND1_STOP_A>);
impl SEQEND1_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQEND1_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_STOP_A {
        match self.bits {
            false => SEQEND1_STOP_A::DISABLED,
            true => SEQEND1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQEND1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQEND1_STOP_A::ENABLED
    }
}
impl core::ops::Deref for SEQEND1_STOP_R {
    type Target = crate::FieldReader<bool, SEQEND1_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQEND1_STOP` writer - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub struct SEQEND1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND1_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND1_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::ENABLED)
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
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART0_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub struct LOOPSDONE_SEQSTART0_R(crate::FieldReader<bool, LOOPSDONE_SEQSTART0_A>);
impl LOOPSDONE_SEQSTART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPSDONE_SEQSTART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART0_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART0_A::DISABLED,
            true => LOOPSDONE_SEQSTART0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOOPSDONE_SEQSTART0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOOPSDONE_SEQSTART0_A::ENABLED
    }
}
impl core::ops::Deref for LOOPSDONE_SEQSTART0_R {
    type Target = crate::FieldReader<bool, LOOPSDONE_SEQSTART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub struct LOOPSDONE_SEQSTART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_SEQSTART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::ENABLED)
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
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART1_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART1_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub struct LOOPSDONE_SEQSTART1_R(crate::FieldReader<bool, LOOPSDONE_SEQSTART1_A>);
impl LOOPSDONE_SEQSTART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPSDONE_SEQSTART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART1_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART1_A::DISABLED,
            true => LOOPSDONE_SEQSTART1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOOPSDONE_SEQSTART1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOOPSDONE_SEQSTART1_A::ENABLED
    }
}
impl core::ops::Deref for LOOPSDONE_SEQSTART1_R {
    type Target = crate::FieldReader<bool, LOOPSDONE_SEQSTART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub struct LOOPSDONE_SEQSTART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_SEQSTART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::ENABLED)
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
#[doc = "Shortcut between event LOOPSDONE and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_STOP` reader - Shortcut between event LOOPSDONE and task STOP"]
pub struct LOOPSDONE_STOP_R(crate::FieldReader<bool, LOOPSDONE_STOP_A>);
impl LOOPSDONE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPSDONE_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_STOP_A {
        match self.bits {
            false => LOOPSDONE_STOP_A::DISABLED,
            true => LOOPSDONE_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOOPSDONE_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOOPSDONE_STOP_A::ENABLED
    }
}
impl core::ops::Deref for LOOPSDONE_STOP_R {
    type Target = crate::FieldReader<bool, LOOPSDONE_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPSDONE_STOP` writer - Shortcut between event LOOPSDONE and task STOP"]
pub struct LOOPSDONE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend0_stop(&self) -> SEQEND0_STOP_R {
        SEQEND0_STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend1_stop(&self) -> SEQEND1_STOP_R {
        SEQEND1_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&self) -> LOOPSDONE_SEQSTART0_R {
        LOOPSDONE_SEQSTART0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&self) -> LOOPSDONE_SEQSTART1_R {
        LOOPSDONE_SEQSTART1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    pub fn loopsdone_stop(&self) -> LOOPSDONE_STOP_R {
        LOOPSDONE_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend0_stop(&mut self) -> SEQEND0_STOP_W {
        SEQEND0_STOP_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend1_stop(&mut self) -> SEQEND1_STOP_W {
        SEQEND1_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&mut self) -> LOOPSDONE_SEQSTART0_W {
        LOOPSDONE_SEQSTART0_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&mut self) -> LOOPSDONE_SEQSTART1_W {
        LOOPSDONE_SEQSTART1_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    pub fn loopsdone_stop(&mut self) -> LOOPSDONE_STOP_W {
        LOOPSDONE_STOP_W { w: self }
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
