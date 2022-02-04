#[doc = "Register `HFXODEBOUNCE` reader"]
pub struct R(crate::R<HFXODEBOUNCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXODEBOUNCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXODEBOUNCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXODEBOUNCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXODEBOUNCE` writer"]
pub struct W(crate::W<HFXODEBOUNCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXODEBOUNCE_SPEC>;
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
impl From<crate::W<HFXODEBOUNCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXODEBOUNCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXODEBOUNCE_A {
    #[doc = "16: 256 us debounce time. Recommended for 1.6 mm x 2.0 mm crystals and larger."]
    DB256US = 16,
    #[doc = "64: 1024 us debounce time. Recommended for 1.6 mm x 1.2 mm crystals and smaller."]
    DB1024US = 64,
}
impl From<HFXODEBOUNCE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXODEBOUNCE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFXODEBOUNCE` reader - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub struct HFXODEBOUNCE_R(crate::FieldReader<u8, HFXODEBOUNCE_A>);
impl HFXODEBOUNCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HFXODEBOUNCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFXODEBOUNCE_A> {
        match self.bits {
            16 => Some(HFXODEBOUNCE_A::DB256US),
            64 => Some(HFXODEBOUNCE_A::DB1024US),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DB256US`"]
    #[inline(always)]
    pub fn is_db256us(&self) -> bool {
        **self == HFXODEBOUNCE_A::DB256US
    }
    #[doc = "Checks if the value of the field is `DB1024US`"]
    #[inline(always)]
    pub fn is_db1024us(&self) -> bool {
        **self == HFXODEBOUNCE_A::DB1024US
    }
}
impl core::ops::Deref for HFXODEBOUNCE_R {
    type Target = crate::FieldReader<u8, HFXODEBOUNCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXODEBOUNCE` writer - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub struct HFXODEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODEBOUNCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXODEBOUNCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 us debounce time. Recommended for 1.6 mm x 2.0 mm crystals and larger."]
    #[inline(always)]
    pub fn db256us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB256US)
    }
    #[doc = "1024 us debounce time. Recommended for 1.6 mm x 1.2 mm crystals and smaller."]
    #[inline(always)]
    pub fn db1024us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB1024US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&self) -> HFXODEBOUNCE_R {
        HFXODEBOUNCE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&mut self) -> HFXODEBOUNCE_W {
        HFXODEBOUNCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxodebounce](index.html) module"]
pub struct HFXODEBOUNCE_SPEC;
impl crate::RegisterSpec for HFXODEBOUNCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxodebounce::R](R) reader structure"]
impl crate::Readable for HFXODEBOUNCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxodebounce::W](W) writer structure"]
impl crate::Writable for HFXODEBOUNCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXODEBOUNCE to value 0x10"]
impl crate::Resettable for HFXODEBOUNCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
