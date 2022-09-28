#[doc = "Register `ISOINCONFIG` reader"]
pub struct R(crate::R<ISOINCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOINCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOINCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOINCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOINCONFIG` writer"]
pub struct W(crate::W<ISOINCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOINCONFIG_SPEC>;
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
impl From<crate::W<ISOINCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOINCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE` reader - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub type RESPONSE_R = crate::BitReader<RESPONSE_A>;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPONSE_A {
    #[doc = "0: Endpoint does not respond in that case"]
    NO_RESP = 0,
    #[doc = "1: Endpoint responds with a zero-length data packet in that case"]
    ZERO_DATA = 1,
}
impl From<RESPONSE_A> for bool {
    #[inline(always)]
    fn from(variant: RESPONSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESPONSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPONSE_A {
        match self.bits {
            false => RESPONSE_A::NO_RESP,
            true => RESPONSE_A::ZERO_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESP`"]
    #[inline(always)]
    pub fn is_no_resp(&self) -> bool {
        *self == RESPONSE_A::NO_RESP
    }
    #[doc = "Checks if the value of the field is `ZERO_DATA`"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == RESPONSE_A::ZERO_DATA
    }
}
#[doc = "Field `RESPONSE` writer - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub type RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISOINCONFIG_SPEC, RESPONSE_A, O>;
impl<'a, const O: u8> RESPONSE_W<'a, O> {
    #[doc = "Endpoint does not respond in that case"]
    #[inline(always)]
    pub fn no_resp(self) -> &'a mut W {
        self.variant(RESPONSE_A::NO_RESP)
    }
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    #[inline(always)]
    pub fn zero_data(self) -> &'a mut W {
        self.variant(RESPONSE_A::ZERO_DATA)
    }
}
impl R {
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&mut self) -> RESPONSE_W<0> {
        RESPONSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoinconfig](index.html) module"]
pub struct ISOINCONFIG_SPEC;
impl crate::RegisterSpec for ISOINCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoinconfig::R](R) reader structure"]
impl crate::Readable for ISOINCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isoinconfig::W](W) writer structure"]
impl crate::Writable for ISOINCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISOINCONFIG to value 0"]
impl crate::Resettable for ISOINCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
