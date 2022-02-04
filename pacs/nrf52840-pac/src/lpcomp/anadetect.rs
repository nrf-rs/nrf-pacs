#[doc = "Register `ANADETECT` reader"]
pub struct R(crate::R<ANADETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANADETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANADETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANADETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANADETECT` writer"]
pub struct W(crate::W<ANADETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANADETECT_SPEC>;
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
impl From<crate::W<ANADETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANADETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog detect configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANADETECT_A {
    #[doc = "0: Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS = 0,
    #[doc = "1: Generate ANADETECT on upward crossing only"]
    UP = 1,
    #[doc = "2: Generate ANADETECT on downward crossing only"]
    DOWN = 2,
}
impl From<ANADETECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANADETECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ANADETECT` reader - Analog detect configuration"]
pub struct ANADETECT_R(crate::FieldReader<u8, ANADETECT_A>);
impl ANADETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ANADETECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANADETECT_A> {
        match self.bits {
            0 => Some(ANADETECT_A::CROSS),
            1 => Some(ANADETECT_A::UP),
            2 => Some(ANADETECT_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CROSS`"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        **self == ANADETECT_A::CROSS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        **self == ANADETECT_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        **self == ANADETECT_A::DOWN
    }
}
impl core::ops::Deref for ANADETECT_R {
    type Target = crate::FieldReader<u8, ANADETECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANADETECT` writer - Analog detect configuration"]
pub struct ANADETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ANADETECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANADETECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut W {
        self.variant(ANADETECT_A::CROSS)
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(ANADETECT_A::UP)
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(ANADETECT_A::DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&self) -> ANADETECT_R {
        ANADETECT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&mut self) -> ANADETECT_W {
        ANADETECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog detect configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anadetect](index.html) module"]
pub struct ANADETECT_SPEC;
impl crate::RegisterSpec for ANADETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anadetect::R](R) reader structure"]
impl crate::Readable for ANADETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anadetect::W](W) writer structure"]
impl crate::Writable for ANADETECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANADETECT to value 0"]
impl crate::Resettable for ANADETECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
