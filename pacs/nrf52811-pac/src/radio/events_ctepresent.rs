#[doc = "Register `EVENTS_CTEPRESENT` reader"]
pub struct R(crate::R<EVENTS_CTEPRESENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_CTEPRESENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_CTEPRESENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_CTEPRESENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_CTEPRESENT` writer"]
pub struct W(crate::W<EVENTS_CTEPRESENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_CTEPRESENT_SPEC>;
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
impl From<crate::W<EVENTS_CTEPRESENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_CTEPRESENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_CTEPRESENT_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_CTEPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_CTEPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTEPRESENT` reader - CTE is present (early warning right after receiving CTEInfo byte)"]
pub struct EVENTS_CTEPRESENT_R(crate::FieldReader<bool, EVENTS_CTEPRESENT_A>);
impl EVENTS_CTEPRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTS_CTEPRESENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_CTEPRESENT_A {
        match self.bits {
            false => EVENTS_CTEPRESENT_A::NOTGENERATED,
            true => EVENTS_CTEPRESENT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        **self == EVENTS_CTEPRESENT_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        **self == EVENTS_CTEPRESENT_A::GENERATED
    }
}
impl core::ops::Deref for EVENTS_CTEPRESENT_R {
    type Target = crate::FieldReader<bool, EVENTS_CTEPRESENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTS_CTEPRESENT` writer - CTE is present (early warning right after receiving CTEInfo byte)"]
pub struct EVENTS_CTEPRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_CTEPRESENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_CTEPRESENT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_CTEPRESENT_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_CTEPRESENT_A::GENERATED)
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
    #[doc = "Bit 0 - CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub fn events_ctepresent(&self) -> EVENTS_CTEPRESENT_R {
        EVENTS_CTEPRESENT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub fn events_ctepresent(&mut self) -> EVENTS_CTEPRESENT_W {
        EVENTS_CTEPRESENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ctepresent](index.html) module"]
pub struct EVENTS_CTEPRESENT_SPEC;
impl crate::RegisterSpec for EVENTS_CTEPRESENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_ctepresent::R](R) reader structure"]
impl crate::Readable for EVENTS_CTEPRESENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_ctepresent::W](W) writer structure"]
impl crate::Writable for EVENTS_CTEPRESENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_CTEPRESENT to value 0"]
impl crate::Resettable for EVENTS_CTEPRESENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
