#[doc = "Register `LOOP` reader"]
pub struct R(crate::R<LOOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP` writer"]
pub struct W(crate::W<LOOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP_SPEC>;
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
impl From<crate::W<LOOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Number of playbacks of pattern cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CNT_A {
    #[doc = "0: Looping disabled (stop at the end of the sequence)"]
    DISABLED = 0,
}
impl From<CNT_A> for u16 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNT` reader - Number of playbacks of pattern cycles"]
pub struct CNT_R(crate::FieldReader<u16, CNT_A>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNT_A> {
        match self.bits {
            0 => Some(CNT_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CNT_A::DISABLED
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Number of playbacks of pattern cycles"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNT_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of playbacks of a loop\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop_](index.html) module"]
pub struct LOOP_SPEC;
impl crate::RegisterSpec for LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop_::R](R) reader structure"]
impl crate::Readable for LOOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop_::W](W) writer structure"]
impl crate::Writable for LOOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LOOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
