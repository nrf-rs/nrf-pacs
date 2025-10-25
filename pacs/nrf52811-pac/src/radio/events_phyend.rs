#[doc = "Register `EVENTS_PHYEND` reader"]
pub type R = crate::R<EventsPhyendSpec>;
#[doc = "Register `EVENTS_PHYEND` writer"]
pub type W = crate::W<EventsPhyendSpec>;
#[doc = "Generated when last bit is sent on air, or received from air\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPhyend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPhyend> for bool {
    #[inline(always)]
    fn from(variant: EventsPhyend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PHYEND` reader - Generated when last bit is sent on air, or received from air"]
pub type EventsPhyendR = crate::BitReader<EventsPhyend>;
impl EventsPhyendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPhyend {
        match self.bits {
            false => EventsPhyend::NotGenerated,
            true => EventsPhyend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPhyend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPhyend::Generated
    }
}
#[doc = "Field `EVENTS_PHYEND` writer - Generated when last bit is sent on air, or received from air"]
pub type EventsPhyendW<'a, REG> = crate::BitWriter<'a, REG, EventsPhyend>;
impl<'a, REG> EventsPhyendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPhyend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPhyend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Generated when last bit is sent on air, or received from air"]
    #[inline(always)]
    pub fn events_phyend(&self) -> EventsPhyendR {
        EventsPhyendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generated when last bit is sent on air, or received from air"]
    #[inline(always)]
    pub fn events_phyend(&mut self) -> EventsPhyendW<'_, EventsPhyendSpec> {
        EventsPhyendW::new(self, 0)
    }
}
#[doc = "Generated when last bit is sent on air, or received from air\n\nYou can [`read`](crate::Reg::read) this register and get [`events_phyend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_phyend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPhyendSpec;
impl crate::RegisterSpec for EventsPhyendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_phyend::R`](R) reader structure"]
impl crate::Readable for EventsPhyendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_phyend::W`](W) writer structure"]
impl crate::Writable for EventsPhyendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PHYEND to value 0"]
impl crate::Resettable for EventsPhyendSpec {}
