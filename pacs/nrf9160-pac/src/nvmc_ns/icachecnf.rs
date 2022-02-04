#[doc = "Register `ICACHECNF` reader"]
pub struct R(crate::R<ICACHECNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHECNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHECNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHECNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHECNF` writer"]
pub struct W(crate::W<ICACHECNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHECNF_SPEC>;
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
impl From<crate::W<ICACHECNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHECNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEEN_A {
    #[doc = "0: Disable cache. Invalidates all cache entries."]
    DISABLED = 0,
    #[doc = "1: Enable cache"]
    ENABLED = 1,
}
impl From<CACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEEN` reader - Cache enable"]
pub struct CACHEEN_R(crate::FieldReader<bool, CACHEEN_A>);
impl CACHEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEEN_A {
        match self.bits {
            false => CACHEEN_A::DISABLED,
            true => CACHEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CACHEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CACHEEN_A::ENABLED
    }
}
impl core::ops::Deref for CACHEEN_R {
    type Target = crate::FieldReader<bool, CACHEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEEN` writer - Cache enable"]
pub struct CACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::DISABLED)
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::ENABLED)
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
#[doc = "Cache profiling enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEPROFEN_A {
    #[doc = "0: Disable cache profiling"]
    DISABLED = 0,
    #[doc = "1: Enable cache profiling"]
    ENABLED = 1,
}
impl From<CACHEPROFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEPROFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEPROFEN` reader - Cache profiling enable"]
pub struct CACHEPROFEN_R(crate::FieldReader<bool, CACHEPROFEN_A>);
impl CACHEPROFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHEPROFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEPROFEN_A {
        match self.bits {
            false => CACHEPROFEN_A::DISABLED,
            true => CACHEPROFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CACHEPROFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CACHEPROFEN_A::ENABLED
    }
}
impl core::ops::Deref for CACHEPROFEN_R {
    type Target = crate::FieldReader<bool, CACHEPROFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEPROFEN` writer - Cache profiling enable"]
pub struct CACHEPROFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEPROFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEPROFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable cache profiling"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::DISABLED)
    }
    #[doc = "Enable cache profiling"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&self) -> CACHEEN_R {
        CACHEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&self) -> CACHEPROFEN_R {
        CACHEPROFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&mut self) -> CACHEEN_W {
        CACHEEN_W { w: self }
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&mut self) -> CACHEPROFEN_W {
        CACHEPROFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I-code cache configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icachecnf](index.html) module"]
pub struct ICACHECNF_SPEC;
impl crate::RegisterSpec for ICACHECNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icachecnf::R](R) reader structure"]
impl crate::Readable for ICACHECNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icachecnf::W](W) writer structure"]
impl crate::Writable for ICACHECNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHECNF to value 0"]
impl crate::Resettable for ICACHECNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
