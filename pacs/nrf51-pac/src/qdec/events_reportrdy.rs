#[doc = "Register `EVENTS_REPORTRDY` reader"]
pub type R = crate::R<EventsReportrdySpec>;
#[doc = "Register `EVENTS_REPORTRDY` writer"]
pub type W = crate::W<EventsReportrdySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "REPORTPER number of samples accumulated in ACC register, and ACC register different than zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_reportrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_reportrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReportrdySpec;
impl crate::RegisterSpec for EventsReportrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_reportrdy::R`](R) reader structure"]
impl crate::Readable for EventsReportrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_reportrdy::W`](W) writer structure"]
impl crate::Writable for EventsReportrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_REPORTRDY to value 0"]
impl crate::Resettable for EventsReportrdySpec {}
