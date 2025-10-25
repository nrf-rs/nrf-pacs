#[doc = "Register `EVENTS_REPORTRDY` reader"]
pub type R = crate::R<EventsReportrdySpec>;
#[doc = "Register `EVENTS_REPORTRDY` writer"]
pub type W = crate::W<EventsReportrdySpec>;
#[doc = "Non-null report ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsReportrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsReportrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsReportrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_REPORTRDY` reader - Non-null report ready"]
pub type EventsReportrdyR = crate::BitReader<EventsReportrdy>;
impl EventsReportrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsReportrdy {
        match self.bits {
            false => EventsReportrdy::NotGenerated,
            true => EventsReportrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsReportrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsReportrdy::Generated
    }
}
#[doc = "Field `EVENTS_REPORTRDY` writer - Non-null report ready"]
pub type EventsReportrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsReportrdy>;
impl<'a, REG> EventsReportrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReportrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReportrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Non-null report ready"]
    #[inline(always)]
    pub fn events_reportrdy(&self) -> EventsReportrdyR {
        EventsReportrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-null report ready"]
    #[inline(always)]
    pub fn events_reportrdy(&mut self) -> EventsReportrdyW<'_, EventsReportrdySpec> {
        EventsReportrdyW::new(self, 0)
    }
}
#[doc = "Non-null report ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_reportrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_reportrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
