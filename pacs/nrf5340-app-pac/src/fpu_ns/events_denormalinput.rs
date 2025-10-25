#[doc = "Register `EVENTS_DENORMALINPUT` reader"]
pub type R = crate::R<EventsDenormalinputSpec>;
#[doc = "Register `EVENTS_DENORMALINPUT` writer"]
pub type W = crate::W<EventsDenormalinputSpec>;
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDenormalinput {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDenormalinput> for bool {
    #[inline(always)]
    fn from(variant: EventsDenormalinput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DENORMALINPUT` reader - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub type EventsDenormalinputR = crate::BitReader<EventsDenormalinput>;
impl EventsDenormalinputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDenormalinput {
        match self.bits {
            false => EventsDenormalinput::NotGenerated,
            true => EventsDenormalinput::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDenormalinput::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDenormalinput::Generated
    }
}
#[doc = "Field `EVENTS_DENORMALINPUT` writer - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub type EventsDenormalinputW<'a, REG> = crate::BitWriter<'a, REG, EventsDenormalinput>;
impl<'a, REG> EventsDenormalinputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDenormalinput::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDenormalinput::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    #[inline(always)]
    pub fn events_denormalinput(&self) -> EventsDenormalinputR {
        EventsDenormalinputR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    #[inline(always)]
    pub fn events_denormalinput(&mut self) -> EventsDenormalinputW<'_, EventsDenormalinputSpec> {
        EventsDenormalinputW::new(self, 0)
    }
}
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_denormalinput::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_denormalinput::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDenormalinputSpec;
impl crate::RegisterSpec for EventsDenormalinputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_denormalinput::R`](R) reader structure"]
impl crate::Readable for EventsDenormalinputSpec {}
#[doc = "`write(|w| ..)` method takes [`events_denormalinput::W`](W) writer structure"]
impl crate::Writable for EventsDenormalinputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DENORMALINPUT to value 0"]
impl crate::Resettable for EventsDenormalinputSpec {}
