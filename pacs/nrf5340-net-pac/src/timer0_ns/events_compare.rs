#[doc = "Register `EVENTS_COMPARE[%s]` reader"]
pub type R = crate::R<EventsCompareSpec>;
#[doc = "Register `EVENTS_COMPARE[%s]` writer"]
pub type W = crate::W<EventsCompareSpec>;
#[doc = "Compare event on CC\\[n\\] match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCompare {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCompare> for bool {
    #[inline(always)]
    fn from(variant: EventsCompare) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_COMPARE` reader - Compare event on CC\\[n\\] match"]
pub type EventsCompareR = crate::BitReader<EventsCompare>;
impl EventsCompareR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCompare {
        match self.bits {
            false => EventsCompare::NotGenerated,
            true => EventsCompare::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCompare::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCompare::Generated
    }
}
#[doc = "Field `EVENTS_COMPARE` writer - Compare event on CC\\[n\\] match"]
pub type EventsCompareW<'a, REG> = crate::BitWriter<'a, REG, EventsCompare>;
impl<'a, REG> EventsCompareW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCompare::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCompare::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare(&self) -> EventsCompareR {
        EventsCompareR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare(&mut self) -> EventsCompareW<'_, EventsCompareSpec> {
        EventsCompareW::new(self, 0)
    }
}
#[doc = "Description collection: Compare event on CC\\[n\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCompareSpec;
impl crate::RegisterSpec for EventsCompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_compare::R`](R) reader structure"]
impl crate::Readable for EventsCompareSpec {}
#[doc = "`write(|w| ..)` method takes [`events_compare::W`](W) writer structure"]
impl crate::Writable for EventsCompareSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_COMPARE[%s] to value 0"]
impl crate::Resettable for EventsCompareSpec {}
