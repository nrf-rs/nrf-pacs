#[doc = "Register `ISOSPLIT` reader"]
pub struct R(crate::R<ISOSPLIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOSPLIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOSPLIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOSPLIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOSPLIT` writer"]
pub struct W(crate::W<ISOSPLIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOSPLIT_SPEC>;
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
impl From<crate::W<ISOSPLIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOSPLIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the split of ISO buffers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SPLIT_A {
    #[doc = "0: Full buffer dedicated to either iso IN or OUT"]
    ONEDIR = 0,
    #[doc = "128: Lower half for IN, upper half for OUT"]
    HALFIN = 128,
}
impl From<SPLIT_A> for u16 {
    #[inline(always)]
    fn from(variant: SPLIT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPLIT` reader - Controls the split of ISO buffers"]
pub struct SPLIT_R(crate::FieldReader<u16, SPLIT_A>);
impl SPLIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SPLIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPLIT_A> {
        match self.bits {
            0 => Some(SPLIT_A::ONEDIR),
            128 => Some(SPLIT_A::HALFIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEDIR`"]
    #[inline(always)]
    pub fn is_one_dir(&self) -> bool {
        **self == SPLIT_A::ONEDIR
    }
    #[doc = "Checks if the value of the field is `HALFIN`"]
    #[inline(always)]
    pub fn is_half_in(&self) -> bool {
        **self == SPLIT_A::HALFIN
    }
}
impl core::ops::Deref for SPLIT_R {
    type Target = crate::FieldReader<u16, SPLIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIT` writer - Controls the split of ISO buffers"]
pub struct SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    #[inline(always)]
    pub fn one_dir(self) -> &'a mut W {
        self.variant(SPLIT_A::ONEDIR)
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline(always)]
    pub fn half_in(self) -> &'a mut W {
        self.variant(SPLIT_A::HALFIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&self) -> SPLIT_R {
        SPLIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&mut self) -> SPLIT_W {
        SPLIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the split of ISO buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isosplit](index.html) module"]
pub struct ISOSPLIT_SPEC;
impl crate::RegisterSpec for ISOSPLIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isosplit::R](R) reader structure"]
impl crate::Readable for ISOSPLIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isosplit::W](W) writer structure"]
impl crate::Writable for ISOSPLIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISOSPLIT to value 0"]
impl crate::Resettable for ISOSPLIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
