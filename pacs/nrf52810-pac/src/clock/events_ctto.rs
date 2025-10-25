#[doc = "Register `EVENTS_CTTO` reader"]
pub type R = crate::R<EventsCttoSpec>;
#[doc = "Register `EVENTS_CTTO` writer"]
pub type W = crate::W<EventsCttoSpec>;
#[doc = "Calibration timer timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCtto {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCtto> for bool {
    #[inline(always)]
    fn from(variant: EventsCtto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTTO` reader - Calibration timer timeout"]
pub type EventsCttoR = crate::BitReader<EventsCtto>;
impl EventsCttoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCtto {
        match self.bits {
            false => EventsCtto::NotGenerated,
            true => EventsCtto::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCtto::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCtto::Generated
    }
}
#[doc = "Field `EVENTS_CTTO` writer - Calibration timer timeout"]
pub type EventsCttoW<'a, REG> = crate::BitWriter<'a, REG, EventsCtto>;
impl<'a, REG> EventsCttoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtto::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtto::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration timer timeout"]
    #[inline(always)]
    pub fn events_ctto(&self) -> EventsCttoR {
        EventsCttoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration timer timeout"]
    #[inline(always)]
    pub fn events_ctto(&mut self) -> EventsCttoW<'_, EventsCttoSpec> {
        EventsCttoW::new(self, 0)
    }
}
#[doc = "Calibration timer timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCttoSpec;
impl crate::RegisterSpec for EventsCttoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ctto::R`](R) reader structure"]
impl crate::Readable for EventsCttoSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ctto::W`](W) writer structure"]
impl crate::Writable for EventsCttoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTTO to value 0"]
impl crate::Resettable for EventsCttoSpec {}
