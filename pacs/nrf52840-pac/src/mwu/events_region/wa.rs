#[doc = "Register `WA` reader"]
pub struct R(crate::R<WA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WA` writer"]
pub struct W(crate::W<WA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WA_SPEC>;
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
impl From<crate::W<WA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WA` reader - "]
pub struct WA_R(crate::FieldReader<bool, bool>);
impl WA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA` writer - "]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
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
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Write access to region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wa](index.html) module"]
pub struct WA_SPEC;
impl crate::RegisterSpec for WA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wa::R](R) reader structure"]
impl crate::Readable for WA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wa::W](W) writer structure"]
impl crate::Writable for WA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WA to value 0"]
impl crate::Resettable for WA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
