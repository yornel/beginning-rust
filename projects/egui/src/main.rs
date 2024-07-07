  fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 {x:400.0, y:200.0});
  
    let _ = eframe::run_native("My egui App", native_options,
      Box::new(|cc| Box::new(MyEguiApp::new(cc))));
  }

  #[derive(PartialEq, Debug)]
  enum MyItem { First, Second, Third }

  struct MyEguiApp {
    pub message: String,
    pub content: String,
  }
  
  impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
      MyEguiApp{
        message: String::from("Hello"),
        content: String::from("This is content."),
      }
    }
  }
  
  impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
      Self::default()
    }
  }

  impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello World!");
        
        ui.spacing();
  
        let msg = format!("Title:\"{}\"\nContent:[{}]", self.message, self.content);
        let label_txt = egui::RichText::new(msg)
          .font(egui::FontId::proportional(24.0));
        let label = egui::Label::new(label_txt);
        ui.add(label);
  
        ui.separator();
  
        let te_sl = egui::TextEdit::singleline(&mut self.message)
          .font(egui::FontId::proportional(20.0));
        ui.add(te_sl);
        let te_ml = egui::TextEdit::multiline(&mut self.content)
          .font(egui::FontId::proportional(20.0));
        ui.add(te_ml);
      });
    }
  }