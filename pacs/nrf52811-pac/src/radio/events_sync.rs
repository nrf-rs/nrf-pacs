#[doc = "Register `EVENTS_SYNC` reader"]
pub type R = crate::R<EventsSyncSpec>;
#[doc = "Register `EVENTS_SYNC` writer"]
pub type W = crate::W<EventsSyncSpec>;
#[doc = "Preamble indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSync {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSync> for bool {
    #[inline(always)]
    fn from(variant: EventsSync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SYNC` reader - Preamble indicator"]
pub type EventsSyncR = crate::BitReader<EventsSync>;
impl EventsSyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSync {
        match self.bits {
            false => EventsSync::NotGenerated,
            true => EventsSync::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSync::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSync::Generated
    }
}
#[doc = "Field `EVENTS_SYNC` writer - Preamble indicator"]
pub type EventsSyncW<'a, REG> = crate::BitWriter<'a, REG, EventsSync>;
impl<'a, REG> EventsSyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSync::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSync::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Preamble indicator"]
    #[inline(always)]
    pub fn events_sync(&self) -> EventsSyncR {
        EventsSyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Preamble indicator"]
    #[inline(always)]
    pub fn events_sync(&mut self) -> EventsSyncW<'_, EventsSyncSpec> {
        EventsSyncW::new(self, 0)
    }
}
#[doc = "Preamble indicator\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSyncSpec;
impl crate::RegisterSpec for EventsSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_sync::R`](R) reader structure"]
impl crate::Readable for EventsSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`events_sync::W`](W) writer structure"]
impl crate::Writable for EventsSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SYNC to value 0"]
impl crate::Resettable for EventsSyncSpec {}
