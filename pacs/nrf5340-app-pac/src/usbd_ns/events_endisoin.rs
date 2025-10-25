#[doc = "Register `EVENTS_ENDISOIN` reader"]
pub type R = crate::R<EventsEndisoinSpec>;
#[doc = "Register `EVENTS_ENDISOIN` writer"]
pub type W = crate::W<EventsEndisoinSpec>;
#[doc = "The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndisoin {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndisoin> for bool {
    #[inline(always)]
    fn from(variant: EventsEndisoin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDISOIN` reader - The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndisoinR = crate::BitReader<EventsEndisoin>;
impl EventsEndisoinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndisoin {
        match self.bits {
            false => EventsEndisoin::NotGenerated,
            true => EventsEndisoin::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndisoin::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndisoin::Generated
    }
}
#[doc = "Field `EVENTS_ENDISOIN` writer - The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndisoinW<'a, REG> = crate::BitWriter<'a, REG, EventsEndisoin>;
impl<'a, REG> EventsEndisoinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndisoin::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndisoin::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoin(&self) -> EventsEndisoinR {
        EventsEndisoinR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoin(&mut self) -> EventsEndisoinW<'_, EventsEndisoinSpec> {
        EventsEndisoinW::new(self, 0)
    }
}
#[doc = "The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endisoin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endisoin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndisoinSpec;
impl crate::RegisterSpec for EventsEndisoinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endisoin::R`](R) reader structure"]
impl crate::Readable for EventsEndisoinSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endisoin::W`](W) writer structure"]
impl crate::Writable for EventsEndisoinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDISOIN to value 0"]
impl crate::Resettable for EventsEndisoinSpec {}
