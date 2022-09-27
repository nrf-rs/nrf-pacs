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
#[doc = "Field `REPORTRDY_READCLRACC` reader - Shortcut between REPORTRDY event and READCLRACC task."]
pub type REPORTRDY_READCLRACC_R = crate::BitReader<REPORTRDY_READCLRACC_A>;
#[doc = "Shortcut between REPORTRDY event and READCLRACC task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_READCLRACC_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<REPORTRDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
impl REPORTRDY_READCLRACC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == REPORTRDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_READCLRACC_A::ENABLED
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` writer - Shortcut between REPORTRDY event and READCLRACC task."]
pub type REPORTRDY_READCLRACC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, REPORTRDY_READCLRACC_A, O>;
impl<'a, const O: u8> REPORTRDY_READCLRACC_W<'a, O> {
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::ENABLED)
    }
}
#[doc = "Field `SAMPLERDY_STOP` reader - Shortcut between SAMPLERDY event and STOP task."]
pub type SAMPLERDY_STOP_R = crate::BitReader<SAMPLERDY_STOP_A>;
#[doc = "Shortcut between SAMPLERDY event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<SAMPLERDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPLERDY_STOP_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SAMPLERDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_STOP_A::ENABLED
    }
}
#[doc = "Field `SAMPLERDY_STOP` writer - Shortcut between SAMPLERDY event and STOP task."]
pub type SAMPLERDY_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, SAMPLERDY_STOP_A, O>;
impl<'a, const O: u8> SAMPLERDY_STOP_W<'a, O> {
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> REPORTRDY_READCLRACC_R {
        REPORTRDY_READCLRACC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SAMPLERDY_STOP_R {
        SAMPLERDY_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&mut self) -> REPORTRDY_READCLRACC_W<0> {
        REPORTRDY_READCLRACC_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&mut self) -> SAMPLERDY_STOP_W<1> {
        SAMPLERDY_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts for the QDEC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
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
