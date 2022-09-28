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
#[doc = "Field `FIELDDETECTED_ACTIVATE` reader - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FIELDDETECTED_ACTIVATE_R = crate::BitReader<FIELDDETECTED_ACTIVATE_A>;
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
impl FIELDDETECTED_ACTIVATE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FIELDDETECTED_ACTIVATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATE_A::ENABLED
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` writer - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FIELDDETECTED_ACTIVATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, FIELDDETECTED_ACTIVATE_A, O>;
impl<'a, const O: u8> FIELDDETECTED_ACTIVATE_W<'a, O> {
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
}
#[doc = "Field `FIELDLOST_SENSE` reader - Shortcut between event FIELDLOST and task SENSE"]
pub type FIELDLOST_SENSE_R = crate::BitReader<FIELDLOST_SENSE_A>;
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
impl FIELDLOST_SENSE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FIELDLOST_SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_SENSE_A::ENABLED
    }
}
#[doc = "Field `FIELDLOST_SENSE` writer - Shortcut between event FIELDLOST and task SENSE"]
pub type FIELDLOST_SENSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, FIELDLOST_SENSE_A, O>;
impl<'a, const O: u8> FIELDLOST_SENSE_W<'a, O> {
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
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` reader - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub type TXFRAMEEND_ENABLERXDATA_R = crate::BitReader<TXFRAMEEND_ENABLERXDATA_A>;
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
impl TXFRAMEEND_ENABLERXDATA_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TXFRAMEEND_ENABLERXDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEEND_ENABLERXDATA_A::ENABLED
    }
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` writer - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub type TXFRAMEEND_ENABLERXDATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, TXFRAMEEND_ENABLERXDATA_A, O>;
impl<'a, const O: u8> TXFRAMEEND_ENABLERXDATA_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&self) -> FIELDDETECTED_ACTIVATE_R {
        FIELDDETECTED_ACTIVATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&self) -> FIELDLOST_SENSE_R {
        FIELDLOST_SENSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&self) -> TXFRAMEEND_ENABLERXDATA_R {
        TXFRAMEEND_ENABLERXDATA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&mut self) -> FIELDDETECTED_ACTIVATE_W<0> {
        FIELDDETECTED_ACTIVATE_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&mut self) -> FIELDLOST_SENSE_W<1> {
        FIELDLOST_SENSE_W::new(self)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&mut self) -> TXFRAMEEND_ENABLERXDATA_W<5> {
        TXFRAMEEND_ENABLERXDATA_W::new(self)
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
