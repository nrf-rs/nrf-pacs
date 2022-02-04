#[doc = "Register `PROFILINGENABLE` reader"]
pub struct R(crate::R<PROFILINGENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROFILINGENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROFILINGENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROFILINGENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROFILINGENABLE` writer"]
pub struct W(crate::W<PROFILINGENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROFILINGENABLE_SPEC>;
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
impl From<crate::W<PROFILINGENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROFILINGENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the profiling counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable profiling"]
    DISABLE = 0,
    #[doc = "1: Enable profiling"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable the profiling counters"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable the profiling counters"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable profiling"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Enable profiling"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
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
    #[doc = "Bit 0 - Enable the profiling counters"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the profiling counters"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable the profiling counters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [profilingenable](index.html) module"]
pub struct PROFILINGENABLE_SPEC;
impl crate::RegisterSpec for PROFILINGENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [profilingenable::R](R) reader structure"]
impl crate::Readable for PROFILINGENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [profilingenable::W](W) writer structure"]
impl crate::Writable for PROFILINGENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROFILINGENABLE to value 0"]
impl crate::Resettable for PROFILINGENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
