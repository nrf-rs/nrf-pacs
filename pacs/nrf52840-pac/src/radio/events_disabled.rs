#[doc = "Register `EVENTS_DISABLED` reader"]
pub type R = crate::R<EventsDisabledSpec>;
#[doc = "Register `EVENTS_DISABLED` writer"]
pub type W = crate::W<EventsDisabledSpec>;
#[doc = "RADIO has been disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDisabled {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDisabled> for bool {
    #[inline(always)]
    fn from(variant: EventsDisabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DISABLED` reader - RADIO has been disabled"]
pub type EventsDisabledR = crate::BitReader<EventsDisabled>;
impl EventsDisabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDisabled {
        match self.bits {
            false => EventsDisabled::NotGenerated,
            true => EventsDisabled::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDisabled::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDisabled::Generated
    }
}
#[doc = "Field `EVENTS_DISABLED` writer - RADIO has been disabled"]
pub type EventsDisabledW<'a, REG> = crate::BitWriter<'a, REG, EventsDisabled>;
impl<'a, REG> EventsDisabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDisabled::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDisabled::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RADIO has been disabled"]
    #[inline(always)]
    pub fn events_disabled(&self) -> EventsDisabledR {
        EventsDisabledR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO has been disabled"]
    #[inline(always)]
    pub fn events_disabled(&mut self) -> EventsDisabledW<'_, EventsDisabledSpec> {
        EventsDisabledW::new(self, 0)
    }
}
#[doc = "RADIO has been disabled\n\nYou can [`read`](crate::Reg::read) this register and get [`events_disabled::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_disabled::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDisabledSpec;
impl crate::RegisterSpec for EventsDisabledSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_disabled::R`](R) reader structure"]
impl crate::Readable for EventsDisabledSpec {}
#[doc = "`write(|w| ..)` method takes [`events_disabled::W`](W) writer structure"]
impl crate::Writable for EventsDisabledSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DISABLED to value 0"]
impl crate::Resettable for EventsDisabledSpec {}
