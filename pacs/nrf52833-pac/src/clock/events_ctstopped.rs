#[doc = "Register `EVENTS_CTSTOPPED` reader"]
pub type R = crate::R<EventsCtstoppedSpec>;
#[doc = "Register `EVENTS_CTSTOPPED` writer"]
pub type W = crate::W<EventsCtstoppedSpec>;
#[doc = "Calibration timer has been stopped and is ready to process new tasks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCtstopped {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCtstopped> for bool {
    #[inline(always)]
    fn from(variant: EventsCtstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTSTOPPED` reader - Calibration timer has been stopped and is ready to process new tasks"]
pub type EventsCtstoppedR = crate::BitReader<EventsCtstopped>;
impl EventsCtstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCtstopped {
        match self.bits {
            false => EventsCtstopped::NotGenerated,
            true => EventsCtstopped::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCtstopped::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCtstopped::Generated
    }
}
#[doc = "Field `EVENTS_CTSTOPPED` writer - Calibration timer has been stopped and is ready to process new tasks"]
pub type EventsCtstoppedW<'a, REG> = crate::BitWriter<'a, REG, EventsCtstopped>;
impl<'a, REG> EventsCtstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtstopped::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtstopped::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration timer has been stopped and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstopped(&self) -> EventsCtstoppedR {
        EventsCtstoppedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration timer has been stopped and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstopped(&mut self) -> EventsCtstoppedW<'_, EventsCtstoppedSpec> {
        EventsCtstoppedW::new(self, 0)
    }
}
#[doc = "Calibration timer has been stopped and is ready to process new tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctstopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctstopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCtstoppedSpec;
impl crate::RegisterSpec for EventsCtstoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ctstopped::R`](R) reader structure"]
impl crate::Readable for EventsCtstoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ctstopped::W`](W) writer structure"]
impl crate::Writable for EventsCtstoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTSTOPPED to value 0"]
impl crate::Resettable for EventsCtstoppedSpec {}
