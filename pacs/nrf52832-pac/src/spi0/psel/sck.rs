#[doc = "Register `SCK` reader"]
pub struct R(crate::R<SCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCK` writer"]
pub struct W(crate::W<SCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCK_SPEC>;
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
impl From<crate::W<SCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin number configuration for SPI SCK signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELSCK_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELSCK_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELSCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSELSCK` reader - Pin number configuration for SPI SCK signal"]
pub struct PSELSCK_R(crate::FieldReader<u32, PSELSCK_A>);
impl PSELSCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PSELSCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELSCK_A> {
        match self.bits {
            4294967295 => Some(PSELSCK_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PSELSCK_A::DISCONNECTED
    }
}
impl core::ops::Deref for PSELSCK_R {
    type Target = crate::FieldReader<u32, PSELSCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSELSCK` writer - Pin number configuration for SPI SCK signal"]
pub struct PSELSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELSCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELSCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSCK_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&self) -> PSELSCK_R {
        PSELSCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&mut self) -> PSELSCK_W {
        PSELSCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for SCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck](index.html) module"]
pub struct SCK_SPEC;
impl crate::RegisterSpec for SCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sck::R](R) reader structure"]
impl crate::Readable for SCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sck::W](W) writer structure"]
impl crate::Writable for SCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCK to value 0xffff_ffff"]
impl crate::Resettable for SCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
