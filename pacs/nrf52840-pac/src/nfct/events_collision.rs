#[doc = "Register `EVENTS_COLLISION` reader"]
pub type R = crate::R<EventsCollisionSpec>;
#[doc = "Register `EVENTS_COLLISION` writer"]
pub type W = crate::W<EventsCollisionSpec>;
#[doc = "NFC auto collision resolution error reported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCollision {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCollision> for bool {
    #[inline(always)]
    fn from(variant: EventsCollision) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_COLLISION` reader - NFC auto collision resolution error reported."]
pub type EventsCollisionR = crate::BitReader<EventsCollision>;
impl EventsCollisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCollision {
        match self.bits {
            false => EventsCollision::NotGenerated,
            true => EventsCollision::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCollision::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCollision::Generated
    }
}
#[doc = "Field `EVENTS_COLLISION` writer - NFC auto collision resolution error reported."]
pub type EventsCollisionW<'a, REG> = crate::BitWriter<'a, REG, EventsCollision>;
impl<'a, REG> EventsCollisionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCollision::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCollision::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - NFC auto collision resolution error reported."]
    #[inline(always)]
    pub fn events_collision(&self) -> EventsCollisionR {
        EventsCollisionR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NFC auto collision resolution error reported."]
    #[inline(always)]
    pub fn events_collision(&mut self) -> EventsCollisionW<'_, EventsCollisionSpec> {
        EventsCollisionW::new(self, 0)
    }
}
#[doc = "NFC auto collision resolution error reported.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_collision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_collision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCollisionSpec;
impl crate::RegisterSpec for EventsCollisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_collision::R`](R) reader structure"]
impl crate::Readable for EventsCollisionSpec {}
#[doc = "`write(|w| ..)` method takes [`events_collision::W`](W) writer structure"]
impl crate::Writable for EventsCollisionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_COLLISION to value 0"]
impl crate::Resettable for EventsCollisionSpec {}
