#[doc = "Register `EVENTS_ENDECB` reader"]
pub type R = crate::R<EventsEndecbSpec>;
#[doc = "Register `EVENTS_ENDECB` writer"]
pub type W = crate::W<EventsEndecbSpec>;
#[doc = "ECB block encrypt complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndecb {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndecb> for bool {
    #[inline(always)]
    fn from(variant: EventsEndecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDECB` reader - ECB block encrypt complete"]
pub type EventsEndecbR = crate::BitReader<EventsEndecb>;
impl EventsEndecbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndecb {
        match self.bits {
            false => EventsEndecb::NotGenerated,
            true => EventsEndecb::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndecb::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndecb::Generated
    }
}
#[doc = "Field `EVENTS_ENDECB` writer - ECB block encrypt complete"]
pub type EventsEndecbW<'a, REG> = crate::BitWriter<'a, REG, EventsEndecb>;
impl<'a, REG> EventsEndecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndecb::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndecb::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - ECB block encrypt complete"]
    #[inline(always)]
    pub fn events_endecb(&self) -> EventsEndecbR {
        EventsEndecbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECB block encrypt complete"]
    #[inline(always)]
    pub fn events_endecb(&mut self) -> EventsEndecbW<'_, EventsEndecbSpec> {
        EventsEndecbW::new(self, 0)
    }
}
#[doc = "ECB block encrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endecb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endecb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndecbSpec;
impl crate::RegisterSpec for EventsEndecbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endecb::R`](R) reader structure"]
impl crate::Readable for EventsEndecbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endecb::W`](W) writer structure"]
impl crate::Writable for EventsEndecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDECB to value 0"]
impl crate::Resettable for EventsEndecbSpec {}
