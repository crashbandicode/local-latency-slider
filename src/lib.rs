mod framerate;
mod ldn;
mod utils;

#[cfg(feature = "run_tests")]
mod experiments;

use skyline::nn::ui2d::Pane;

use utils::{PaneExt, TextBoxExt};

#[skyline::hook(offset = 0x1a12f60)]
unsafe fn update_css(arg: u64) {
    if ldn::is_local_online() {
        ldn::latency_slider::poll();
        let delay_str = ldn::latency_slider::current_input_delay().to_string();
        let banner_display_str = format!("Buffer: {}\0", delay_str);

        // pointer to p1's title text pane
        let p1_pane = (*((*((arg + 0xe58) as *const u64) + 0x10) as *const u64)) as *mut Pane;
        let p1_pane = &mut *p1_pane;
        p1_pane.as_textbox().set_default_material_colors();
        p1_pane.as_textbox().set_text_string(&banner_display_str);

        let p1_pane_bg = p1_pane.parent().unwrap().traverse_backward(2).unwrap();
        p1_pane_bg.set_visible(false);

        let p2_pane = p1_pane_bg
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .prev()
            .unwrap();
        p2_pane.as_textbox().set_default_material_colors();
        p2_pane.as_textbox().set_text_string(&banner_display_str);

        let panel_root = p2_pane.parent().unwrap().traverse_forward(4).unwrap();

        for player_index in 0..8 {
            let player_panel_root = panel_root
                .get_child(&format!("set_panel_{}p", player_index + 1), false)
                .unwrap();

            let player_panel = player_panel_root.children().unwrap();

            let player_panel_name = player_panel
                .get_child(&format!("set_btn_panel"), false)
                .unwrap();
            let player_panel_name = player_panel_name.children().unwrap().next().unwrap();
            let player_panel_name = player_panel_name
                .get_child(&format!("set_txt_00"), true)
                .unwrap();

            let player_net_info = ldn::net::get_player_net_info(player_index);
            match player_net_info.is_connected() {
                true => {
                    player_panel_name.as_textbox().set_text_string(&format!(
                        "{}f",
                        player_net_info.delay.to_string()
                    ));
                }
                false => {
                    player_panel_name
                        .as_textbox()
                        .set_text_string(&format!("P{}", player_index + 1));
                }
            }
        }
    }
    call_original!(arg);
}

#[skyline::main(name = "local-latency-slider")]
pub fn main() {
    framerate::install();
    ldn::install();
    skyline::install_hook!(update_css);

    #[cfg(feature = "run_tests")]
    {
        println!("RUNNING TESTS...");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("MEASURING SYSTEM SLEEP ACCURACY...");
        experiments::measure_sleep_accuracy(Some(8));
        println!("MEASURING SPIN STRATEGY LATENCY...");
        experiments::measure_spin_strategy_latency(Some(8));
    }
}
