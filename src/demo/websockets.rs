use leptos::*;
use leptos_use::{
    core::ConnectionReadyState, use_websocket, use_websocket_with_options, UseWebSocketOptions,
    UseWebsocketReturn,
};

use web_sys::{CloseEvent, Event};

#[component]
pub fn WebsocketDemo() -> impl IntoView {
    let (history, set_history) = create_signal(vec![]);

    fn update_history(&history: &WriteSignal<Vec<String>>, message: String) {
        let _ = &history.update(|history: &mut Vec<_>| history.push(message));
    }
    // ----------------------------
    // use_websocket
    // ----------------------------

    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket("wss://rpc.secret.express/websocket");

    let send_message = move |_| {
        let m = "Hello, world!".to_string();
        send(m.clone());
        set_history.update(|history: &mut Vec<_>| history.push(format! {"[send]: {:?}", m}));
    };

    let send_byte_message = move |_| {
        let m = b"Hello, world!\r\n".to_vec();
        send_bytes(m.clone());
        set_history.update(|history: &mut Vec<_>| history.push(format! {"[send_bytes]: {:?}", m}));
    };

    let status = move || ready_state().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
    };
    let close_connection = move |_| {
        close();
    };

    create_effect(move |_| {
        if let Some(m) = message.get() {
            update_history(&set_history, format! {"[message]: {:?}", m});
        };
    });

    create_effect(move |_| {
        if let Some(m) = message_bytes.get() {
            update_history(&set_history, format! {"[message_bytes]: {:?}", m});
        };
    });

    // ----------------------------
    // use_websocket_with_options
    // ----------------------------

    let (history2, set_history2) = create_signal(vec![]);

    let on_open_callback = move |e: Event| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(format! {"[onopen]: event {:?}", e.type_()})
        });
    };

    let on_close_callback = move |e: CloseEvent| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(format! {"[onclose]: event {:?}", e.type_()})
        });
    };

    let on_error_callback = move |e: Event| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(format! {"[onerror]: event {:?}", e.type_()})
        });
    };

    let on_message_callback = move |m: String| {
        set_history2.update(|history: &mut Vec<_>| history.push(format! {"[onmessage]: {:?}", m}));
    };

    let on_message_bytes_callback = move |m: Vec<u8>| {
        set_history2
            .update(|history: &mut Vec<_>| history.push(format! {"[onmessage_bytes]: {:?}", m}));
    };

    let UseWebsocketReturn {
        ready_state: ready_state2,
        send: send2,
        send_bytes: send_bytes2,
        open: open2,
        close: close2,
        message: message2,
        message_bytes: message_bytes2,
        ..
    } = use_websocket_with_options(
        "wss://rpc.secret.express/websocket",
        UseWebSocketOptions::default()
            .immediate(false)
            .on_open(on_open_callback.clone())
            .on_close(on_close_callback.clone())
            .on_error(on_error_callback.clone())
            .on_message(on_message_callback.clone())
            .on_message_bytes(on_message_bytes_callback.clone()),
    );

    let open_connection2 = move |_| {
        open2();
    };
    let close_connection2 = move |_| {
        close2();
    };

    let send_message2 = move |_| {
        // let message = "Hello, use_leptos!".to_string();
        let message = "{ \"jsonrpc\": \"2.0\", \"method\": \"subscribe\", \"params\": [\"tm.event='NewBlock'\"], \"id\": 1 }".to_string();
        // { "jsonrpc": "2.0", "method": "subscribe", "params": ["tm.event='NewBlock'"], "id": 1 }
        // { "jsonrpc": "2.0", "method": "subscribe", "params": ["tm.event = 'Tx' AND transfer.recipient = 'secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek'"], "id": 1 }
        send2(message.clone());
        update_history(&set_history2, format! {"[send]: {:?}", message});
    };

    let send_byte_message2 = move |_| {
        let m = b"Hello, world!\r\n".to_vec();
        send_bytes2(m.clone());
        update_history(&set_history2, format! {"[send_bytes]: {:?}", m});
    };

    let status2 = move || ready_state2.get().to_string();

    create_effect(move |_| {
        if let Some(m) = message2.get() {
            update_history(&set_history2, format! {"[message]: {:?}", m});
        };
    });

    create_effect(move |_| {
        if let Some(m) = message_bytes2.get() {
            update_history(&set_history2, format! {"[message_bytes]: {:?}", m});
        };
    });

    let connected2 = move || ready_state2.get() == ConnectionReadyState::Open;

    view! {
        <div class="container">
            <div class="flex flex-col lg:flex-row gap-4">
                <div class="w-full lg:w-1/2">
                    <h1 class="text-xl lg:text-4xl mb-2">"use_websocket"</h1>
                    <p>"status: " {status}</p>
                    <button on:click=send_message disabled=move || !connected()>
                        "Send"
                    </button>
                    <button on:click=send_byte_message disabled=move || !connected()>
                        "Send bytes"
                    </button>
                    <button on:click=open_connection disabled=connected>
                        "Open"
                    </button>
                    <button on:click=close_connection disabled=move || !connected()>
                        "Close"
                    </button>
                    <div class="flex items-center">
                        <h3 class="text-2xl mr-2">"History"</h3>
                        <button
                            on:click=move |_| set_history(vec![])
                            disabled=move || history.get().is_empty()
                        >
                            "Clear"
                        </button>
                    </div>
                    <For
                        each=move || history.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        view=move |(_, message)| {
                            view! { <div>{message}</div> }
                        }
                    />

                </div>
                <div class="w-full lg:w-1/2">
                    <h1 class="text-xl lg:text-4xl mb-2">"use_websocket_with_options"</h1>
                    <p>"status: " {status2}</p>
                    <button on:click=open_connection2 disabled=connected2>
                        "Connect"
                    </button>
                    <button on:click=close_connection2 disabled=move || !connected2()>
                        "Close"
                    </button>
                    <button on:click=send_message2 disabled=move || !connected2()>
                        "Send"
                    </button>
                    <button on:click=send_byte_message2 disabled=move || !connected2()>
                        "Send Bytes"
                    </button>
                    <div class="flex items-center">
                        <h3 class="text-2xl mr-2">"History"</h3>
                        <button
                            on:click=move |_| set_history2(vec![])
                            disabled=move || history2.get().is_empty()
                        >
                            "Clear"
                        </button>
                    </div>
                    <ul>
                        <For
                            each=move || history2.get().into_iter().enumerate()
                            key=|(index, _)| *index
                            view=move |(_, message)| {
                                view! { <li>{message}</li> }
                            }
                        />

                    </ul>
                </div>
            </div>

        </div>
    }
}
