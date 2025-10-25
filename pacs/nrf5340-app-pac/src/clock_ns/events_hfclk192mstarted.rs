#[doc = "Register `EVENTS_HFCLK192MSTARTED` reader"]
pub type R = crate::R<EventsHfclk192mstartedSpec>;
#[doc = "Register `EVENTS_HFCLK192MSTARTED` writer"]
pub type W = crate::W<EventsHfclk192mstartedSpec>;
#[doc = "HFCLK192M source started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsHfclk192mstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsHfclk192mstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsHfclk192mstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_HFCLK192MSTARTED` reader - HFCLK192M source started"]
pub type EventsHfclk192mstartedR = crate::BitReader<EventsHfclk192mstarted>;
impl EventsHfclk192mstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsHfclk192mstarted {
        match self.bits {
            false => EventsHfclk192mstarted::NotGenerated,
            true => EventsHfclk192mstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsHfclk192mstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsHfclk192mstarted::Generated
    }
}
#[doc = "Field `EVENTS_HFCLK192MSTARTED` writer - HFCLK192M source started"]
pub type EventsHfclk192mstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsHfclk192mstarted>;
impl<'a, REG> EventsHfclk192mstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclk192mstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclk192mstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFCLK192M source started"]
    #[inline(always)]
    pub fn events_hfclk192mstarted(&self) -> EventsHfclk192mstartedR {
        EventsHfclk192mstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFCLK192M source started"]
    #[inline(always)]
    pub fn events_hfclk192mstarted(
        &mut self,
    ) -> EventsHfclk192mstartedW<'_, EventsHfclk192mstartedSpec> {
        EventsHfclk192mstartedW::new(self, 0)
    }
}
#[doc = "HFCLK192M source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclk192mstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclk192mstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsHfclk192mstartedSpec;
impl crate::RegisterSpec for EventsHfclk192mstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_hfclk192mstarted::R`](R) reader structure"]
impl crate::Readable for EventsHfclk192mstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_hfclk192mstarted::W`](W) writer structure"]
impl crate::Writable for EventsHfclk192mstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_HFCLK192MSTARTED to value 0"]
impl crate::Resettable for EventsHfclk192mstartedSpec {}
