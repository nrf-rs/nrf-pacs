#[doc = "Register `TRNG_RESET` writer"]
pub type W = crate::W<TrngResetSpec>;
#[doc = "Writing any value to this address resets the internal bits counter and registers EHR_DATA and TRNG_VALID. Register NOISE_SOURCE must be disabled in order for the reset to take place.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "1: Reset TRNG."]
    Enable = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Writing any value to this address resets the internal bits counter and registers EHR_DATA and TRNG_VALID. Register NOISE_SOURCE must be disabled in order for the reset to take place."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset TRNG."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the internal bits counter and registers EHR_DATA and TRNG_VALID. Register NOISE_SOURCE must be disabled in order for the reset to take place."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, TrngResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Reset the TRNG, including internal counter of collected bits and registers EHR_DATA and TRNG_VALID.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngResetSpec;
impl crate::RegisterSpec for TrngResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trng_reset::W`](W) writer structure"]
impl crate::Writable for TrngResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_RESET to value 0"]
impl crate::Resettable for TrngResetSpec {}
