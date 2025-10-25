#[doc = "Register `ADDRPTR` reader"]
pub type R = crate::R<AddrptrSpec>;
#[doc = "Register `ADDRPTR` writer"]
pub type W = crate::W<AddrptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to the resolvable address (6 bytes).\n\nYou can [`read`](crate::Reg::read) this register and get [`addrptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrptrSpec;
impl crate::RegisterSpec for AddrptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrptr::R`](R) reader structure"]
impl crate::Readable for AddrptrSpec {}
#[doc = "`write(|w| ..)` method takes [`addrptr::W`](W) writer structure"]
impl crate::Writable for AddrptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRPTR to value 0"]
impl crate::Resettable for AddrptrSpec {}
