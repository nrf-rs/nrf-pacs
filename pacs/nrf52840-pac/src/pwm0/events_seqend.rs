#[doc = "Register `EVENTS_SEQEND[%s]` reader"]
pub type R = crate::R<EventsSeqendSpec>;
#[doc = "Register `EVENTS_SEQEND[%s]` writer"]
pub type W = crate::W<EventsSeqendSpec>;
#[doc = "Emitted at end of every sequence n, when last value from RAM has been applied to wave counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSeqend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSeqend> for bool {
    #[inline(always)]
    fn from(variant: EventsSeqend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SEQEND` reader - Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub type EventsSeqendR = crate::BitReader<EventsSeqend>;
impl EventsSeqendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSeqend {
        match self.bits {
            false => EventsSeqend::NotGenerated,
            true => EventsSeqend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSeqend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSeqend::Generated
    }
}
#[doc = "Field `EVENTS_SEQEND` writer - Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub type EventsSeqendW<'a, REG> = crate::BitWriter<'a, REG, EventsSeqend>;
impl<'a, REG> EventsSeqendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSeqend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSeqend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub fn events_seqend(&self) -> EventsSeqendR {
        EventsSeqendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub fn events_seqend(&mut self) -> EventsSeqendW<'_, EventsSeqendSpec> {
        EventsSeqendW::new(self, 0)
    }
}
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSeqendSpec;
impl crate::RegisterSpec for EventsSeqendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_seqend::R`](R) reader structure"]
impl crate::Readable for EventsSeqendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_seqend::W`](W) writer structure"]
impl crate::Writable for EventsSeqendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SEQEND[%s] to value 0"]
impl crate::Resettable for EventsSeqendSpec {}
