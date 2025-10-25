#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0datadoneStartepin0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Ep0datadoneStartepin0> for bool {
    #[inline(always)]
    fn from(variant: Ep0datadoneStartepin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPIN0` reader - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
pub type Ep0datadoneStartepin0R = crate::BitReader<Ep0datadoneStartepin0>;
impl Ep0datadoneStartepin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0datadoneStartepin0 {
        match self.bits {
            false => Ep0datadoneStartepin0::Disabled,
            true => Ep0datadoneStartepin0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ep0datadoneStartepin0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ep0datadoneStartepin0::Enabled
    }
}
#[doc = "Field `EP0DATADONE_STARTEPIN0` writer - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
pub type Ep0datadoneStartepin0W<'a, REG> = crate::BitWriter<'a, REG, Ep0datadoneStartepin0>;
impl<'a, REG> Ep0datadoneStartepin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneStartepin0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneStartepin0::Enabled)
    }
}
#[doc = "Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0datadoneStartepout0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Ep0datadoneStartepout0> for bool {
    #[inline(always)]
    fn from(variant: Ep0datadoneStartepout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPOUT0` reader - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
pub type Ep0datadoneStartepout0R = crate::BitReader<Ep0datadoneStartepout0>;
impl Ep0datadoneStartepout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0datadoneStartepout0 {
        match self.bits {
            false => Ep0datadoneStartepout0::Disabled,
            true => Ep0datadoneStartepout0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ep0datadoneStartepout0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ep0datadoneStartepout0::Enabled
    }
}
#[doc = "Field `EP0DATADONE_STARTEPOUT0` writer - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
pub type Ep0datadoneStartepout0W<'a, REG> = crate::BitWriter<'a, REG, Ep0datadoneStartepout0>;
impl<'a, REG> Ep0datadoneStartepout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneStartepout0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneStartepout0::Enabled)
    }
}
#[doc = "Shortcut between event EP0DATADONE and task EP0STATUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0datadoneEp0status {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Ep0datadoneEp0status> for bool {
    #[inline(always)]
    fn from(variant: Ep0datadoneEp0status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_EP0STATUS` reader - Shortcut between event EP0DATADONE and task EP0STATUS"]
pub type Ep0datadoneEp0statusR = crate::BitReader<Ep0datadoneEp0status>;
impl Ep0datadoneEp0statusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0datadoneEp0status {
        match self.bits {
            false => Ep0datadoneEp0status::Disabled,
            true => Ep0datadoneEp0status::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ep0datadoneEp0status::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ep0datadoneEp0status::Enabled
    }
}
#[doc = "Field `EP0DATADONE_EP0STATUS` writer - Shortcut between event EP0DATADONE and task EP0STATUS"]
pub type Ep0datadoneEp0statusW<'a, REG> = crate::BitWriter<'a, REG, Ep0datadoneEp0status>;
impl<'a, REG> Ep0datadoneEp0statusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneEp0status::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneEp0status::Enabled)
    }
}
#[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout0Ep0status {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Endepout0Ep0status> for bool {
    #[inline(always)]
    fn from(variant: Endepout0Ep0status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0_EP0STATUS` reader - Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
pub type Endepout0Ep0statusR = crate::BitReader<Endepout0Ep0status>;
impl Endepout0Ep0statusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout0Ep0status {
        match self.bits {
            false => Endepout0Ep0status::Disabled,
            true => Endepout0Ep0status::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout0Ep0status::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout0Ep0status::Enabled
    }
}
#[doc = "Field `ENDEPOUT0_EP0STATUS` writer - Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
pub type Endepout0Ep0statusW<'a, REG> = crate::BitWriter<'a, REG, Endepout0Ep0status>;
impl<'a, REG> Endepout0Ep0statusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout0Ep0status::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout0Ep0status::Enabled)
    }
}
#[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout0Ep0rcvout {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Endepout0Ep0rcvout> for bool {
    #[inline(always)]
    fn from(variant: Endepout0Ep0rcvout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0_EP0RCVOUT` reader - Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
pub type Endepout0Ep0rcvoutR = crate::BitReader<Endepout0Ep0rcvout>;
impl Endepout0Ep0rcvoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout0Ep0rcvout {
        match self.bits {
            false => Endepout0Ep0rcvout::Disabled,
            true => Endepout0Ep0rcvout::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout0Ep0rcvout::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout0Ep0rcvout::Enabled
    }
}
#[doc = "Field `ENDEPOUT0_EP0RCVOUT` writer - Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
pub type Endepout0Ep0rcvoutW<'a, REG> = crate::BitWriter<'a, REG, Endepout0Ep0rcvout>;
impl<'a, REG> Endepout0Ep0rcvoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout0Ep0rcvout::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout0Ep0rcvout::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&self) -> Ep0datadoneStartepin0R {
        Ep0datadoneStartepin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&self) -> Ep0datadoneStartepout0R {
        Ep0datadoneStartepout0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&self) -> Ep0datadoneEp0statusR {
        Ep0datadoneEp0statusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
    #[inline(always)]
    pub fn endepout0_ep0status(&self) -> Endepout0Ep0statusR {
        Endepout0Ep0statusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&self) -> Endepout0Ep0rcvoutR {
        Endepout0Ep0rcvoutR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&mut self) -> Ep0datadoneStartepin0W<'_, ShortsSpec> {
        Ep0datadoneStartepin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&mut self) -> Ep0datadoneStartepout0W<'_, ShortsSpec> {
        Ep0datadoneStartepout0W::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&mut self) -> Ep0datadoneEp0statusW<'_, ShortsSpec> {
        Ep0datadoneEp0statusW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
    #[inline(always)]
    pub fn endepout0_ep0status(&mut self) -> Endepout0Ep0statusW<'_, ShortsSpec> {
        Endepout0Ep0statusW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&mut self) -> Endepout0Ep0rcvoutW<'_, ShortsSpec> {
        Endepout0Ep0rcvoutW::new(self, 4)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
