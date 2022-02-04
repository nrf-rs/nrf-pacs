#[doc = "Register `PUBLISH_ENDEPOUT[%s]` reader"]
pub struct R(crate::R<PUBLISH_ENDEPOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUBLISH_ENDEPOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUBLISH_ENDEPOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUBLISH_ENDEPOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUBLISH_ENDEPOUT[%s]` writer"]
pub struct W(crate::W<PUBLISH_ENDEPOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUBLISH_ENDEPOUT_SPEC>;
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
impl From<crate::W<PUBLISH_ENDEPOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUBLISH_ENDEPOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIDX` reader - DPPI channel that event ENDEPOUT\\[n\\]
will publish to."]
pub struct CHIDX_R(crate::FieldReader<u8, u8>);
impl CHIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIDX` writer - DPPI channel that event ENDEPOUT\\[n\\]
will publish to."]
pub struct CHIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable publishing"]
    DISABLED = 0,
    #[doc = "1: Enable publishing"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - "]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_A::ENABLED
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - "]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable publishing"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Enable publishing"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that event ENDEPOUT\\[n\\]
will publish to."]
    #[inline(always)]
    pub fn chidx(&self) -> CHIDX_R {
        CHIDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPPI channel that event ENDEPOUT\\[n\\]
will publish to."]
    #[inline(always)]
    pub fn chidx(&mut self) -> CHIDX_W {
        CHIDX_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Publish configuration for event ENDEPOUT\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_endepout](index.html) module"]
pub struct PUBLISH_ENDEPOUT_SPEC;
impl crate::RegisterSpec for PUBLISH_ENDEPOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [publish_endepout::R](R) reader structure"]
impl crate::Readable for PUBLISH_ENDEPOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [publish_endepout::W](W) writer structure"]
impl crate::Writable for PUBLISH_ENDEPOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUBLISH_ENDEPOUT[%s]
to value 0"]
impl crate::Resettable for PUBLISH_ENDEPOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
