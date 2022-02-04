#[doc = "Register `RATEOVERRIDE` reader"]
pub struct R(crate::R<RATEOVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATEOVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATEOVERRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATEOVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATEOVERRIDE` writer"]
pub struct W(crate::W<RATEOVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATEOVERRIDE_SPEC>;
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
impl From<crate::W<RATEOVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATEOVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data rate override setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATEOVERRIDE_A {
    #[doc = "0: 1 Mbps"]
    _1MBIT = 0,
    #[doc = "1: 2 Mbps"]
    _2MBIT = 1,
    #[doc = "2: 125 Kbps"]
    _125KBPS = 2,
    #[doc = "3: 500 Kbps"]
    _500KBPS = 3,
}
impl From<RATEOVERRIDE_A> for u8 {
    #[inline(always)]
    fn from(variant: RATEOVERRIDE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RATEOVERRIDE` reader - Data rate override setting."]
pub struct RATEOVERRIDE_R(crate::FieldReader<u8, RATEOVERRIDE_A>);
impl RATEOVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RATEOVERRIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATEOVERRIDE_A {
        match self.bits {
            0 => RATEOVERRIDE_A::_1MBIT,
            1 => RATEOVERRIDE_A::_2MBIT,
            2 => RATEOVERRIDE_A::_125KBPS,
            3 => RATEOVERRIDE_A::_500KBPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        **self == RATEOVERRIDE_A::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        **self == RATEOVERRIDE_A::_2MBIT
    }
    #[doc = "Checks if the value of the field is `_125KBPS`"]
    #[inline(always)]
    pub fn is_125kbps(&self) -> bool {
        **self == RATEOVERRIDE_A::_125KBPS
    }
    #[doc = "Checks if the value of the field is `_500KBPS`"]
    #[inline(always)]
    pub fn is_500kbps(&self) -> bool {
        **self == RATEOVERRIDE_A::_500KBPS
    }
}
impl core::ops::Deref for RATEOVERRIDE_R {
    type Target = crate::FieldReader<u8, RATEOVERRIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATEOVERRIDE` writer - Data rate override setting."]
pub struct RATEOVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RATEOVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATEOVERRIDE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_1MBIT)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_2MBIT)
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_125KBPS)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_500KBPS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&self) -> RATEOVERRIDE_R {
        RATEOVERRIDE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&mut self) -> RATEOVERRIDE_W {
        RATEOVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data rate override setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rateoverride](index.html) module"]
pub struct RATEOVERRIDE_SPEC;
impl crate::RegisterSpec for RATEOVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rateoverride::R](R) reader structure"]
impl crate::Readable for RATEOVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rateoverride::W](W) writer structure"]
impl crate::Writable for RATEOVERRIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RATEOVERRIDE to value 0"]
impl crate::Resettable for RATEOVERRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
