#[doc = "Register `EVENTS_CCASTOPPED` reader"]
pub type R = crate::R<EventsCcastoppedSpec>;
#[doc = "Register `EVENTS_CCASTOPPED` writer"]
pub type W = crate::W<EventsCcastoppedSpec>;
#[doc = "The CCA has stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCcastopped {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCcastopped> for bool {
    #[inline(always)]
    fn from(variant: EventsCcastopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CCASTOPPED` reader - The CCA has stopped"]
pub type EventsCcastoppedR = crate::BitReader<EventsCcastopped>;
impl EventsCcastoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCcastopped {
        match self.bits {
            false => EventsCcastopped::NotGenerated,
            true => EventsCcastopped::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCcastopped::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCcastopped::Generated
    }
}
#[doc = "Field `EVENTS_CCASTOPPED` writer - The CCA has stopped"]
pub type EventsCcastoppedW<'a, REG> = crate::BitWriter<'a, REG, EventsCcastopped>;
impl<'a, REG> EventsCcastoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcastopped::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcastopped::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The CCA has stopped"]
    #[inline(always)]
    pub fn events_ccastopped(&self) -> EventsCcastoppedR {
        EventsCcastoppedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The CCA has stopped"]
    #[inline(always)]
    pub fn events_ccastopped(&mut self) -> EventsCcastoppedW<'_, EventsCcastoppedSpec> {
        EventsCcastoppedW::new(self, 0)
    }
}
#[doc = "The CCA has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccastopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccastopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCcastoppedSpec;
impl crate::RegisterSpec for EventsCcastoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ccastopped::R`](R) reader structure"]
impl crate::Readable for EventsCcastoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ccastopped::W`](W) writer structure"]
impl crate::Writable for EventsCcastoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CCASTOPPED to value 0"]
impl crate::Resettable for EventsCcastoppedSpec {}
