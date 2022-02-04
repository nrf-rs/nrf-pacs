#[doc = "Register `INTCAP` reader"]
pub struct R(crate::R<INTCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCAP` writer"]
pub struct W(crate::W<INTCAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCAP_SPEC>;
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
impl From<crate::W<INTCAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control usage of internal load capacitors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTCAP_A {
    #[doc = "0: Use external load capacitors"]
    EXTERNAL = 0,
    #[doc = "1: 6 pF internal load capacitance"]
    C6PF = 1,
    #[doc = "2: 7 pF internal load capacitance"]
    C7PF = 2,
    #[doc = "3: 9 pF internal load capacitance"]
    C9PF = 3,
}
impl From<INTCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: INTCAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTCAP` reader - Control usage of internal load capacitors"]
pub struct INTCAP_R(crate::FieldReader<u8, INTCAP_A>);
impl INTCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTCAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCAP_A {
        match self.bits {
            0 => INTCAP_A::EXTERNAL,
            1 => INTCAP_A::C6PF,
            2 => INTCAP_A::C7PF,
            3 => INTCAP_A::C9PF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == INTCAP_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `C6PF`"]
    #[inline(always)]
    pub fn is_c6pf(&self) -> bool {
        **self == INTCAP_A::C6PF
    }
    #[doc = "Checks if the value of the field is `C7PF`"]
    #[inline(always)]
    pub fn is_c7pf(&self) -> bool {
        **self == INTCAP_A::C7PF
    }
    #[doc = "Checks if the value of the field is `C9PF`"]
    #[inline(always)]
    pub fn is_c9pf(&self) -> bool {
        **self == INTCAP_A::C9PF
    }
}
impl core::ops::Deref for INTCAP_R {
    type Target = crate::FieldReader<u8, INTCAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTCAP` writer - Control usage of internal load capacitors"]
pub struct INTCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCAP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Use external load capacitors"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(INTCAP_A::EXTERNAL)
    }
    #[doc = "6 pF internal load capacitance"]
    #[inline(always)]
    pub fn c6pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C6PF)
    }
    #[doc = "7 pF internal load capacitance"]
    #[inline(always)]
    pub fn c7pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C7PF)
    }
    #[doc = "9 pF internal load capacitance"]
    #[inline(always)]
    pub fn c9pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C9PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&self) -> INTCAP_R {
        INTCAP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&mut self) -> INTCAP_W {
        INTCAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control usage of internal load capacitors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcap](index.html) module"]
pub struct INTCAP_SPEC;
impl crate::RegisterSpec for INTCAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intcap::R](R) reader structure"]
impl crate::Readable for INTCAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intcap::W](W) writer structure"]
impl crate::Writable for INTCAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCAP to value 0"]
impl crate::Resettable for INTCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
