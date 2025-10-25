#[doc = "Register `WA` reader"]
pub type R = crate::R<WaSpec>;
#[doc = "Register `WA` writer"]
pub type W = crate::W<WaSpec>;
#[doc = "Write access to peripheral region n detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wa {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Wa> for bool {
    #[inline(always)]
    fn from(variant: Wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Write access to peripheral region n detected"]
pub type WaR = crate::BitReader<Wa>;
impl WaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wa {
        match self.bits {
            false => Wa::NotGenerated,
            true => Wa::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Wa::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Wa::Generated
    }
}
#[doc = "Field `WA` writer - Write access to peripheral region n detected"]
pub type WaW<'a, REG> = crate::BitWriter<'a, REG, Wa>;
impl<'a, REG> WaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Write access to peripheral region n detected"]
    #[inline(always)]
    pub fn wa(&self) -> WaR {
        WaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write access to peripheral region n detected"]
    #[inline(always)]
    pub fn wa(&mut self) -> WaW<'_, WaSpec> {
        WaW::new(self, 0)
    }
}
#[doc = "Description cluster: Write access to peripheral region n detected\n\nYou can [`read`](crate::Reg::read) this register and get [`wa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaSpec;
impl crate::RegisterSpec for WaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wa::R`](R) reader structure"]
impl crate::Readable for WaSpec {}
#[doc = "`write(|w| ..)` method takes [`wa::W`](W) writer structure"]
impl crate::Writable for WaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WA to value 0"]
impl crate::Resettable for WaSpec {}
