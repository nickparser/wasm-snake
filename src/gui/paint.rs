use crate::gui::web::Web;
use crate::model::data::Position;

pub struct Paint {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    px: u32,
}

impl Paint {
    pub fn new(canvas: web_sys::HtmlCanvasElement, px: u32) -> Paint {
        let context = Web::context_2d(&canvas);

        Paint {
            canvas,
            context,
            px,
        }
    }

    pub fn fill(&self, color: &str) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.fill_rect(0.0, 0.0, width, height, color);
    }

    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.context.begin_path();
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(x, y, width, height);
        self.context.stroke();
    }

    pub fn fill_rect_px(&self, position: Position, color: &str) {
        let px_x = (position.x() * self.px) as f64;
        let px_y = (position.y() * self.px) as f64;
        self.fill_rect(px_x, px_y, self.px as f64, self.px as f64, color);
    }

    pub fn fill_rect_vec_px(&self, positions: Vec<Position>, color: &str) {
        positions
            .into_iter()
            .for_each(|position| self.fill_rect_px(position, color));
    }
}
