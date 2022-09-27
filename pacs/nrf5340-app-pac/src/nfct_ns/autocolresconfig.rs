#[doc = "Register `AUTOCOLRESCONFIG` reader"]
pub struct R(crate::R<AUTOCOLRESCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCOLRESCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCOLRESCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCOLRESCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCOLRESCONFIG` writer"]
pub struct W(crate::W<AUTOCOLRESCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCOLRESCONFIG_SPEC>;
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
impl From<crate::W<AUTOCOLRESCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCOLRESCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Enables/disables auto collision resolution"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Enables/disables auto collision resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Auto collision resolution enabled"]
    ENABLED = 0,
    #[doc = "1: Auto collision resolution disabled"]
    DISABLED = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::ENABLED,
            true => MODE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MODE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
}
#[doc = "Field `MODE` writer - Enables/disables auto collision resolution"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCOLRESCONFIG_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Auto collision resolution enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MODE_A::ENABLED)
    }
    #[doc = "Auto collision resolution disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enables/disables auto collision resolution"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables/disables auto collision resolution"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocolresconfig](index.html) module"]
pub struct AUTOCOLRESCONFIG_SPEC;
impl crate::RegisterSpec for AUTOCOLRESCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocolresconfig::R](R) reader structure"]
impl crate::Readable for AUTOCOLRESCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocolresconfig::W](W) writer structure"]
impl crate::Writable for AUTOCOLRESCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOCOLRESCONFIG to value 0x02"]
impl crate::Resettable for AUTOCOLRESCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
