#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Configure write and erase permissions for region n. Writing a '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Allow write and erase instructions to region n."]
    Enable = 0,
    #[doc = "1: Block write and erase instructions to region n."]
    Disable = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Configure write and erase permissions for region n. Writing a '0' has no effect."]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            false => Write::Enable,
            true => Write::Disable,
        }
    }
    #[doc = "Allow write and erase instructions to region n."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Write::Enable
    }
    #[doc = "Block write and erase instructions to region n."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Write::Disable
    }
}
#[doc = "Field `WRITE` writer - Configure write and erase permissions for region n. Writing a '0' has no effect."]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow write and erase instructions to region n."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Enable)
    }
    #[doc = "Block write and erase instructions to region n."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Disable)
    }
}
#[doc = "Configure read permissions for region n. Writing a '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Allow read instructions to region n."]
    Enable = 0,
    #[doc = "1: Block read instructions to region n."]
    Disable = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Configure read permissions for region n. Writing a '0' has no effect."]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::Enable,
            true => Read::Disable,
        }
    }
    #[doc = "Allow read instructions to region n."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Read::Enable
    }
    #[doc = "Block read instructions to region n."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Read::Disable
    }
}
#[doc = "Field `READ` writer - Configure read permissions for region n. Writing a '0' has no effect."]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow read instructions to region n."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Enable)
    }
    #[doc = "Block read instructions to region n."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Disable)
    }
}
impl R {
    #[doc = "Bit 1 - Configure write and erase permissions for region n. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for region n. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Configure write and erase permissions for region n. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, PermSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Configure read permissions for region n. Writing a '0' has no effect."]
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<'_, PermSpec> {
        ReadW::new(self, 2)
    }
}
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERM to value 0"]
impl crate::Resettable for PermSpec {}
