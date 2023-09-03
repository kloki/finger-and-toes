#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::{
    thread_rng,
    Rng,
};

const SIZE: usize = 3;

#[derive(PartialEq)]
enum GameState {
    Progress,
    Lost,
    Won,
}

pub fn App(cx: Scope) -> Element {
    let current_number = use_ref(cx, || 0);
    let board = use_ref(cx, || [0; SIZE]);
    let game_state = use_ref(cx, || GameState::Progress);
    let tiles = (0..SIZE).map(|i| {
        rsx!(Tile {
            board: board,
            current_number: current_number,
            game_state: game_state,
            index: i
        })
    });
    cx.render(rsx! {
        GameStateHeader{board:board,game_state:game_state}
        div { "{current_number.read()}" }
        RandomButton{current_number:current_number, game_state:game_state}
        div{"board"}
        div{tiles}
    })
}

#[inline_props]
fn RandomButton<'a>(
    cx: Scope,
    current_number: &'a UseRef<usize>,
    game_state: &'a UseRef<GameState>,
) -> Element {
    let mut rng = thread_rng();
    if GameState::Progress != *game_state.read() || *current_number.read() != 0 {
        return None;
    }
    cx.render(rsx! {
        button{ onclick: move |_| current_number.set(rng.gen_range(1..=1000)),"Spin!"}
    })
}

fn game_lost(board: [usize; SIZE], index: usize) -> bool {
    if index != 0 && board[index - 1] != 0 && board[index - 1] >= board[index] {
        return true;
    };
    if index != SIZE - 1 && board[index + 1] != 0 && board[index] >= board[index + 1] {
        return true;
    };
    false
}

fn game_won(board: [usize; SIZE]) -> bool {
    !board.iter().any(|x| *x == 0)
}

#[inline_props]
fn Tile<'a>(
    cx: Scope,
    board: &'a UseRef<[usize; SIZE]>,
    current_number: &'a UseRef<usize>,
    game_state: &'a UseRef<GameState>,
    index: usize,
) -> Element {
    if board.read()[*index] == 0 && *current_number.read() != 0 {
        cx.render(rsx! {
            div{"{board.read()[*index]}"}
            button{onclick: move |_| {
                board.write()[*index] = *current_number.read();
                current_number.set(0);
                if game_lost(*board.read(),*index){
                    game_state.set(GameState::Lost);
                } else if game_won(*board.read()){
                    game_state.set(GameState::Won)
                }
            },"Set"}
        })
    } else {
        cx.render(rsx! {
            div{"{board.read()[*index]}"}
        })
    }
}

#[inline_props]
fn GameStateHeader<'a>(
    cx: Scope,
    board: &'a UseRef<[usize; SIZE]>,
    game_state: &'a UseRef<GameState>,
) -> Element {
    match *game_state.read() {
        GameState::Progress => None,
        GameState::Won => cx.render(rsx! {
            div {"Won! "}
            button{ onclick: move |_| {game_state.set(GameState::Progress); board.set([0;SIZE])},"Try again!"}
        }),
        GameState::Lost => cx.render(rsx! {
            div {"Lost! "}
            button{ onclick: move |_| {game_state.set(GameState::Progress); board.set([0;SIZE])},"Try again!"}
        }),
    }
}
