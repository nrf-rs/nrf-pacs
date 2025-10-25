#[doc = "Register `EVENTS_CRCERROR` reader"]
pub type R = crate::R<EventsCrcerrorSpec>;
#[doc = "Register `EVENTS_CRCERROR` writer"]
pub type W = crate::W<EventsCrcerrorSpec>;
#[doc = "Packet received with CRC error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCrcerror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCrcerror> for bool {
    #[inline(always)]
    fn from(variant: EventsCrcerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CRCERROR` reader - Packet received with CRC error"]
pub type EventsCrcerrorR = crate::BitReader<EventsCrcerror>;
impl EventsCrcerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCrcerror {
        match self.bits {
            false => EventsCrcerror::NotGenerated,
            true => EventsCrcerror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCrcerror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCrcerror::Generated
    }
}
#[doc = "Field `EVENTS_CRCERROR` writer - Packet received with CRC error"]
pub type EventsCrcerrorW<'a, REG> = crate::BitWriter<'a, REG, EventsCrcerror>;
impl<'a, REG> EventsCrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCrcerror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCrcerror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Packet received with CRC error"]
    #[inline(always)]
    pub fn events_crcerror(&self) -> EventsCrcerrorR {
        EventsCrcerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet received with CRC error"]
    #[inline(always)]
    pub fn events_crcerror(&mut self) -> EventsCrcerrorW<'_, EventsCrcerrorSpec> {
        EventsCrcerrorW::new(self, 0)
    }
}
#[doc = "Packet received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrcerrorSpec;
impl crate::RegisterSpec for EventsCrcerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_crcerror::R`](R) reader structure"]
impl crate::Readable for EventsCrcerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_crcerror::W`](W) writer structure"]
impl crate::Writable for EventsCrcerrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CRCERROR to value 0"]
impl crate::Resettable for EventsCrcerrorSpec {}
