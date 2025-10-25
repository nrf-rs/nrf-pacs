#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Enable or disable address matching on ADDRESS\\[0\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address0 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Address0> for bool {
    #[inline(always)]
    fn from(variant: Address0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS0` reader - Enable or disable address matching on ADDRESS\\[0\\]"]
pub type Address0R = crate::BitReader<Address0>;
impl Address0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Address0 {
        match self.bits {
            false => Address0::Disabled,
            true => Address0::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Address0::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Address0::Enabled
    }
}
#[doc = "Field `ADDRESS0` writer - Enable or disable address matching on ADDRESS\\[0\\]"]
pub type Address0W<'a, REG> = crate::BitWriter<'a, REG, Address0>;
impl<'a, REG> Address0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Address0::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Address0::Enabled)
    }
}
#[doc = "Enable or disable address matching on ADDRESS\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address1 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Address1> for bool {
    #[inline(always)]
    fn from(variant: Address1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS1` reader - Enable or disable address matching on ADDRESS\\[1\\]"]
pub type Address1R = crate::BitReader<Address1>;
impl Address1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Address1 {
        match self.bits {
            false => Address1::Disabled,
            true => Address1::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Address1::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Address1::Enabled
    }
}
#[doc = "Field `ADDRESS1` writer - Enable or disable address matching on ADDRESS\\[1\\]"]
pub type Address1W<'a, REG> = crate::BitWriter<'a, REG, Address1>;
impl<'a, REG> Address1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Address1::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Address1::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn address0(&self) -> Address0R {
        Address0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn address1(&self) -> Address1R {
        Address1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn address0(&mut self) -> Address0W<'_, ConfigSpec> {
        Address0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn address1(&mut self) -> Address1W<'_, ConfigSpec> {
        Address1W::new(self, 1)
    }
}
#[doc = "Configuration register for the address match mechanism\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x01;
}
