#[doc = "Register `MOSI` reader"]
pub struct R(crate::R<MOSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSI` writer"]
pub struct W(crate::W<MOSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSI_SPEC>;
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
impl From<crate::W<MOSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Pin number"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - Pin number"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOSI_SPEC, u8, u8, 5, O>;
#[doc = "Field `PORT` reader - Port number"]
pub type PORT_R = crate::BitReader<bool>;
#[doc = "Field `PORT` writer - Port number"]
pub type PORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOSI_SPEC, bool, O>;
#[doc = "Field `CONNECT` reader - Connection"]
pub type CONNECT_R = crate::BitReader<CONNECT_A>;
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECT_A {
    #[doc = "1: Disconnect"]
    DISCONNECTED = 1,
    #[doc = "0: Connect"]
    CONNECTED = 0,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONNECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            true => CONNECT_A::DISCONNECTED,
            false => CONNECT_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CONNECT_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CONNECT_A::CONNECTED
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type CONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOSI_SPEC, CONNECT_A, O>;
impl<'a, const O: u8> CONNECT_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CONNECT_A::DISCONNECTED)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(CONNECT_A::CONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Port number"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W<0> {
        PIN_W::new(self)
    }
    #[doc = "Bit 5 - Port number"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W<5> {
        PORT_W::new(self)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W<31> {
        CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for MOSI signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi](index.html) module"]
pub struct MOSI_SPEC;
impl crate::RegisterSpec for MOSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mosi::R](R) reader structure"]
impl crate::Readable for MOSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosi::W](W) writer structure"]
impl crate::Writable for MOSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOSI to value 0xffff_ffff"]
impl crate::Resettable for MOSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
