use crate::slint_generatedAppWindow::{AppWindow, BaseLogic};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<BaseLogic>().on_line_count(move |text| {
        text.lines().count() as i32
    });

    ui.global::<BaseLogic>().on_code_line_text(move |text| {
        let mut number_text = "".to_string();
        let count = text.lines().count();
        if count > 1 {
            for i in 0..count {
                if i == 0 {
                    number_text = "01".to_string();
                } else {
                    number_text = format!("{}\n{:02}", number_text, i+1);
                }
            }
        }

        number_text.into()
    });
}
