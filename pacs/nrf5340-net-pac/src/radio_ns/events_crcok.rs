#[doc = "Register `EVENTS_CRCOK` reader"]
pub type R = crate::R<EventsCrcokSpec>;
#[doc = "Register `EVENTS_CRCOK` writer"]
pub type W = crate::W<EventsCrcokSpec>;
#[doc = "Packet received with CRC ok\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCrcok {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCrcok> for bool {
    #[inline(always)]
    fn from(variant: EventsCrcok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CRCOK` reader - Packet received with CRC ok"]
pub type EventsCrcokR = crate::BitReader<EventsCrcok>;
impl EventsCrcokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCrcok {
        match self.bits {
            false => EventsCrcok::NotGenerated,
            true => EventsCrcok::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCrcok::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCrcok::Generated
    }
}
#[doc = "Field `EVENTS_CRCOK` writer - Packet received with CRC ok"]
pub type EventsCrcokW<'a, REG> = crate::BitWriter<'a, REG, EventsCrcok>;
impl<'a, REG> EventsCrcokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCrcok::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCrcok::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Packet received with CRC ok"]
    #[inline(always)]
    pub fn events_crcok(&self) -> EventsCrcokR {
        EventsCrcokR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet received with CRC ok"]
    #[inline(always)]
    pub fn events_crcok(&mut self) -> EventsCrcokW<'_, EventsCrcokSpec> {
        EventsCrcokW::new(self, 0)
    }
}
#[doc = "Packet received with CRC ok\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcok::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcok::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrcokSpec;
impl crate::RegisterSpec for EventsCrcokSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_crcok::R`](R) reader structure"]
impl crate::Readable for EventsCrcokSpec {}
#[doc = "`write(|w| ..)` method takes [`events_crcok::W`](W) writer structure"]
impl crate::Writable for EventsCrcokSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CRCOK to value 0"]
impl crate::Resettable for EventsCrcokSpec {}
