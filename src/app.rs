#![allow(non_snake_case)]

use dioxus::prelude::*;
use rand::{
    thread_rng,
    Rng,
};

const SIZE: usize = 3;

fn invalid_board(board: [usize; SIZE]) -> bool {
    for i in 1..(SIZE - 2) {
        if board[i] != 0 && board[i - 1] != 0 && board[i] < board[i - 1] {
            return true;
        }
    }
    false
}

pub fn App(cx: Scope) -> Element {
    let current_number = use_ref(cx, || 0);
    let board = use_ref(cx, || [0; SIZE]);

    let lost = invalid_board(*board.read());
    if lost {
        return cx.render(rsx! {
            div { "lost" }
        });
    }

    let tiles = (0..SIZE).map(|i| {
        rsx!(Tile {
            board: board,
            current_number: current_number,
            index: i
        })
    });
    cx.render(rsx! {
        div { "{current_number.read()}" }
        RandomButton{current_number:current_number}
        div{"board"}
        div{tiles}
    })
}

#[inline_props]
pub fn RandomButton<'a>(cx: Scope, current_number: &'a UseRef<usize>) -> Element {
    let mut rng = thread_rng();
    if *current_number.read() != 0 {
        return None;
    }
    cx.render(rsx! {
        button{ onclick: move |_| current_number.set(rng.gen_range(1..=1000)),"Spin!"}
    })
}

#[inline_props]
pub fn Tile<'a>(
    cx: Scope,
    board: &'a UseRef<[usize; SIZE]>,
    current_number: &'a UseRef<usize>,
    index: usize,
) -> Element {
    if board.read()[*index] == 0 && *current_number.read() != 0 {
        cx.render(rsx! {
            div{"{board.read()[*index]}"}
            button{onclick: move |_| {
                if *current_number.read() != 0 {
                    board.write()[*index] = *current_number.read();
                    current_number.set(0)
                }
            },"Set"}
        })
    } else {
        cx.render(rsx! {
            div{"{board.read()[*index]}"}
        })
    }
}
