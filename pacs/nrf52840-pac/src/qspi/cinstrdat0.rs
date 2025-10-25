#[doc = "Register `CINSTRDAT0` reader"]
pub type R = crate::R<Cinstrdat0Spec>;
#[doc = "Register `CINSTRDAT0` writer"]
pub type W = crate::W<Cinstrdat0Spec>;
#[doc = "Field `BYTE0` reader - Data byte 0"]
pub type Byte0R = crate::FieldReader;
#[doc = "Field `BYTE0` writer - Data byte 0"]
pub type Byte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Data byte 1"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Data byte 1"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE2` reader - Data byte 2"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - Data byte 2"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE3` reader - Data byte 3"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE3` writer - Data byte 3"]
pub type Byte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        Byte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&mut self) -> Byte0W<'_, Cinstrdat0Spec> {
        Byte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<'_, Cinstrdat0Spec> {
        Byte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> Byte2W<'_, Cinstrdat0Spec> {
        Byte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> Byte3W<'_, Cinstrdat0Spec> {
        Byte3W::new(self, 24)
    }
}
#[doc = "Custom instruction data register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrdat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrdat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cinstrdat0Spec;
impl crate::RegisterSpec for Cinstrdat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cinstrdat0::R`](R) reader structure"]
impl crate::Readable for Cinstrdat0Spec {}
#[doc = "`write(|w| ..)` method takes [`cinstrdat0::W`](W) writer structure"]
impl crate::Writable for Cinstrdat0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CINSTRDAT0 to value 0"]
impl crate::Resettable for Cinstrdat0Spec {}
