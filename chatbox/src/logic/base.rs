use crate::slint_generatedAppWindow::{AppWindow, BaseLogic};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<BaseLogic>()
        .on_line_count(move |text| text.lines().count() as i32);

    ui.global::<BaseLogic>().on_code_line_text(move |text| {
        let mut number_text = "".to_string();
        let count = text.lines().count();
        if count > 1 {
            for i in 0..(count as f32 * 1.5) as u32 {
                if i == 0 {
                    number_text = "01".to_string();
                } else {
                    number_text = format!("{}\n{:02}", number_text, i + 1);
                }
            }
        }

        number_text.into()
    });

    ui.global::<BaseLogic>()
        .on_list_item_trim(move |text| text.replacen("-", " ", 1).trim().into());

    ui.global::<BaseLogic>().on_leading_spaces(move |text| {
        text.chars()
            .take_while(|&c| c == ' ')
            .collect::<String>()
            .into()
    });
}
