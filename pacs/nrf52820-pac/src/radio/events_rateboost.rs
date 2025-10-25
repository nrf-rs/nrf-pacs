#[doc = "Register `EVENTS_RATEBOOST` reader"]
pub type R = crate::R<EventsRateboostSpec>;
#[doc = "Register `EVENTS_RATEBOOST` writer"]
pub type W = crate::W<EventsRateboostSpec>;
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRateboost {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRateboost> for bool {
    #[inline(always)]
    fn from(variant: EventsRateboost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RATEBOOST` reader - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
pub type EventsRateboostR = crate::BitReader<EventsRateboost>;
impl EventsRateboostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRateboost {
        match self.bits {
            false => EventsRateboost::NotGenerated,
            true => EventsRateboost::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRateboost::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRateboost::Generated
    }
}
#[doc = "Field `EVENTS_RATEBOOST` writer - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
pub type EventsRateboostW<'a, REG> = crate::BitWriter<'a, REG, EventsRateboost>;
impl<'a, REG> EventsRateboostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRateboost::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRateboost::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub fn events_rateboost(&self) -> EventsRateboostR {
        EventsRateboostR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub fn events_rateboost(&mut self) -> EventsRateboostW<'_, EventsRateboostSpec> {
        EventsRateboostW::new(self, 0)
    }
}
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rateboost::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rateboost::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRateboostSpec;
impl crate::RegisterSpec for EventsRateboostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rateboost::R`](R) reader structure"]
impl crate::Readable for EventsRateboostSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rateboost::W`](W) writer structure"]
impl crate::Writable for EventsRateboostSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RATEBOOST to value 0"]
impl crate::Resettable for EventsRateboostSpec {}
