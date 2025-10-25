#[doc = "Register `EVENTS_DEVMATCH` reader"]
pub type R = crate::R<EventsDevmatchSpec>;
#[doc = "Register `EVENTS_DEVMATCH` writer"]
pub type W = crate::W<EventsDevmatchSpec>;
#[doc = "A device address match occurred on the last received packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDevmatch {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDevmatch> for bool {
    #[inline(always)]
    fn from(variant: EventsDevmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DEVMATCH` reader - A device address match occurred on the last received packet"]
pub type EventsDevmatchR = crate::BitReader<EventsDevmatch>;
impl EventsDevmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDevmatch {
        match self.bits {
            false => EventsDevmatch::NotGenerated,
            true => EventsDevmatch::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDevmatch::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDevmatch::Generated
    }
}
#[doc = "Field `EVENTS_DEVMATCH` writer - A device address match occurred on the last received packet"]
pub type EventsDevmatchW<'a, REG> = crate::BitWriter<'a, REG, EventsDevmatch>;
impl<'a, REG> EventsDevmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDevmatch::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDevmatch::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmatch(&self) -> EventsDevmatchR {
        EventsDevmatchR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmatch(&mut self) -> EventsDevmatchW<'_, EventsDevmatchSpec> {
        EventsDevmatchW::new(self, 0)
    }
}
#[doc = "A device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDevmatchSpec;
impl crate::RegisterSpec for EventsDevmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_devmatch::R`](R) reader structure"]
impl crate::Readable for EventsDevmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_devmatch::W`](W) writer structure"]
impl crate::Writable for EventsDevmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DEVMATCH to value 0"]
impl crate::Resettable for EventsDevmatchSpec {}
