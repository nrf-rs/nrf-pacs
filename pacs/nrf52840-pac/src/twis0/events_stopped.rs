#[doc = "Register `EVENTS_STOPPED` reader"]
pub type R = crate::R<EventsStoppedSpec>;
#[doc = "Register `EVENTS_STOPPED` writer"]
pub type W = crate::W<EventsStoppedSpec>;
#[doc = "TWI stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsStopped {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsStopped> for bool {
    #[inline(always)]
    fn from(variant: EventsStopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_STOPPED` reader - TWI stopped"]
pub type EventsStoppedR = crate::BitReader<EventsStopped>;
impl EventsStoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsStopped {
        match self.bits {
            false => EventsStopped::NotGenerated,
            true => EventsStopped::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsStopped::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsStopped::Generated
    }
}
#[doc = "Field `EVENTS_STOPPED` writer - TWI stopped"]
pub type EventsStoppedW<'a, REG> = crate::BitWriter<'a, REG, EventsStopped>;
impl<'a, REG> EventsStoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsStopped::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsStopped::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TWI stopped"]
    #[inline(always)]
    pub fn events_stopped(&self) -> EventsStoppedR {
        EventsStoppedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI stopped"]
    #[inline(always)]
    pub fn events_stopped(&mut self) -> EventsStoppedW<'_, EventsStoppedSpec> {
        EventsStoppedW::new(self, 0)
    }
}
#[doc = "TWI stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsStoppedSpec;
impl crate::RegisterSpec for EventsStoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_stopped::R`](R) reader structure"]
impl crate::Readable for EventsStoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_stopped::W`](W) writer structure"]
impl crate::Writable for EventsStoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_STOPPED to value 0"]
impl crate::Resettable for EventsStoppedSpec {}
