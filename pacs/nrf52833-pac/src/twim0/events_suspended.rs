#[doc = "Register `EVENTS_SUSPENDED` reader"]
pub type R = crate::R<EventsSuspendedSpec>;
#[doc = "Register `EVENTS_SUSPENDED` writer"]
pub type W = crate::W<EventsSuspendedSpec>;
#[doc = "SUSPEND task has been issued, TWI traffic is now suspended.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSuspended {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSuspended> for bool {
    #[inline(always)]
    fn from(variant: EventsSuspended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SUSPENDED` reader - SUSPEND task has been issued, TWI traffic is now suspended."]
pub type EventsSuspendedR = crate::BitReader<EventsSuspended>;
impl EventsSuspendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSuspended {
        match self.bits {
            false => EventsSuspended::NotGenerated,
            true => EventsSuspended::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSuspended::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSuspended::Generated
    }
}
#[doc = "Field `EVENTS_SUSPENDED` writer - SUSPEND task has been issued, TWI traffic is now suspended."]
pub type EventsSuspendedW<'a, REG> = crate::BitWriter<'a, REG, EventsSuspended>;
impl<'a, REG> EventsSuspendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSuspended::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSuspended::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub fn events_suspended(&self) -> EventsSuspendedR {
        EventsSuspendedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub fn events_suspended(&mut self) -> EventsSuspendedW<'_, EventsSuspendedSpec> {
        EventsSuspendedW::new(self, 0)
    }
}
#[doc = "SUSPEND task has been issued, TWI traffic is now suspended.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_suspended::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_suspended::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSuspendedSpec;
impl crate::RegisterSpec for EventsSuspendedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_suspended::R`](R) reader structure"]
impl crate::Readable for EventsSuspendedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_suspended::W`](W) writer structure"]
impl crate::Writable for EventsSuspendedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SUSPENDED to value 0"]
impl crate::Resettable for EventsSuspendedSpec {}
