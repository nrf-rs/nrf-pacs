#[doc = "Register `WAY[%s]` reader"]
pub type R = crate::R<WaySpec>;
#[doc = "Register `WAY[%s]` writer"]
pub type W = crate::W<WaySpec>;
#[doc = "Field `TAG` reader - Cache tag."]
pub type TagR = crate::FieldReader<u32>;
#[doc = "Field `TAG` writer - Cache tag."]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Valid bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V {
    #[doc = "0: Invalid cache line"]
    Invalid = 0,
    #[doc = "1: Valid cache line"]
    Valid = 1,
}
impl From<V> for bool {
    #[inline(always)]
    fn from(variant: V) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Valid bit"]
pub type VR = crate::BitReader<V>;
impl VR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V {
        match self.bits {
            false => V::Invalid,
            true => V::Valid,
        }
    }
    #[doc = "Invalid cache line"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == V::Invalid
    }
    #[doc = "Valid cache line"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == V::Valid
    }
}
#[doc = "Most recently used way.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mru {
    #[doc = "0: Way0 was most recently used"]
    Way0 = 0,
    #[doc = "1: Way1 was most recently used"]
    Way1 = 1,
}
impl From<Mru> for bool {
    #[inline(always)]
    fn from(variant: Mru) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRU` reader - Most recently used way."]
pub type MruR = crate::BitReader<Mru>;
impl MruR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mru {
        match self.bits {
            false => Mru::Way0,
            true => Mru::Way1,
        }
    }
    #[doc = "Way0 was most recently used"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == Mru::Way0
    }
    #[doc = "Way1 was most recently used"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == Mru::Way1
    }
}
impl R {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 30 - Valid bit"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Most recently used way."]
    #[inline(always)]
    pub fn mru(&self) -> MruR {
        MruR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&mut self) -> TagW<'_, WaySpec> {
        TagW::new(self, 0)
    }
}
#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`way::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`way::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaySpec;
impl crate::RegisterSpec for WaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`way::R`](R) reader structure"]
impl crate::Readable for WaySpec {}
#[doc = "`write(|w| ..)` method takes [`way::W`](W) writer structure"]
impl crate::Writable for WaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAY[%s] to value 0"]
impl crate::Resettable for WaySpec {}
