#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Program write enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wen {
    #[doc = "0: Read only access."]
    Ren = 0,
    #[doc = "1: Write enabled."]
    Wen = 1,
    #[doc = "2: Erase enabled."]
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
#[doc = "Field `WEN` reader - Program write enable."]
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
    #[doc = "Read only access."]
    #[inline(always)]
    pub fn is_ren(&self) -> bool {
        *self == Wen::Ren
    }
    #[doc = "Write enabled."]
    #[inline(always)]
    pub fn is_wen(&self) -> bool {
        *self == Wen::Wen
    }
    #[doc = "Erase enabled."]
    #[inline(always)]
    pub fn is_een(&self) -> bool {
        *self == Wen::Een
    }
}
#[doc = "Field `WEN` writer - Program write enable."]
pub type WenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wen>;
impl<'a, REG> WenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read only access."]
    #[inline(always)]
    pub fn ren(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Ren)
    }
    #[doc = "Write enabled."]
    #[inline(always)]
    pub fn wen(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Wen)
    }
    #[doc = "Erase enabled."]
    #[inline(always)]
    pub fn een(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Een)
    }
}
impl R {
    #[doc = "Bits 0:1 - Program write enable."]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Program write enable."]
    #[inline(always)]
    pub fn wen(&mut self) -> WenW<'_, ConfigSpec> {
        WenW::new(self, 0)
    }
}
#[doc = "Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
