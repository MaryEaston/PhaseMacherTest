use super::csv::conv_csv;

use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use libpicker::data::Data;
use libpicker::data_analog::DataAnalog;
use libpicker::data_digital::DataDigital;
use log::debug;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, FileReader, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

use libpicker::search_closest;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let weight_form = use_state(|| Vec::<Html>::new());
    let expected_form = use_state(|| Vec::<Html>::new());
    let input_file_content = use_state(|| String::new());
    let step_count = use_state(|| String::new());
    let debug = use_state(|| String::new());
    let graph_data = use_state(|| DataDigital::new(vec![]));

    let get_weight_form = || {
        let weight_form: Vec<yew::virtual_dom::VNode> = weight_form.iter().rev().cloned().collect();
        weight_form
    };

    let get_expected_form = || {
        let expected_form: Vec<yew::virtual_dom::VNode> =
            expected_form.iter().rev().cloned().collect();
        expected_form
    };

    html! {
        <main class="container">
            <h1>{"Phase Matcher"}</h1>
            <ol>
                <li>{"目標値を入力(入力欄は+と-で増減)"}</li>
                <li>{"測定したCSVファイルを選択(左から何行をデータの名前として使うか入力)"}</li>
                <li>{"VS を押す"}</li>
            </ol>

            <form onsubmit={compare(&weight_form,&expected_form,&input_file_content,&debug,&graph_data)}>
                <hr/>
                <h2>{"重み"}</h2>
                <div class="row">
                    <form onclick={add_new_line(&weight_form,html!{<WeightLine index={weight_form.len()}/>})}>
                        <button class="blank-around" type="submit">{"+"}</button>
                    </form>
                    <form onclick={remove_last_line(&weight_form)}>
                        <button class="blank-around" type="submit">{"-"}</button>
                    </form>
                </div>
                <div class="blank-around">
                    {get_weight_form()}
                </div>

                <hr/>
                <h2>{"目標値"}</h2>
                <p>{"Start周波数とStop周波数とそれぞれの値を指定、それらを直線で結んだものを目標値として使う"}</p>
                <div class="row">
                    <input class="number-input blank-around short" id="start_freq" type="number" step="0.01" placeholder="Start Freq" />
                    <input class="number-input blank-around short" id="end_freq" type="number" step="0.01" placeholder="Stop Freq" />
                </div>
                <div class="row">
                    <form onsubmit={generate_line(&expected_form)}>
                        <input class="number-input blank-around short" id="s" type="number" step="0.01" placeholder="Start" />
                        <input class="number-input blank-around short" id="e" type="number" step="0.01" placeholder="End" />
                        <input class="number-input blank-around short" id="step_size" type="number" step="0.00001" placeholder="Step Size" onchange={calculate_step_count(&step_count)}/>
                        <input class="number-input blank-around short" id="step_count" type="number" placeholder={(*step_count).clone()} />
                        <button class="blank-around" type="submit">{"生成"}</button>
                    </form>
                    <div class="spacer"></div>
                    <form onclick={add_new_line(&expected_form,html!{<ExpectedLine index={expected_form.len()}/>})}>
                        <button class="blank-around" type="submit">{"+"}</button>
                    </form>
                    <form onclick={remove_last_line(&expected_form)}>
                        <button class="blank-around" type="submit">{"-"}</button>
                    </form>
                </div>

                <div class="blank-around">
                    {get_expected_form()}
                </div>

                <hr/>
                <h2>{"測定データ"}</h2>
                <div class="row">
                    <input id="ignore-row-num" class="blank-around long" type="number" placeholder="Ignore Row Num" />
                    <input id="content-input" class="blank-around" onchange={update_file_content(&input_file_content)} type="file" />
                </div>

                <button type="submit" class="blank-around big">{"VS"}</button>

                <hr/>
                <h2>{"結果"}</h2>
                <textarea id="result" placeholder="VS を押してください" value={(*debug).clone()} />
                <Figure data={(*graph_data).clone()}/>
            </form>
        </main>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ExpectedLineProps {
    index: usize,
    #[prop_or_default]
    s_value: f64,
    #[prop_or_default]
    e_value: f64,
}

#[function_component]
fn ExpectedLine(props: &ExpectedLineProps) -> Html {
    html!(
        <div class="row">
            <p>{props.index}</p>
            <input class="number-input-id blank-around short" value={format!("{}:" , props.index)} type="hidden" readonly=true />
            <input class="number-input blank-around short" id={format!("s{}" , props.index)} type="number" step="0.00001" placeholder="Start" value={props.s_value.to_string()}/>
            <input class="number-input blank-around short" id={format!("e{}" , props.index)} type="number" step="0.00001" placeholder="End" value={props.e_value.to_string()}/>
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
struct WeightLineProps {
    index: usize,
}

#[function_component]
fn WeightLine(props: &WeightLineProps) -> Html {
    let freq: UseStateHandle<u16> = use_state(|| 10);
    let weight: UseStateHandle<u16> = use_state(|| 100);
    html!(
        <div class="row">
            <input class="blank-around short" id={format!("weight_freq_{}",props.index)} type="number" step="0.00001" min="0" max="1000" value={freq.to_string()} oninput={update(&freq)}/>
            <p>{"GHz : "}</p>
            <input class="blank-around short" id={format!("weight_{}",props.index)} type="range" min="0" max="100" value={weight.to_string()} oninput={update(&weight)}/>
            <input class="blank-around short" type="number" min="0" max="100" value={weight.to_string()} oninput={update(&weight)}/>
        </div>
    )
}

fn update<T>(value: &UseStateHandle<T>) -> Callback<InputEvent>
where
    T: FromStr + 'static,
{
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(v) = input.value().parse::<T>() {
            value.set(v);
        }
    })
}

fn update_file_content(file_content: &UseStateHandle<String>) -> Callback<Event> {
    let file_content = file_content.clone();
    Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Some(file) = input.files().and_then(|files| files.get(0)) {
            let reader = FileReader::new().unwrap();
            let reader_clone = reader.clone();
            let file_content = file_content.clone();

            let onloadend = Closure::wrap(Box::new(move |_e: ProgressEvent| {
                if let Ok(content) = reader_clone.result() {
                    if let Some(text) = content.as_string() {
                        file_content.set(text);
                    }
                }
            }) as Box<dyn FnMut(_)>);

            reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
            reader.read_as_text(&file).unwrap();
            onloadend.forget();
        }
    })
}

