#[doc = "Register `RA` reader"]
pub type R = crate::R<RaSpec>;
#[doc = "Register `RA` writer"]
pub type W = crate::W<RaSpec>;
#[doc = "Read access to peripheral region n detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ra {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Ra> for bool {
    #[inline(always)]
    fn from(variant: Ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RA` reader - Read access to peripheral region n detected"]
pub type RaR = crate::BitReader<Ra>;
impl RaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ra {
        match self.bits {
            false => Ra::NotGenerated,
            true => Ra::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Ra::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Ra::Generated
    }
}
#[doc = "Field `RA` writer - Read access to peripheral region n detected"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG, Ra>;
impl<'a, REG> RaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Ra::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Ra::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Read access to peripheral region n detected"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read access to peripheral region n detected"]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<'_, RaSpec> {
        RaW::new(self, 0)
    }
}
#[doc = "Description cluster: Read access to peripheral region n detected\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RaSpec;
impl crate::RegisterSpec for RaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra::R`](R) reader structure"]
impl crate::Readable for RaSpec {}
#[doc = "`write(|w| ..)` method takes [`ra::W`](W) writer structure"]
impl crate::Writable for RaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RaSpec {}
