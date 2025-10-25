#[doc = "Register `LFXODEBOUNCE` reader"]
pub type R = crate::R<LfxodebounceSpec>;
#[doc = "Register `LFXODEBOUNCE` writer"]
pub type W = crate::W<LfxodebounceSpec>;
#[doc = "LFXO debounce time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxodebounce {
    #[doc = "0: 8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    Normal = 0,
    #[doc = "1: 16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    Extended = 1,
}
impl From<Lfxodebounce> for bool {
    #[inline(always)]
    fn from(variant: Lfxodebounce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXODEBOUNCE` reader - LFXO debounce time."]
pub type LfxodebounceR = crate::BitReader<Lfxodebounce>;
impl LfxodebounceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxodebounce {
        match self.bits {
            false => Lfxodebounce::Normal,
            true => Lfxodebounce::Extended,
        }
    }
    #[doc = "8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Lfxodebounce::Normal
    }
    #[doc = "16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Lfxodebounce::Extended
    }
}
#[doc = "Field `LFXODEBOUNCE` writer - LFXO debounce time."]
pub type LfxodebounceW<'a, REG> = crate::BitWriter<'a, REG, Lfxodebounce>;
impl<'a, REG> LfxodebounceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxodebounce::Normal)
    }
    #[doc = "16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxodebounce::Extended)
    }
}
impl R {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&self) -> LfxodebounceR {
        LfxodebounceR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&mut self) -> LfxodebounceW<'_, LfxodebounceSpec> {
        LfxodebounceW::new(self, 0)
    }
}
#[doc = "LFXO debounce time. The LFXO is started by triggering the TASKS_LFCLKSTART task when the LFCLKSRC register is configured for Xtal.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfxodebounce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfxodebounce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfxodebounceSpec;
impl crate::RegisterSpec for LfxodebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfxodebounce::R`](R) reader structure"]
impl crate::Readable for LfxodebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`lfxodebounce::W`](W) writer structure"]
impl crate::Writable for LfxodebounceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFXODEBOUNCE to value 0"]
impl crate::Resettable for LfxodebounceSpec {}
