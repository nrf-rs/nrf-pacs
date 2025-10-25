#[doc = "Register `DIN_FIFO_RESET` writer"]
pub type W = crate::W<DinFifoResetSpec>;
#[doc = "Writing any value to this address resets the DIN FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "1: Reset DIN FIFO."]
    Enable = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Writing any value to this address resets the DIN FIFO."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset DIN FIFO."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the DIN FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, DinFifoResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Reset the DIN FIFO, effectively clearing the FIFO for new data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_fifo_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinFifoResetSpec;
impl crate::RegisterSpec for DinFifoResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din_fifo_reset::W`](W) writer structure"]
impl crate::Writable for DinFifoResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIN_FIFO_RESET to value 0"]
impl crate::Resettable for DinFifoResetSpec {}
