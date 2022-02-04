#[doc = "Register `MISO` reader"]
pub struct R(crate::R<MISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISO` writer"]
pub struct W(crate::W<MISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISO_SPEC>;
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
impl From<crate::W<MISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin number configuration for SPI MISO signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELMISO_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELMISO_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELMISO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSELMISO` reader - Pin number configuration for SPI MISO signal"]
pub struct PSELMISO_R(crate::FieldReader<u32, PSELMISO_A>);
impl PSELMISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PSELMISO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELMISO_A> {
        match self.bits {
            4294967295 => Some(PSELMISO_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PSELMISO_A::DISCONNECTED
    }
}
impl core::ops::Deref for PSELMISO_R {
    type Target = crate::FieldReader<u32, PSELMISO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSELMISO` writer - Pin number configuration for SPI MISO signal"]
pub struct PSELMISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELMISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELMISO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELMISO_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&self) -> PSELMISO_R {
        PSELMISO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&mut self) -> PSELMISO_W {
        PSELMISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for MISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso](index.html) module"]
pub struct MISO_SPEC;
impl crate::RegisterSpec for MISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miso::R](R) reader structure"]
impl crate::Readable for MISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miso::W](W) writer structure"]
impl crate::Writable for MISO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISO to value 0xffff_ffff"]
impl crate::Resettable for MISO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
