#[doc = "Register `ERASESTATUS` reader"]
pub type R = crate::R<ErasestatusSpec>;
#[doc = "Register `ERASESTATUS` writer"]
pub type W = crate::W<ErasestatusSpec>;
#[doc = "Cache erase status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erasestatus {
    #[doc = "0: Erase is not complete or hasn't started"]
    Idle = 0,
    #[doc = "1: Cache erase is finished"]
    Finished = 1,
}
impl From<Erasestatus> for bool {
    #[inline(always)]
    fn from(variant: Erasestatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASESTATUS` reader - Cache erase status"]
pub type ErasestatusR = crate::BitReader<Erasestatus>;
impl ErasestatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erasestatus {
        match self.bits {
            false => Erasestatus::Idle,
            true => Erasestatus::Finished,
        }
    }
    #[doc = "Erase is not complete or hasn't started"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Erasestatus::Idle
    }
    #[doc = "Cache erase is finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Erasestatus::Finished
    }
}
#[doc = "Field `ERASESTATUS` writer - Cache erase status"]
pub type ErasestatusW<'a, REG> = crate::BitWriter<'a, REG, Erasestatus>;
impl<'a, REG> ErasestatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase is not complete or hasn't started"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Erasestatus::Idle)
    }
    #[doc = "Cache erase is finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(Erasestatus::Finished)
    }
}
impl R {
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&self) -> ErasestatusR {
        ErasestatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&mut self) -> ErasestatusW<'_, ErasestatusSpec> {
        ErasestatusW::new(self, 0)
    }
}
#[doc = "Cache erase status.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasestatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasestatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErasestatusSpec;
impl crate::RegisterSpec for ErasestatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasestatus::R`](R) reader structure"]
impl crate::Readable for ErasestatusSpec {}
#[doc = "`write(|w| ..)` method takes [`erasestatus::W`](W) writer structure"]
impl crate::Writable for ErasestatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASESTATUS to value 0"]
impl crate::Resettable for ErasestatusSpec {}
