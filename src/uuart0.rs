#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI Control Register"]
    pub uuart_ctl: crate::Reg<uuart_ctl::UUART_CTL_SPEC>,
    #[doc = "0x04 - USCI Interrupt Enable Register"]
    pub uuart_inten: crate::Reg<uuart_inten::UUART_INTEN_SPEC>,
    #[doc = "0x08 - USCI Baud Rate Generator Register"]
    pub uuart_brgen: crate::Reg<uuart_brgen::UUART_BRGEN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - USCI Input Data Signal Configuration Register 0"]
    pub uuart_datin0: crate::Reg<uuart_datin0::UUART_DATIN0_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - USCI Input Control Signal Configuration Register 0"]
    pub uuart_ctlin0: crate::Reg<uuart_ctlin0::UUART_CTLIN0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - USCI Input Clock Signal Configuration Register"]
    pub uuart_clkin: crate::Reg<uuart_clkin::UUART_CLKIN_SPEC>,
    #[doc = "0x2c - USCI Line Control Register"]
    pub uuart_linectl: crate::Reg<uuart_linectl::UUART_LINECTL_SPEC>,
    #[doc = "0x30 - USCI Transmit Data Register"]
    pub uuart_txdat: crate::Reg<uuart_txdat::UUART_TXDAT_SPEC>,
    #[doc = "0x34 - USCI Receive Data Register"]
    pub uuart_rxdat: crate::Reg<uuart_rxdat::UUART_RXDAT_SPEC>,
    #[doc = "0x38 - USCI Transmit/Receive Buffer Control Register"]
    pub uuart_bufctl: crate::Reg<uuart_bufctl::UUART_BUFCTL_SPEC>,
    #[doc = "0x3c - USCI Transmit/Receive Buffer Status Register"]
    pub uuart_bufsts: crate::Reg<uuart_bufsts::UUART_BUFSTS_SPEC>,
    #[doc = "0x40 - USCI PDMA Control Register"]
    pub uuart_pdmactl: crate::Reg<uuart_pdmactl::UUART_PDMACTL_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x54 - USCI Wake-up Control Register"]
    pub uuart_wkctl: crate::Reg<uuart_wkctl::UUART_WKCTL_SPEC>,
    #[doc = "0x58 - USCI Wake-up Status Register"]
    pub uuart_wksts: crate::Reg<uuart_wksts::UUART_WKSTS_SPEC>,
    #[doc = "0x5c - USCI Protocol Control Register"]
    pub uuart_protctl: crate::Reg<uuart_protctl::UUART_PROTCTL_SPEC>,
    #[doc = "0x60 - USCI Protocol Interrupt Enable Register"]
    pub uuart_protien: crate::Reg<uuart_protien::UUART_PROTIEN_SPEC>,
    #[doc = "0x64 - USCI Protocol Status Register"]
    pub uuart_protsts: crate::Reg<uuart_protsts::UUART_PROTSTS_SPEC>,
}
#[doc = "UUART_CTL register accessor: an alias for `Reg<UUART_CTL_SPEC>`"]
pub type UUART_CTL = crate::Reg<uuart_ctl::UUART_CTL_SPEC>;
#[doc = "USCI Control Register"]
pub mod uuart_ctl;
#[doc = "UUART_INTEN register accessor: an alias for `Reg<UUART_INTEN_SPEC>`"]
pub type UUART_INTEN = crate::Reg<uuart_inten::UUART_INTEN_SPEC>;
#[doc = "USCI Interrupt Enable Register"]
pub mod uuart_inten;
#[doc = "UUART_BRGEN register accessor: an alias for `Reg<UUART_BRGEN_SPEC>`"]
pub type UUART_BRGEN = crate::Reg<uuart_brgen::UUART_BRGEN_SPEC>;
#[doc = "USCI Baud Rate Generator Register"]
pub mod uuart_brgen;
#[doc = "UUART_DATIN0 register accessor: an alias for `Reg<UUART_DATIN0_SPEC>`"]
pub type UUART_DATIN0 = crate::Reg<uuart_datin0::UUART_DATIN0_SPEC>;
#[doc = "USCI Input Data Signal Configuration Register 0"]
pub mod uuart_datin0;
#[doc = "UUART_CTLIN0 register accessor: an alias for `Reg<UUART_CTLIN0_SPEC>`"]
pub type UUART_CTLIN0 = crate::Reg<uuart_ctlin0::UUART_CTLIN0_SPEC>;
#[doc = "USCI Input Control Signal Configuration Register 0"]
pub mod uuart_ctlin0;
#[doc = "UUART_CLKIN register accessor: an alias for `Reg<UUART_CLKIN_SPEC>`"]
pub type UUART_CLKIN = crate::Reg<uuart_clkin::UUART_CLKIN_SPEC>;
#[doc = "USCI Input Clock Signal Configuration Register"]
pub mod uuart_clkin;
#[doc = "UUART_LINECTL register accessor: an alias for `Reg<UUART_LINECTL_SPEC>`"]
pub type UUART_LINECTL = crate::Reg<uuart_linectl::UUART_LINECTL_SPEC>;
#[doc = "USCI Line Control Register"]
pub mod uuart_linectl;
#[doc = "UUART_TXDAT register accessor: an alias for `Reg<UUART_TXDAT_SPEC>`"]
pub type UUART_TXDAT = crate::Reg<uuart_txdat::UUART_TXDAT_SPEC>;
#[doc = "USCI Transmit Data Register"]
pub mod uuart_txdat;
#[doc = "UUART_RXDAT register accessor: an alias for `Reg<UUART_RXDAT_SPEC>`"]
pub type UUART_RXDAT = crate::Reg<uuart_rxdat::UUART_RXDAT_SPEC>;
#[doc = "USCI Receive Data Register"]
pub mod uuart_rxdat;
#[doc = "UUART_BUFCTL register accessor: an alias for `Reg<UUART_BUFCTL_SPEC>`"]
pub type UUART_BUFCTL = crate::Reg<uuart_bufctl::UUART_BUFCTL_SPEC>;
#[doc = "USCI Transmit/Receive Buffer Control Register"]
pub mod uuart_bufctl;
#[doc = "UUART_BUFSTS register accessor: an alias for `Reg<UUART_BUFSTS_SPEC>`"]
pub type UUART_BUFSTS = crate::Reg<uuart_bufsts::UUART_BUFSTS_SPEC>;
#[doc = "USCI Transmit/Receive Buffer Status Register"]
pub mod uuart_bufsts;
#[doc = "UUART_PDMACTL register accessor: an alias for `Reg<UUART_PDMACTL_SPEC>`"]
pub type UUART_PDMACTL = crate::Reg<uuart_pdmactl::UUART_PDMACTL_SPEC>;
#[doc = "USCI PDMA Control Register"]
pub mod uuart_pdmactl;
#[doc = "UUART_WKCTL register accessor: an alias for `Reg<UUART_WKCTL_SPEC>`"]
pub type UUART_WKCTL = crate::Reg<uuart_wkctl::UUART_WKCTL_SPEC>;
#[doc = "USCI Wake-up Control Register"]
pub mod uuart_wkctl;
#[doc = "UUART_WKSTS register accessor: an alias for `Reg<UUART_WKSTS_SPEC>`"]
pub type UUART_WKSTS = crate::Reg<uuart_wksts::UUART_WKSTS_SPEC>;
#[doc = "USCI Wake-up Status Register"]
pub mod uuart_wksts;
#[doc = "UUART_PROTCTL register accessor: an alias for `Reg<UUART_PROTCTL_SPEC>`"]
pub type UUART_PROTCTL = crate::Reg<uuart_protctl::UUART_PROTCTL_SPEC>;
#[doc = "USCI Protocol Control Register"]
pub mod uuart_protctl;
#[doc = "UUART_PROTIEN register accessor: an alias for `Reg<UUART_PROTIEN_SPEC>`"]
pub type UUART_PROTIEN = crate::Reg<uuart_protien::UUART_PROTIEN_SPEC>;
#[doc = "USCI Protocol Interrupt Enable Register"]
pub mod uuart_protien;
#[doc = "UUART_PROTSTS register accessor: an alias for `Reg<UUART_PROTSTS_SPEC>`"]
pub type UUART_PROTSTS = crate::Reg<uuart_protsts::UUART_PROTSTS_SPEC>;
#[doc = "USCI Protocol Status Register"]
pub mod uuart_protsts;
