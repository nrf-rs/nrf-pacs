#[doc = "Register `EVENTS_RSSIEND` reader"]
pub type R = crate::R<EventsRssiendSpec>;
#[doc = "Register `EVENTS_RSSIEND` writer"]
pub type W = crate::W<EventsRssiendSpec>;
#[doc = "Sampling of receive signal strength complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRssiend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRssiend> for bool {
    #[inline(always)]
    fn from(variant: EventsRssiend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RSSIEND` reader - Sampling of receive signal strength complete."]
pub type EventsRssiendR = crate::BitReader<EventsRssiend>;
impl EventsRssiendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRssiend {
        match self.bits {
            false => EventsRssiend::NotGenerated,
            true => EventsRssiend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRssiend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRssiend::Generated
    }
}
#[doc = "Field `EVENTS_RSSIEND` writer - Sampling of receive signal strength complete."]
pub type EventsRssiendW<'a, REG> = crate::BitWriter<'a, REG, EventsRssiend>;
impl<'a, REG> EventsRssiendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRssiend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRssiend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Sampling of receive signal strength complete."]
    #[inline(always)]
    pub fn events_rssiend(&self) -> EventsRssiendR {
        EventsRssiendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sampling of receive signal strength complete."]
    #[inline(always)]
    pub fn events_rssiend(&mut self) -> EventsRssiendW<'_, EventsRssiendSpec> {
        EventsRssiendW::new(self, 0)
    }
}
#[doc = "Sampling of receive signal strength complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rssiend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rssiend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRssiendSpec;
impl crate::RegisterSpec for EventsRssiendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rssiend::R`](R) reader structure"]
impl crate::Readable for EventsRssiendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rssiend::W`](W) writer structure"]
impl crate::Writable for EventsRssiendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RSSIEND to value 0"]
impl crate::Resettable for EventsRssiendSpec {}
