#[doc = "Register `DAP[%s]` reader"]
pub struct R(crate::R<DAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAP[%s]` writer"]
pub struct W(crate::W<DAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAP_SPEC>;
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
impl From<crate::W<DAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAP` reader - Device address prefix n"]
pub struct DAP_R(crate::FieldReader<u16, u16>);
impl DAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAP` writer - Device address prefix n"]
pub struct DAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W {
        DAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Device address prefix n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dap](index.html) module"]
pub struct DAP_SPEC;
impl crate::RegisterSpec for DAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dap::R](R) reader structure"]
impl crate::Readable for DAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dap::W](W) writer structure"]
impl crate::Writable for DAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAP[%s]
to value 0"]
impl crate::Resettable for DAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
