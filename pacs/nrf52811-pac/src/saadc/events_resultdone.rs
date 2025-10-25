#[doc = "Register `EVENTS_RESULTDONE` reader"]
pub type R = crate::R<EventsResultdoneSpec>;
#[doc = "Register `EVENTS_RESULTDONE` writer"]
pub type W = crate::W<EventsResultdoneSpec>;
#[doc = "A result is ready to get transferred to RAM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsResultdone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsResultdone> for bool {
    #[inline(always)]
    fn from(variant: EventsResultdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RESULTDONE` reader - A result is ready to get transferred to RAM."]
pub type EventsResultdoneR = crate::BitReader<EventsResultdone>;
impl EventsResultdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsResultdone {
        match self.bits {
            false => EventsResultdone::NotGenerated,
            true => EventsResultdone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsResultdone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsResultdone::Generated
    }
}
#[doc = "Field `EVENTS_RESULTDONE` writer - A result is ready to get transferred to RAM."]
pub type EventsResultdoneW<'a, REG> = crate::BitWriter<'a, REG, EventsResultdone>;
impl<'a, REG> EventsResultdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsResultdone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsResultdone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A result is ready to get transferred to RAM."]
    #[inline(always)]
    pub fn events_resultdone(&self) -> EventsResultdoneR {
        EventsResultdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A result is ready to get transferred to RAM."]
    #[inline(always)]
    pub fn events_resultdone(&mut self) -> EventsResultdoneW<'_, EventsResultdoneSpec> {
        EventsResultdoneW::new(self, 0)
    }
}
#[doc = "A result is ready to get transferred to RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resultdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resultdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsResultdoneSpec;
impl crate::RegisterSpec for EventsResultdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_resultdone::R`](R) reader structure"]
impl crate::Readable for EventsResultdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_resultdone::W`](W) writer structure"]
impl crate::Writable for EventsResultdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RESULTDONE to value 0"]
impl crate::Resettable for EventsResultdoneSpec {}
