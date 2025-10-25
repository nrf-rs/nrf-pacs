#[doc = "Register `CTIINTACK` writer"]
pub type W = crate::W<CtiintackSpec>;
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugreq {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Debugreq> for bool {
    #[inline(always)]
    fn from(variant: Debugreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` writer - Processor debug request"]
pub type DebugreqW<'a, REG> = crate::BitWriter<'a, REG, Debugreq>;
impl<'a, REG> DebugreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Debugreq::Acknowledge)
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpurestart {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Cpurestart> for bool {
    #[inline(always)]
    fn from(variant: Cpurestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` writer - Processor Restart"]
pub type CpurestartW<'a, REG> = crate::BitWriter<'a, REG, Cpurestart>;
impl<'a, REG> CpurestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpurestart::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused0 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused0> for bool {
    #[inline(always)]
    fn from(variant: Unused0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` writer - N/A"]
pub type Unused0W<'a, REG> = crate::BitWriter<'a, REG, Unused0>;
impl<'a, REG> Unused0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused0::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused1 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused1> for bool {
    #[inline(always)]
    fn from(variant: Unused1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` writer - N/A"]
pub type Unused1W<'a, REG> = crate::BitWriter<'a, REG, Unused1>;
impl<'a, REG> Unused1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused1::Acknowledge)
    }
}
#[doc = "ETM Event Input 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin0 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Etmevtin0> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN0` writer - ETM Event Input 0"]
pub type Etmevtin0W<'a, REG> = crate::BitWriter<'a, REG, Etmevtin0>;
impl<'a, REG> Etmevtin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmevtin0::Acknowledge)
    }
}
#[doc = "ETM Event Input 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin1 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Etmevtin1> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN1` writer - ETM Event Input 1"]
pub type Etmevtin1W<'a, REG> = crate::BitWriter<'a, REG, Etmevtin1>;
impl<'a, REG> Etmevtin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmevtin1::Acknowledge)
    }
}
#[doc = "ETM Event Input 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin2 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Etmevtin2> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN2` writer - ETM Event Input 2"]
pub type Etmevtin2W<'a, REG> = crate::BitWriter<'a, REG, Etmevtin2>;
impl<'a, REG> Etmevtin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmevtin2::Acknowledge)
    }
}
#[doc = "ETM Event Input 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin3 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Etmevtin3> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN3` writer - ETM Event Input 3"]
pub type Etmevtin3W<'a, REG> = crate::BitWriter<'a, REG, Etmevtin3>;
impl<'a, REG> Etmevtin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmevtin3::Acknowledge)
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&mut self) -> DebugreqW<'_, CtiintackSpec> {
        DebugreqW::new(self, 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&mut self) -> CpurestartW<'_, CtiintackSpec> {
        CpurestartW::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&mut self) -> Unused0W<'_, CtiintackSpec> {
        Unused0W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&mut self) -> Unused1W<'_, CtiintackSpec> {
        Unused1W::new(self, 3)
    }
    #[doc = "Bit 4 - ETM Event Input 0"]
    #[inline(always)]
    pub fn etmevtin0(&mut self) -> Etmevtin0W<'_, CtiintackSpec> {
        Etmevtin0W::new(self, 4)
    }
    #[doc = "Bit 5 - ETM Event Input 1"]
    #[inline(always)]
    pub fn etmevtin1(&mut self) -> Etmevtin1W<'_, CtiintackSpec> {
        Etmevtin1W::new(self, 5)
    }
    #[doc = "Bit 6 - ETM Event Input 2"]
    #[inline(always)]
    pub fn etmevtin2(&mut self) -> Etmevtin2W<'_, CtiintackSpec> {
        Etmevtin2W::new(self, 6)
    }
    #[doc = "Bit 7 - ETM Event Input 3"]
    #[inline(always)]
    pub fn etmevtin3(&mut self) -> Etmevtin3W<'_, CtiintackSpec> {
        Etmevtin3W::new(self, 7)
    }
}
#[doc = "CTI Interrupt Acknowledge register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiintack::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiintackSpec;
impl crate::RegisterSpec for CtiintackSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctiintack::W`](W) writer structure"]
impl crate::Writable for CtiintackSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIINTACK to value 0"]
impl crate::Resettable for CtiintackSpec {}
