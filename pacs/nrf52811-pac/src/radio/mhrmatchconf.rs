#[doc = "Register `MHRMATCHCONF` reader"]
pub struct R(crate::R<MHRMATCHCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MHRMATCHCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MHRMATCHCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MHRMATCHCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MHRMATCHCONF` writer"]
pub struct W(crate::W<MHRMATCHCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MHRMATCHCONF_SPEC>;
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
impl From<crate::W<MHRMATCHCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MHRMATCHCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MHRMATCHCONF` reader - Search pattern configuration"]
pub struct MHRMATCHCONF_R(crate::FieldReader<u32, u32>);
impl MHRMATCHCONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MHRMATCHCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MHRMATCHCONF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MHRMATCHCONF` writer - Search pattern configuration"]
pub struct MHRMATCHCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MHRMATCHCONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&self) -> MHRMATCHCONF_R {
        MHRMATCHCONF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&mut self) -> MHRMATCHCONF_W {
        MHRMATCHCONF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Search pattern configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhrmatchconf](index.html) module"]
pub struct MHRMATCHCONF_SPEC;
impl crate::RegisterSpec for MHRMATCHCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mhrmatchconf::R](R) reader structure"]
impl crate::Readable for MHRMATCHCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mhrmatchconf::W](W) writer structure"]
impl crate::Writable for MHRMATCHCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MHRMATCHCONF to value 0"]
impl crate::Resettable for MHRMATCHCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
