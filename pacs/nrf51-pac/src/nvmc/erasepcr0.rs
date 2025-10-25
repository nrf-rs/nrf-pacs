#[doc = "Register `ERASEPCR0` reader"]
pub type R = crate::R<Erasepcr0Spec>;
#[doc = "Register `ERASEPCR0` writer"]
pub type W = crate::W<Erasepcr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Register for erasing a protected non-volatile memory page.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Erasepcr0Spec;
impl crate::RegisterSpec for Erasepcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasepcr0::R`](R) reader structure"]
impl crate::Readable for Erasepcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`erasepcr0::W`](W) writer structure"]
impl crate::Writable for Erasepcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPCR0 to value 0"]
impl crate::Resettable for Erasepcr0Spec {}
