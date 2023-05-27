pub fn create_window(ui: &mut imgui::Ui) {
    ui.window("Hello world")
        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.text("こんにちは世界！");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
}
