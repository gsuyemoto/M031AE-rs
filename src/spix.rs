#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spix_ctl: crate::Reg<spix_ctl::SPIX_CTL_SPEC>,
    #[doc = "0x04 - SPI Clock Divider Register"]
    pub spix_clkdiv: crate::Reg<spix_clkdiv::SPIX_CLKDIV_SPEC>,
    #[doc = "0x08 - SPI Slave Select Control Register"]
    pub spix_ssctl: crate::Reg<spix_ssctl::SPIX_SSCTL_SPEC>,
    #[doc = "0x0c - SPI PDMA Control Register"]
    pub spix_pdmactl: crate::Reg<spix_pdmactl::SPIX_PDMACTL_SPEC>,
    #[doc = "0x10 - SPI FIFO Control Register"]
    pub spix_fifoctl: crate::Reg<spix_fifoctl::SPIX_FIFOCTL_SPEC>,
    #[doc = "0x14 - SPI Status Register"]
    pub spix_status: crate::Reg<spix_status::SPIX_STATUS_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - SPI Data Transmit Register"]
    pub spix_tx: crate::Reg<spix_tx::SPIX_TX_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x30 - SPI Data Receive Register"]
    pub spix_rx: crate::Reg<spix_rx::SPIX_RX_SPEC>,
    _reserved8: [u8; 0x2c],
    #[doc = "0x60 - I2S Control Register"]
    pub spix_i2sctl: crate::Reg<spix_i2sctl::SPIX_I2SCTL_SPEC>,
    #[doc = "0x64 - I2S Clock Divider Control Register"]
    pub spix_i2sclk: crate::Reg<spix_i2sclk::SPIX_I2SCLK_SPEC>,
    #[doc = "0x68 - I2S Status Register"]
    pub spix_i2ssts: crate::Reg<spix_i2ssts::SPIX_I2SSTS_SPEC>,
}
#[doc = "SPIx_CTL register accessor: an alias for `Reg<SPIX_CTL_SPEC>`"]
pub type SPIX_CTL = crate::Reg<spix_ctl::SPIX_CTL_SPEC>;
#[doc = "SPI Control Register"]
pub mod spix_ctl;
#[doc = "SPIx_CLKDIV register accessor: an alias for `Reg<SPIX_CLKDIV_SPEC>`"]
pub type SPIX_CLKDIV = crate::Reg<spix_clkdiv::SPIX_CLKDIV_SPEC>;
#[doc = "SPI Clock Divider Register"]
pub mod spix_clkdiv;
#[doc = "SPIx_SSCTL register accessor: an alias for `Reg<SPIX_SSCTL_SPEC>`"]
pub type SPIX_SSCTL = crate::Reg<spix_ssctl::SPIX_SSCTL_SPEC>;
#[doc = "SPI Slave Select Control Register"]
pub mod spix_ssctl;
#[doc = "SPIx_PDMACTL register accessor: an alias for `Reg<SPIX_PDMACTL_SPEC>`"]
pub type SPIX_PDMACTL = crate::Reg<spix_pdmactl::SPIX_PDMACTL_SPEC>;
#[doc = "SPI PDMA Control Register"]
pub mod spix_pdmactl;
#[doc = "SPIx_FIFOCTL register accessor: an alias for `Reg<SPIX_FIFOCTL_SPEC>`"]
pub type SPIX_FIFOCTL = crate::Reg<spix_fifoctl::SPIX_FIFOCTL_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spix_fifoctl;
#[doc = "SPIx_STATUS register accessor: an alias for `Reg<SPIX_STATUS_SPEC>`"]
pub type SPIX_STATUS = crate::Reg<spix_status::SPIX_STATUS_SPEC>;
#[doc = "SPI Status Register"]
pub mod spix_status;
#[doc = "SPIx_TX register accessor: an alias for `Reg<SPIX_TX_SPEC>`"]
pub type SPIX_TX = crate::Reg<spix_tx::SPIX_TX_SPEC>;
#[doc = "SPI Data Transmit Register"]
pub mod spix_tx;
#[doc = "SPIx_RX register accessor: an alias for `Reg<SPIX_RX_SPEC>`"]
pub type SPIX_RX = crate::Reg<spix_rx::SPIX_RX_SPEC>;
#[doc = "SPI Data Receive Register"]
pub mod spix_rx;
#[doc = "SPIx_I2SCTL register accessor: an alias for `Reg<SPIX_I2SCTL_SPEC>`"]
pub type SPIX_I2SCTL = crate::Reg<spix_i2sctl::SPIX_I2SCTL_SPEC>;
#[doc = "I2S Control Register"]
pub mod spix_i2sctl;
#[doc = "SPIx_I2SCLK register accessor: an alias for `Reg<SPIX_I2SCLK_SPEC>`"]
pub type SPIX_I2SCLK = crate::Reg<spix_i2sclk::SPIX_I2SCLK_SPEC>;
#[doc = "I2S Clock Divider Control Register"]
pub mod spix_i2sclk;
#[doc = "SPIx_I2SSTS register accessor: an alias for `Reg<SPIX_I2SSTS_SPEC>`"]
pub type SPIX_I2SSTS = crate::Reg<spix_i2ssts::SPIX_I2SSTS_SPEC>;
#[doc = "I2S Status Register"]
pub mod spix_i2ssts;
