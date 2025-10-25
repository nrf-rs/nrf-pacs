#[doc = "Register `EVENTS_INVALIDOPERATION` reader"]
pub type R = crate::R<EventsInvalidoperationSpec>;
#[doc = "Register `EVENTS_INVALIDOPERATION` writer"]
pub type W = crate::W<EventsInvalidoperationSpec>;
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsInvalidoperation {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsInvalidoperation> for bool {
    #[inline(always)]
    fn from(variant: EventsInvalidoperation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_INVALIDOPERATION` reader - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub type EventsInvalidoperationR = crate::BitReader<EventsInvalidoperation>;
impl EventsInvalidoperationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsInvalidoperation {
        match self.bits {
            false => EventsInvalidoperation::NotGenerated,
            true => EventsInvalidoperation::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsInvalidoperation::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsInvalidoperation::Generated
    }
}
#[doc = "Field `EVENTS_INVALIDOPERATION` writer - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub type EventsInvalidoperationW<'a, REG> = crate::BitWriter<'a, REG, EventsInvalidoperation>;
impl<'a, REG> EventsInvalidoperationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsInvalidoperation::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsInvalidoperation::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_invalidoperation(&self) -> EventsInvalidoperationR {
        EventsInvalidoperationR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_invalidoperation(
        &mut self,
    ) -> EventsInvalidoperationW<'_, EventsInvalidoperationSpec> {
        EventsInvalidoperationW::new(self, 0)
    }
}
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_invalidoperation::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_invalidoperation::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsInvalidoperationSpec;
impl crate::RegisterSpec for EventsInvalidoperationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_invalidoperation::R`](R) reader structure"]
impl crate::Readable for EventsInvalidoperationSpec {}
#[doc = "`write(|w| ..)` method takes [`events_invalidoperation::W`](W) writer structure"]
impl crate::Writable for EventsInvalidoperationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_INVALIDOPERATION to value 0"]
impl crate::Resettable for EventsInvalidoperationSpec {}
