#[doc = "Register `EVENTS_SUSPENDED` reader"]
pub struct R(crate::R<EVENTS_SUSPENDED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_SUSPENDED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_SUSPENDED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_SUSPENDED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_SUSPENDED` writer"]
pub struct W(crate::W<EVENTS_SUSPENDED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_SUSPENDED_SPEC>;
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
impl From<crate::W<EVENTS_SUSPENDED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_SUSPENDED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TWI entered the suspended state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SUSPENDED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_SUSPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_SUSPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SUSPENDED` reader - TWI entered the suspended state"]
pub struct EVENTS_SUSPENDED_R(crate::FieldReader<bool, EVENTS_SUSPENDED_A>);
impl EVENTS_SUSPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTS_SUSPENDED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_SUSPENDED_A {
        match self.bits {
            false => EVENTS_SUSPENDED_A::NOTGENERATED,
            true => EVENTS_SUSPENDED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        **self == EVENTS_SUSPENDED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        **self == EVENTS_SUSPENDED_A::GENERATED
    }
}
impl core::ops::Deref for EVENTS_SUSPENDED_R {
    type Target = crate::FieldReader<bool, EVENTS_SUSPENDED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTS_SUSPENDED` writer - TWI entered the suspended state"]
pub struct EVENTS_SUSPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_SUSPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_SUSPENDED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_SUSPENDED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_SUSPENDED_A::GENERATED)
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
    #[doc = "Bit 0 - TWI entered the suspended state"]
    #[inline(always)]
    pub fn events_suspended(&self) -> EVENTS_SUSPENDED_R {
        EVENTS_SUSPENDED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI entered the suspended state"]
    #[inline(always)]
    pub fn events_suspended(&mut self) -> EVENTS_SUSPENDED_W {
        EVENTS_SUSPENDED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI entered the suspended state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_suspended](index.html) module"]
pub struct EVENTS_SUSPENDED_SPEC;
impl crate::RegisterSpec for EVENTS_SUSPENDED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_suspended::R](R) reader structure"]
impl crate::Readable for EVENTS_SUSPENDED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_suspended::W](W) writer structure"]
impl crate::Writable for EVENTS_SUSPENDED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_SUSPENDED to value 0"]
impl crate::Resettable for EVENTS_SUSPENDED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
