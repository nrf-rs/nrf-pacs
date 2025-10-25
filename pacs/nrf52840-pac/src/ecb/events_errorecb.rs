#[doc = "Register `EVENTS_ERRORECB` reader"]
pub type R = crate::R<EventsErrorecbSpec>;
#[doc = "Register `EVENTS_ERRORECB` writer"]
pub type W = crate::W<EventsErrorecbSpec>;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsErrorecb {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsErrorecb> for bool {
    #[inline(always)]
    fn from(variant: EventsErrorecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ERRORECB` reader - ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub type EventsErrorecbR = crate::BitReader<EventsErrorecb>;
impl EventsErrorecbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsErrorecb {
        match self.bits {
            false => EventsErrorecb::NotGenerated,
            true => EventsErrorecb::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsErrorecb::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsErrorecb::Generated
    }
}
#[doc = "Field `EVENTS_ERRORECB` writer - ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub type EventsErrorecbW<'a, REG> = crate::BitWriter<'a, REG, EventsErrorecb>;
impl<'a, REG> EventsErrorecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsErrorecb::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsErrorecb::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub fn events_errorecb(&self) -> EventsErrorecbR {
        EventsErrorecbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub fn events_errorecb(&mut self) -> EventsErrorecbW<'_, EventsErrorecbSpec> {
        EventsErrorecbW::new(self, 0)
    }
}
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_errorecb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_errorecb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsErrorecbSpec;
impl crate::RegisterSpec for EventsErrorecbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_errorecb::R`](R) reader structure"]
impl crate::Readable for EventsErrorecbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_errorecb::W`](W) writer structure"]
impl crate::Writable for EventsErrorecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ERRORECB to value 0"]
impl crate::Resettable for EventsErrorecbSpec {}