fn add_new_line(
    expected_form: &UseStateHandle<Vec<yew::virtual_dom::VNode>>,
    line: Html,
) -> Callback<MouseEvent> {
    let expected_form = expected_form.clone();
    Callback::from(move |e: MouseEvent| {
        let new_line = line.clone();
        e.prevent_default();
        let mut new_reference_form = (*expected_form).clone();
        new_reference_form.push(new_line);
        expected_form.set(new_reference_form);
    })
}

fn remove_last_line(
    expected_form: &UseStateHandle<Vec<yew::virtual_dom::VNode>>,
) -> Callback<MouseEvent> {
    let expected_form = expected_form.clone();
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        let mut new_reference_form = (*expected_form).clone();
        new_reference_form.pop();
        expected_form.set(new_reference_form);
    })
}

fn generate_line(
    expected_form: &UseStateHandle<Vec<yew::virtual_dom::VNode>>,
) -> Callback<SubmitEvent> {
    let expected_form = expected_form.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        e.stop_propagation();
        let form: HtmlFormElement = e.target_unchecked_into();
        let s: f64 = get_input_by_name(&form, "s");
        let e: f64 = get_input_by_name(&form, "e");
        let step_size: f64 = get_input_by_name(&form, "step_size");
        let step_count: usize = get_input_by_name(&form, "step_count");

        let mut new_reference_form = (*expected_form).clone();
        let mut new_lines: Vec<Html> = (expected_form.len()..step_count)
            .collect::<Vec<usize>>()
            .iter()
            .map(|i| {
                let s = s + step_size * *i as f64;
                let e = e + step_size * *i as f64;
                let new_line = html! {
                    <ExpectedLine index={*i} s_value={s} e_value={e} />
                };
                new_line
            })
            .collect();
        new_reference_form.append(&mut new_lines);
        expected_form.set(new_reference_form);
        // let mut new_reference_form = (*expected_form).clone();
        // new_reference_form.pop();
        // expected_form.set(new_reference_form);
    })
}

fn calculate_step_count(step_count: &UseStateHandle<String>) -> Callback<Event> {
    let step_count = step_count.clone();
    Callback::from(move |e: Event| {
        e.prevent_default();
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            step_count.set(((360.0 / input.value().parse::<f64>().unwrap()) as usize).to_string());
        }
    })
}

