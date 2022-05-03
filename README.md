# Carts

## Goals of the repo
Write a simple codebase combining FP and DDD.

## Problem description
Some carts have to be moved across a rectangular grid. A cart position on the grid is determined by a set of coordinates x and y plus a direction represented by one of the four cardinal points: north, east, south and west. The grid is defined by width and height and the bottom left of the grid can be assumed to be positioned at coordinates 0,0.

Each cart starts from a given position (e.g. 1,2 EAST) and its moved processing a sequence of commands. The commands are:
* move: advance the cart onto the grid of 1 by keeping the same direction
* left: rotate the direction to 90 degrees to the left, keeping the same coordinates
* right: rotate the direction to 90 degrees to the right, keeping the same coordinates


Implement a program that given a cart and a sequence of commands, move the cart accordingly on the grid.

## Setup
Install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

## How to run
In the root folder: `cargo run`

## How to test
In the root folder: `cargo test`


