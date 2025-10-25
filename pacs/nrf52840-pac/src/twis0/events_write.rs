#[doc = "Register `EVENTS_WRITE` reader"]
pub type R = crate::R<EventsWriteSpec>;
#[doc = "Register `EVENTS_WRITE` writer"]
pub type W = crate::W<EventsWriteSpec>;
#[doc = "Write command received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsWrite {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsWrite> for bool {
    #[inline(always)]
    fn from(variant: EventsWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_WRITE` reader - Write command received"]
pub type EventsWriteR = crate::BitReader<EventsWrite>;
impl EventsWriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsWrite {
        match self.bits {
            false => EventsWrite::NotGenerated,
            true => EventsWrite::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsWrite::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsWrite::Generated
    }
}
#[doc = "Field `EVENTS_WRITE` writer - Write command received"]
pub type EventsWriteW<'a, REG> = crate::BitWriter<'a, REG, EventsWrite>;
impl<'a, REG> EventsWriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWrite::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWrite::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Write command received"]
    #[inline(always)]
    pub fn events_write(&self) -> EventsWriteR {
        EventsWriteR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write command received"]
    #[inline(always)]
    pub fn events_write(&mut self) -> EventsWriteW<'_, EventsWriteSpec> {
        EventsWriteW::new(self, 0)
    }
}
#[doc = "Write command received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_write::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_write::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsWriteSpec;
impl crate::RegisterSpec for EventsWriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_write::R`](R) reader structure"]
impl crate::Readable for EventsWriteSpec {}
#[doc = "`write(|w| ..)` method takes [`events_write::W`](W) writer structure"]
impl crate::Writable for EventsWriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_WRITE to value 0"]
impl crate::Resettable for EventsWriteSpec {}
