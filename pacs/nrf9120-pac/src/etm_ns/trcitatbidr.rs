#[doc = "Register `TRCITATBIDR` reader"]
pub type R = crate::R<TrcitatbidrSpec>;
#[doc = "Register `TRCITATBIDR` writer"]
pub type W = crate::W<TrcitatbidrSpec>;
#[doc = "Field `ID_0` reader - Drives the ATIDMI\\[0\\] output pin."]
pub type Id0R = crate::BitReader;
#[doc = "Field `ID_0` writer - Drives the ATIDMI\\[0\\] output pin."]
pub type Id0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_1` reader - Drives the ATIDMI\\[1\\] output pin."]
pub type Id1R = crate::BitReader;
#[doc = "Field `ID_1` writer - Drives the ATIDMI\\[1\\] output pin."]
pub type Id1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_2` reader - Drives the ATIDMI\\[2\\] output pin."]
pub type Id2R = crate::BitReader;
#[doc = "Field `ID_2` writer - Drives the ATIDMI\\[2\\] output pin."]
pub type Id2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_3` reader - Drives the ATIDMI\\[3\\] output pin."]
pub type Id3R = crate::BitReader;
#[doc = "Field `ID_3` writer - Drives the ATIDMI\\[3\\] output pin."]
pub type Id3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_4` reader - Drives the ATIDMI\\[4\\] output pin."]
pub type Id4R = crate::BitReader;
#[doc = "Field `ID_4` writer - Drives the ATIDMI\\[4\\] output pin."]
pub type Id4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_5` reader - Drives the ATIDMI\\[5\\] output pin."]
pub type Id5R = crate::BitReader;
#[doc = "Field `ID_5` writer - Drives the ATIDMI\\[5\\] output pin."]
pub type Id5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_6` reader - Drives the ATIDMI\\[6\\] output pin."]
pub type Id6R = crate::BitReader;
#[doc = "Field `ID_6` writer - Drives the ATIDMI\\[6\\] output pin."]
pub type Id6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drives the ATIDMI\\[0\\] output pin."]
    #[inline(always)]
    pub fn id_0(&self) -> Id0R {
        Id0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drives the ATIDMI\\[1\\] output pin."]
    #[inline(always)]
    pub fn id_1(&self) -> Id1R {
        Id1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drives the ATIDMI\\[2\\] output pin."]
    #[inline(always)]
    pub fn id_2(&self) -> Id2R {
        Id2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drives the ATIDMI\\[3\\] output pin."]
    #[inline(always)]
    pub fn id_3(&self) -> Id3R {
        Id3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Drives the ATIDMI\\[4\\] output pin."]
    #[inline(always)]
    pub fn id_4(&self) -> Id4R {
        Id4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Drives the ATIDMI\\[5\\] output pin."]
    #[inline(always)]
    pub fn id_5(&self) -> Id5R {
        Id5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drives the ATIDMI\\[6\\] output pin."]
    #[inline(always)]
    pub fn id_6(&self) -> Id6R {
        Id6R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drives the ATIDMI\\[0\\] output pin."]
    #[inline(always)]
    pub fn id_0(&mut self) -> Id0W<'_, TrcitatbidrSpec> {
        Id0W::new(self, 0)
    }
    #[doc = "Bit 1 - Drives the ATIDMI\\[1\\] output pin."]
    #[inline(always)]
    pub fn id_1(&mut self) -> Id1W<'_, TrcitatbidrSpec> {
        Id1W::new(self, 1)
    }
    #[doc = "Bit 2 - Drives the ATIDMI\\[2\\] output pin."]
    #[inline(always)]
    pub fn id_2(&mut self) -> Id2W<'_, TrcitatbidrSpec> {
        Id2W::new(self, 2)
    }
    #[doc = "Bit 3 - Drives the ATIDMI\\[3\\] output pin."]
    #[inline(always)]
    pub fn id_3(&mut self) -> Id3W<'_, TrcitatbidrSpec> {
        Id3W::new(self, 3)
    }
    #[doc = "Bit 4 - Drives the ATIDMI\\[4\\] output pin."]
    #[inline(always)]
    pub fn id_4(&mut self) -> Id4W<'_, TrcitatbidrSpec> {
        Id4W::new(self, 4)
    }
    #[doc = "Bit 5 - Drives the ATIDMI\\[5\\] output pin."]
    #[inline(always)]
    pub fn id_5(&mut self) -> Id5W<'_, TrcitatbidrSpec> {
        Id5W::new(self, 5)
    }
    #[doc = "Bit 6 - Drives the ATIDMI\\[6\\] output pin."]
    #[inline(always)]
    pub fn id_6(&mut self) -> Id6W<'_, TrcitatbidrSpec> {
        Id6W::new(self, 6)
    }
}
#[doc = "Sets the state of output pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitatbidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitatbidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcitatbidrSpec;
impl crate::RegisterSpec for TrcitatbidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitatbidr::R`](R) reader structure"]
impl crate::Readable for TrcitatbidrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcitatbidr::W`](W) writer structure"]
impl crate::Writable for TrcitatbidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCITATBIDR to value 0"]
impl crate::Resettable for TrcitatbidrSpec {}
