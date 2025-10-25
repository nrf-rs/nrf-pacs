#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0900],
    crypto_ctl: CryptoCtl,
    _reserved1: [u8; 0x0c],
    crypto_busy: CryptoBusy,
    _reserved2: [u8; 0x08],
    hash_busy: HashBusy,
    _reserved3: [u8; 0x10],
    context_id: ContextId,
}
impl RegisterBlock {
    #[doc = "0x900 - Defines the cryptographic flow."]
    #[inline(always)]
    pub const fn crypto_ctl(&self) -> &CryptoCtl {
        &self.crypto_ctl
    }
    #[doc = "0x910 - Status register for cryptographic cores engine activity."]
    #[inline(always)]
    pub const fn crypto_busy(&self) -> &CryptoBusy {
        &self.crypto_busy
    }
    #[doc = "0x91c - Status register for HASH engine activity."]
    #[inline(always)]
    pub const fn hash_busy(&self) -> &HashBusy {
        &self.hash_busy
    }
    #[doc = "0x930 - A general-purpose read/write register."]
    #[inline(always)]
    pub const fn context_id(&self) -> &ContextId {
        &self.context_id
    }
}
#[doc = "CRYPTO_CTL (w) register accessor: Defines the cryptographic flow.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_ctl`] module"]
#[doc(alias = "CRYPTO_CTL")]
pub type CryptoCtl = crate::Reg<crypto_ctl::CryptoCtlSpec>;
#[doc = "Defines the cryptographic flow."]
pub mod crypto_ctl;
#[doc = "CRYPTO_BUSY (r) register accessor: Status register for cryptographic cores engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_busy`] module"]
#[doc(alias = "CRYPTO_BUSY")]
pub type CryptoBusy = crate::Reg<crypto_busy::CryptoBusySpec>;
#[doc = "Status register for cryptographic cores engine activity."]
pub mod crypto_busy;
#[doc = "HASH_BUSY (r) register accessor: Status register for HASH engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_busy`] module"]
#[doc(alias = "HASH_BUSY")]
pub type HashBusy = crate::Reg<hash_busy::HashBusySpec>;
#[doc = "Status register for HASH engine activity."]
pub mod hash_busy;
#[doc = "CONTEXT_ID (rw) register accessor: A general-purpose read/write register.\n\nYou can [`read`](crate::Reg::read) this register and get [`context_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`context_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@context_id`] module"]
#[doc(alias = "CONTEXT_ID")]
pub type ContextId = crate::Reg<context_id::ContextIdSpec>;
#[doc = "A general-purpose read/write register."]
pub mod context_id;
