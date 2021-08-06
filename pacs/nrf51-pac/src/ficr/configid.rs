#[doc = "Register `CONFIGID` reader"]
pub struct R(crate::R<CONFIGID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIGID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIGID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIGID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIGID` writer"]
pub struct W(crate::W<CONFIGID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIGID_SPEC>;
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
impl From<crate::W<CONFIGID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIGID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWID` reader - Hardware Identification Number."]
pub struct HWID_R(crate::FieldReader<u16, u16>);
impl HWID_R {
    pub(crate) fn new(bits: u16) -> Self {
        HWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWID` writer - Hardware Identification Number."]
pub struct HWID_W<'a> {
    w: &'a mut W,
}
impl<'a> HWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FWID` reader - Firmware Identification Number pre-loaded into the flash."]
pub struct FWID_R(crate::FieldReader<u16, u16>);
impl FWID_R {
    pub(crate) fn new(bits: u16) -> Self {
        FWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWID` writer - Firmware Identification Number pre-loaded into the flash."]
pub struct FWID_W<'a> {
    w: &'a mut W,
}
impl<'a> FWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&self) -> HWID_R {
        HWID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&self) -> FWID_R {
        FWID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&mut self) -> HWID_W {
        HWID_W { w: self }
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&mut self) -> FWID_W {
        FWID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration identifier.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configid](index.html) module"]
pub struct CONFIGID_SPEC;
impl crate::RegisterSpec for CONFIGID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [configid::R](R) reader structure"]
impl crate::Readable for CONFIGID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [configid::W](W) writer structure"]
impl crate::Writable for CONFIGID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIGID to value 0xffff_ffff"]
impl crate::Resettable for CONFIGID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
