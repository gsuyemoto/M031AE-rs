#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QSPI Control Register"]
    pub qspix_ctl: crate::Reg<qspix_ctl::QSPIX_CTL_SPEC>,
    #[doc = "0x04 - QSPI Clock Divider Register"]
    pub qspix_clkdiv: crate::Reg<qspix_clkdiv::QSPIX_CLKDIV_SPEC>,
    #[doc = "0x08 - QSPI Slave Select Control Register"]
    pub qspix_ssctl: crate::Reg<qspix_ssctl::QSPIX_SSCTL_SPEC>,
    #[doc = "0x0c - QSPI PDMA Control Register"]
    pub qspix_pdmactl: crate::Reg<qspix_pdmactl::QSPIX_PDMACTL_SPEC>,
    #[doc = "0x10 - QSPI FIFO Control Register"]
    pub qspix_fifoctl: crate::Reg<qspix_fifoctl::QSPIX_FIFOCTL_SPEC>,
    #[doc = "0x14 - QSPI Status Register"]
    pub qspix_status: crate::Reg<qspix_status::QSPIX_STATUS_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - QSPI Data Transmit Register"]
    pub qspix_tx: crate::Reg<qspix_tx::QSPIX_TX_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x30 - QSPI Data Receive Register"]
    pub qspix_rx: crate::Reg<qspix_rx::QSPIX_RX_SPEC>,
}
#[doc = "QSPIx_CTL register accessor: an alias for `Reg<QSPIX_CTL_SPEC>`"]
pub type QSPIX_CTL = crate::Reg<qspix_ctl::QSPIX_CTL_SPEC>;
#[doc = "QSPI Control Register"]
pub mod qspix_ctl;
#[doc = "QSPIx_CLKDIV register accessor: an alias for `Reg<QSPIX_CLKDIV_SPEC>`"]
pub type QSPIX_CLKDIV = crate::Reg<qspix_clkdiv::QSPIX_CLKDIV_SPEC>;
#[doc = "QSPI Clock Divider Register"]
pub mod qspix_clkdiv;
#[doc = "QSPIx_SSCTL register accessor: an alias for `Reg<QSPIX_SSCTL_SPEC>`"]
pub type QSPIX_SSCTL = crate::Reg<qspix_ssctl::QSPIX_SSCTL_SPEC>;
#[doc = "QSPI Slave Select Control Register"]
pub mod qspix_ssctl;
#[doc = "QSPIx_PDMACTL register accessor: an alias for `Reg<QSPIX_PDMACTL_SPEC>`"]
pub type QSPIX_PDMACTL = crate::Reg<qspix_pdmactl::QSPIX_PDMACTL_SPEC>;
#[doc = "QSPI PDMA Control Register"]
pub mod qspix_pdmactl;
#[doc = "QSPIx_FIFOCTL register accessor: an alias for `Reg<QSPIX_FIFOCTL_SPEC>`"]
pub type QSPIX_FIFOCTL = crate::Reg<qspix_fifoctl::QSPIX_FIFOCTL_SPEC>;
#[doc = "QSPI FIFO Control Register"]
pub mod qspix_fifoctl;
#[doc = "QSPIx_STATUS register accessor: an alias for `Reg<QSPIX_STATUS_SPEC>`"]
pub type QSPIX_STATUS = crate::Reg<qspix_status::QSPIX_STATUS_SPEC>;
#[doc = "QSPI Status Register"]
pub mod qspix_status;
#[doc = "QSPIx_TX register accessor: an alias for `Reg<QSPIX_TX_SPEC>`"]
pub type QSPIX_TX = crate::Reg<qspix_tx::QSPIX_TX_SPEC>;
#[doc = "QSPI Data Transmit Register"]
pub mod qspix_tx;
#[doc = "QSPIx_RX register accessor: an alias for `Reg<QSPIX_RX_SPEC>`"]
pub type QSPIX_RX = crate::Reg<qspix_rx::QSPIX_RX_SPEC>;
#[doc = "QSPI Data Receive Register"]
pub mod qspix_rx;
