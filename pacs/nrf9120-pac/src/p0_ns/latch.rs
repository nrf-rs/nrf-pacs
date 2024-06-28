#[doc = "Register `LATCH` reader"]
pub type R = crate::R<LatchSpec>;
#[doc = "Register `LATCH` writer"]
pub type W = crate::W<LatchSpec>;
#[doc = "Status on whether PIN\\[0\\]
has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Status on whether PIN\\[0\\]
has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            false => Pin0::NotLatched,
            true => Pin0::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin0::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin0::Latched
    }
}
#[doc = "Field `PIN0` writer - Status on whether PIN\\[0\\]
has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Latched)
    }
}
#[doc = "Status on whether PIN\\[1\\]
has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Status on whether PIN\\[1\\]
has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            false => Pin1::NotLatched,
            true => Pin1::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin1::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin1::Latched
    }
}
#[doc = "Field `PIN1` writer - Status on whether PIN\\[1\\]
has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Latched)
    }
}
#[doc = "Status on whether PIN\\[2\\]
has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Status on whether PIN\\[2\\]
has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            false => Pin2::NotLatched,
            true => Pin2::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin2::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin2::Latched
    }
}
#[doc = "Field `PIN2` writer - Status on whether PIN\\[2\\]
has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Latched)
    }
}
#[doc = "Status on whether PIN\\[3\\]
has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Status on whether PIN\\[3\\]
has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            false => Pin3::NotLatched,
            true => Pin3::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin3::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin3::Latched
    }
}
#[doc = "Field `PIN3` writer - Status on whether PIN\\[3\\]
has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Latched)
    }
}
#[doc = "Status on whether PIN\\[4\\]
has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Status on whether PIN\\[4\\]
has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            false => Pin4::NotLatched,
            true => Pin4::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin4::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin4::Latched
    }
}
#[doc = "Field `PIN4` writer - Status on whether PIN\\[4\\]
has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Latched)
    }
}
#[doc = "Status on whether PIN\\[5\\]
has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Status on whether PIN\\[5\\]
has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            false => Pin5::NotLatched,
            true => Pin5::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin5::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin5::Latched
    }
}
#[doc = "Field `PIN5` writer - Status on whether PIN\\[5\\]
has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Latched)
    }
}
#[doc = "Status on whether PIN\\[6\\]
has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Status on whether PIN\\[6\\]
has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            false => Pin6::NotLatched,
            true => Pin6::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin6::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin6::Latched
    }
}
#[doc = "Field `PIN6` writer - Status on whether PIN\\[6\\]
has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Latched)
    }
}
#[doc = "Status on whether PIN\\[7\\]
has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Status on whether PIN\\[7\\]
has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            false => Pin7::NotLatched,
            true => Pin7::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin7::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin7::Latched
    }
}
#[doc = "Field `PIN7` writer - Status on whether PIN\\[7\\]
has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Latched)
    }
}
#[doc = "Status on whether PIN\\[8\\]
has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Status on whether PIN\\[8\\]
has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            false => Pin8::NotLatched,
            true => Pin8::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin8::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin8::Latched
    }
}
#[doc = "Field `PIN8` writer - Status on whether PIN\\[8\\]
has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Latched)
    }
}
#[doc = "Status on whether PIN\\[9\\]
has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Status on whether PIN\\[9\\]
has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            false => Pin9::NotLatched,
            true => Pin9::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin9::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin9::Latched
    }
}
#[doc = "Field `PIN9` writer - Status on whether PIN\\[9\\]
has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Latched)
    }
}
#[doc = "Status on whether PIN\\[10\\]
has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Status on whether PIN\\[10\\]
has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            false => Pin10::NotLatched,
            true => Pin10::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin10::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin10::Latched
    }
}
#[doc = "Field `PIN10` writer - Status on whether PIN\\[10\\]
has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Latched)
    }
}
#[doc = "Status on whether PIN\\[11\\]
has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Status on whether PIN\\[11\\]
has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            false => Pin11::NotLatched,
            true => Pin11::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin11::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin11::Latched
    }
}
#[doc = "Field `PIN11` writer - Status on whether PIN\\[11\\]
has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Latched)
    }
}
#[doc = "Status on whether PIN\\[12\\]
has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Status on whether PIN\\[12\\]
has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            false => Pin12::NotLatched,
            true => Pin12::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin12::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin12::Latched
    }
}
#[doc = "Field `PIN12` writer - Status on whether PIN\\[12\\]
has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Latched)
    }
}
#[doc = "Status on whether PIN\\[13\\]
has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Status on whether PIN\\[13\\]
has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            false => Pin13::NotLatched,
            true => Pin13::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin13::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin13::Latched
    }
}
#[doc = "Field `PIN13` writer - Status on whether PIN\\[13\\]
has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Latched)
    }
}
#[doc = "Status on whether PIN\\[14\\]
has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Status on whether PIN\\[14\\]
has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            false => Pin14::NotLatched,
            true => Pin14::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin14::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin14::Latched
    }
}
#[doc = "Field `PIN14` writer - Status on whether PIN\\[14\\]
has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Latched)
    }
}
#[doc = "Status on whether PIN\\[15\\]
has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Status on whether PIN\\[15\\]
has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            false => Pin15::NotLatched,
            true => Pin15::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin15::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin15::Latched
    }
}
#[doc = "Field `PIN15` writer - Status on whether PIN\\[15\\]
has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Latched)
    }
}
#[doc = "Status on whether PIN\\[16\\]
has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin16> for bool {
    #[inline(always)]
    fn from(variant: Pin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` reader - Status on whether PIN\\[16\\]
has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
pub type Pin16R = crate::BitReader<Pin16>;
impl Pin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin16 {
        match self.bits {
            false => Pin16::NotLatched,
            true => Pin16::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin16::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin16::Latched
    }
}
#[doc = "Field `PIN16` writer - Status on whether PIN\\[16\\]
has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG, Pin16>;
impl<'a, REG> Pin16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::Latched)
    }
}
#[doc = "Status on whether PIN\\[17\\]
has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin17> for bool {
    #[inline(always)]
    fn from(variant: Pin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` reader - Status on whether PIN\\[17\\]
has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
pub type Pin17R = crate::BitReader<Pin17>;
impl Pin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin17 {
        match self.bits {
            false => Pin17::NotLatched,
            true => Pin17::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin17::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin17::Latched
    }
}
#[doc = "Field `PIN17` writer - Status on whether PIN\\[17\\]
has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG, Pin17>;
impl<'a, REG> Pin17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::Latched)
    }
}
#[doc = "Status on whether PIN\\[18\\]
has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin18> for bool {
    #[inline(always)]
    fn from(variant: Pin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` reader - Status on whether PIN\\[18\\]
has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
pub type Pin18R = crate::BitReader<Pin18>;
impl Pin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin18 {
        match self.bits {
            false => Pin18::NotLatched,
            true => Pin18::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin18::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin18::Latched
    }
}
#[doc = "Field `PIN18` writer - Status on whether PIN\\[18\\]
has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG, Pin18>;
impl<'a, REG> Pin18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::Latched)
    }
}
#[doc = "Status on whether PIN\\[19\\]
has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin19> for bool {
    #[inline(always)]
    fn from(variant: Pin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` reader - Status on whether PIN\\[19\\]
has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
pub type Pin19R = crate::BitReader<Pin19>;
impl Pin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin19 {
        match self.bits {
            false => Pin19::NotLatched,
            true => Pin19::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin19::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin19::Latched
    }
}
#[doc = "Field `PIN19` writer - Status on whether PIN\\[19\\]
has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG, Pin19>;
impl<'a, REG> Pin19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::Latched)
    }
}
#[doc = "Status on whether PIN\\[20\\]
has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin20> for bool {
    #[inline(always)]
    fn from(variant: Pin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` reader - Status on whether PIN\\[20\\]
has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
pub type Pin20R = crate::BitReader<Pin20>;
impl Pin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin20 {
        match self.bits {
            false => Pin20::NotLatched,
            true => Pin20::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin20::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin20::Latched
    }
}
#[doc = "Field `PIN20` writer - Status on whether PIN\\[20\\]
has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG, Pin20>;
impl<'a, REG> Pin20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::Latched)
    }
}
#[doc = "Status on whether PIN\\[21\\]
has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin21> for bool {
    #[inline(always)]
    fn from(variant: Pin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` reader - Status on whether PIN\\[21\\]
has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
pub type Pin21R = crate::BitReader<Pin21>;
impl Pin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin21 {
        match self.bits {
            false => Pin21::NotLatched,
            true => Pin21::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin21::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin21::Latched
    }
}
#[doc = "Field `PIN21` writer - Status on whether PIN\\[21\\]
has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
pub type Pin21W<'a, REG> = crate::BitWriter<'a, REG, Pin21>;
impl<'a, REG> Pin21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::Latched)
    }
}
#[doc = "Status on whether PIN\\[22\\]
has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin22> for bool {
    #[inline(always)]
    fn from(variant: Pin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` reader - Status on whether PIN\\[22\\]
has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
pub type Pin22R = crate::BitReader<Pin22>;
impl Pin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin22 {
        match self.bits {
            false => Pin22::NotLatched,
            true => Pin22::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin22::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin22::Latched
    }
}
#[doc = "Field `PIN22` writer - Status on whether PIN\\[22\\]
has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
pub type Pin22W<'a, REG> = crate::BitWriter<'a, REG, Pin22>;
impl<'a, REG> Pin22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::Latched)
    }
}
#[doc = "Status on whether PIN\\[23\\]
has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin23> for bool {
    #[inline(always)]
    fn from(variant: Pin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` reader - Status on whether PIN\\[23\\]
has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
pub type Pin23R = crate::BitReader<Pin23>;
impl Pin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin23 {
        match self.bits {
            false => Pin23::NotLatched,
            true => Pin23::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin23::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin23::Latched
    }
}
#[doc = "Field `PIN23` writer - Status on whether PIN\\[23\\]
has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
pub type Pin23W<'a, REG> = crate::BitWriter<'a, REG, Pin23>;
impl<'a, REG> Pin23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::Latched)
    }
}
#[doc = "Status on whether PIN\\[24\\]
has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin24> for bool {
    #[inline(always)]
    fn from(variant: Pin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` reader - Status on whether PIN\\[24\\]
has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
pub type Pin24R = crate::BitReader<Pin24>;
impl Pin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin24 {
        match self.bits {
            false => Pin24::NotLatched,
            true => Pin24::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin24::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin24::Latched
    }
}
#[doc = "Field `PIN24` writer - Status on whether PIN\\[24\\]
has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
pub type Pin24W<'a, REG> = crate::BitWriter<'a, REG, Pin24>;
impl<'a, REG> Pin24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::Latched)
    }
}
#[doc = "Status on whether PIN\\[25\\]
has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin25> for bool {
    #[inline(always)]
    fn from(variant: Pin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` reader - Status on whether PIN\\[25\\]
has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
pub type Pin25R = crate::BitReader<Pin25>;
impl Pin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin25 {
        match self.bits {
            false => Pin25::NotLatched,
            true => Pin25::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin25::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin25::Latched
    }
}
#[doc = "Field `PIN25` writer - Status on whether PIN\\[25\\]
has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
pub type Pin25W<'a, REG> = crate::BitWriter<'a, REG, Pin25>;
impl<'a, REG> Pin25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::Latched)
    }
}
#[doc = "Status on whether PIN\\[26\\]
has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin26> for bool {
    #[inline(always)]
    fn from(variant: Pin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` reader - Status on whether PIN\\[26\\]
has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
pub type Pin26R = crate::BitReader<Pin26>;
impl Pin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin26 {
        match self.bits {
            false => Pin26::NotLatched,
            true => Pin26::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin26::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin26::Latched
    }
}
#[doc = "Field `PIN26` writer - Status on whether PIN\\[26\\]
has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
pub type Pin26W<'a, REG> = crate::BitWriter<'a, REG, Pin26>;
impl<'a, REG> Pin26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::Latched)
    }
}
#[doc = "Status on whether PIN\\[27\\]
has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin27> for bool {
    #[inline(always)]
    fn from(variant: Pin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` reader - Status on whether PIN\\[27\\]
has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
pub type Pin27R = crate::BitReader<Pin27>;
impl Pin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin27 {
        match self.bits {
            false => Pin27::NotLatched,
            true => Pin27::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin27::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin27::Latched
    }
}
#[doc = "Field `PIN27` writer - Status on whether PIN\\[27\\]
has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
pub type Pin27W<'a, REG> = crate::BitWriter<'a, REG, Pin27>;
impl<'a, REG> Pin27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::Latched)
    }
}
#[doc = "Status on whether PIN\\[28\\]
has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin28> for bool {
    #[inline(always)]
    fn from(variant: Pin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` reader - Status on whether PIN\\[28\\]
has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
pub type Pin28R = crate::BitReader<Pin28>;
impl Pin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin28 {
        match self.bits {
            false => Pin28::NotLatched,
            true => Pin28::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin28::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin28::Latched
    }
}
#[doc = "Field `PIN28` writer - Status on whether PIN\\[28\\]
has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
pub type Pin28W<'a, REG> = crate::BitWriter<'a, REG, Pin28>;
impl<'a, REG> Pin28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::Latched)
    }
}
#[doc = "Status on whether PIN\\[29\\]
has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin29> for bool {
    #[inline(always)]
    fn from(variant: Pin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` reader - Status on whether PIN\\[29\\]
has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
pub type Pin29R = crate::BitReader<Pin29>;
impl Pin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin29 {
        match self.bits {
            false => Pin29::NotLatched,
            true => Pin29::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin29::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin29::Latched
    }
}
#[doc = "Field `PIN29` writer - Status on whether PIN\\[29\\]
has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
pub type Pin29W<'a, REG> = crate::BitWriter<'a, REG, Pin29>;
impl<'a, REG> Pin29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::Latched)
    }
}
#[doc = "Status on whether PIN\\[30\\]
has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin30> for bool {
    #[inline(always)]
    fn from(variant: Pin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` reader - Status on whether PIN\\[30\\]
has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
pub type Pin30R = crate::BitReader<Pin30>;
impl Pin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin30 {
        match self.bits {
            false => Pin30::NotLatched,
            true => Pin30::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin30::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin30::Latched
    }
}
#[doc = "Field `PIN30` writer - Status on whether PIN\\[30\\]
has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
pub type Pin30W<'a, REG> = crate::BitWriter<'a, REG, Pin30>;
impl<'a, REG> Pin30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::Latched)
    }
}
#[doc = "Status on whether PIN\\[31\\]
has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31 {
    #[doc = "0: Criteria has not been met"]
    NotLatched = 0,
    #[doc = "1: Criteria has been met"]
    Latched = 1,
}
impl From<Pin31> for bool {
    #[inline(always)]
    fn from(variant: Pin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` reader - Status on whether PIN\\[31\\]
has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
pub type Pin31R = crate::BitReader<Pin31>;
impl Pin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin31 {
        match self.bits {
            false => Pin31::NotLatched,
            true => Pin31::Latched,
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == Pin31::NotLatched
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == Pin31::Latched
    }
}
#[doc = "Field `PIN31` writer - Status on whether PIN\\[31\\]
has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
pub type Pin31W<'a, REG> = crate::BitWriter<'a, REG, Pin31>;
impl<'a, REG> Pin31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::NotLatched)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::Latched)
    }
}
impl R {
    #[doc = "Bit 0 - Status on whether PIN\\[0\\]
has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status on whether PIN\\[1\\]
has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status on whether PIN\\[2\\]
has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status on whether PIN\\[3\\]
has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status on whether PIN\\[4\\]
has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status on whether PIN\\[5\\]
has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status on whether PIN\\[6\\]
has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status on whether PIN\\[7\\]
has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status on whether PIN\\[8\\]
has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status on whether PIN\\[9\\]
has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status on whether PIN\\[10\\]
has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status on whether PIN\\[11\\]
has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Status on whether PIN\\[12\\]
has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status on whether PIN\\[13\\]
has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Status on whether PIN\\[14\\]
has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status on whether PIN\\[15\\]
has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Status on whether PIN\\[16\\]
has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status on whether PIN\\[17\\]
has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Status on whether PIN\\[18\\]
has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status on whether PIN\\[19\\]
has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Status on whether PIN\\[20\\]
has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Status on whether PIN\\[21\\]
has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin21(&self) -> Pin21R {
        Pin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Status on whether PIN\\[22\\]
has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin22(&self) -> Pin22R {
        Pin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Status on whether PIN\\[23\\]
has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin23(&self) -> Pin23R {
        Pin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Status on whether PIN\\[24\\]
has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin24(&self) -> Pin24R {
        Pin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Status on whether PIN\\[25\\]
has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin25(&self) -> Pin25R {
        Pin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Status on whether PIN\\[26\\]
has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin26(&self) -> Pin26R {
        Pin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Status on whether PIN\\[27\\]
has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin27(&self) -> Pin27R {
        Pin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Status on whether PIN\\[28\\]
has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin28(&self) -> Pin28R {
        Pin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Status on whether PIN\\[29\\]
has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin29(&self) -> Pin29R {
        Pin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Status on whether PIN\\[30\\]
has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin30(&self) -> Pin30R {
        Pin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Status on whether PIN\\[31\\]
has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin31(&self) -> Pin31R {
        Pin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status on whether PIN\\[0\\]
has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<LatchSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Status on whether PIN\\[1\\]
has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<LatchSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Status on whether PIN\\[2\\]
has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<LatchSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Status on whether PIN\\[3\\]
has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<LatchSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Status on whether PIN\\[4\\]
has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<LatchSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Status on whether PIN\\[5\\]
has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<LatchSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Status on whether PIN\\[6\\]
has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<LatchSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Status on whether PIN\\[7\\]
has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<LatchSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Status on whether PIN\\[8\\]
has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<LatchSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Status on whether PIN\\[9\\]
has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<LatchSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Status on whether PIN\\[10\\]
has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<LatchSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Status on whether PIN\\[11\\]
has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<LatchSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Status on whether PIN\\[12\\]
has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<LatchSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Status on whether PIN\\[13\\]
has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<LatchSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Status on whether PIN\\[14\\]
has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<LatchSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Status on whether PIN\\[15\\]
has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<LatchSpec> {
        Pin15W::new(self, 15)
    }
    #[doc = "Bit 16 - Status on whether PIN\\[16\\]
has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin16(&mut self) -> Pin16W<LatchSpec> {
        Pin16W::new(self, 16)
    }
    #[doc = "Bit 17 - Status on whether PIN\\[17\\]
has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin17(&mut self) -> Pin17W<LatchSpec> {
        Pin17W::new(self, 17)
    }
    #[doc = "Bit 18 - Status on whether PIN\\[18\\]
has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin18(&mut self) -> Pin18W<LatchSpec> {
        Pin18W::new(self, 18)
    }
    #[doc = "Bit 19 - Status on whether PIN\\[19\\]
has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin19(&mut self) -> Pin19W<LatchSpec> {
        Pin19W::new(self, 19)
    }
    #[doc = "Bit 20 - Status on whether PIN\\[20\\]
has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin20(&mut self) -> Pin20W<LatchSpec> {
        Pin20W::new(self, 20)
    }
    #[doc = "Bit 21 - Status on whether PIN\\[21\\]
has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin21(&mut self) -> Pin21W<LatchSpec> {
        Pin21W::new(self, 21)
    }
    #[doc = "Bit 22 - Status on whether PIN\\[22\\]
has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin22(&mut self) -> Pin22W<LatchSpec> {
        Pin22W::new(self, 22)
    }
    #[doc = "Bit 23 - Status on whether PIN\\[23\\]
has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin23(&mut self) -> Pin23W<LatchSpec> {
        Pin23W::new(self, 23)
    }
    #[doc = "Bit 24 - Status on whether PIN\\[24\\]
has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin24(&mut self) -> Pin24W<LatchSpec> {
        Pin24W::new(self, 24)
    }
    #[doc = "Bit 25 - Status on whether PIN\\[25\\]
has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin25(&mut self) -> Pin25W<LatchSpec> {
        Pin25W::new(self, 25)
    }
    #[doc = "Bit 26 - Status on whether PIN\\[26\\]
has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin26(&mut self) -> Pin26W<LatchSpec> {
        Pin26W::new(self, 26)
    }
    #[doc = "Bit 27 - Status on whether PIN\\[27\\]
has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin27(&mut self) -> Pin27W<LatchSpec> {
        Pin27W::new(self, 27)
    }
    #[doc = "Bit 28 - Status on whether PIN\\[28\\]
has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin28(&mut self) -> Pin28W<LatchSpec> {
        Pin28W::new(self, 28)
    }
    #[doc = "Bit 29 - Status on whether PIN\\[29\\]
has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin29(&mut self) -> Pin29W<LatchSpec> {
        Pin29W::new(self, 29)
    }
    #[doc = "Bit 30 - Status on whether PIN\\[30\\]
has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin30(&mut self) -> Pin30W<LatchSpec> {
        Pin30W::new(self, 30)
    }
    #[doc = "Bit 31 - Status on whether PIN\\[31\\]
has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    #[must_use]
    pub fn pin31(&mut self) -> Pin31W<LatchSpec> {
        Pin31W::new(self, 31)
    }
}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`latch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`latch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LatchSpec;
impl crate::RegisterSpec for LatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`latch::R`](R) reader structure"]
impl crate::Readable for LatchSpec {}
#[doc = "`write(|w| ..)` method takes [`latch::W`](W) writer structure"]
impl crate::Writable for LatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LATCH to value 0"]
impl crate::Resettable for LatchSpec {
    const RESET_VALUE: u32 = 0;
}
