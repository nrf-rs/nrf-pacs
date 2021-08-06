#[doc = "Register `PSELCTS` reader"]
pub struct R(crate::R<PSELCTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELCTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELCTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELCTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELCTS` writer"]
pub struct W(crate::W<PSELCTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELCTS_SPEC>;
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
impl From<crate::W<PSELCTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELCTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin number configuration for UART CTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELCTS_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELCTS_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELCTS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSELCTS` reader - Pin number configuration for UART CTS signal"]
pub struct PSELCTS_R(crate::FieldReader<u32, PSELCTS_A>);
impl PSELCTS_R {
    pub(crate) fn new(bits: u32) -> Self {
        PSELCTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELCTS_A> {
        match self.bits {
            4294967295 => Some(PSELCTS_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PSELCTS_A::DISCONNECTED
    }
}
impl core::ops::Deref for PSELCTS_R {
    type Target = crate::FieldReader<u32, PSELCTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSELCTS` writer - Pin number configuration for UART CTS signal"]
pub struct PSELCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELCTS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELCTS_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&self) -> PSELCTS_R {
        PSELCTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&mut self) -> PSELCTS_W {
        PSELCTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselcts](index.html) module"]
pub struct PSELCTS_SPEC;
impl crate::RegisterSpec for PSELCTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselcts::R](R) reader structure"]
impl crate::Readable for PSELCTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselcts::W](W) writer structure"]
impl crate::Writable for PSELCTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELCTS to value 0xffff_ffff"]
impl crate::Resettable for PSELCTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