fn get_input_by_name<T>(form: &HtmlFormElement, name: &str) -> T
where
    T: Default + FromStr,
{
    // log::debug!("get_input_by_name: {:?}", name);
    form.get_elements_by_tag_name("input")
        .get_with_name(name)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<T>()
        .unwrap_or(Default::default())
}

fn compare(
    weight_form: &UseStateHandle<Vec<yew::virtual_dom::VNode>>,
    expected_form: &UseStateHandle<Vec<yew::virtual_dom::VNode>>,
    target: &UseStateHandle<String>,
    debug: &UseStateHandle<String>,
    graph_data: &UseStateHandle<DataDigital>,
) -> Callback<SubmitEvent> {
    let weight_form = weight_form.clone();
    let expected_form = expected_form.clone();
    let target = target.clone();
    let debug = debug.clone();
    let graph_data = graph_data.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let form: HtmlFormElement = e.target_unchecked_into();
        let start_freq = get_input_by_name(&form, "start_freq");
        let end_freq = get_input_by_name(&form, "end_freq");
        let ignore_row_num = get_input_by_name(&form, "ignore-row-num");

        // 重みを生成
        let mut weight: Vec<(Decimal, Decimal)> = Vec::new();
        for (index, _) in weight_form.iter().enumerate() {
            let f = get_input_by_name(&form, format!("weight_freq_{}", index).as_str());
            let w: f64 = get_input_by_name(&form, format!("weight_{}", index).as_str());
            let f = Decimal::from_f64(f).unwrap();
            let w = Decimal::from_f64(w / 100.0).unwrap();
            weight.push((f, w));
        }
        let weight = DataDigital::new(weight);
        log::info!("weight: {:?}", weight);

        // 理想値の直線を生成
        let mut expected_datas: HashMap<String, DataAnalog> = HashMap::new();
        for (index, _) in expected_form.iter().enumerate() {
            let s = get_input_by_name(&form, format!("s{}", index).as_str());
            let e = get_input_by_name(&form, format!("e{}", index).as_str());
            expected_datas.insert(
                index.to_string(),
                DataAnalog::get_line(
                    (
                        Decimal::from_f64(start_freq).unwrap(),
                        Decimal::from_f64(s).unwrap(),
                    ),
                    (
                        Decimal::from_f64(end_freq).unwrap(),
                        Decimal::from_f64(e).unwrap(),
                    ),
                ),
            );
        }
        for (id_expected, data_expected) in expected_datas.clone() {
            log::info!(
                "ID: {:?}, f(10)={}, f(20)={}",
                id_expected,
                data_expected.y(&dec!(10)).unwrap(),
                data_expected.y(&dec!(20)).unwrap(),
            );
        }

        // 測定データを生成
        let target_datas: HashMap<String, DataDigital> =
            conv_csv(target.to_string(), ignore_row_num)
                .iter()
                .map(|(name, data)| (name.clone(), DataDigital::build_from_csv(data).unwrap()))
                .collect();

        let score = search_closest(
            expected_datas.clone(),
            target_datas,
            weight.clone(),
            weight.get_x_list(),
        )
        .unwrap();
        let table = gen_result_table(score.clone());
        debug.set(table);

        let score = calc_diff_score_with_expected(score, expected_datas);
        let rms = calc_rms_phase_error(transform(score));
        graph_data.set(rms);
    })
}

fn gen_result_table(score: HashMap<(String, String), DataDigital>) -> String {
    let mut score = score
        .iter()
        .map(|((expected_id, min_data_id), min_data)| {
            (
                expected_id.parse::<usize>().unwrap(),
                min_data_id.to_string(),
                min_data.to_csv(),
            )
        })
        .collect::<Vec<(usize, String, String)>>();
    score.sort();
    let table = score
        .iter()
        .map(|(expected_id, data_id, data)| format!("{expected_id},{data_id}\n{data}"))
        .collect::<Vec<String>>()
        .join("\n");
    table
}

fn calc_diff_score_with_expected(
    score: HashMap<(String, String), DataDigital>,
    expected_datas: HashMap<String, DataAnalog>,
) -> HashMap<(String, String), DataDigital> {
    let score = score.clone();
    let mut new_score = HashMap::<(String, String), DataDigital>::new();
    for ((id, name), data) in score {
        let expected_data = expected_datas.get(&id).unwrap();
        let mut new_data = Vec::<(Decimal, Decimal)>::new();
        for (x, y) in data.get_data() {
            let delta_y = libpicker::phase_difference(y, expected_data.y(&x).unwrap());
            new_data.push((x, delta_y));
        }
        let new_data = DataDigital::new(new_data);
        new_score.insert((id, name), new_data);
    }
    new_score
}

