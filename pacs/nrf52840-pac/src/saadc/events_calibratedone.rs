#[doc = "Register `EVENTS_CALIBRATEDONE` reader"]
pub type R = crate::R<EventsCalibratedoneSpec>;
#[doc = "Register `EVENTS_CALIBRATEDONE` writer"]
pub type W = crate::W<EventsCalibratedoneSpec>;
#[doc = "Calibration is complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCalibratedone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCalibratedone> for bool {
    #[inline(always)]
    fn from(variant: EventsCalibratedone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CALIBRATEDONE` reader - Calibration is complete"]
pub type EventsCalibratedoneR = crate::BitReader<EventsCalibratedone>;
impl EventsCalibratedoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCalibratedone {
        match self.bits {
            false => EventsCalibratedone::NotGenerated,
            true => EventsCalibratedone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCalibratedone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCalibratedone::Generated
    }
}
#[doc = "Field `EVENTS_CALIBRATEDONE` writer - Calibration is complete"]
pub type EventsCalibratedoneW<'a, REG> = crate::BitWriter<'a, REG, EventsCalibratedone>;
impl<'a, REG> EventsCalibratedoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCalibratedone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCalibratedone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration is complete"]
    #[inline(always)]
    pub fn events_calibratedone(&self) -> EventsCalibratedoneR {
        EventsCalibratedoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration is complete"]
    #[inline(always)]
    pub fn events_calibratedone(&mut self) -> EventsCalibratedoneW<'_, EventsCalibratedoneSpec> {
        EventsCalibratedoneW::new(self, 0)
    }
}
#[doc = "Calibration is complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_calibratedone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_calibratedone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCalibratedoneSpec;
impl crate::RegisterSpec for EventsCalibratedoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_calibratedone::R`](R) reader structure"]
impl crate::Readable for EventsCalibratedoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_calibratedone::W`](W) writer structure"]
impl crate::Writable for EventsCalibratedoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CALIBRATEDONE to value 0"]
impl crate::Resettable for EventsCalibratedoneSpec {}
