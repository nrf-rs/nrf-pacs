#[doc = "Register `EVENTS_ERROR` reader"]
pub type R = crate::R<EventsErrorSpec>;
#[doc = "Register `EVENTS_ERROR` writer"]
pub type W = crate::W<EventsErrorSpec>;
#[doc = "TWI error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsError {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsError> for bool {
    #[inline(always)]
    fn from(variant: EventsError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ERROR` reader - TWI error"]
pub type EventsErrorR = crate::BitReader<EventsError>;
impl EventsErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsError {
        match self.bits {
            false => EventsError::NotGenerated,
            true => EventsError::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsError::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsError::Generated
    }
}
#[doc = "Field `EVENTS_ERROR` writer - TWI error"]
pub type EventsErrorW<'a, REG> = crate::BitWriter<'a, REG, EventsError>;
impl<'a, REG> EventsErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsError::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsError::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TWI error"]
    #[inline(always)]
    pub fn events_error(&self) -> EventsErrorR {
        EventsErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI error"]
    #[inline(always)]
    pub fn events_error(&mut self) -> EventsErrorW<'_, EventsErrorSpec> {
        EventsErrorW::new(self, 0)
    }
}
#[doc = "TWI error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsErrorSpec;
impl crate::RegisterSpec for EventsErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_error::R`](R) reader structure"]
impl crate::Readable for EventsErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_error::W`](W) writer structure"]
impl crate::Writable for EventsErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ERROR to value 0"]
impl crate::Resettable for EventsErrorSpec {}
