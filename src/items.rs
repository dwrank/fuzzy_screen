
//use crate::debug::log;

#[derive(Clone)]
pub struct ScreenItem {
    pub name: String,
    pub value: f64,
}

impl ScreenItem {
    pub fn new(s: &String) -> Self {
        ScreenItem {
            name: s.clone(),
            value: 1.0,
        }
    }
}

pub struct ScreenItems {
    item_vec: Vec<ScreenItem>,  // display strings
    rows: i32,  // display rows
    pub max_display_items: i32,  // max num of items to display
    num_items: i32,          // total num of items
    pub num_display_items: i32,  // num of items to display
    start: i32,  // display start index
    end: i32,    // display end index
    last: i32,   // last index
    sel: i32,    // selected index
}

impl ScreenItems {
    pub fn new(item_vec: &[ScreenItem], rows: i32) -> ScreenItems {
        let mut fscreen = ScreenItems {
            item_vec: item_vec.to_vec(),
            rows,
            max_display_items: 0,
            num_items: 0,
            num_display_items: 0,
            start: 0,
            end: 0,
            last: 0,
            sel: 0,
        };

        fscreen.init();
        fscreen
    }

    fn init(&mut self) {
        self.max_display_items = self.rows - 2;

        self.num_items = self.item_vec.len() as i32;
        self.end = self.num_items - 1;
        self.start = 0;
        self.last = self.end;
        self.sel = self.end;
        
        if self.num_items > self.max_display_items {
            self.start = self.end - self.max_display_items + 1;
        }
        else {
            self.start = self.end - self.num_items + 1;
        }

        self.num_display_items = self.end - self.start + 1;
    }
}

impl ScreenItems {
    pub fn num_items(&self) -> i32 {
        self.num_items
    }

    pub fn selected(&self) -> i32 {
        self.sel
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn item_vec_display(&self) -> &[ScreenItem] {
        let start = self.start as usize;
        let stop = self.end as usize + 1;
        &self.item_vec[start..stop]
    }
}

impl ScreenItems {
    pub fn select_up(&mut self) {
        if self.sel > 0 {
            self.sel -= 1;

            if self.sel < self.start {
                self.start -= 1;
                self.end -= 1;
            }
        }
    }

    pub fn select_down(&mut self) {
        if self.sel < self.last {
            self.sel += 1;

            if self.sel > self.end {
                self.start += 1;
                self.end += 1;
            }
        }
    }

    pub fn select_page_up(&mut self) {
        let n = self.num_display_items;

        if self.sel > self.start {
            self.sel = self.start;
        }
        else {
            self.sel -= n;

            if self.sel < 0 {
                self.sel = 0;
            }

            self.start = self.sel;
            self.end = self.start + n - 1;
        }
        //log(&format!("\n{} {} {} {}\n", self.start, self.end, self.sel, self.num_display_items)[..]);
    }

    pub fn select_page_down(&mut self) {
        let n = self.num_display_items;

        if self.sel < self.end {
            self.sel = self.end;
        }
        else {
            self.sel += n;

            if self.sel > self.last {
                self.sel = self.last;
            }

            self.end = self.sel;
            self.start = self.end - n + 1;
        }
    }
}

