#[doc = "Register `EVENTS_DONE` reader"]
pub type R = crate::R<EventsDoneSpec>;
#[doc = "Register `EVENTS_DONE` writer"]
pub type W = crate::W<EventsDoneSpec>;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.\n\nValue on reset: 0"]
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
#[doc = "Field `EVENTS_DONE` reader - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
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
#[doc = "Field `EVENTS_DONE` writer - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
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
    #[doc = "Bit 0 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    pub fn events_done(&self) -> EventsDoneR {
        EventsDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    #[must_use]
    pub fn events_done(&mut self) -> EventsDoneW<EventsDoneSpec> {
        EventsDoneW::new(self, 0)
    }
}
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDoneSpec;
impl crate::RegisterSpec for EventsDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_done::R`](R) reader structure"]
impl crate::Readable for EventsDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_done::W`](W) writer structure"]
impl crate::Writable for EventsDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_DONE to value 0"]
impl crate::Resettable for EventsDoneSpec {
    const RESET_VALUE: u32 = 0;
}
