#[doc = "Register `WA` reader"]
pub type R = crate::R<WaSpec>;
#[doc = "Register `WA` writer"]
pub type W = crate::W<WaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description cluster\\[0\\]: Write access to region 0 detected\n\nYou can [`read`](crate::Reg::read) this register and get [`wa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
