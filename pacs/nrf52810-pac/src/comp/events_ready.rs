#[doc = "Register `EVENTS_READY` reader"]
pub type R = crate::R<EventsReadySpec>;
#[doc = "Register `EVENTS_READY` writer"]
pub type W = crate::W<EventsReadySpec>;
#[doc = "COMP is ready and output is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsReady {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsReady> for bool {
    #[inline(always)]
    fn from(variant: EventsReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_READY` reader - COMP is ready and output is valid"]
pub type EventsReadyR = crate::BitReader<EventsReady>;
impl EventsReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsReady {
        match self.bits {
            false => EventsReady::NotGenerated,
            true => EventsReady::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsReady::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsReady::Generated
    }
}
#[doc = "Field `EVENTS_READY` writer - COMP is ready and output is valid"]
pub type EventsReadyW<'a, REG> = crate::BitWriter<'a, REG, EventsReady>;
impl<'a, REG> EventsReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReady::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReady::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - COMP is ready and output is valid"]
    #[inline(always)]
    pub fn events_ready(&self) -> EventsReadyR {
        EventsReadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP is ready and output is valid"]
    #[inline(always)]
    pub fn events_ready(&mut self) -> EventsReadyW<'_, EventsReadySpec> {
        EventsReadyW::new(self, 0)
    }
}
#[doc = "COMP is ready and output is valid\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReadySpec;
impl crate::RegisterSpec for EventsReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ready::R`](R) reader structure"]
impl crate::Readable for EventsReadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_ready::W`](W) writer structure"]
impl crate::Writable for EventsReadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_READY to value 0"]
impl crate::Resettable for EventsReadySpec {}
