#[doc = "Register `EVENTS_DEVMISS` reader"]
pub type R = crate::R<EventsDevmissSpec>;
#[doc = "Register `EVENTS_DEVMISS` writer"]
pub type W = crate::W<EventsDevmissSpec>;
#[doc = "No device address match occurred on the last received packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDevmiss {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDevmiss> for bool {
    #[inline(always)]
    fn from(variant: EventsDevmiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DEVMISS` reader - No device address match occurred on the last received packet"]
pub type EventsDevmissR = crate::BitReader<EventsDevmiss>;
impl EventsDevmissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDevmiss {
        match self.bits {
            false => EventsDevmiss::NotGenerated,
            true => EventsDevmiss::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDevmiss::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDevmiss::Generated
    }
}
#[doc = "Field `EVENTS_DEVMISS` writer - No device address match occurred on the last received packet"]
pub type EventsDevmissW<'a, REG> = crate::BitWriter<'a, REG, EventsDevmiss>;
impl<'a, REG> EventsDevmissW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDevmiss::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDevmiss::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmiss(&self) -> EventsDevmissR {
        EventsDevmissR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmiss(&mut self) -> EventsDevmissW<'_, EventsDevmissSpec> {
        EventsDevmissW::new(self, 0)
    }
}
#[doc = "No device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmiss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmiss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDevmissSpec;
impl crate::RegisterSpec for EventsDevmissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_devmiss::R`](R) reader structure"]
impl crate::Readable for EventsDevmissSpec {}
#[doc = "`write(|w| ..)` method takes [`events_devmiss::W`](W) writer structure"]
impl crate::Writable for EventsDevmissSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DEVMISS to value 0"]
impl crate::Resettable for EventsDevmissSpec {}
