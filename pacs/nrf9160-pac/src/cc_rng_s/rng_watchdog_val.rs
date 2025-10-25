#[doc = "Register `RNG_WATCHDOG_VAL` reader"]
pub type R = crate::R<RngWatchdogValSpec>;
#[doc = "Register `RNG_WATCHDOG_VAL` writer"]
pub type W = crate::W<RngWatchdogValSpec>;
#[doc = "Field `RNG_WATCHDOG_VAL` reader - Defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
pub type RngWatchdogValR = crate::FieldReader<u32>;
#[doc = "Field `RNG_WATCHDOG_VAL` writer - Defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
pub type RngWatchdogValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
    #[inline(always)]
    pub fn rng_watchdog_val(&self) -> RngWatchdogValR {
        RngWatchdogValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
    #[inline(always)]
    pub fn rng_watchdog_val(&mut self) -> RngWatchdogValW<'_, RngWatchdogValSpec> {
        RngWatchdogValW::new(self, 0)
    }
}
#[doc = "This register defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_watchdog_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_watchdog_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngWatchdogValSpec;
impl crate::RegisterSpec for RngWatchdogValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_watchdog_val::R`](R) reader structure"]
impl crate::Readable for RngWatchdogValSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_watchdog_val::W`](W) writer structure"]
impl crate::Writable for RngWatchdogValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_WATCHDOG_VAL to value 0"]
impl crate::Resettable for RngWatchdogValSpec {}
