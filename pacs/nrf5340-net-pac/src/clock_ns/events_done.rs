#[doc = "Register `EVENTS_DONE` reader"]
pub type R = crate::R<EventsDoneSpec>;
#[doc = "Register `EVENTS_DONE` writer"]
pub type W = crate::W<EventsDoneSpec>;
#[doc = "Calibration of LFRC oscillator complete event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDone> for bool {
    #[inline(always)]
    fn from(variant: EventsDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DONE` reader - Calibration of LFRC oscillator complete event"]
pub type EventsDoneR = crate::BitReader<EventsDone>;
impl EventsDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDone {
        match self.bits {
            false => EventsDone::NotGenerated,
            true => EventsDone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDone::Generated
    }
}
#[doc = "Field `EVENTS_DONE` writer - Calibration of LFRC oscillator complete event"]
pub type EventsDoneW<'a, REG> = crate::BitWriter<'a, REG, EventsDone>;
impl<'a, REG> EventsDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration of LFRC oscillator complete event"]
    #[inline(always)]
    pub fn events_done(&self) -> EventsDoneR {
        EventsDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration of LFRC oscillator complete event"]
    #[inline(always)]
    pub fn events_done(&mut self) -> EventsDoneW<'_, EventsDoneSpec> {
        EventsDoneW::new(self, 0)
    }
}
#[doc = "Calibration of LFRC oscillator complete event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDoneSpec;
impl crate::RegisterSpec for EventsDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_done::R`](R) reader structure"]
impl crate::Readable for EventsDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_done::W`](W) writer structure"]
impl crate::Writable for EventsDoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DONE to value 0"]
impl crate::Resettable for EventsDoneSpec {}
