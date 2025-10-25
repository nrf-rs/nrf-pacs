#[doc = "Register `PKA_SW_RESET` writer"]
pub type W = crate::W<PkaSwResetSpec>;
#[doc = "Writing any value to this address resets the PKA engine. The reset takes 4 CPU clock cycles to complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "1: Reset PKA engine."]
    Enable = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Writing any value to this address resets the PKA engine. The reset takes 4 CPU clock cycles to complete."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset PKA engine."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the PKA engine. The reset takes 4 CPU clock cycles to complete."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, PkaSwResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Reset the PKA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sw_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSwResetSpec;
impl crate::RegisterSpec for PkaSwResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pka_sw_reset::W`](W) writer structure"]
impl crate::Writable for PkaSwResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_SW_RESET to value 0"]
impl crate::Resettable for PkaSwResetSpec {}