fn transform(score: HashMap<(String, String), DataDigital>) -> Vec<Vec<(usize, Decimal)>> {
    let mut result: Vec<Vec<(usize, Decimal)>> = Vec::new();
    let mut keys: Vec<_> = score.keys().collect();
    keys.sort();

    // データの最大長を取得
    let max_length = score
        .values()
        .map(|data| data.get_data_count())
        .max()
        .unwrap_or(0);

    for i in 0..max_length {
        let mut head_value = dec!(0);
        let mut row: Vec<(usize, Decimal)> = Vec::new();
        for (j, key) in keys.iter().enumerate() {
            if let Some(data) = score.get(*key) {
                if let Some((k, value)) = data.get(i) {
                    head_value = k;
                    row.push((j + 1, value.clone()));
                }
            }
        }
        row.insert(0, (0, head_value));
        result.push(row);
    }

    result
}

fn calc_standard_deviation(values: Vec<Decimal>) -> Decimal {
    let mean = values.iter().sum::<Decimal>() / Decimal::from(values.len() as u32);
    let variance = values
        .iter()
        .map(|value| {
            let diff = libpicker::phase_difference(*value, mean);
            diff * diff
        })
        .sum::<Decimal>()
        / Decimal::from(values.len() as u32);
    let std_dev = variance.sqrt().unwrap();
    std_dev
}

