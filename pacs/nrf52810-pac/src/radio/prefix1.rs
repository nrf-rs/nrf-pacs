#[doc = "Register `PREFIX1` reader"]
pub type R = crate::R<Prefix1Spec>;
#[doc = "Register `PREFIX1` writer"]
pub type W = crate::W<Prefix1Spec>;
#[doc = "Field `AP4` reader - Address prefix 4."]
pub type Ap4R = crate::FieldReader;
#[doc = "Field `AP4` writer - Address prefix 4."]
pub type Ap4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP5` reader - Address prefix 5."]
pub type Ap5R = crate::FieldReader;
#[doc = "Field `AP5` writer - Address prefix 5."]
pub type Ap5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP6` reader - Address prefix 6."]
pub type Ap6R = crate::FieldReader;
#[doc = "Field `AP6` writer - Address prefix 6."]
pub type Ap6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP7` reader - Address prefix 7."]
pub type Ap7R = crate::FieldReader;
#[doc = "Field `AP7` writer - Address prefix 7."]
pub type Ap7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&self) -> Ap4R {
        Ap4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&self) -> Ap5R {
        Ap5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&self) -> Ap6R {
        Ap6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&self) -> Ap7R {
        Ap7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&mut self) -> Ap4W<'_, Prefix1Spec> {
        Ap4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&mut self) -> Ap5W<'_, Prefix1Spec> {
        Ap5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&mut self) -> Ap6W<'_, Prefix1Spec> {
        Ap6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&mut self) -> Ap7W<'_, Prefix1Spec> {
        Ap7W::new(self, 24)
    }
}
#[doc = "Prefixes bytes for logical addresses 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prefix1Spec;
impl crate::RegisterSpec for Prefix1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefix1::R`](R) reader structure"]
impl crate::Readable for Prefix1Spec {}
#[doc = "`write(|w| ..)` method takes [`prefix1::W`](W) writer structure"]
impl crate::Writable for Prefix1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PREFIX1 to value 0"]
impl crate::Resettable for Prefix1Spec {}
