#[doc = "Register `EVENTS_NCTS` reader"]
pub type R = crate::R<EventsNctsSpec>;
#[doc = "Register `EVENTS_NCTS` writer"]
pub type W = crate::W<EventsNctsSpec>;
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsNcts {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsNcts> for bool {
    #[inline(always)]
    fn from(variant: EventsNcts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_NCTS` reader - CTS is deactivated (set high). Not Clear To Send."]
pub type EventsNctsR = crate::BitReader<EventsNcts>;
impl EventsNctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsNcts {
        match self.bits {
            false => EventsNcts::NotGenerated,
            true => EventsNcts::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsNcts::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsNcts::Generated
    }
}
#[doc = "Field `EVENTS_NCTS` writer - CTS is deactivated (set high). Not Clear To Send."]
pub type EventsNctsW<'a, REG> = crate::BitWriter<'a, REG, EventsNcts>;
impl<'a, REG> EventsNctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsNcts::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsNcts::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub fn events_ncts(&self) -> EventsNctsR {
        EventsNctsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub fn events_ncts(&mut self) -> EventsNctsW<'_, EventsNctsSpec> {
        EventsNctsW::new(self, 0)
    }
}
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ncts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ncts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsNctsSpec;
impl crate::RegisterSpec for EventsNctsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ncts::R`](R) reader structure"]
impl crate::Readable for EventsNctsSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ncts::W`](W) writer structure"]
impl crate::Writable for EventsNctsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_NCTS to value 0"]
impl crate::Resettable for EventsNctsSpec {}
