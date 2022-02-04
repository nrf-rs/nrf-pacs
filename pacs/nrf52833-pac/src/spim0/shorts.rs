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
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_START_A> for bool {
    #[inline(always)]
    fn from(variant: END_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub struct END_START_R(crate::FieldReader<bool, END_START_A>);
impl END_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_START_A {
        match self.bits {
            false => END_START_A::DISABLED,
            true => END_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == END_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_START_A::ENABLED
    }
}
impl core::ops::Deref for END_START_R {
    type Target = crate::FieldReader<bool, END_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub struct END_START_W<'a> {
    w: &'a mut W,
}
impl<'a> END_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_START_A::ENABLED)
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
impl R {
    #[doc = "Bit 17 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> END_START_R {
        END_START_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&mut self) -> END_START_W {
        END_START_W { w: self }
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
