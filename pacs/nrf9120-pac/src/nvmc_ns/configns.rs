#[doc = "Register `CONFIGNS` reader"]
pub type R = crate::R<ConfignsSpec>;
#[doc = "Register `CONFIGNS` writer"]
pub type W = crate::W<ConfignsSpec>;
#[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wen {
    #[doc = "0: Read only access"]
    Ren = 0,
    #[doc = "1: Write enabled"]
    Wen = 1,
    #[doc = "2: Erase enabled"]
    Een = 2,
}
impl From<Wen> for u8 {
    #[inline(always)]
    fn from(variant: Wen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wen {
    type Ux = u8;
}
impl crate::IsEnum for Wen {}
#[doc = "Field `WEN` reader - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub type WenR = crate::FieldReader<Wen>;
impl WenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wen> {
        match self.bits {
            0 => Some(Wen::Ren),
            1 => Some(Wen::Wen),
            2 => Some(Wen::Een),
            _ => None,
        }
    }
    #[doc = "Read only access"]
    #[inline(always)]
    pub fn is_ren(&self) -> bool {
        *self == Wen::Ren
    }
    #[doc = "Write enabled"]
    #[inline(always)]
    pub fn is_wen(&self) -> bool {
        *self == Wen::Wen
    }
    #[doc = "Erase enabled"]
    #[inline(always)]
    pub fn is_een(&self) -> bool {
        *self == Wen::Een
    }
}
#[doc = "Field `WEN` writer - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub type WenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wen>;
impl<'a, REG> WenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read only access"]
    #[inline(always)]
    pub fn ren(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Ren)
    }
    #[doc = "Write enabled"]
    #[inline(always)]
    pub fn wen(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Wen)
    }
    #[doc = "Erase enabled"]
    #[inline(always)]
    pub fn een(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Een)
    }
}
impl R {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WenW<ConfignsSpec> {
        WenW::new(self, 0)
    }
}
#[doc = "Unspecified\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configns::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configns::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfignsSpec;
impl crate::RegisterSpec for ConfignsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configns::R`](R) reader structure"]
impl crate::Readable for ConfignsSpec {}
#[doc = "`write(|w| ..)` method takes [`configns::W`](W) writer structure"]
impl crate::Writable for ConfignsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIGNS to value 0"]
impl crate::Resettable for ConfignsSpec {
    const RESET_VALUE: u32 = 0;
}
