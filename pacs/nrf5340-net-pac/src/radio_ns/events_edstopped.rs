#[doc = "Register `EVENTS_EDSTOPPED` reader"]
pub type R = crate::R<EventsEdstoppedSpec>;
#[doc = "Register `EVENTS_EDSTOPPED` writer"]
pub type W = crate::W<EventsEdstoppedSpec>;
#[doc = "The sampling of energy detection has stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEdstopped {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEdstopped> for bool {
    #[inline(always)]
    fn from(variant: EventsEdstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_EDSTOPPED` reader - The sampling of energy detection has stopped"]
pub type EventsEdstoppedR = crate::BitReader<EventsEdstopped>;
impl EventsEdstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEdstopped {
        match self.bits {
            false => EventsEdstopped::NotGenerated,
            true => EventsEdstopped::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEdstopped::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEdstopped::Generated
    }
}
#[doc = "Field `EVENTS_EDSTOPPED` writer - The sampling of energy detection has stopped"]
pub type EventsEdstoppedW<'a, REG> = crate::BitWriter<'a, REG, EventsEdstopped>;
impl<'a, REG> EventsEdstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEdstopped::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEdstopped::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The sampling of energy detection has stopped"]
    #[inline(always)]
    pub fn events_edstopped(&self) -> EventsEdstoppedR {
        EventsEdstoppedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The sampling of energy detection has stopped"]
    #[inline(always)]
    pub fn events_edstopped(&mut self) -> EventsEdstoppedW<'_, EventsEdstoppedSpec> {
        EventsEdstoppedW::new(self, 0)
    }
}
#[doc = "The sampling of energy detection has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edstopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edstopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEdstoppedSpec;
impl crate::RegisterSpec for EventsEdstoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_edstopped::R`](R) reader structure"]
impl crate::Readable for EventsEdstoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_edstopped::W`](W) writer structure"]
impl crate::Writable for EventsEdstoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_EDSTOPPED to value 0"]
impl crate::Resettable for EventsEdstoppedSpec {}
