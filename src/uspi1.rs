#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI Control Register"]
    pub uspi_ctl: crate::Reg<uspi_ctl::USPI_CTL_SPEC>,
    #[doc = "0x04 - USCI Interrupt Enable Register"]
    pub uspi_inten: crate::Reg<uspi_inten::USPI_INTEN_SPEC>,
    #[doc = "0x08 - USCI Baud Rate Generator Register"]
    pub uspi_brgen: crate::Reg<uspi_brgen::USPI_BRGEN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - USCI Input Data Signal Configuration Register 0"]
    pub uspi_datin0: crate::Reg<uspi_datin0::USPI_DATIN0_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - USCI Input Control Signal Configuration Register 0"]
    pub uspi_ctlin0: crate::Reg<uspi_ctlin0::USPI_CTLIN0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - USCI Input Clock Signal Configuration Register"]
    pub uspi_clkin: crate::Reg<uspi_clkin::USPI_CLKIN_SPEC>,
    #[doc = "0x2c - USCI Line Control Register"]
    pub uspi_linectl: crate::Reg<uspi_linectl::USPI_LINECTL_SPEC>,
    #[doc = "0x30 - USCI Transmit Data Register"]
    pub uspi_txdat: crate::Reg<uspi_txdat::USPI_TXDAT_SPEC>,
    #[doc = "0x34 - USCI Receive Data Register"]
    pub uspi_rxdat: crate::Reg<uspi_rxdat::USPI_RXDAT_SPEC>,
    #[doc = "0x38 - USCI Transmit/Receive Buffer Control Register"]
    pub uspi_bufctl: crate::Reg<uspi_bufctl::USPI_BUFCTL_SPEC>,
    #[doc = "0x3c - USCI Transmit/Receive Buffer Status Register"]
    pub uspi_bufsts: crate::Reg<uspi_bufsts::USPI_BUFSTS_SPEC>,
    #[doc = "0x40 - USCI PDMA Control Register"]
    pub uspi_pdmactl: crate::Reg<uspi_pdmactl::USPI_PDMACTL_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x54 - USCI Wake-up Control Register"]
    pub uspi_wkctl: crate::Reg<uspi_wkctl::USPI_WKCTL_SPEC>,
    #[doc = "0x58 - USCI Wake-up Status Register"]
    pub uspi_wksts: crate::Reg<uspi_wksts::USPI_WKSTS_SPEC>,
    #[doc = "0x5c - USCI Protocol Control Register"]
    pub uspi_protctl: crate::Reg<uspi_protctl::USPI_PROTCTL_SPEC>,
    #[doc = "0x60 - USCI Protocol Interrupt Enable Register"]
    pub uspi_protien: crate::Reg<uspi_protien::USPI_PROTIEN_SPEC>,
    #[doc = "0x64 - USCI Protocol Status Register"]
    pub uspi_protsts: crate::Reg<uspi_protsts::USPI_PROTSTS_SPEC>,
}
#[doc = "USPI_CTL register accessor: an alias for `Reg<USPI_CTL_SPEC>`"]
pub type USPI_CTL = crate::Reg<uspi_ctl::USPI_CTL_SPEC>;
#[doc = "USCI Control Register"]
pub mod uspi_ctl;
#[doc = "USPI_INTEN register accessor: an alias for `Reg<USPI_INTEN_SPEC>`"]
pub type USPI_INTEN = crate::Reg<uspi_inten::USPI_INTEN_SPEC>;
#[doc = "USCI Interrupt Enable Register"]
pub mod uspi_inten;
#[doc = "USPI_BRGEN register accessor: an alias for `Reg<USPI_BRGEN_SPEC>`"]
pub type USPI_BRGEN = crate::Reg<uspi_brgen::USPI_BRGEN_SPEC>;
#[doc = "USCI Baud Rate Generator Register"]
pub mod uspi_brgen;
#[doc = "USPI_DATIN0 register accessor: an alias for `Reg<USPI_DATIN0_SPEC>`"]
pub type USPI_DATIN0 = crate::Reg<uspi_datin0::USPI_DATIN0_SPEC>;
#[doc = "USCI Input Data Signal Configuration Register 0"]
pub mod uspi_datin0;
#[doc = "USPI_CTLIN0 register accessor: an alias for `Reg<USPI_CTLIN0_SPEC>`"]
pub type USPI_CTLIN0 = crate::Reg<uspi_ctlin0::USPI_CTLIN0_SPEC>;
#[doc = "USCI Input Control Signal Configuration Register 0"]
pub mod uspi_ctlin0;
#[doc = "USPI_CLKIN register accessor: an alias for `Reg<USPI_CLKIN_SPEC>`"]
pub type USPI_CLKIN = crate::Reg<uspi_clkin::USPI_CLKIN_SPEC>;
#[doc = "USCI Input Clock Signal Configuration Register"]
pub mod uspi_clkin;
#[doc = "USPI_LINECTL register accessor: an alias for `Reg<USPI_LINECTL_SPEC>`"]
pub type USPI_LINECTL = crate::Reg<uspi_linectl::USPI_LINECTL_SPEC>;
#[doc = "USCI Line Control Register"]
pub mod uspi_linectl;
#[doc = "USPI_TXDAT register accessor: an alias for `Reg<USPI_TXDAT_SPEC>`"]
pub type USPI_TXDAT = crate::Reg<uspi_txdat::USPI_TXDAT_SPEC>;
#[doc = "USCI Transmit Data Register"]
pub mod uspi_txdat;
#[doc = "USPI_RXDAT register accessor: an alias for `Reg<USPI_RXDAT_SPEC>`"]
pub type USPI_RXDAT = crate::Reg<uspi_rxdat::USPI_RXDAT_SPEC>;
#[doc = "USCI Receive Data Register"]
pub mod uspi_rxdat;
#[doc = "USPI_BUFCTL register accessor: an alias for `Reg<USPI_BUFCTL_SPEC>`"]
pub type USPI_BUFCTL = crate::Reg<uspi_bufctl::USPI_BUFCTL_SPEC>;
#[doc = "USCI Transmit/Receive Buffer Control Register"]
pub mod uspi_bufctl;
#[doc = "USPI_BUFSTS register accessor: an alias for `Reg<USPI_BUFSTS_SPEC>`"]
pub type USPI_BUFSTS = crate::Reg<uspi_bufsts::USPI_BUFSTS_SPEC>;
#[doc = "USCI Transmit/Receive Buffer Status Register"]
pub mod uspi_bufsts;
#[doc = "USPI_PDMACTL register accessor: an alias for `Reg<USPI_PDMACTL_SPEC>`"]
pub type USPI_PDMACTL = crate::Reg<uspi_pdmactl::USPI_PDMACTL_SPEC>;
#[doc = "USCI PDMA Control Register"]
pub mod uspi_pdmactl;
#[doc = "USPI_WKCTL register accessor: an alias for `Reg<USPI_WKCTL_SPEC>`"]
pub type USPI_WKCTL = crate::Reg<uspi_wkctl::USPI_WKCTL_SPEC>;
#[doc = "USCI Wake-up Control Register"]
pub mod uspi_wkctl;
#[doc = "USPI_WKSTS register accessor: an alias for `Reg<USPI_WKSTS_SPEC>`"]
pub type USPI_WKSTS = crate::Reg<uspi_wksts::USPI_WKSTS_SPEC>;
#[doc = "USCI Wake-up Status Register"]
pub mod uspi_wksts;
#[doc = "USPI_PROTCTL register accessor: an alias for `Reg<USPI_PROTCTL_SPEC>`"]
pub type USPI_PROTCTL = crate::Reg<uspi_protctl::USPI_PROTCTL_SPEC>;
#[doc = "USCI Protocol Control Register"]
pub mod uspi_protctl;
#[doc = "USPI_PROTIEN register accessor: an alias for `Reg<USPI_PROTIEN_SPEC>`"]
pub type USPI_PROTIEN = crate::Reg<uspi_protien::USPI_PROTIEN_SPEC>;
#[doc = "USCI Protocol Interrupt Enable Register"]
pub mod uspi_protien;
#[doc = "USPI_PROTSTS register accessor: an alias for `Reg<USPI_PROTSTS_SPEC>`"]
pub type USPI_PROTSTS = crate::Reg<uspi_protsts::USPI_PROTSTS_SPEC>;
#[doc = "USCI Protocol Status Register"]
pub mod uspi_protsts;
