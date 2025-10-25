#[doc = "Register `EVENTS_EPDATA` reader"]
pub type R = crate::R<EventsEpdataSpec>;
#[doc = "Register `EVENTS_EPDATA` writer"]
pub type W = crate::W<EventsEpdataSpec>;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEpdata {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEpdata> for bool {
    #[inline(always)]
    fn from(variant: EventsEpdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_EPDATA` reader - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub type EventsEpdataR = crate::BitReader<EventsEpdata>;
impl EventsEpdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEpdata {
        match self.bits {
            false => EventsEpdata::NotGenerated,
            true => EventsEpdata::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEpdata::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEpdata::Generated
    }
}
#[doc = "Field `EVENTS_EPDATA` writer - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub type EventsEpdataW<'a, REG> = crate::BitWriter<'a, REG, EventsEpdata>;
impl<'a, REG> EventsEpdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEpdata::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEpdata::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub fn events_epdata(&self) -> EventsEpdataR {
        EventsEpdataR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub fn events_epdata(&mut self) -> EventsEpdataW<'_, EventsEpdataSpec> {
        EventsEpdataW::new(self, 0)
    }
}
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_epdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_epdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEpdataSpec;
impl crate::RegisterSpec for EventsEpdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_epdata::R`](R) reader structure"]
impl crate::Readable for EventsEpdataSpec {}
#[doc = "`write(|w| ..)` method takes [`events_epdata::W`](W) writer structure"]
impl crate::Writable for EventsEpdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_EPDATA to value 0"]
impl crate::Resettable for EventsEpdataSpec {}
