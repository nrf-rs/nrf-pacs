#[doc = "Register `EVENTS_PERIPHACCERR` reader"]
pub type R = crate::R<EventsPeriphaccerrSpec>;
#[doc = "Register `EVENTS_PERIPHACCERR` writer"]
pub type W = crate::W<EventsPeriphaccerrSpec>;
#[doc = "A security violation has been detected on one or several peripherals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPeriphaccerr {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPeriphaccerr> for bool {
    #[inline(always)]
    fn from(variant: EventsPeriphaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PERIPHACCERR` reader - A security violation has been detected on one or several peripherals"]
pub type EventsPeriphaccerrR = crate::BitReader<EventsPeriphaccerr>;
impl EventsPeriphaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPeriphaccerr {
        match self.bits {
            false => EventsPeriphaccerr::NotGenerated,
            true => EventsPeriphaccerr::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPeriphaccerr::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPeriphaccerr::Generated
    }
}
#[doc = "Field `EVENTS_PERIPHACCERR` writer - A security violation has been detected on one or several peripherals"]
pub type EventsPeriphaccerrW<'a, REG> = crate::BitWriter<'a, REG, EventsPeriphaccerr>;
impl<'a, REG> EventsPeriphaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPeriphaccerr::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPeriphaccerr::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&self) -> EventsPeriphaccerrR {
        EventsPeriphaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&mut self) -> EventsPeriphaccerrW<'_, EventsPeriphaccerrSpec> {
        EventsPeriphaccerrW::new(self, 0)
    }
}
#[doc = "A security violation has been detected on one or several peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`events_periphaccerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_periphaccerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPeriphaccerrSpec;
impl crate::RegisterSpec for EventsPeriphaccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_periphaccerr::R`](R) reader structure"]
impl crate::Readable for EventsPeriphaccerrSpec {}
#[doc = "`write(|w| ..)` method takes [`events_periphaccerr::W`](W) writer structure"]
impl crate::Writable for EventsPeriphaccerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PERIPHACCERR to value 0"]
impl crate::Resettable for EventsPeriphaccerrSpec {}
