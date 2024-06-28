#[doc = "Register `EXTPOFCON` reader"]
pub struct R(crate::R<EXTPOFCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTPOFCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTPOFCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTPOFCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTPOFCON` writer"]
pub struct W(crate::W<EXTPOFCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTPOFCON_SPEC>;
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
impl From<crate::W<EXTPOFCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTPOFCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POF` reader - Enable or disable external power failure warning"]
pub type POF_R = crate::BitReader<POF_A>;
#[doc = "Enable or disable external power failure warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POF_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        variant as u8 != 0
    }
}
impl POF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POF_A::ENABLED
    }
}
#[doc = "Field `POF` writer - Enable or disable external power failure warning"]
pub type POF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTPOFCON_SPEC, POF_A, O>;
impl<'a, const O: u8> POF_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable external power failure warning"]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable external power failure warning"]
    #[inline(always)]
    pub fn pof(&mut self) -> POF_W<0> {
        POF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External power failure warning configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extpofcon](index.html) module"]
pub struct EXTPOFCON_SPEC;
impl crate::RegisterSpec for EXTPOFCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extpofcon::R](R) reader structure"]
impl crate::Readable for EXTPOFCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extpofcon::W](W) writer structure"]
impl crate::Writable for EXTPOFCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTPOFCON to value 0"]
impl crate::Resettable for EXTPOFCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
