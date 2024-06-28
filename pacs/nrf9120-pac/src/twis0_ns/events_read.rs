#[doc = "Register `EVENTS_READ` reader"]
pub type R = crate::R<EventsReadSpec>;
#[doc = "Register `EVENTS_READ` writer"]
pub type W = crate::W<EventsReadSpec>;
#[doc = "Read command received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRead {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRead> for bool {
    #[inline(always)]
    fn from(variant: EventsRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_READ` reader - Read command received"]
pub type EventsReadR = crate::BitReader<EventsRead>;
impl EventsReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRead {
        match self.bits {
            false => EventsRead::NotGenerated,
            true => EventsRead::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRead::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRead::Generated
    }
}
#[doc = "Field `EVENTS_READ` writer - Read command received"]
pub type EventsReadW<'a, REG> = crate::BitWriter<'a, REG, EventsRead>;
impl<'a, REG> EventsReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRead::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRead::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Read command received"]
    #[inline(always)]
    pub fn events_read(&self) -> EventsReadR {
        EventsReadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read command received"]
    #[inline(always)]
    #[must_use]
    pub fn events_read(&mut self) -> EventsReadW<EventsReadSpec> {
        EventsReadW::new(self, 0)
    }
}
#[doc = "Read command received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_read::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_read::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReadSpec;
impl crate::RegisterSpec for EventsReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_read::R`](R) reader structure"]
impl crate::Readable for EventsReadSpec {}
#[doc = "`write(|w| ..)` method takes [`events_read::W`](W) writer structure"]
impl crate::Writable for EventsReadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_READ to value 0"]
impl crate::Resettable for EventsReadSpec {
    const RESET_VALUE: u32 = 0;
}
