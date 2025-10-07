use reqwasm;
use serde::Deserialize;
use serde_json;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::utils;
#[derive(Deserialize, Debug, Clone)]
struct ListResp {
    namelist: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
struct SingleServerResp {
    label: String,
    data: Vec<SingleServerData>,
}
#[derive(Deserialize, Debug, Clone)]
struct SingleServerData {
    timestamp: String,
    latency: i32,
    player: i32,
    playerlist: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub n: String,
}
#[function_component(Home)]
pub fn home() -> Html {
    let r = use_state(|| html! { <div>{"Loading"}</div>});

    /*{
        let r_c1 = r.clone();
        use_effect_with(move |_| {
            spawn_local(async move {
                r_c1.set("aaa".to_string());
            });
            || ()
        },)

    }*/
    {
        let r = r.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = reqwasm::http::Request::get("http://127.0.0.1:18650/api/list")
                    .send()
                    .await
                    .unwrap();
                let data = resp.text().await.unwrap();
                let json: ListResp = serde_json::from_str(&data).unwrap();
                let nl = json.namelist;
                let mut sl: Vec<SingleServerResp> = Vec::new();
                for i in nl.iter() {
                    let sresp = reqwasm::http::Request::get(&format!(
                        "http://127.0.0.1:18650/api/serverod/{}",
                        *i
                    ))
                    .send()
                    .await
                    .unwrap();
                    console::log_1(&format!("The answer is {}", *i).into());
                    let sdata = sresp.text().await.unwrap();
                    console::log_1(&format!("The answer is {}", sdata).into());
                    let sjson: SingleServerResp = serde_json::from_str(&sdata).unwrap();
                    sl.push(sjson);
                }
                r.set(html!{
                    <div class="d-flex flex-wrap">
                        {
                            for (0..nl.len()).map(|i| {
                                html!{
                                    <a href={ format!("server/{}",&nl[i])} class="text-decoration-none">
                                        <div class="card m-2" style="width: 18rem;">
                                            //<div class="card-body"><a href={ format!("server/{}",&nl[i])} class="btn btn-primary">{&nl[i]} {" : "} {&ll[i]}</a></div>
                                            <div class="card-body">
                                                <h5>{ format!("Server: {}", sl[i].label) }</h5>
                                                <h6>{ format!("Server id: {}", nl[i]) }</h6>
                                                <p>{ /*format!("Current latency: {}", sl[i].data[0].latency)*/
                                                    (|| {
                                                        let clatency = sl[i].data[0].latency;
                                                        let cpercentage = (clatency) as f64 / 1000_f64;
                                                        let color = utils::percent_color((144, 238, 144), (255, 0, 0), cpercentage);
                                                        return html!{
                                                            <>
                                                                <span>{ "Current latency: " }</span>
                                                                <span style={ format!("color: #{}",color) }>{ clatency }</span>
                                                            </>
                                                        }
                                                    })()
                                                }</p>
                                            </div>
                                        </div>
                                    </a>
                                }
                            })
                        }
                    </div>
                });
            });
            || ()
        });
    }

    html! {
        <div>
            <h1 class="text-2xl font-bold text-center">{"CaSilicate's Minecraft Server Monitor"}</h1>
            <h3>{"Registered servers:"}</h3>
            { (*r).clone() }
        </div>
    }
}
#[function_component(Server)]
pub fn server(props: &Props) -> Html {
    let name = props.n.clone();
    let r = use_state(|| html! { <div>{"Loading"}</div>});
    {
        let r = r.clone();
        let name = name.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = reqwasm::http::Request::get(&format!(
                    "http://127.0.0.1:18650/api/servers/{}",
                    name
                ))
                .send()
                .await
                .unwrap();
                let data = resp.text().await.unwrap();
                let json: SingleServerResp = serde_json::from_str(&data).unwrap();
                let data = json.data;

                r.set(html! {
                    /*<pre style="white-space: pre;">
                        { format!("Current server: {}", label) }
                        {
                            for (0..data.len()).map(|i| html!{
                                    <div style="margin:0;"> { format!("Timestamp:{}\t|  Latency:{}\t|  Player:{}",data[i].timestamp,data[i].latency,data[i].player) } </div>
                            })
                        }
                    </pre>*/
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>{"Timestamp"}</th>
                                <th>{"Latency"}</th>
                                <th>{"Player"}</th>
                                <th>{"Playerlist"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                for (0..data.len()).map(|i| html!{
                                        /*<div style="margin:0;"> { format!("Timestamp:{}\t|  Latency:{}\t|  Player:{}",data[i].timestamp,data[i].latency,data[i].player) } </div>*///&data[i].playerlist
                                        <tr>
                                            <td>{ &data[i].timestamp }</td>
                                            <td>{ &data[i].latency }</td>
                                            <td>{ &data[i].player }</td>
                                            <td>{ (|| {
                                                let playerliststr = &data[i].playerlist;
                                                if playerliststr == "[]" {
                                                    return "Empty"
                                                }
                                                return playerliststr
                                            })() }</td>
                                        </tr>
                                })
                            }
                        </tbody>
                    </table>
                });
            });
            || ()
        });
    }
    html! {
        <div>
            { (*r).clone() }
        </div>
    }
}
