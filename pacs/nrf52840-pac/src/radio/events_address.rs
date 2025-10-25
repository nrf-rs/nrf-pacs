#[doc = "Register `EVENTS_ADDRESS` reader"]
pub type R = crate::R<EventsAddressSpec>;
#[doc = "Register `EVENTS_ADDRESS` writer"]
pub type W = crate::W<EventsAddressSpec>;
#[doc = "Address sent or received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsAddress {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsAddress> for bool {
    #[inline(always)]
    fn from(variant: EventsAddress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ADDRESS` reader - Address sent or received"]
pub type EventsAddressR = crate::BitReader<EventsAddress>;
impl EventsAddressR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsAddress {
        match self.bits {
            false => EventsAddress::NotGenerated,
            true => EventsAddress::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsAddress::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsAddress::Generated
    }
}
#[doc = "Field `EVENTS_ADDRESS` writer - Address sent or received"]
pub type EventsAddressW<'a, REG> = crate::BitWriter<'a, REG, EventsAddress>;
impl<'a, REG> EventsAddressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAddress::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAddress::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Address sent or received"]
    #[inline(always)]
    pub fn events_address(&self) -> EventsAddressR {
        EventsAddressR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address sent or received"]
    #[inline(always)]
    pub fn events_address(&mut self) -> EventsAddressW<'_, EventsAddressSpec> {
        EventsAddressW::new(self, 0)
    }
}
#[doc = "Address sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAddressSpec;
impl crate::RegisterSpec for EventsAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_address::R`](R) reader structure"]
impl crate::Readable for EventsAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`events_address::W`](W) writer structure"]
impl crate::Writable for EventsAddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ADDRESS to value 0"]
impl crate::Resettable for EventsAddressSpec {}
