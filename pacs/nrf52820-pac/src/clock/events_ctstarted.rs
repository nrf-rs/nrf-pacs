#[doc = "Register `EVENTS_CTSTARTED` reader"]
pub type R = crate::R<EventsCtstartedSpec>;
#[doc = "Register `EVENTS_CTSTARTED` writer"]
pub type W = crate::W<EventsCtstartedSpec>;
#[doc = "Calibration timer has been started and is ready to process new tasks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCtstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCtstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsCtstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTSTARTED` reader - Calibration timer has been started and is ready to process new tasks"]
pub type EventsCtstartedR = crate::BitReader<EventsCtstarted>;
impl EventsCtstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCtstarted {
        match self.bits {
            false => EventsCtstarted::NotGenerated,
            true => EventsCtstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCtstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCtstarted::Generated
    }
}
#[doc = "Field `EVENTS_CTSTARTED` writer - Calibration timer has been started and is ready to process new tasks"]
pub type EventsCtstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsCtstarted>;
impl<'a, REG> EventsCtstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration timer has been started and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstarted(&self) -> EventsCtstartedR {
        EventsCtstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration timer has been started and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstarted(&mut self) -> EventsCtstartedW<'_, EventsCtstartedSpec> {
        EventsCtstartedW::new(self, 0)
    }
}
#[doc = "Calibration timer has been started and is ready to process new tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCtstartedSpec;
impl crate::RegisterSpec for EventsCtstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ctstarted::R`](R) reader structure"]
impl crate::Readable for EventsCtstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ctstarted::W`](W) writer structure"]
impl crate::Writable for EventsCtstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTSTARTED to value 0"]
impl crate::Resettable for EventsCtstartedSpec {}
