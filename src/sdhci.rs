use log::info;
use rk3568_clk::cru::CRU;
use rk3568_clk::cru::cru_clksel_con28_bits::{*};

use crate::delay_us;
use crate::sdhci_reg::Reg;
use crate::sdhci_reg::emmc_cmd_bits::{*};
use crate::sdhci_reg::emmc_normal_int_en_bits::{*};
use crate::sdhci_reg::emmc_dll_ctrl_bits::{*};
use crate::sdhci_cmd::Cmd;

pub struct SDHCI {
    reg: Reg,
    clk: CRU,
}

impl SDHCI {
    pub fn new (base_addr: u64, clk_addr: u64) -> Self {
        Self { reg: Reg::new(base_addr as u64), clk: CRU::new(clk_addr as u64) }
    }

    pub fn init(&self) {
        self.reg.emmc_reset_all();

        self.clk.cru_clksel_set_cclk_emmc(CRU_CLKSEL_CCLK_EMMC_SOC0_375K);
        info!("clock.cru_clksel_get_cclk_emmc(): {:#x}", self.clk.cru_clksel_get_cclk_emmc());

        self.reg.emmc_pwr_on();

        self.reg.emmc_set_normal_int_en(EMMC_CMD_COMPLETE_EN
                                        | EMMC_XFER_COMPLETE_EN
                                        | EMMC_DMA_INTERRUPT_EN
                                        | EMMC_BUF_WR_READY_EN
                                        | EMMC_BUF_RD_READY_EN);
        // self.reg.emmc_set_error_int_en(0x027f);

        // self.reg.emmc_clear_all_error_int_flags();
        // self.reg.emmc_clear_all_normal_int_flags();
        self.reg.emmc_disable_all_normal_int_sig();
        self.reg.emmc_disable_all_error_int_sig();

        self.reg.emmc_enable_data_xfer_width_1bit();

        self.reg.emmc_set_dll_ctrl(0);

        info!("DWCMSHC_EMMC_DLL_CTRL: {:#x}", self.reg.emmc_get_dll_ctrl());
        info!("DWCMSHC_EMMC_DLL_RXCLK: {:#x}", self.reg.emmc_get_dll_rxclk());
        info!("DWCMSHC_EMMC_DLL_TXCLK: {:#x}", self.reg.emmc_get_dll_txclk());
        self.reg.emmc_set_dll_ctrl(EMMC_DLL_START | EMMC_DLL_BYPASS);
        self.reg.emmc_set_dll_rxclk(1 << 31);
        self.reg.emmc_set_dll_txclk(0);
        info!("DWCMSHC_EMMC_DLL_CTRL: {:#x}", self.reg.emmc_get_dll_ctrl());
        info!("DWCMSHC_EMMC_DLL_RXCLK: {:#x}", self.reg.emmc_get_dll_rxclk());
        info!("DWCMSHC_EMMC_DLL_TXCLK: {:#x}", self.reg.emmc_get_dll_txclk());

        self.reg.emmc_enable_internal_clk();
        while !self.reg.emmc_internal_clk_is_stable() {
            info!("emmc internal clk is not stable!");
        }
        self.reg.emmc_enable_sd_clk();

        info!("emmc enable sd clk: {:#x}", self.reg.emmc_get_clk_ctrl());

        self.sdhci_send_cmd(0, 0, 0, 0); // CMD0
        info!("CMD0 response: {:#x}", self.reg.emmc_get_resp01());
        delay_us(10000);
        self.sdhci_send_cmd(1, 0, 2, 0); // CMD1
        info!("CMD1 response: {:#x}", self.reg.emmc_get_resp01());
        delay_us(10000);
    }

    pub fn sdhci_send_cmd(&self, idx: u16, ctype: u16, resp_type: u16, arg: u32) {
        self.reg.emmc_clear_all_error_int_flags();
        self.reg.emmc_clear_all_normal_int_flags();

        self.reg.emmc_reset_cmd();
        while !self.reg.emmc_reset_cmd_is_finished() {
            info!("emmc reset cmd is not finished!");
        }

        while !self.reg.emmc_cmd_is_ready() {
            info!("emmc cmd is not ready!");
        }
        self.reg.emmc_set_argument(arg);

        info!("emmc set argument: {:#x}", arg);
        info!("emmc set cmd: {:#x}", idx << EMMC_CMD_INDEX_POS | ctype | resp_type);
        self.reg.emmc_set_cmd(idx << EMMC_CMD_INDEX_POS | ctype | resp_type);
    }
}