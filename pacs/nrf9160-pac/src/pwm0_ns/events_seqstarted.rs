#[doc = "Register `EVENTS_SEQSTARTED[%s]` reader"]
pub type R = crate::R<EventsSeqstartedSpec>;
#[doc = "Register `EVENTS_SEQSTARTED[%s]` writer"]
pub type W = crate::W<EventsSeqstartedSpec>;
#[doc = "First PWM period started on sequence n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSeqstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSeqstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsSeqstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SEQSTARTED` reader - First PWM period started on sequence n"]
pub type EventsSeqstartedR = crate::BitReader<EventsSeqstarted>;
impl EventsSeqstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSeqstarted {
        match self.bits {
            false => EventsSeqstarted::NotGenerated,
            true => EventsSeqstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSeqstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSeqstarted::Generated
    }
}
#[doc = "Field `EVENTS_SEQSTARTED` writer - First PWM period started on sequence n"]
pub type EventsSeqstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsSeqstarted>;
impl<'a, REG> EventsSeqstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSeqstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSeqstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - First PWM period started on sequence n"]
    #[inline(always)]
    pub fn events_seqstarted(&self) -> EventsSeqstartedR {
        EventsSeqstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - First PWM period started on sequence n"]
    #[inline(always)]
    pub fn events_seqstarted(&mut self) -> EventsSeqstartedW<'_, EventsSeqstartedSpec> {
        EventsSeqstartedW::new(self, 0)
    }
}
#[doc = "Description collection: First PWM period started on sequence n\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSeqstartedSpec;
impl crate::RegisterSpec for EventsSeqstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_seqstarted::R`](R) reader structure"]
impl crate::Readable for EventsSeqstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_seqstarted::W`](W) writer structure"]
impl crate::Writable for EventsSeqstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SEQSTARTED[%s] to value 0"]
impl crate::Resettable for EventsSeqstartedSpec {}
