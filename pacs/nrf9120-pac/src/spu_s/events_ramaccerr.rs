#[doc = "Register `EVENTS_RAMACCERR` reader"]
pub type R = crate::R<EventsRamaccerrSpec>;
#[doc = "Register `EVENTS_RAMACCERR` writer"]
pub type W = crate::W<EventsRamaccerrSpec>;
#[doc = "A security violation has been detected for the RAM memory space\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRamaccerr {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRamaccerr> for bool {
    #[inline(always)]
    fn from(variant: EventsRamaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RAMACCERR` reader - A security violation has been detected for the RAM memory space"]
pub type EventsRamaccerrR = crate::BitReader<EventsRamaccerr>;
impl EventsRamaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRamaccerr {
        match self.bits {
            false => EventsRamaccerr::NotGenerated,
            true => EventsRamaccerr::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRamaccerr::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRamaccerr::Generated
    }
}
#[doc = "Field `EVENTS_RAMACCERR` writer - A security violation has been detected for the RAM memory space"]
pub type EventsRamaccerrW<'a, REG> = crate::BitWriter<'a, REG, EventsRamaccerr>;
impl<'a, REG> EventsRamaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRamaccerr::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRamaccerr::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A security violation has been detected for the RAM memory space"]
    #[inline(always)]
    pub fn events_ramaccerr(&self) -> EventsRamaccerrR {
        EventsRamaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected for the RAM memory space"]
    #[inline(always)]
    #[must_use]
    pub fn events_ramaccerr(&mut self) -> EventsRamaccerrW<EventsRamaccerrSpec> {
        EventsRamaccerrW::new(self, 0)
    }
}
#[doc = "A security violation has been detected for the RAM memory space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_ramaccerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_ramaccerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRamaccerrSpec;
impl crate::RegisterSpec for EventsRamaccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ramaccerr::R`](R) reader structure"]
impl crate::Readable for EventsRamaccerrSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ramaccerr::W`](W) writer structure"]
impl crate::Writable for EventsRamaccerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RAMACCERR to value 0"]
impl crate::Resettable for EventsRamaccerrSpec {
    const RESET_VALUE: u32 = 0;
}
