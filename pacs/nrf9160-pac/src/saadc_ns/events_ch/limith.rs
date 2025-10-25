#[doc = "Register `LIMITH` reader"]
pub type R = crate::R<LimithSpec>;
#[doc = "Register `LIMITH` writer"]
pub type W = crate::W<LimithSpec>;
#[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Limith {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Limith> for bool {
    #[inline(always)]
    fn from(variant: Limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMITH` reader - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
pub type LimithR = crate::BitReader<Limith>;
impl LimithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Limith {
        match self.bits {
            false => Limith::NotGenerated,
            true => Limith::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Limith::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Limith::Generated
    }
}
#[doc = "Field `LIMITH` writer - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
pub type LimithW<'a, REG> = crate::BitWriter<'a, REG, Limith>;
impl<'a, REG> LimithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Limith::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Limith::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub fn limith(&self) -> LimithR {
        LimithR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub fn limith(&mut self) -> LimithW<'_, LimithSpec> {
        LimithW::new(self, 0)
    }
}
#[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH\n\nYou can [`read`](crate::Reg::read) this register and get [`limith::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limith::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimithSpec;
impl crate::RegisterSpec for LimithSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limith::R`](R) reader structure"]
impl crate::Readable for LimithSpec {}
#[doc = "`write(|w| ..)` method takes [`limith::W`](W) writer structure"]
impl crate::Writable for LimithSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMITH to value 0"]
impl crate::Resettable for LimithSpec {}
