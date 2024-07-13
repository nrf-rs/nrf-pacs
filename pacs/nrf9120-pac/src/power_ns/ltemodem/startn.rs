#[doc = "Register `STARTN` reader"]
pub struct R(crate::R<STARTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTN` writer"]
pub struct W(crate::W<STARTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTN_SPEC>;
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
impl From<crate::W<STARTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTN` reader - Start LTE modem"]
pub type STARTN_R = crate::BitReader<STARTN_A>;
#[doc = "Start LTE modem\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTN_A {
    #[doc = "0: Start LTE modem"]
    START = 0,
    #[doc = "1: Hold LTE modem disabled"]
    HOLD = 1,
}
impl From<STARTN_A> for bool {
    #[inline(always)]
    fn from(variant: STARTN_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTN_A {
        match self.bits {
            false => STARTN_A::START,
            true => STARTN_A::HOLD,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTN_A::START
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == STARTN_A::HOLD
    }
}
#[doc = "Field `STARTN` writer - Start LTE modem"]
pub type STARTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTN_SPEC, STARTN_A, O>;
impl<'a, const O: u8> STARTN_W<'a, O> {
    #[doc = "Start LTE modem"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTN_A::START)
    }
    #[doc = "Hold LTE modem disabled"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(STARTN_A::HOLD)
    }
}
impl R {
    #[doc = "Bit 0 - Start LTE modem"]
    #[inline(always)]
    pub fn startn(&self) -> STARTN_R {
        STARTN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start LTE modem"]
    #[inline(always)]
    pub fn startn(&mut self) -> STARTN_W<0> {
        STARTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start LTE modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startn](index.html) module"]
pub struct STARTN_SPEC;
impl crate::RegisterSpec for STARTN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startn::R](R) reader structure"]
impl crate::Readable for STARTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startn::W](W) writer structure"]
impl crate::Writable for STARTN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTN to value 0x01"]
impl crate::Resettable for STARTN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
