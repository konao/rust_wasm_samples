// *************************************************************
//  迷路生成部分
//
//  2020/8/21 konao
// *************************************************************

#![allow(dead_code)]
#![allow(non_snake_case)]

mod utils;

use wasm_bindgen::prelude::*;

use js_sys; // 乱数生成にjsのMath::random()を使うため

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    //  jsのconsole.logをRustから使えるようにする
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);    
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Space = 0,
    Wall = 1,
    TempWall = 2    // 迷路作成時に使う一時的な壁
}

#[wasm_bindgen]
pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

fn getRandomDir() -> (i32, i32) {
    let dir = js_sys::Math::random();

    if dir < 0.25 {
        return (0, -1); // up
    } else if dir < 0.5 {
        return (1, 0);  // right
    } else if dir < 0.75 {
        return (0, 1);  // down
    } else {
        return (-1, 0); // left
    }
}

#[wasm_bindgen]
impl Maze {
    fn getIndex(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn new() -> Maze {
        let width: u32 = 109;
        let height: u32 = 69;

        // 最初は全セル空白
        let mut cells : Vec<Cell> = (0..width * height)
            .map(|_| {
                Cell::Space
            })
            .collect();

        // ヘルパー関数
        // cellsの(x, y)にcをセット
        fn setCell(x: i32, y: i32, c: Cell, cells: &mut Vec<Cell>, width: u32) {
            cells[(x + y*(width as i32)) as usize] = c;
        };
        // cellsの(x, y)の値を返す
        fn getCell(x: i32, y: i32, cells: &mut Vec<Cell>, width: u32) -> Cell {
            cells[(x + y*(width as i32)) as usize]
        };

        // 周りの壁を作る
        for x in 0..width as i32 {
            setCell(x, 0, Cell::Wall, &mut cells, width);
            setCell(x, (height-1) as i32, Cell::Wall, &mut cells, width);
        }

        for y in 0..height as i32 {
            setCell(0, y, Cell::Wall, &mut cells, width);
            setCell((width-1) as i32, y, Cell::Wall, &mut cells, width);
        }

        // ----------------------------------------------------------
        // 迷路本体作成
        //
        // ＜アルゴリズム＞
        // 迷路内の各点から一時的な壁(TempWall)をランダムに伸ばす
        // 壁にぶつかれば一時的な壁を本物の壁(=Wall)に変える
        // TempWallにぶつかったらやり直し
        // これをすべての迷路内の点について行う．
        // ----------------------------------------------------------
        let mut yStart :i32 = 2;
        let mut xStart :i32 = 2;
        while yStart<=((height as i32)-3) {
            while xStart<=((width as i32)-3) {

                // (xStart, yStart)から一時的な壁をランダムに伸ばす
                let mut bEstablished = false;   // ルートが確定したらtrueになる
                while !bEstablished {
                    let mut x: i32=xStart;
                    let mut y: i32=yStart;
                    let mut dirRecord: Vec<(i32, i32)> = vec![];    // 壁を伸ばした軌跡

                    if getCell(x, y, &mut cells, width) == Cell::Wall {
                        // すでに壁になっているなら次へ
                        bEstablished = true;
                        continue;
                    }

                    setCell(x, y, Cell::TempWall, &mut cells, width);
                    loop {
                        // 壁を伸ばす方向を乱数で得る
                        let (dx, dy) = getRandomDir();
                        // log(&format!("(dx, dy)=({}, {})", dx, dy));

                        // ルートを記録
                        dirRecord.push((dx, dy));
        
                        // 伸ばした先
                        let px = x + (dx*2);
                        let py = y + (dy*2);
                        
                        match getCell(px, py, &mut cells, width) {                            
                            Cell::Wall => {
                                // 通ってきたルートをWallにセット
                                let mut x2: i32 = xStart;
                                let mut y2: i32 = yStart;
                                
                                setCell(x2, y2, Cell::Wall, &mut cells, width);
                                for (dx2, dy2) in &dirRecord {
                                    setCell(x2+dx2, y2+dy2, Cell::Wall, &mut cells, width);
                                    setCell(x2+dx2*2, y2+dy2*2, Cell::Wall, &mut cells, width);
                                    x2 += dx2*2;
                                    y2 += dy2*2;
                                }

                                bEstablished = true;    // ルート確定
                                break;
                            }
                            Cell::TempWall => {
                                // やり直し
                                // 通ってきたルートをSpaceに戻す
                                let mut x2: i32 = xStart;
                                let mut y2: i32 = yStart;
                                
                                setCell(x2, y2, Cell::Space, &mut cells, width);
                                for (dx2, dy2) in &dirRecord {
                                    setCell(x2+dx2, y2+dy2, Cell::Space, &mut cells, width);
                                    setCell(x2+dx2*2, y2+dy2*2, Cell::Space, &mut cells, width);
                                    x2 += dx2*2;
                                    y2 += dy2*2;
                                }
                                break;
                            }
                            _ => {
                                // 通過したセルをTempWallにする
                                setCell(px, py, Cell::TempWall, &mut cells, width);
                                x=px;
                                y=py;
                            }
                        }
                    }
                }

                xStart+=2;
            }
            xStart=2;
            yStart+=2;
        }

        Maze {
            width,
            height,
            cells
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
