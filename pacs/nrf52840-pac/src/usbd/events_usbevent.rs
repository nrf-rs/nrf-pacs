#[doc = "Register `EVENTS_USBEVENT` reader"]
pub struct R(crate::R<EVENTS_USBEVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_USBEVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_USBEVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_USBEVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_USBEVENT` writer"]
pub struct W(crate::W<EVENTS_USBEVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_USBEVENT_SPEC>;
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
impl From<crate::W<EVENTS_USBEVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_USBEVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_USBEVENT` reader - "]
pub struct EVENTS_USBEVENT_R(crate::FieldReader<bool, bool>);
impl EVENTS_USBEVENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENTS_USBEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTS_USBEVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTS_USBEVENT` writer - "]
pub struct EVENTS_USBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_USBEVENT_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_usbevent(&self) -> EVENTS_USBEVENT_R {
        EVENTS_USBEVENT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_usbevent(&mut self) -> EVENTS_USBEVENT_W {
        EVENTS_USBEVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbevent](index.html) module"]
pub struct EVENTS_USBEVENT_SPEC;
impl crate::RegisterSpec for EVENTS_USBEVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_usbevent::R](R) reader structure"]
impl crate::Readable for EVENTS_USBEVENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_usbevent::W](W) writer structure"]
impl crate::Writable for EVENTS_USBEVENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_USBEVENT to value 0"]
impl crate::Resettable for EVENTS_USBEVENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