fn calc_rms_phase_error(data: Vec<Vec<(usize, Decimal)>>) -> DataDigital {
    let mut result = DataDigital::new(vec![]);

    debug!("{:?}", data);

    for d in data {
        let mut d = d;
        d.sort();
        d.reverse();
        let (mut extracted, remaining): (Vec<(usize, Decimal)>, Vec<(usize, Decimal)>) =
            d.iter().partition(|(index, _d)| *index == 0);
        let values: Vec<Decimal> = remaining.iter().map(|&(_, value)| value).collect();
        let y = calc_standard_deviation(values);
        result.add(extracted.pop().unwrap().1, y);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use libpicker::data_digital::DataDigital;
    use rust_decimal_macros::dec;

    #[test]
    fn transform_1() {
        let mut input: HashMap<(String, String), DataDigital> = HashMap::new();
        input.insert(
            ("0".to_string(), "foo".to_string()),
            DataDigital::new(vec![
                (dec!(10), dec!(1.1594)),
                (dec!(20), dec!(2.4639)),
                (dec!(30), dec!(3.2370)),
                (dec!(40), dec!(4.0438)),
                (dec!(50), dec!(5.3450)),
                (dec!(60), dec!(6.0934)),
            ]),
        );
        input.insert(
            ("1".to_string(), "baz".to_string()),
            DataDigital::new(vec![
                (dec!(10), dec!(1.0423)),
                (dec!(20), dec!(2.9832)),
                (dec!(30), dec!(3.5347)),
                (dec!(40), dec!(4.3402)),
                (dec!(50), dec!(5.1039)),
                (dec!(60), dec!(6.0814)),
            ]),
        );
        input.insert(
            ("2".to_string(), "bar".to_string()),
            DataDigital::new(vec![
                (dec!(10), dec!(1.7542)),
                (dec!(20), dec!(2.1237)),
                (dec!(30), dec!(3.0922)),
                (dec!(40), dec!(4.4072)),
                (dec!(50), dec!(5.0913)),
                (dec!(60), dec!(6.3488)),
            ]),
        );

        let expected = vec![
            vec![
                (0, dec!(10)),
                (1, dec!(1.1594)),
                (2, dec!(1.0423)),
                (3, dec!(1.7542)),
            ],
            vec![
                (0, dec!(20)),
                (1, dec!(2.4639)),
                (2, dec!(2.9832)),
                (3, dec!(2.1237)),
            ],
            vec![
                (0, dec!(30)),
                (1, dec!(3.2370)),
                (2, dec!(3.5347)),
                (3, dec!(3.0922)),
            ],
            vec![
                (0, dec!(40)),
                (1, dec!(4.0438)),
                (2, dec!(4.3402)),
                (3, dec!(4.4072)),
            ],
            vec![
                (0, dec!(50)),
                (1, dec!(5.3450)),
                (2, dec!(5.1039)),
                (3, dec!(5.0913)),
            ],
            vec![
                (0, dec!(60)),
                (1, dec!(6.0934)),
                (2, dec!(6.0814)),
                (3, dec!(6.3488)),
            ],
        ];

        let output = transform(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn transform_2() {
        let mut input: HashMap<(String, String), DataDigital> = HashMap::new();
        input.insert(
            ("0".to_string(), "foo".to_string()),
            DataDigital::new(vec![
                (dec!(1), dec!(1.1594)),
                (dec!(2), dec!(2.4639)),
                (dec!(3), dec!(3.2370)),
                (dec!(4), dec!(4.0438)),
            ]),
        );
        input.insert(
            ("1".to_string(), "baz".to_string()),
            DataDigital::new(vec![
                (dec!(1), dec!(1.0423)),
                (dec!(2), dec!(2.9832)),
                (dec!(3), dec!(3.5347)),
                (dec!(4), dec!(4.3402)),
            ]),
        );
        input.insert(
            ("2".to_string(), "bar".to_string()),
            DataDigital::new(vec![
                (dec!(1), dec!(1.7542)),
                (dec!(2), dec!(2.1237)),
                (dec!(3), dec!(3.0922)),
                (dec!(4), dec!(4.4072)),
            ]),
        );

        let expected = vec![
            vec![
                (0, dec!(1)),
                (1, dec!(1.1594)),
                (2, dec!(1.0423)),
                (3, dec!(1.7542)),
            ],
            vec![
                (0, dec!(2)),
                (1, dec!(2.4639)),
                (2, dec!(2.9832)),
                (3, dec!(2.1237)),
            ],
            vec![
                (0, dec!(3)),
                (1, dec!(3.2370)),
                (2, dec!(3.5347)),
                (3, dec!(3.0922)),
            ],
            vec![
                (0, dec!(4)),
                (1, dec!(4.0438)),
                (2, dec!(4.3402)),
                (3, dec!(4.4072)),
            ],
        ];

        let output = transform(input);
        assert_eq!(output, expected);
    }
}

use web_sys::HtmlCanvasElement;
use yew::NodeRef;

#[derive(Properties, PartialEq, Clone)]
struct FigureProps {
    pub data: DataDigital,
}

#[function_component]
fn Figure(props: &FigureProps) -> Html {
    let canvas_ref: NodeRef = use_node_ref();
    draw(&canvas_ref, props.data.clone());
    html! {
        <div>
            <canvas ref={canvas_ref.clone()} width="800" height="600"></canvas>
        </div>
    }
}

fn draw(canvas: &NodeRef, data: DataDigital) -> Option<()> {
    use plotters::prelude::*;
    use plotters_canvas::CanvasBackend;

    fn get_range(data: &Vec<Decimal>) -> Option<Range<f64>> {
        let min = data
            .iter()
            .map(|d| d.to_f64().unwrap_or(f64::INFINITY))
            .fold(f64::INFINITY, f64::min);
        let max = data
            .iter()
            .map(|d| d.to_f64().unwrap_or(f64::NEG_INFINITY))
            .fold(f64::NEG_INFINITY, f64::max);
        if min.is_infinite() || max.is_infinite() {
            None
        } else {
            Some(min..max)
        }
    }

    let x_range = get_range(&data.get_x_list()).unwrap_or_else(|| 0.0..1.0);
    let y_range = get_range(&data.get_y_list()).unwrap_or_else(|| 0.0..1.0);

    let canvas = match canvas.cast::<HtmlCanvasElement>() {
        Some(canvas) => canvas,
        None => return None,
    };

    let rect = canvas.get_bounding_client_rect();
    canvas.set_height(rect.height() as u32);
    canvas.set_width(rect.width() as u32);

    let backend =
        CanvasBackend::with_canvas_object(canvas.clone()).expect("Failed to create CanvasBackend");

    let drawing_area = backend.into_drawing_area();
    drawing_area
        .fill(&RGBColor(255, 255, 255))
        .expect("Failed to fill drawing area");

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("RMS Phased Error", ("sans-serif", 14).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range, y_range)
        .expect("Failed to build chart");

    chart.configure_mesh().draw().expect("Failed to draw mesh");

    debug!("1");

    let x_list = data.get_data();
    chart
        .draw_series(LineSeries::new(
            x_list
                .iter()
                .map(|(x, y)| (x.to_f64().unwrap(), y.to_f64().unwrap())),
            &RED,
        ))
        .expect("Failed to draw series");

    Some(())
}
