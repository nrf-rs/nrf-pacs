#[doc = "Register `EVENTS_EDEND` reader"]
pub type R = crate::R<EventsEdendSpec>;
#[doc = "Register `EVENTS_EDEND` writer"]
pub type W = crate::W<EventsEdendSpec>;
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEdend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEdend> for bool {
    #[inline(always)]
    fn from(variant: EventsEdend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_EDEND` reader - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
pub type EventsEdendR = crate::BitReader<EventsEdend>;
impl EventsEdendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEdend {
        match self.bits {
            false => EventsEdend::NotGenerated,
            true => EventsEdend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEdend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEdend::Generated
    }
}
#[doc = "Field `EVENTS_EDEND` writer - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
pub type EventsEdendW<'a, REG> = crate::BitWriter<'a, REG, EventsEdend>;
impl<'a, REG> EventsEdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEdend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEdend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    #[inline(always)]
    pub fn events_edend(&self) -> EventsEdendR {
        EventsEdendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    #[inline(always)]
    pub fn events_edend(&mut self) -> EventsEdendW<'_, EventsEdendSpec> {
        EventsEdendW::new(self, 0)
    }
}
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEdendSpec;
impl crate::RegisterSpec for EventsEdendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_edend::R`](R) reader structure"]
impl crate::Readable for EventsEdendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_edend::W`](W) writer structure"]
impl crate::Writable for EventsEdendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_EDEND to value 0"]
impl crate::Resettable for EventsEdendSpec {}
