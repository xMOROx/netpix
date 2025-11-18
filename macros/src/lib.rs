use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// This macro sets up the clients hashmap (Arc<RwLock<>>).
#[proc_macro]
pub fn setup_clients(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        crate::server::client::new_clients()
    };
    expanded.into()
}

/// This macro sets up the packet handlers given sniffers and clients.
#[proc_macro]
pub fn setup_packet_handlers(_input: TokenStream) -> TokenStream {
    // We'll require that the caller passes `sniffers, clients` as parameters.
    // For example:
    //
    // setup_packet_handlers!(sniffers, clients)
    //
    // We'll parse the input to get these identifiers.
    let input = parse_macro_input!(_input as syn::ExprTuple);
    let sniffers = &input.elems[0];
    let clients = &input.elems[1];
    let config = &input.elems[2];

    let expanded = quote! {
        crate::server::handler::setup_packet_handlers(#sniffers, #clients.clone(), #config).await
    };
    expanded.into()
}

/// This macro sets up routes. The user will pass the variables it needs.
#[proc_macro]
pub fn setup_routes(_input: TokenStream) -> TokenStream {
    // Example usage:
    // setup_routes!(clients, source_to_packets, config)
    let input = parse_macro_input!(_input as syn::ExprTuple);
    let clients = &input.elems[0];
    let source_to_packets = &input.elems[1];
    let config = &input.elems[2];

    let expanded = quote! {
        {
            let clients_filter = warp::any().map(move || #clients.clone());
            let source_to_packets_filter = warp::any().map(move || #source_to_packets.clone());
            let config_filter = warp::any().map(move || #config);

            let ws = warp::path(crate::server::constants::WEBSOCKET_PATH)
                .and(warp::ws())
                .and(clients_filter)
                .and(source_to_packets_filter)
                .and(config_filter)
                .map(|ws: warp::ws::Ws, clients_cl, source_to_packets_cl, config_cl| {
                    ws.on_upgrade(move |socket| {
                        crate::server::client::handle_connection(socket, clients_cl, source_to_packets_cl, config_cl)
                    })
                });

            let index_html = warp::path::end().and_then(crate::server::asset::serve_index);
            let other = warp::path::tail().and_then(crate::server::asset::serve);
            ws.or(index_html).or(other)
        }
    };
    expanded.into()
}

/// This macro spawns the periodic client message sender.
#[proc_macro]
pub fn spawn_message_sender(_input: TokenStream) -> TokenStream {
    // Usage:
    // spawn_message_sender!(clients, CLIENT_MESSAGE_INTERVAL_MS, MESSAGE_BATCH_SIZE)
    let input = parse_macro_input!(_input as syn::ExprTuple);
    let clients = &input.elems[0];
    let interval_ms = &input.elems[1];
    let batch_size = &input.elems[2];

    let expanded = quote! {
        {
            let clients_for_sender = #clients.clone();
            let batch_size = #batch_size;
            tokio::spawn(async move {
                let mut ticker = tokio::time::interval(std::time::Duration::from_millis(#interval_ms));
                loop {
                    ticker.tick().await;
                    let mut clients = clients_for_sender.write().await;
                    for client in clients.values_mut() {
                        // Send multiple messages per tick for better throughput
                        let mut sent = 0;
                        while sent < batch_size {
                            if let Some(msg) = client.queue.pop_front() {
                                if client.sender.send(msg).is_err() {
                                    // Client disconnected, stop trying to send
                                    break;
                                }
                                sent += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            });
        }
    };
    expanded.into()
}

/// This macro runs the warp server.
#[proc_macro]
pub fn run_server(_input: TokenStream) -> TokenStream {
    // Usage:
    // run_server!(routes, addr)
    let input = parse_macro_input!(_input as syn::ExprTuple);
    let routes = &input.elems[0];
    let addr = &input.elems[1];

    let expanded = quote! {
        {
            println!("Netpix running on http://{}/", #addr);
            warp::serve(#routes).try_bind(#addr).await;
        }
    };
    expanded.into()
}
