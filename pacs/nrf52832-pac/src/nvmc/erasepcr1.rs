#[doc = "Register `ERASEPCR1` reader"]
pub type R = crate::R<Erasepcr1Spec>;
#[doc = "Register `ERASEPCR1` writer"]
pub type W = crate::W<Erasepcr1Spec>;
#[doc = "Field `ERASEPCR1` reader - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub type Erasepcr1R = crate::FieldReader<u32>;
#[doc = "Field `ERASEPCR1` writer - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub type Erasepcr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr1(&self) -> Erasepcr1R {
        Erasepcr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr1(&mut self) -> Erasepcr1W<'_, Erasepcr1Spec> {
        Erasepcr1W::new(self, 0)
    }
}
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Erasepcr1Spec;
impl crate::RegisterSpec for Erasepcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasepcr1::R`](R) reader structure"]
impl crate::Readable for Erasepcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`erasepcr1::W`](W) writer structure"]
impl crate::Writable for Erasepcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPCR1 to value 0"]
impl crate::Resettable for Erasepcr1Spec {}
