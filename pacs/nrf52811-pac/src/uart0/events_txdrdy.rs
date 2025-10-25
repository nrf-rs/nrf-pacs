#[doc = "Register `EVENTS_TXDRDY` reader"]
pub type R = crate::R<EventsTxdrdySpec>;
#[doc = "Register `EVENTS_TXDRDY` writer"]
pub type W = crate::W<EventsTxdrdySpec>;
#[doc = "Data sent from TXD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxdrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxdrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsTxdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXDRDY` reader - Data sent from TXD"]
pub type EventsTxdrdyR = crate::BitReader<EventsTxdrdy>;
impl EventsTxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxdrdy {
        match self.bits {
            false => EventsTxdrdy::NotGenerated,
            true => EventsTxdrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxdrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxdrdy::Generated
    }
}
#[doc = "Field `EVENTS_TXDRDY` writer - Data sent from TXD"]
pub type EventsTxdrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsTxdrdy>;
impl<'a, REG> EventsTxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Data sent from TXD"]
    #[inline(always)]
    pub fn events_txdrdy(&self) -> EventsTxdrdyR {
        EventsTxdrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data sent from TXD"]
    #[inline(always)]
    pub fn events_txdrdy(&mut self) -> EventsTxdrdyW<'_, EventsTxdrdySpec> {
        EventsTxdrdyW::new(self, 0)
    }
}
#[doc = "Data sent from TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxdrdySpec;
impl crate::RegisterSpec for EventsTxdrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txdrdy::R`](R) reader structure"]
impl crate::Readable for EventsTxdrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_txdrdy::W`](W) writer structure"]
impl crate::Writable for EventsTxdrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXDRDY to value 0"]
impl crate::Resettable for EventsTxdrdySpec {}
