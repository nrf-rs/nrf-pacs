#[doc = "Register `EVENTS_DATARDY` reader"]
pub type R = crate::R<EventsDatardySpec>;
#[doc = "Register `EVENTS_DATARDY` writer"]
pub type W = crate::W<EventsDatardySpec>;
#[doc = "Temperature measurement complete, data ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDatardy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDatardy> for bool {
    #[inline(always)]
    fn from(variant: EventsDatardy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DATARDY` reader - Temperature measurement complete, data ready"]
pub type EventsDatardyR = crate::BitReader<EventsDatardy>;
impl EventsDatardyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDatardy {
        match self.bits {
            false => EventsDatardy::NotGenerated,
            true => EventsDatardy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDatardy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDatardy::Generated
    }
}
#[doc = "Field `EVENTS_DATARDY` writer - Temperature measurement complete, data ready"]
pub type EventsDatardyW<'a, REG> = crate::BitWriter<'a, REG, EventsDatardy>;
impl<'a, REG> EventsDatardyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDatardy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDatardy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature measurement complete, data ready"]
    #[inline(always)]
    pub fn events_datardy(&self) -> EventsDatardyR {
        EventsDatardyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature measurement complete, data ready"]
    #[inline(always)]
    pub fn events_datardy(&mut self) -> EventsDatardyW<'_, EventsDatardySpec> {
        EventsDatardyW::new(self, 0)
    }
}
#[doc = "Temperature measurement complete, data ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_datardy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_datardy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDatardySpec;
impl crate::RegisterSpec for EventsDatardySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_datardy::R`](R) reader structure"]
impl crate::Readable for EventsDatardySpec {}
#[doc = "`write(|w| ..)` method takes [`events_datardy::W`](W) writer structure"]
impl crate::Writable for EventsDatardySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DATARDY to value 0"]
impl crate::Resettable for EventsDatardySpec {}
