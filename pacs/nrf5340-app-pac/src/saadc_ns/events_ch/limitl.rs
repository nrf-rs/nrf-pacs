#[doc = "Register `LIMITL` reader"]
pub type R = crate::R<LimitlSpec>;
#[doc = "Register `LIMITL` writer"]
pub type W = crate::W<LimitlSpec>;
#[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Limitl {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Limitl> for bool {
    #[inline(always)]
    fn from(variant: Limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIMITL` reader - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub type LimitlR = crate::BitReader<Limitl>;
impl LimitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Limitl {
        match self.bits {
            false => Limitl::NotGenerated,
            true => Limitl::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Limitl::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Limitl::Generated
    }
}
#[doc = "Field `LIMITL` writer - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub type LimitlW<'a, REG> = crate::BitWriter<'a, REG, Limitl>;
impl<'a, REG> LimitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Limitl::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Limitl::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&self) -> LimitlR {
        LimitlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&mut self) -> LimitlW<'_, LimitlSpec> {
        LimitlW::new(self, 0)
    }
}
#[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nYou can [`read`](crate::Reg::read) this register and get [`limitl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limitl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitlSpec;
impl crate::RegisterSpec for LimitlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limitl::R`](R) reader structure"]
impl crate::Readable for LimitlSpec {}
#[doc = "`write(|w| ..)` method takes [`limitl::W`](W) writer structure"]
impl crate::Writable for LimitlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMITL to value 0"]
impl crate::Resettable for LimitlSpec {}
