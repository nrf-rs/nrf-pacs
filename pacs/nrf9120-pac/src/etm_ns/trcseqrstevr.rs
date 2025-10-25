#[doc = "Register `TRCSEQRSTEVR` reader"]
pub type R = crate::R<TrcseqrstevrSpec>;
#[doc = "Register `TRCSEQRSTEVR` writer"]
pub type W = crate::W<TrcseqrstevrSpec>;
#[doc = "Field `EVENT` reader - Select which event should reset the sequencer."]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - Select which event should reset the sequencer."]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Select which event should reset the sequencer."]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select which event should reset the sequencer."]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<'_, TrcseqrstevrSpec> {
        EventW::new(self, 0)
    }
}
#[doc = "Moves the sequencer to state 0 when a programmed event occurs. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqrstevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqrstevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcseqrstevrSpec;
impl crate::RegisterSpec for TrcseqrstevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcseqrstevr::R`](R) reader structure"]
impl crate::Readable for TrcseqrstevrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcseqrstevr::W`](W) writer structure"]
impl crate::Writable for TrcseqrstevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSEQRSTEVR to value 0"]
impl crate::Resettable for TrcseqrstevrSpec {}
