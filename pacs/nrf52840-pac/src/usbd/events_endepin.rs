#[doc = "Register `EVENTS_ENDEPIN[%s]` reader"]
pub type R = crate::R<EventsEndepinSpec>;
#[doc = "Register `EVENTS_ENDEPIN[%s]` writer"]
pub type W = crate::W<EventsEndepinSpec>;
#[doc = "The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndepin {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndepin> for bool {
    #[inline(always)]
    fn from(variant: EventsEndepin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDEPIN` reader - The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndepinR = crate::BitReader<EventsEndepin>;
impl EventsEndepinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndepin {
        match self.bits {
            false => EventsEndepin::NotGenerated,
            true => EventsEndepin::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndepin::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndepin::Generated
    }
}
#[doc = "Field `EVENTS_ENDEPIN` writer - The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndepinW<'a, REG> = crate::BitWriter<'a, REG, EventsEndepin>;
impl<'a, REG> EventsEndepinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndepin::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndepin::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin(&self) -> EventsEndepinR {
        EventsEndepinR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin(&mut self) -> EventsEndepinW<'_, EventsEndepinSpec> {
        EventsEndepinW::new(self, 0)
    }
}
#[doc = "Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endepin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endepin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndepinSpec;
impl crate::RegisterSpec for EventsEndepinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endepin::R`](R) reader structure"]
impl crate::Readable for EventsEndepinSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endepin::W`](W) writer structure"]
impl crate::Writable for EventsEndepinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDEPIN[%s] to value 0"]
impl crate::Resettable for EventsEndepinSpec {}
