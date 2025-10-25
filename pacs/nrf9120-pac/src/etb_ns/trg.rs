#[doc = "Register `TRG` reader"]
pub type R = crate::R<TrgSpec>;
#[doc = "Register `TRG` writer"]
pub type W = crate::W<TrgSpec>;
#[doc = "Field `TRIGGER_COUNTER` reader - The counter is used as follows:Trace after - The counter is set to a large value, slightly less than the number of entries in the RAM. Trace before - The counter is set to a small value. Trace about - The counter is set to half the depth of the Trace RAM. This register must not be written to when trace capture is enabled (FtStopped=0, TraceCaptEn=1). If a write is attempted, the register is not updated. A read access is permitted with trace capture enabled."]
pub type TriggerCounterR = crate::FieldReader<u16>;
#[doc = "Field `TRIGGER_COUNTER` writer - The counter is used as follows:Trace after - The counter is set to a large value, slightly less than the number of entries in the RAM. Trace before - The counter is set to a small value. Trace about - The counter is set to half the depth of the Trace RAM. This register must not be written to when trace capture is enabled (FtStopped=0, TraceCaptEn=1). If a write is attempted, the register is not updated. A read access is permitted with trace capture enabled."]
pub type TriggerCounterW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The counter is used as follows:Trace after - The counter is set to a large value, slightly less than the number of entries in the RAM. Trace before - The counter is set to a small value. Trace about - The counter is set to half the depth of the Trace RAM. This register must not be written to when trace capture is enabled (FtStopped=0, TraceCaptEn=1). If a write is attempted, the register is not updated. A read access is permitted with trace capture enabled."]
    #[inline(always)]
    pub fn trigger_counter(&self) -> TriggerCounterR {
        TriggerCounterR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The counter is used as follows:Trace after - The counter is set to a large value, slightly less than the number of entries in the RAM. Trace before - The counter is set to a small value. Trace about - The counter is set to half the depth of the Trace RAM. This register must not be written to when trace capture is enabled (FtStopped=0, TraceCaptEn=1). If a write is attempted, the register is not updated. A read access is permitted with trace capture enabled."]
    #[inline(always)]
    pub fn trigger_counter(&mut self) -> TriggerCounterW<'_, TrgSpec> {
        TriggerCounterW::new(self, 0)
    }
}
#[doc = "ETB Trigger Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrgSpec;
impl crate::RegisterSpec for TrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trg::R`](R) reader structure"]
impl crate::Readable for TrgSpec {}
#[doc = "`write(|w| ..)` method takes [`trg::W`](W) writer structure"]
impl crate::Writable for TrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRG to value 0"]
impl crate::Resettable for TrgSpec {}
