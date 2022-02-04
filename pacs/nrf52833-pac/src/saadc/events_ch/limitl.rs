#[doc = "Register `LIMITL` reader"]
pub struct R(crate::R<LIMITL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMITL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMITL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMITL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMITL` writer"]
pub struct W(crate::W<LIMITL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMITL_SPEC>;
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
impl From<crate::W<LIMITL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMITL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Last result is equal or below CH\\[n\\].LIMIT.LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIMITL_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMITL` reader - Last result is equal or below CH\\[n\\].LIMIT.LOW"]
pub struct LIMITL_R(crate::FieldReader<bool, LIMITL_A>);
impl LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMITL_A {
        match self.bits {
            false => LIMITL_A::NOTGENERATED,
            true => LIMITL_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        **self == LIMITL_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        **self == LIMITL_A::GENERATED
    }
}
impl core::ops::Deref for LIMITL_R {
    type Target = crate::FieldReader<bool, LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIMITL` writer - Last result is equal or below CH\\[n\\].LIMIT.LOW"]
pub struct LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIMITL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(LIMITL_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(LIMITL_A::GENERATED)
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
    #[doc = "Bit 0 - Last result is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&self) -> LIMITL_R {
        LIMITL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last result is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&mut self) -> LIMITL_W {
        LIMITL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Last result is equal or below CH\\[n\\].LIMIT.LOW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limitl](index.html) module"]
pub struct LIMITL_SPEC;
impl crate::RegisterSpec for LIMITL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limitl::R](R) reader structure"]
impl crate::Readable for LIMITL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limitl::W](W) writer structure"]
impl crate::Writable for LIMITL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMITL to value 0"]
impl crate::Resettable for LIMITL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
