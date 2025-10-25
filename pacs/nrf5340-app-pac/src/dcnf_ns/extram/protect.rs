#[doc = "Register `PROTECT` reader"]
pub type R = crate::R<ProtectSpec>;
#[doc = "Register `PROTECT` writer"]
pub type W = crate::W<ProtectSpec>;
#[doc = "Control access to slave 0 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave0 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave0> for bool {
    #[inline(always)]
    fn from(variant: Slave0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE0` reader - Control access to slave 0 of master EXTRAM\\[n\\]"]
pub type Slave0R = crate::BitReader<Slave0>;
impl Slave0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave0 {
        match self.bits {
            false => Slave0::Allowed,
            true => Slave0::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave0::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave0::Blocked
    }
}
#[doc = "Field `SLAVE0` writer - Control access to slave 0 of master EXTRAM\\[n\\]"]
pub type Slave0W<'a, REG> = crate::BitWriter<'a, REG, Slave0>;
impl<'a, REG> Slave0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave0::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave0::Blocked)
    }
}
#[doc = "Control access to slave 1 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave1 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave1> for bool {
    #[inline(always)]
    fn from(variant: Slave1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE1` reader - Control access to slave 1 of master EXTRAM\\[n\\]"]
pub type Slave1R = crate::BitReader<Slave1>;
impl Slave1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave1 {
        match self.bits {
            false => Slave1::Allowed,
            true => Slave1::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave1::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave1::Blocked
    }
}
#[doc = "Field `SLAVE1` writer - Control access to slave 1 of master EXTRAM\\[n\\]"]
pub type Slave1W<'a, REG> = crate::BitWriter<'a, REG, Slave1>;
impl<'a, REG> Slave1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave1::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave1::Blocked)
    }
}
#[doc = "Control access to slave 2 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave2 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave2> for bool {
    #[inline(always)]
    fn from(variant: Slave2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE2` reader - Control access to slave 2 of master EXTRAM\\[n\\]"]
pub type Slave2R = crate::BitReader<Slave2>;
impl Slave2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave2 {
        match self.bits {
            false => Slave2::Allowed,
            true => Slave2::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave2::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave2::Blocked
    }
}
#[doc = "Field `SLAVE2` writer - Control access to slave 2 of master EXTRAM\\[n\\]"]
pub type Slave2W<'a, REG> = crate::BitWriter<'a, REG, Slave2>;
impl<'a, REG> Slave2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave2::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave2::Blocked)
    }
}
#[doc = "Control access to slave 3 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave3 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave3> for bool {
    #[inline(always)]
    fn from(variant: Slave3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE3` reader - Control access to slave 3 of master EXTRAM\\[n\\]"]
pub type Slave3R = crate::BitReader<Slave3>;
impl Slave3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave3 {
        match self.bits {
            false => Slave3::Allowed,
            true => Slave3::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave3::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave3::Blocked
    }
}
#[doc = "Field `SLAVE3` writer - Control access to slave 3 of master EXTRAM\\[n\\]"]
pub type Slave3W<'a, REG> = crate::BitWriter<'a, REG, Slave3>;
impl<'a, REG> Slave3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave3::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave3::Blocked)
    }
}
#[doc = "Control access to slave 4 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave4 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave4> for bool {
    #[inline(always)]
    fn from(variant: Slave4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE4` reader - Control access to slave 4 of master EXTRAM\\[n\\]"]
pub type Slave4R = crate::BitReader<Slave4>;
impl Slave4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave4 {
        match self.bits {
            false => Slave4::Allowed,
            true => Slave4::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave4::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave4::Blocked
    }
}
#[doc = "Field `SLAVE4` writer - Control access to slave 4 of master EXTRAM\\[n\\]"]
pub type Slave4W<'a, REG> = crate::BitWriter<'a, REG, Slave4>;
impl<'a, REG> Slave4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave4::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave4::Blocked)
    }
}
#[doc = "Control access to slave 5 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave5 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave5> for bool {
    #[inline(always)]
    fn from(variant: Slave5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE5` reader - Control access to slave 5 of master EXTRAM\\[n\\]"]
pub type Slave5R = crate::BitReader<Slave5>;
impl Slave5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave5 {
        match self.bits {
            false => Slave5::Allowed,
            true => Slave5::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave5::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave5::Blocked
    }
}
#[doc = "Field `SLAVE5` writer - Control access to slave 5 of master EXTRAM\\[n\\]"]
pub type Slave5W<'a, REG> = crate::BitWriter<'a, REG, Slave5>;
impl<'a, REG> Slave5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave5::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave5::Blocked)
    }
}
#[doc = "Control access to slave 6 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave6 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave6> for bool {
    #[inline(always)]
    fn from(variant: Slave6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE6` reader - Control access to slave 6 of master EXTRAM\\[n\\]"]
pub type Slave6R = crate::BitReader<Slave6>;
impl Slave6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave6 {
        match self.bits {
            false => Slave6::Allowed,
            true => Slave6::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave6::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave6::Blocked
    }
}
#[doc = "Field `SLAVE6` writer - Control access to slave 6 of master EXTRAM\\[n\\]"]
pub type Slave6W<'a, REG> = crate::BitWriter<'a, REG, Slave6>;
impl<'a, REG> Slave6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave6::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave6::Blocked)
    }
}
#[doc = "Control access to slave 7 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave7 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave7> for bool {
    #[inline(always)]
    fn from(variant: Slave7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE7` reader - Control access to slave 7 of master EXTRAM\\[n\\]"]
pub type Slave7R = crate::BitReader<Slave7>;
impl Slave7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave7 {
        match self.bits {
            false => Slave7::Allowed,
            true => Slave7::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave7::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave7::Blocked
    }
}
#[doc = "Field `SLAVE7` writer - Control access to slave 7 of master EXTRAM\\[n\\]"]
pub type Slave7W<'a, REG> = crate::BitWriter<'a, REG, Slave7>;
impl<'a, REG> Slave7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave7::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave7::Blocked)
    }
}
impl R {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> Slave0R {
        Slave0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&self) -> Slave1R {
        Slave1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&self) -> Slave2R {
        Slave2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&self) -> Slave3R {
        Slave3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&self) -> Slave4R {
        Slave4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&self) -> Slave5R {
        Slave5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&self) -> Slave6R {
        Slave6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&self) -> Slave7R {
        Slave7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> Slave0W<'_, ProtectSpec> {
        Slave0W::new(self, 0)
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&mut self) -> Slave1W<'_, ProtectSpec> {
        Slave1W::new(self, 1)
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&mut self) -> Slave2W<'_, ProtectSpec> {
        Slave2W::new(self, 2)
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&mut self) -> Slave3W<'_, ProtectSpec> {
        Slave3W::new(self, 3)
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&mut self) -> Slave4W<'_, ProtectSpec> {
        Slave4W::new(self, 4)
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&mut self) -> Slave5W<'_, ProtectSpec> {
        Slave5W::new(self, 5)
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&mut self) -> Slave6W<'_, ProtectSpec> {
        Slave6W::new(self, 6)
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&mut self) -> Slave7W<'_, ProtectSpec> {
        Slave7W::new(self, 7)
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectSpec;
impl crate::RegisterSpec for ProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protect::R`](R) reader structure"]
impl crate::Readable for ProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`protect::W`](W) writer structure"]
impl crate::Writable for ProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for ProtectSpec {}
