#[doc = "Register `EVENTS_SAMPLERDY` reader"]
pub struct R(crate::R<EVENTS_SAMPLERDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_SAMPLERDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_SAMPLERDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_SAMPLERDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_SAMPLERDY` writer"]
pub struct W(crate::W<EVENTS_SAMPLERDY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_SAMPLERDY_SPEC>;
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
impl From<crate::W<EVENTS_SAMPLERDY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_SAMPLERDY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SAMPLERDY_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_SAMPLERDY_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_SAMPLERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SAMPLERDY` reader - Event being generated for every new sample value written to the SAMPLE register"]
pub struct EVENTS_SAMPLERDY_R(crate::FieldReader<bool, EVENTS_SAMPLERDY_A>);
impl EVENTS_SAMPLERDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTS_SAMPLERDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_SAMPLERDY_A {
        match self.bits {
            false => EVENTS_SAMPLERDY_A::NOTGENERATED,
            true => EVENTS_SAMPLERDY_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        **self == EVENTS_SAMPLERDY_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        **self == EVENTS_SAMPLERDY_A::GENERATED
    }
}
impl core::ops::Deref for EVENTS_SAMPLERDY_R {
    type Target = crate::FieldReader<bool, EVENTS_SAMPLERDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTS_SAMPLERDY` writer - Event being generated for every new sample value written to the SAMPLE register"]
pub struct EVENTS_SAMPLERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_SAMPLERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_SAMPLERDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_SAMPLERDY_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_SAMPLERDY_A::GENERATED)
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
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&self) -> EVENTS_SAMPLERDY_R {
        EVENTS_SAMPLERDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&mut self) -> EVENTS_SAMPLERDY_W {
        EVENTS_SAMPLERDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_samplerdy](index.html) module"]
pub struct EVENTS_SAMPLERDY_SPEC;
impl crate::RegisterSpec for EVENTS_SAMPLERDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_samplerdy::R](R) reader structure"]
impl crate::Readable for EVENTS_SAMPLERDY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_samplerdy::W](W) writer structure"]
impl crate::Writable for EVENTS_SAMPLERDY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_SAMPLERDY to value 0"]
impl crate::Resettable for EVENTS_SAMPLERDY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
