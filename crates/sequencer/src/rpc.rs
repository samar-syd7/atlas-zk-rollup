use axum::{
    extract::State,
    routing::{
        get,
        post,
    },
    Json,
    Router,
};

use serde::{
    Serialize,
    Deserialize,
};

use std::{
    net::SocketAddr,
    sync::Arc,
};

use crate::mempool::Mempool;

use primitives::tx::RollupTx;

#[derive(Clone)]
pub struct RpcState {

    pub mempool:
        Arc<Mempool>,
}

#[derive(
    Serialize,
    Deserialize,
)]
pub struct SubmitTxRequest {

    pub from: [u8; 32],

    pub to: [u8; 32],

    pub amount: u64,

    pub nonce: u64,
}

pub async fn submit_tx(

    State(state): State<RpcState>,

    Json(payload): Json<SubmitTxRequest>,

) -> Json<&'static str> {

    let tx = RollupTx {

        from: payload.from,

        to: payload.to,

        amount: payload.amount,

        nonce: payload.nonce,
    };

    state
        .mempool
        .insert(tx);

    Json("tx accepted")
}

pub async fn health() -> Json<&'static str> {

    Json("rollup alive")
}

pub async fn start_rpc(

    mempool: Arc<Mempool>,
) {

    let state =
        RpcState {
            mempool,
        };

    let app =
        Router::new()

            .route(
                "/health",
                get(health),
            )

            .route(
                "/tx",
                post(submit_tx),
            )

            .with_state(state);

    let addr =
        SocketAddr::from(
            ([127,0,0,1], 3000)
        );

    println!(
        "rpc listening on {}",
        addr
    );

    let listener =
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap();

    axum::serve(
        listener,
        app,
    )
    .await
    .unwrap();
}