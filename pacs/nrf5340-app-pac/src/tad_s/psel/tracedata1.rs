#[doc = "Register `TRACEDATA1` reader"]
pub struct R(crate::R<TRACEDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEDATA1` writer"]
pub struct W(crate::W<TRACEDATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEDATA1_SPEC>;
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
impl From<crate::W<TRACEDATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEDATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin number\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIN_A {
    #[doc = "10: TRACEDATA1 pin"]
    TRACEDATA1 = 10,
}
impl From<PIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PIN` reader - Pin number"]
pub struct PIN_R(crate::FieldReader<u8, PIN_A>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIN_A> {
        match self.bits {
            10 => Some(PIN_A::TRACEDATA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRACEDATA1`"]
    #[inline(always)]
    pub fn is_tracedata1(&self) -> bool {
        **self == PIN_A::TRACEDATA1
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<u8, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN` writer - Pin number"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TRACEDATA1 pin"]
    #[inline(always)]
    pub fn tracedata1(self) -> &'a mut W {
        self.variant(PIN_A::TRACEDATA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
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
#[doc = "Field `CONNECT` reader - Connection"]
pub struct CONNECT_R(crate::FieldReader<bool, CONNECT_A>);
impl CONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONNECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CONNECT_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == CONNECT_A::CONNECTED
    }
}
impl core::ops::Deref for CONNECT_R {
    type Target = crate::FieldReader<bool, CONNECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub struct CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONNECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W {
        CONNECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin configuration for TRACEDATA\\[1\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tracedata1](index.html) module"]
pub struct TRACEDATA1_SPEC;
impl crate::RegisterSpec for TRACEDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tracedata1::R](R) reader structure"]
impl crate::Readable for TRACEDATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tracedata1::W](W) writer structure"]
impl crate::Writable for TRACEDATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACEDATA1 to value 0xffff_ffff"]
impl crate::Resettable for TRACEDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
