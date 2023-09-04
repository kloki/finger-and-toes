#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::{
    thread_rng,
    Rng,
};

const SIZE: usize = 20;

#[derive(PartialEq, Clone, Copy)]
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
        div {
            style { include_str!("../src/style.css")}
            header{ "👐 Fingers and Toes 🦶"}
            div{
                class:"top",
                RandomButton{
                    current_number:current_number,
                    game_state:game_state,
                    onclick_reset: move |_| {game_state.set(GameState::Progress); board.set([0;SIZE])},
                }
                GameState{game_state:game_state, current_number:current_number}
            }

            div{class:"tiles",
                tiles}
        }
    })
}

#[inline_props]
fn RandomButton<'a>(
    cx: Scope,
    current_number: &'a UseRef<usize>,
    game_state: &'a UseRef<GameState>,
    onclick_reset: EventHandler<'a, MouseEvent>,
) -> Element {
    let mut rng = thread_rng();
    match (*game_state.read(), *current_number.read()) {
        (GameState::Won, _) => cx.render(rsx! {
        button{
            class:"randombutton",
            onclick: move |event| onclick_reset.call(event),
            " 👑 "}}),
        (GameState::Lost, _) => cx.render(rsx! {
        button{
            class:"randombutton",
            onclick: move |event| onclick_reset.call(event),
            " 🙊 "}}),
        (GameState::Progress, n) if n != 0 => cx.render(rsx! {
        button{
            class:"randombutton",
            disabled:true,"
            {current_number.read()}"}}),
        _ => cx.render(rsx! {
        button{
            class:"randombutton",
            onclick: move |_| current_number.set(rng.gen_range(1..=1000)),
            "Spin!"}}),
    }
}

fn game_lost(board: [usize; SIZE], index: usize) -> bool {
    let current = &board[index];
    let (left, right) = board.split_at(index);

    for l in left.iter().rev() {
        if l > current {
            return true;
        }
    }
    for r in right.iter() {
        if r != &0 && r < current {
            return true;
        }
    }
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
    match (board.read()[*index], *current_number.read()) {
        (0, 0) => cx.render(rsx! {button{class:"tile",disabled:true,"{*index+1}"} }),
        (0, _) => cx.render(rsx! {
        button{
            class:"tile",
            onclick: move |_| {
                board.write()[*index] = *current_number.read();
                current_number.set(0);
                if game_lost(*board.read(),*index){
                    game_state.set(GameState::Lost);
                } else if game_won(*board.read()){
                    game_state.set(GameState::Won)
            }
        },"{*index+1}"} }),
        (s, _) => cx.render(rsx! {button{class:"tile_filled",disabled:true,"{s}"} }),
    }
}

#[inline_props]
fn GameState<'a>(
    cx: Scope,
    game_state: &'a UseRef<GameState>,
    current_number: &'a UseRef<usize>,
) -> Element {
    let text = match (*game_state.read(), *current_number.read()) {
        (GameState::Progress, n) if n == 0 => "Spin for a random number!",
        (GameState::Progress, _)  => "Pick a slot to place the number. You'll have to place all your numbers in order. The number range is 1-1000. So choose wisely",
        (GameState::Won, _)  => "You won!. 🍀 You're very lucky! 🍀 Try again!",
        (GameState::Lost, _)  => "You lost! Atleast you didn't loose all of your digits! 🔪 Try again.",
    };

    cx.render(rsx! {
        div{class:"gamestate",
        "{text}"
        }
    })
}
