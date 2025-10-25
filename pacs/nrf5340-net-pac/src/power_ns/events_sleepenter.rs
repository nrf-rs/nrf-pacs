#[doc = "Register `EVENTS_SLEEPENTER` reader"]
pub type R = crate::R<EventsSleepenterSpec>;
#[doc = "Register `EVENTS_SLEEPENTER` writer"]
pub type W = crate::W<EventsSleepenterSpec>;
#[doc = "CPU entered WFI/WFE sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSleepenter {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSleepenter> for bool {
    #[inline(always)]
    fn from(variant: EventsSleepenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SLEEPENTER` reader - CPU entered WFI/WFE sleep"]
pub type EventsSleepenterR = crate::BitReader<EventsSleepenter>;
impl EventsSleepenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSleepenter {
        match self.bits {
            false => EventsSleepenter::NotGenerated,
            true => EventsSleepenter::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSleepenter::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSleepenter::Generated
    }
}
#[doc = "Field `EVENTS_SLEEPENTER` writer - CPU entered WFI/WFE sleep"]
pub type EventsSleepenterW<'a, REG> = crate::BitWriter<'a, REG, EventsSleepenter>;
impl<'a, REG> EventsSleepenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSleepenter::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSleepenter::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepenter(&self) -> EventsSleepenterR {
        EventsSleepenterR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepenter(&mut self) -> EventsSleepenterW<'_, EventsSleepenterSpec> {
        EventsSleepenterW::new(self, 0)
    }
}
#[doc = "CPU entered WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepenter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepenter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSleepenterSpec;
impl crate::RegisterSpec for EventsSleepenterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_sleepenter::R`](R) reader structure"]
impl crate::Readable for EventsSleepenterSpec {}
#[doc = "`write(|w| ..)` method takes [`events_sleepenter::W`](W) writer structure"]
impl crate::Writable for EventsSleepenterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SLEEPENTER to value 0"]
impl crate::Resettable for EventsSleepenterSpec {}
