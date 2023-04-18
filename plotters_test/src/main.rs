use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub enum PlotMsg {
    Redraw,
    Nothing,
}

/*
#[derive(Properties, PartialEq)]
pub struct PlotProps {}
*/
pub struct Plot {
    canvas: NodeRef,
}

impl Component for Plot {
    type Message = PlotMsg;
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
          <div>
            <canvas ref = {self.canvas.clone()}/>
          </div>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotMsg::Redraw => {
                let element: HtmlCanvasElement = self.canvas.cast().unwrap();

                let rect = element.get_bounding_client_rect();
                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);

                let backend = CanvasBackend::with_canvas_object(element).unwrap();

                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&RGBColor(200, 200, 200)).unwrap();

                let mut chart = ChartBuilder::on(&drawing_area)
                    .caption("y=x^2", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                chart
                    .draw_series(LineSeries::new(
                        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                        &RED,
                    ))
                    .unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

                false
            }
            _ => true,
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(PlotMsg::Redraw);
        Plot {
            canvas: NodeRef::default(),
        }
    }
}

fn main() {
    yew::Renderer::<Plot>::new().render();
}
