#[doc = "Register `FORCEOFF` reader"]
pub struct R(crate::R<FORCEOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCEOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCEOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCEOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORCEOFF` writer"]
pub struct W(crate::W<FORCEOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCEOFF_SPEC>;
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
impl From<crate::W<FORCEOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCEOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force network core off\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFF_A {
    #[doc = "0: Release Force-OFF"]
    RELEASE = 0,
    #[doc = "1: Hold Force-OFF"]
    HOLD = 1,
}
impl From<FORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEOFF` reader - Force network core off"]
pub struct FORCEOFF_R(crate::FieldReader<bool, FORCEOFF_A>);
impl FORCEOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCEOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFF_A {
        match self.bits {
            false => FORCEOFF_A::RELEASE,
            true => FORCEOFF_A::HOLD,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        **self == FORCEOFF_A::RELEASE
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        **self == FORCEOFF_A::HOLD
    }
}
impl core::ops::Deref for FORCEOFF_R {
    type Target = crate::FieldReader<bool, FORCEOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEOFF` writer - Force network core off"]
pub struct FORCEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Release Force-OFF"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(FORCEOFF_A::RELEASE)
    }
    #[doc = "Hold Force-OFF"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(FORCEOFF_A::HOLD)
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
impl R {
    #[doc = "Bit 0 - Force network core off"]
    #[inline(always)]
    pub fn forceoff(&self) -> FORCEOFF_R {
        FORCEOFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force network core off"]
    #[inline(always)]
    pub fn forceoff(&mut self) -> FORCEOFF_W {
        FORCEOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force network core off\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [forceoff](index.html) module"]
pub struct FORCEOFF_SPEC;
impl crate::RegisterSpec for FORCEOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [forceoff::R](R) reader structure"]
impl crate::Readable for FORCEOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [forceoff::W](W) writer structure"]
impl crate::Writable for FORCEOFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FORCEOFF to value 0x01"]
impl crate::Resettable for FORCEOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
