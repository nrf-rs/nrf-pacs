#[doc = "Register `CTICONTROL` reader"]
pub struct R(crate::R<CTICONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTICONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTICONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTICONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTICONTROL` writer"]
pub struct W(crate::W<CTICONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTICONTROL_SPEC>;
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
impl From<crate::W<CTICONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTICONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLBEN` reader - Enables or disables the CTI."]
pub type GLBEN_R = crate::BitReader<GLBEN_A>;
#[doc = "Enables or disables the CTI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLBEN_A {
    #[doc = "0: All cross-triggering mapping logic functionality is disabled."]
    DISABLED = 0,
    #[doc = "1: Cross-triggering mapping logic functionality is enabled."]
    ENABLED = 1,
}
impl From<GLBEN_A> for bool {
    #[inline(always)]
    fn from(variant: GLBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GLBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLBEN_A {
        match self.bits {
            false => GLBEN_A::DISABLED,
            true => GLBEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GLBEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GLBEN_A::ENABLED
    }
}
#[doc = "Field `GLBEN` writer - Enables or disables the CTI."]
pub type GLBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTICONTROL_SPEC, GLBEN_A, O>;
impl<'a, const O: u8> GLBEN_W<'a, O> {
    #[doc = "All cross-triggering mapping logic functionality is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GLBEN_A::DISABLED)
    }
    #[doc = "Cross-triggering mapping logic functionality is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GLBEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables the CTI."]
    #[inline(always)]
    pub fn glben(&self) -> GLBEN_R {
        GLBEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the CTI."]
    #[inline(always)]
    pub fn glben(&mut self) -> GLBEN_W<0> {
        GLBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cticontrol](index.html) module"]
pub struct CTICONTROL_SPEC;
impl crate::RegisterSpec for CTICONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cticontrol::R](R) reader structure"]
impl crate::Readable for CTICONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cticontrol::W](W) writer structure"]
impl crate::Writable for CTICONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTICONTROL to value 0"]
impl crate::Resettable for CTICONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
