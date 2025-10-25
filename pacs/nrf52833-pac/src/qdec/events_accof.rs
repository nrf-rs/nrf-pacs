#[doc = "Register `EVENTS_ACCOF` reader"]
pub type R = crate::R<EventsAccofSpec>;
#[doc = "Register `EVENTS_ACCOF` writer"]
pub type W = crate::W<EventsAccofSpec>;
#[doc = "ACC or ACCDBL register overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsAccof {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsAccof> for bool {
    #[inline(always)]
    fn from(variant: EventsAccof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ACCOF` reader - ACC or ACCDBL register overflow"]
pub type EventsAccofR = crate::BitReader<EventsAccof>;
impl EventsAccofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsAccof {
        match self.bits {
            false => EventsAccof::NotGenerated,
            true => EventsAccof::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsAccof::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsAccof::Generated
    }
}
#[doc = "Field `EVENTS_ACCOF` writer - ACC or ACCDBL register overflow"]
pub type EventsAccofW<'a, REG> = crate::BitWriter<'a, REG, EventsAccof>;
impl<'a, REG> EventsAccofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAccof::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAccof::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub fn events_accof(&self) -> EventsAccofR {
        EventsAccofR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub fn events_accof(&mut self) -> EventsAccofW<'_, EventsAccofSpec> {
        EventsAccofW::new(self, 0)
    }
}
#[doc = "ACC or ACCDBL register overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_accof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_accof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAccofSpec;
impl crate::RegisterSpec for EventsAccofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_accof::R`](R) reader structure"]
impl crate::Readable for EventsAccofSpec {}
#[doc = "`write(|w| ..)` method takes [`events_accof::W`](W) writer structure"]
impl crate::Writable for EventsAccofSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ACCOF to value 0"]
impl crate::Resettable for EventsAccofSpec {}
