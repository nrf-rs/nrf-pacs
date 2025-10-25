#[doc = "Register `EVENTS_UP` reader"]
pub type R = crate::R<EventsUpSpec>;
#[doc = "Register `EVENTS_UP` writer"]
pub type W = crate::W<EventsUpSpec>;
#[doc = "Upward crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUp {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUp> for bool {
    #[inline(always)]
    fn from(variant: EventsUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_UP` reader - Upward crossing"]
pub type EventsUpR = crate::BitReader<EventsUp>;
impl EventsUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUp {
        match self.bits {
            false => EventsUp::NotGenerated,
            true => EventsUp::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUp::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUp::Generated
    }
}
#[doc = "Field `EVENTS_UP` writer - Upward crossing"]
pub type EventsUpW<'a, REG> = crate::BitWriter<'a, REG, EventsUp>;
impl<'a, REG> EventsUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUp::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUp::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Upward crossing"]
    #[inline(always)]
    pub fn events_up(&self) -> EventsUpR {
        EventsUpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Upward crossing"]
    #[inline(always)]
    pub fn events_up(&mut self) -> EventsUpW<'_, EventsUpSpec> {
        EventsUpW::new(self, 0)
    }
}
#[doc = "Upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_up::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_up::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUpSpec;
impl crate::RegisterSpec for EventsUpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_up::R`](R) reader structure"]
impl crate::Readable for EventsUpSpec {}
#[doc = "`write(|w| ..)` method takes [`events_up::W`](W) writer structure"]
impl crate::Writable for EventsUpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_UP to value 0"]
impl crate::Resettable for EventsUpSpec {}
