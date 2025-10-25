#[doc = "Register `EVENTS_CTS` reader"]
pub type R = crate::R<EventsCtsSpec>;
#[doc = "Register `EVENTS_CTS` writer"]
pub type W = crate::W<EventsCtsSpec>;
#[doc = "CTS is activated (set low). Clear To Send.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCts {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCts> for bool {
    #[inline(always)]
    fn from(variant: EventsCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTS` reader - CTS is activated (set low). Clear To Send."]
pub type EventsCtsR = crate::BitReader<EventsCts>;
impl EventsCtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCts {
        match self.bits {
            false => EventsCts::NotGenerated,
            true => EventsCts::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCts::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCts::Generated
    }
}
#[doc = "Field `EVENTS_CTS` writer - CTS is activated (set low). Clear To Send."]
pub type EventsCtsW<'a, REG> = crate::BitWriter<'a, REG, EventsCts>;
impl<'a, REG> EventsCtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCts::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCts::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub fn events_cts(&self) -> EventsCtsR {
        EventsCtsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub fn events_cts(&mut self) -> EventsCtsW<'_, EventsCtsSpec> {
        EventsCtsW::new(self, 0)
    }
}
#[doc = "CTS is activated (set low). Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCtsSpec;
impl crate::RegisterSpec for EventsCtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_cts::R`](R) reader structure"]
impl crate::Readable for EventsCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`events_cts::W`](W) writer structure"]
impl crate::Writable for EventsCtsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTS to value 0"]
impl crate::Resettable for EventsCtsSpec {}
