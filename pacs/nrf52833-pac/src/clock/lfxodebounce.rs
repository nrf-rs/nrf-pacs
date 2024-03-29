#[doc = "Register `LFXODEBOUNCE` reader"]
pub struct R(crate::R<LFXODEBOUNCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFXODEBOUNCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFXODEBOUNCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFXODEBOUNCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFXODEBOUNCE` writer"]
pub struct W(crate::W<LFXODEBOUNCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFXODEBOUNCE_SPEC>;
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
impl From<crate::W<LFXODEBOUNCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFXODEBOUNCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFXODEBOUNCE` reader - LFXO debounce time."]
pub type LFXODEBOUNCE_R = crate::BitReader<LFXODEBOUNCE_A>;
#[doc = "LFXO debounce time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXODEBOUNCE_A {
    #[doc = "0: 8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    NORMAL = 0,
    #[doc = "1: 16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    EXTENDED = 1,
}
impl From<LFXODEBOUNCE_A> for bool {
    #[inline(always)]
    fn from(variant: LFXODEBOUNCE_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXODEBOUNCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXODEBOUNCE_A {
        match self.bits {
            false => LFXODEBOUNCE_A::NORMAL,
            true => LFXODEBOUNCE_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LFXODEBOUNCE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LFXODEBOUNCE_A::EXTENDED
    }
}
#[doc = "Field `LFXODEBOUNCE` writer - LFXO debounce time."]
pub type LFXODEBOUNCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LFXODEBOUNCE_SPEC, LFXODEBOUNCE_A, O>;
impl<'a, const O: u8> LFXODEBOUNCE_W<'a, O> {
    #[doc = "8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LFXODEBOUNCE_A::NORMAL)
    }
    #[doc = "16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LFXODEBOUNCE_A::EXTENDED)
    }
}
impl R {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&self) -> LFXODEBOUNCE_R {
        LFXODEBOUNCE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&mut self) -> LFXODEBOUNCE_W<0> {
        LFXODEBOUNCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFXO debounce time. The LFXO is started by triggering the TASKS_LFCLKSTART task when the LFCLKSRC register is configured for Xtal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfxodebounce](index.html) module"]
pub struct LFXODEBOUNCE_SPEC;
impl crate::RegisterSpec for LFXODEBOUNCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfxodebounce::R](R) reader structure"]
impl crate::Readable for LFXODEBOUNCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfxodebounce::W](W) writer structure"]
impl crate::Writable for LFXODEBOUNCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFXODEBOUNCE to value 0"]
impl crate::Resettable for LFXODEBOUNCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
