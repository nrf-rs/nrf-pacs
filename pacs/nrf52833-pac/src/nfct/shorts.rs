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
#[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_ACTIVATE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<FIELDDETECTED_ACTIVATE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_ACTIVATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` reader - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub struct FIELDDETECTED_ACTIVATE_R(crate::FieldReader<bool, FIELDDETECTED_ACTIVATE_A>);
impl FIELDDETECTED_ACTIVATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIELDDETECTED_ACTIVATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_ACTIVATE_A {
        match self.bits {
            false => FIELDDETECTED_ACTIVATE_A::DISABLED,
            true => FIELDDETECTED_ACTIVATE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIELDDETECTED_ACTIVATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIELDDETECTED_ACTIVATE_A::ENABLED
    }
}
impl core::ops::Deref for FIELDDETECTED_ACTIVATE_R {
    type Target = crate::FieldReader<bool, FIELDDETECTED_ACTIVATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` writer - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub struct FIELDDETECTED_ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDDETECTED_ACTIVATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDDETECTED_ACTIVATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::ENABLED)
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
#[doc = "Shortcut between event FIELDLOST and task SENSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_SENSE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<FIELDLOST_SENSE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_SENSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST_SENSE` reader - Shortcut between event FIELDLOST and task SENSE"]
pub struct FIELDLOST_SENSE_R(crate::FieldReader<bool, FIELDLOST_SENSE_A>);
impl FIELDLOST_SENSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIELDLOST_SENSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_SENSE_A {
        match self.bits {
            false => FIELDLOST_SENSE_A::DISABLED,
            true => FIELDLOST_SENSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIELDLOST_SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIELDLOST_SENSE_A::ENABLED
    }
}
impl core::ops::Deref for FIELDLOST_SENSE_R {
    type Target = crate::FieldReader<bool, FIELDLOST_SENSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELDLOST_SENSE` writer - Shortcut between event FIELDLOST and task SENSE"]
pub struct FIELDLOST_SENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDLOST_SENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDLOST_SENSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::ENABLED)
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
#[doc = "Shortcut between event TXFRAMEEND and task ENABLERXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_ENABLERXDATA_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<TXFRAMEEND_ENABLERXDATA_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_ENABLERXDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` reader - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub struct TXFRAMEEND_ENABLERXDATA_R(crate::FieldReader<bool, TXFRAMEEND_ENABLERXDATA_A>);
impl TXFRAMEEND_ENABLERXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFRAMEEND_ENABLERXDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMEEND_ENABLERXDATA_A {
        match self.bits {
            false => TXFRAMEEND_ENABLERXDATA_A::DISABLED,
            true => TXFRAMEEND_ENABLERXDATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFRAMEEND_ENABLERXDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFRAMEEND_ENABLERXDATA_A::ENABLED
    }
}
impl core::ops::Deref for TXFRAMEEND_ENABLERXDATA_R {
    type Target = crate::FieldReader<bool, TXFRAMEEND_ENABLERXDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` writer - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub struct TXFRAMEEND_ENABLERXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMEEND_ENABLERXDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMEEND_ENABLERXDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATA_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATA_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&self) -> FIELDDETECTED_ACTIVATE_R {
        FIELDDETECTED_ACTIVATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&self) -> FIELDLOST_SENSE_R {
        FIELDLOST_SENSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&self) -> TXFRAMEEND_ENABLERXDATA_R {
        TXFRAMEEND_ENABLERXDATA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&mut self) -> FIELDDETECTED_ACTIVATE_W {
        FIELDDETECTED_ACTIVATE_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&mut self) -> FIELDLOST_SENSE_W {
        FIELDLOST_SENSE_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&mut self) -> TXFRAMEEND_ENABLERXDATA_W {
        TXFRAMEEND_ENABLERXDATA_W { w: self }
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
