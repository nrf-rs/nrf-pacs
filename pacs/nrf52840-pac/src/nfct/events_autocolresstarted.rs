#[doc = "Register `EVENTS_AUTOCOLRESSTARTED` reader"]
pub type R = crate::R<EventsAutocolresstartedSpec>;
#[doc = "Register `EVENTS_AUTOCOLRESSTARTED` writer"]
pub type W = crate::W<EventsAutocolresstartedSpec>;
#[doc = "Auto collision resolution process has started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsAutocolresstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsAutocolresstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsAutocolresstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_AUTOCOLRESSTARTED` reader - Auto collision resolution process has started"]
pub type EventsAutocolresstartedR = crate::BitReader<EventsAutocolresstarted>;
impl EventsAutocolresstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsAutocolresstarted {
        match self.bits {
            false => EventsAutocolresstarted::NotGenerated,
            true => EventsAutocolresstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsAutocolresstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsAutocolresstarted::Generated
    }
}
#[doc = "Field `EVENTS_AUTOCOLRESSTARTED` writer - Auto collision resolution process has started"]
pub type EventsAutocolresstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsAutocolresstarted>;
impl<'a, REG> EventsAutocolresstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAutocolresstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAutocolresstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Auto collision resolution process has started"]
    #[inline(always)]
    pub fn events_autocolresstarted(&self) -> EventsAutocolresstartedR {
        EventsAutocolresstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto collision resolution process has started"]
    #[inline(always)]
    pub fn events_autocolresstarted(
        &mut self,
    ) -> EventsAutocolresstartedW<'_, EventsAutocolresstartedSpec> {
        EventsAutocolresstartedW::new(self, 0)
    }
}
#[doc = "Auto collision resolution process has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_autocolresstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_autocolresstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAutocolresstartedSpec;
impl crate::RegisterSpec for EventsAutocolresstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_autocolresstarted::R`](R) reader structure"]
impl crate::Readable for EventsAutocolresstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_autocolresstarted::W`](W) writer structure"]
impl crate::Writable for EventsAutocolresstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_AUTOCOLRESSTARTED to value 0"]
impl crate::Resettable for EventsAutocolresstartedSpec {}
