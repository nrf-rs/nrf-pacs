#[doc = "Register `OVERSAMPLE` reader"]
pub type R = crate::R<OversampleSpec>;
#[doc = "Register `OVERSAMPLE` writer"]
pub type W = crate::W<OversampleSpec>;
#[doc = "Oversample control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oversample {
    #[doc = "0: Bypass oversampling"]
    Bypass = 0,
    #[doc = "1: Oversample 2x"]
    Over2x = 1,
    #[doc = "2: Oversample 4x"]
    Over4x = 2,
    #[doc = "3: Oversample 8x"]
    Over8x = 3,
    #[doc = "4: Oversample 16x"]
    Over16x = 4,
    #[doc = "5: Oversample 32x"]
    Over32x = 5,
    #[doc = "6: Oversample 64x"]
    Over64x = 6,
    #[doc = "7: Oversample 128x"]
    Over128x = 7,
    #[doc = "8: Oversample 256x"]
    Over256x = 8,
}
impl From<Oversample> for u8 {
    #[inline(always)]
    fn from(variant: Oversample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oversample {
    type Ux = u8;
}
impl crate::IsEnum for Oversample {}
#[doc = "Field `OVERSAMPLE` reader - Oversample control"]
pub type OversampleR = crate::FieldReader<Oversample>;
impl OversampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Oversample> {
        match self.bits {
            0 => Some(Oversample::Bypass),
            1 => Some(Oversample::Over2x),
            2 => Some(Oversample::Over4x),
            3 => Some(Oversample::Over8x),
            4 => Some(Oversample::Over16x),
            5 => Some(Oversample::Over32x),
            6 => Some(Oversample::Over64x),
            7 => Some(Oversample::Over128x),
            8 => Some(Oversample::Over256x),
            _ => None,
        }
    }
    #[doc = "Bypass oversampling"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Oversample::Bypass
    }
    #[doc = "Oversample 2x"]
    #[inline(always)]
    pub fn is_over2x(&self) -> bool {
        *self == Oversample::Over2x
    }
    #[doc = "Oversample 4x"]
    #[inline(always)]
    pub fn is_over4x(&self) -> bool {
        *self == Oversample::Over4x
    }
    #[doc = "Oversample 8x"]
    #[inline(always)]
    pub fn is_over8x(&self) -> bool {
        *self == Oversample::Over8x
    }
    #[doc = "Oversample 16x"]
    #[inline(always)]
    pub fn is_over16x(&self) -> bool {
        *self == Oversample::Over16x
    }
    #[doc = "Oversample 32x"]
    #[inline(always)]
    pub fn is_over32x(&self) -> bool {
        *self == Oversample::Over32x
    }
    #[doc = "Oversample 64x"]
    #[inline(always)]
    pub fn is_over64x(&self) -> bool {
        *self == Oversample::Over64x
    }
    #[doc = "Oversample 128x"]
    #[inline(always)]
    pub fn is_over128x(&self) -> bool {
        *self == Oversample::Over128x
    }
    #[doc = "Oversample 256x"]
    #[inline(always)]
    pub fn is_over256x(&self) -> bool {
        *self == Oversample::Over256x
    }
}
#[doc = "Field `OVERSAMPLE` writer - Oversample control"]
pub type OversampleW<'a, REG> = crate::FieldWriter<'a, REG, 4, Oversample>;
impl<'a, REG> OversampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass oversampling"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Bypass)
    }
    #[doc = "Oversample 2x"]
    #[inline(always)]
    pub fn over2x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over2x)
    }
    #[doc = "Oversample 4x"]
    #[inline(always)]
    pub fn over4x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over4x)
    }
    #[doc = "Oversample 8x"]
    #[inline(always)]
    pub fn over8x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over8x)
    }
    #[doc = "Oversample 16x"]
    #[inline(always)]
    pub fn over16x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over16x)
    }
    #[doc = "Oversample 32x"]
    #[inline(always)]
    pub fn over32x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over32x)
    }
    #[doc = "Oversample 64x"]
    #[inline(always)]
    pub fn over64x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over64x)
    }
    #[doc = "Oversample 128x"]
    #[inline(always)]
    pub fn over128x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over128x)
    }
    #[doc = "Oversample 256x"]
    #[inline(always)]
    pub fn over256x(self) -> &'a mut crate::W<REG> {
        self.variant(Oversample::Over256x)
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&self) -> OversampleR {
        OversampleR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&mut self) -> OversampleW<'_, OversampleSpec> {
        OversampleW::new(self, 0)
    }
}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used.\n\nYou can [`read`](crate::Reg::read) this register and get [`oversample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oversample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OversampleSpec;
impl crate::RegisterSpec for OversampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oversample::R`](R) reader structure"]
impl crate::Readable for OversampleSpec {}
#[doc = "`write(|w| ..)` method takes [`oversample::W`](W) writer structure"]
impl crate::Writable for OversampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERSAMPLE to value 0"]
impl crate::Resettable for OversampleSpec {}
