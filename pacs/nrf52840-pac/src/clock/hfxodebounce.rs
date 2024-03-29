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
#[doc = "Field `HFXODEBOUNCE` reader - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub type HFXODEBOUNCE_R = crate::FieldReader<u8, HFXODEBOUNCE_A>;
#[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXODEBOUNCE_A {
    #[doc = "16: 256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    DB256US = 16,
    #[doc = "64: 1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    DB1024US = 64,
}
impl From<HFXODEBOUNCE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXODEBOUNCE_A) -> Self {
        variant as _
    }
}
impl HFXODEBOUNCE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == HFXODEBOUNCE_A::DB256US
    }
    #[doc = "Checks if the value of the field is `DB1024US`"]
    #[inline(always)]
    pub fn is_db1024us(&self) -> bool {
        *self == HFXODEBOUNCE_A::DB1024US
    }
}
#[doc = "Field `HFXODEBOUNCE` writer - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub type HFXODEBOUNCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HFXODEBOUNCE_SPEC, u8, HFXODEBOUNCE_A, 8, O>;
impl<'a, const O: u8> HFXODEBOUNCE_W<'a, O> {
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    #[inline(always)]
    pub fn db256us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB256US)
    }
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    #[inline(always)]
    pub fn db1024us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB1024US)
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
    pub fn hfxodebounce(&mut self) -> HFXODEBOUNCE_W<0> {
        HFXODEBOUNCE_W::new(self)
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
