#[doc = "Register `TRCTSCTLR` reader"]
pub type R = crate::R<TrctsctlrSpec>;
#[doc = "Register `TRCTSCTLR` writer"]
pub type W = crate::W<TrctsctlrSpec>;
#[doc = "Field `EVENT` reader - Select which event should generate time stamps."]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - Select which event should generate time stamps."]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Select which event should generate time stamps."]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select which event should generate time stamps."]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<'_, TrctsctlrSpec> {
        EventW::new(self, 0)
    }
}
#[doc = "Controls the insertion of global timestamps in the trace streams. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.TS == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trctsctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctsctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrctsctlrSpec;
impl crate::RegisterSpec for TrctsctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trctsctlr::R`](R) reader structure"]
impl crate::Readable for TrctsctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trctsctlr::W`](W) writer structure"]
impl crate::Writable for TrctsctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCTSCTLR to value 0"]
impl crate::Resettable for TrctsctlrSpec {}
