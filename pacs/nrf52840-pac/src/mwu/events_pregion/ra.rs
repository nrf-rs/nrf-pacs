#[doc = "Register `RA` reader"]
pub struct R(crate::R<RA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA` writer"]
pub struct W(crate::W<RA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA_SPEC>;
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
impl From<crate::W<RA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - "]
pub struct RA_R(crate::FieldReader<bool, bool>);
impl RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - "]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
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
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Read access to peripheral region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](index.html) module"]
pub struct RA_SPEC;
impl crate::RegisterSpec for RA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra::R](R) reader structure"]
impl crate::Readable for RA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra::W](W) writer structure"]
impl crate::Writable for RA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
