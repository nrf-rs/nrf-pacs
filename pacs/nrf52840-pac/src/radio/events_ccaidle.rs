#[doc = "Register `EVENTS_CCAIDLE` reader"]
pub type R = crate::R<EventsCcaidleSpec>;
#[doc = "Register `EVENTS_CCAIDLE` writer"]
pub type W = crate::W<EventsCcaidleSpec>;
#[doc = "Wireless medium in idle - clear to send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCcaidle {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCcaidle> for bool {
    #[inline(always)]
    fn from(variant: EventsCcaidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CCAIDLE` reader - Wireless medium in idle - clear to send"]
pub type EventsCcaidleR = crate::BitReader<EventsCcaidle>;
impl EventsCcaidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCcaidle {
        match self.bits {
            false => EventsCcaidle::NotGenerated,
            true => EventsCcaidle::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCcaidle::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCcaidle::Generated
    }
}
#[doc = "Field `EVENTS_CCAIDLE` writer - Wireless medium in idle - clear to send"]
pub type EventsCcaidleW<'a, REG> = crate::BitWriter<'a, REG, EventsCcaidle>;
impl<'a, REG> EventsCcaidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcaidle::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcaidle::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub fn events_ccaidle(&self) -> EventsCcaidleR {
        EventsCcaidleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub fn events_ccaidle(&mut self) -> EventsCcaidleW<'_, EventsCcaidleSpec> {
        EventsCcaidleW::new(self, 0)
    }
}
#[doc = "Wireless medium in idle - clear to send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccaidle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccaidle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCcaidleSpec;
impl crate::RegisterSpec for EventsCcaidleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ccaidle::R`](R) reader structure"]
impl crate::Readable for EventsCcaidleSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ccaidle::W`](W) writer structure"]
impl crate::Writable for EventsCcaidleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CCAIDLE to value 0"]
impl crate::Resettable for EventsCcaidleSpec {}
