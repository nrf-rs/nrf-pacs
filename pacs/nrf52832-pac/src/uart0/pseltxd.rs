#[doc = "Register `PSELTXD` reader"]
pub struct R(crate::R<PSELTXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELTXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELTXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELTXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELTXD` writer"]
pub struct W(crate::W<PSELTXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELTXD_SPEC>;
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
impl From<crate::W<PSELTXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELTXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin number configuration for UART TXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELTXD_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELTXD_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELTXD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSELTXD` reader - Pin number configuration for UART TXD signal"]
pub struct PSELTXD_R(crate::FieldReader<u32, PSELTXD_A>);
impl PSELTXD_R {
    pub(crate) fn new(bits: u32) -> Self {
        PSELTXD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELTXD_A> {
        match self.bits {
            4294967295 => Some(PSELTXD_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PSELTXD_A::DISCONNECTED
    }
}
impl core::ops::Deref for PSELTXD_R {
    type Target = crate::FieldReader<u32, PSELTXD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSELTXD` writer - Pin number configuration for UART TXD signal"]
pub struct PSELTXD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELTXD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELTXD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELTXD_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&self) -> PSELTXD_R {
        PSELTXD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&mut self) -> PSELTXD_W {
        PSELTXD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pseltxd](index.html) module"]
pub struct PSELTXD_SPEC;
impl crate::RegisterSpec for PSELTXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pseltxd::R](R) reader structure"]
impl crate::Readable for PSELTXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pseltxd::W](W) writer structure"]
impl crate::Writable for PSELTXD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELTXD to value 0xffff_ffff"]
impl crate::Resettable for PSELTXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
