#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALRDY` reader - Disable interrupt on VALRDY event."]
pub type VALRDY_R = crate::BitReader<VALRDY_A>;
#[doc = "Disable interrupt on VALRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALRDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<VALRDY_A> for bool {
    #[inline(always)]
    fn from(variant: VALRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl VALRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALRDY_A {
        match self.bits {
            false => VALRDY_A::DISABLED,
            true => VALRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VALRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VALRDY_A::ENABLED
    }
}
#[doc = "Disable interrupt on VALRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALRDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<VALRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: VALRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALRDY` writer - Disable interrupt on VALRDY event."]
pub type VALRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, VALRDY_AW, O>;
impl<'a, const O: u8> VALRDY_W<'a, O> {
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(VALRDY_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on VALRDY event."]
    #[inline(always)]
    pub fn valrdy(&self) -> VALRDY_R {
        VALRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on VALRDY event."]
    #[inline(always)]
    pub fn valrdy(&mut self) -> VALRDY_W<0> {
        VALRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
