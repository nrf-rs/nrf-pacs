#[doc = "Register `ISOURCE` reader"]
pub struct R(crate::R<ISOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOURCE` writer"]
pub struct W(crate::W<ISOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOURCE_SPEC>;
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
impl From<crate::W<ISOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISOURCE_A {
    #[doc = "0: Current source disabled"]
    OFF = 0,
    #[doc = "1: Current source enabled (+/- 2.5 uA)"]
    IEN2MA5 = 1,
    #[doc = "2: Current source enabled (+/- 5 uA)"]
    IEN5MA = 2,
    #[doc = "3: Current source enabled (+/- 10 uA)"]
    IEN10MA = 3,
}
impl From<ISOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ISOURCE` reader - Comparator hysteresis"]
pub struct ISOURCE_R(crate::FieldReader<u8, ISOURCE_A>);
impl ISOURCE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ISOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOURCE_A {
        match self.bits {
            0 => ISOURCE_A::OFF,
            1 => ISOURCE_A::IEN2MA5,
            2 => ISOURCE_A::IEN5MA,
            3 => ISOURCE_A::IEN10MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == ISOURCE_A::OFF
    }
    #[doc = "Checks if the value of the field is `IEN2MA5`"]
    #[inline(always)]
    pub fn is_ien2m_a5(&self) -> bool {
        **self == ISOURCE_A::IEN2MA5
    }
    #[doc = "Checks if the value of the field is `IEN5MA`"]
    #[inline(always)]
    pub fn is_ien5m_a(&self) -> bool {
        **self == ISOURCE_A::IEN5MA
    }
    #[doc = "Checks if the value of the field is `IEN10MA`"]
    #[inline(always)]
    pub fn is_ien10m_a(&self) -> bool {
        **self == ISOURCE_A::IEN10MA
    }
}
impl core::ops::Deref for ISOURCE_R {
    type Target = crate::FieldReader<u8, ISOURCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOURCE` writer - Comparator hysteresis"]
pub struct ISOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOURCE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Current source disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ISOURCE_A::OFF)
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline(always)]
    pub fn ien2m_a5(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN2MA5)
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline(always)]
    pub fn ien5m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN5MA)
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline(always)]
    pub fn ien10m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN10MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    pub fn isource(&self) -> ISOURCE_R {
        ISOURCE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    pub fn isource(&mut self) -> ISOURCE_W {
        ISOURCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current source select on analog input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isource](index.html) module"]
pub struct ISOURCE_SPEC;
impl crate::RegisterSpec for ISOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isource::R](R) reader structure"]
impl crate::Readable for ISOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isource::W](W) writer structure"]
impl crate::Writable for ISOURCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISOURCE to value 0"]
impl crate::Resettable for ISOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
