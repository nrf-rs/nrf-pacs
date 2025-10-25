#[doc = "Register `EVENTS_DOWN` reader"]
pub type R = crate::R<EventsDownSpec>;
#[doc = "Register `EVENTS_DOWN` writer"]
pub type W = crate::W<EventsDownSpec>;
#[doc = "Downward crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDown {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDown> for bool {
    #[inline(always)]
    fn from(variant: EventsDown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DOWN` reader - Downward crossing"]
pub type EventsDownR = crate::BitReader<EventsDown>;
impl EventsDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDown {
        match self.bits {
            false => EventsDown::NotGenerated,
            true => EventsDown::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDown::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDown::Generated
    }
}
#[doc = "Field `EVENTS_DOWN` writer - Downward crossing"]
pub type EventsDownW<'a, REG> = crate::BitWriter<'a, REG, EventsDown>;
impl<'a, REG> EventsDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDown::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDown::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Downward crossing"]
    #[inline(always)]
    pub fn events_down(&self) -> EventsDownR {
        EventsDownR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Downward crossing"]
    #[inline(always)]
    pub fn events_down(&mut self) -> EventsDownW<'_, EventsDownSpec> {
        EventsDownW::new(self, 0)
    }
}
#[doc = "Downward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_down::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_down::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDownSpec;
impl crate::RegisterSpec for EventsDownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_down::R`](R) reader structure"]
impl crate::Readable for EventsDownSpec {}
#[doc = "`write(|w| ..)` method takes [`events_down::W`](W) writer structure"]
impl crate::Writable for EventsDownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DOWN to value 0"]
impl crate::Resettable for EventsDownSpec {}
