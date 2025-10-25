#[doc = "Register `EVENTS_DIVIDEBYZERO` reader"]
pub type R = crate::R<EventsDividebyzeroSpec>;
#[doc = "Register `EVENTS_DIVIDEBYZERO` writer"]
pub type W = crate::W<EventsDividebyzeroSpec>;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDividebyzero {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDividebyzero> for bool {
    #[inline(always)]
    fn from(variant: EventsDividebyzero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DIVIDEBYZERO` reader - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub type EventsDividebyzeroR = crate::BitReader<EventsDividebyzero>;
impl EventsDividebyzeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDividebyzero {
        match self.bits {
            false => EventsDividebyzero::NotGenerated,
            true => EventsDividebyzero::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDividebyzero::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDividebyzero::Generated
    }
}
#[doc = "Field `EVENTS_DIVIDEBYZERO` writer - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub type EventsDividebyzeroW<'a, REG> = crate::BitWriter<'a, REG, EventsDividebyzero>;
impl<'a, REG> EventsDividebyzeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDividebyzero::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDividebyzero::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&self) -> EventsDividebyzeroR {
        EventsDividebyzeroR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&mut self) -> EventsDividebyzeroW<'_, EventsDividebyzeroSpec> {
        EventsDividebyzeroW::new(self, 0)
    }
}
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_dividebyzero::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_dividebyzero::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDividebyzeroSpec;
impl crate::RegisterSpec for EventsDividebyzeroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_dividebyzero::R`](R) reader structure"]
impl crate::Readable for EventsDividebyzeroSpec {}
#[doc = "`write(|w| ..)` method takes [`events_dividebyzero::W`](W) writer structure"]
impl crate::Writable for EventsDividebyzeroSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DIVIDEBYZERO to value 0"]
impl crate::Resettable for EventsDividebyzeroSpec {}
