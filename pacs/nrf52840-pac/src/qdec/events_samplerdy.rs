#[doc = "Register `EVENTS_SAMPLERDY` reader"]
pub type R = crate::R<EventsSamplerdySpec>;
#[doc = "Register `EVENTS_SAMPLERDY` writer"]
pub type W = crate::W<EventsSamplerdySpec>;
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSamplerdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSamplerdy> for bool {
    #[inline(always)]
    fn from(variant: EventsSamplerdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SAMPLERDY` reader - Event being generated for every new sample value written to the SAMPLE register"]
pub type EventsSamplerdyR = crate::BitReader<EventsSamplerdy>;
impl EventsSamplerdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSamplerdy {
        match self.bits {
            false => EventsSamplerdy::NotGenerated,
            true => EventsSamplerdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSamplerdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSamplerdy::Generated
    }
}
#[doc = "Field `EVENTS_SAMPLERDY` writer - Event being generated for every new sample value written to the SAMPLE register"]
pub type EventsSamplerdyW<'a, REG> = crate::BitWriter<'a, REG, EventsSamplerdy>;
impl<'a, REG> EventsSamplerdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSamplerdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSamplerdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&self) -> EventsSamplerdyR {
        EventsSamplerdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&mut self) -> EventsSamplerdyW<'_, EventsSamplerdySpec> {
        EventsSamplerdyW::new(self, 0)
    }
}
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_samplerdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_samplerdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSamplerdySpec;
impl crate::RegisterSpec for EventsSamplerdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_samplerdy::R`](R) reader structure"]
impl crate::Readable for EventsSamplerdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_samplerdy::W`](W) writer structure"]
impl crate::Writable for EventsSamplerdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SAMPLERDY to value 0"]
impl crate::Resettable for EventsSamplerdySpec {}
