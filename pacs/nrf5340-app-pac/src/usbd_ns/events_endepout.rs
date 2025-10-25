#[doc = "Register `EVENTS_ENDEPOUT[%s]` reader"]
pub type R = crate::R<EventsEndepoutSpec>;
#[doc = "Register `EVENTS_ENDEPOUT[%s]` writer"]
pub type W = crate::W<EventsEndepoutSpec>;
#[doc = "The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndepout {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndepout> for bool {
    #[inline(always)]
    fn from(variant: EventsEndepout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDEPOUT` reader - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndepoutR = crate::BitReader<EventsEndepout>;
impl EventsEndepoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndepout {
        match self.bits {
            false => EventsEndepout::NotGenerated,
            true => EventsEndepout::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndepout::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndepout::Generated
    }
}
#[doc = "Field `EVENTS_ENDEPOUT` writer - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndepoutW<'a, REG> = crate::BitWriter<'a, REG, EventsEndepout>;
impl<'a, REG> EventsEndepoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndepout::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndepout::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&self) -> EventsEndepoutR {
        EventsEndepoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&mut self) -> EventsEndepoutW<'_, EventsEndepoutSpec> {
        EventsEndepoutW::new(self, 0)
    }
}
#[doc = "Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endepout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endepout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndepoutSpec;
impl crate::RegisterSpec for EventsEndepoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endepout::R`](R) reader structure"]
impl crate::Readable for EventsEndepoutSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endepout::W`](W) writer structure"]
impl crate::Writable for EventsEndepoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDEPOUT[%s] to value 0"]
impl crate::Resettable for EventsEndepoutSpec {}
