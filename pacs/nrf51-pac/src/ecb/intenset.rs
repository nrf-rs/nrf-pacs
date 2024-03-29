#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDECB` reader - Enable interrupt on ENDECB event."]
pub type ENDECB_R = crate::BitReader<ENDECB_A>;
#[doc = "Enable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ENDECB_A> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDECB_A {
        match self.bits {
            false => ENDECB_A::DISABLED,
            true => ENDECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDECB_A::ENABLED
    }
}
#[doc = "Enable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<ENDECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Enable interrupt on ENDECB event."]
pub type ENDECB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ENDECB_AW, O>;
impl<'a, const O: u8> ENDECB_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDECB_AW::SET)
    }
}
#[doc = "Field `ERRORECB` reader - Enable interrupt on ERRORECB event."]
pub type ERRORECB_R = crate::BitReader<ERRORECB_A>;
#[doc = "Enable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRORECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORECB_A {
        match self.bits {
            false => ERRORECB_A::DISABLED,
            true => ERRORECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRORECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRORECB_A::ENABLED
    }
}
#[doc = "Enable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<ERRORECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Enable interrupt on ERRORECB event."]
pub type ERRORECB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ERRORECB_AW, O>;
impl<'a, const O: u8> ERRORECB_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERRORECB_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&self) -> ENDECB_R {
        ENDECB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&self) -> ERRORECB_R {
        ERRORECB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&mut self) -> ENDECB_W<0> {
        ENDECB_W::new(self)
    }
    #[doc = "Bit 1 - Enable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ERRORECB_W<1> {
        ERRORECB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable set register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
