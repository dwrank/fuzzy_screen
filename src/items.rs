
//use crate::debug::log;

#[derive(Clone)]
pub struct ScreenItem {
    pub name: String,
    pub value: f64,
}

impl ScreenItem {
    pub fn new(s: String) -> Self {
        ScreenItem {
            name: s,
            value: 1.0,
        }
    }
}

pub struct ScreenManager {
    base_items: Vec<ScreenItem>,  // display strings
    max_display_items: i32,  // max num of items to display
    num_items: i32,          // total num of items
    num_display_items: i32,  // num of items to display
    start: i32,  // display start index
    end: i32,    // display end index
    last: i32,   // last index
    sel: i32,    // selected index
}

impl ScreenManager {
    pub fn new(base_items: Vec<ScreenItem>, max_display_items: i32) -> ScreenManager {
        let mut fscreen = ScreenManager {
            base_items,
            max_display_items,
            num_items: 0,
            num_display_items: 0,
            start: 0,
            end: 0,
            last: 0,
            sel: 0,
        };

        fscreen.init(fscreen.base_items.len() as i32);
        fscreen
    }

    fn init(&mut self, num_items: i32) {
        self.num_items = num_items;
        if num_items == 0 {
            self.start = 0;
            self.end = 0;
            self.last = 0;
            self.sel = 0;
            self.num_display_items = 0;
        }
        else {
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
}

impl ScreenManager {
    pub fn base_num_items(&self) -> i32 {
        self.base_items.len() as i32
    }

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

    pub fn display_items(&self) -> Option<&[ScreenItem]> {
        if self.num_items > 0 {
            let start = self.start as usize;
            let stop = self.end as usize + 1;
            let base_stop = self.base_items.len();
            let base_start = base_stop - self.num_items as usize;
            let items = &self.base_items[base_start..base_stop];
            Some(&items[start..stop])
        }
        else {
            None
        }
    }
}

impl ScreenManager {
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
        //log(&format!("\n{} {} {} {}\n", self.start, self.end, self.sel, self.num_display_items));
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

impl ScreenManager {
    pub fn fuzzy_sort(&mut self, search_str: &str) {
        for i in 0..self.base_items.len() {
            self.base_items[i].value = fuzzy_match(search_str, &self.base_items[i].name);
            //self.base_items[i].value = normalized_damerau_levenshtein(search_str, &self.base_items[i].name);
        }
        self.base_items.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

        let mut n = 0;
        for (_, item) in self.base_items.iter().enumerate() {
            if item.value != 0.0 {
                break;
            }
            n += 1;
        }

        self.init((self.base_items.len() - n) as i32);
    }
}

// s1 is the search str
// s2 is the list item str
fn fuzzy_match(s1: &str, s2: &str) -> f64 {
    let mut value = 1.0;
    let l1 = s1.len();
    let l2 = s2.len();
    let penalty = 1.0 / l2 as f64;
    //log(&format!("fuzzy_value: {} {}\n", s1 , s2));

    if l1 > l2 {
        value = 0.0;
    }
    else {
        let mut i2 = 0;
        let mut s2 = s2;
        for c1 in s1.chars() {
            s2 = &s2[i2..];
            //log(&format!("i2: {} -> {}\n", i2 , s2));
            match s2.find(c1) {
                Some(n) => {
                    //log(&format!("match n: {}\n", n));
                    value -= n as f64 * penalty;
                    //log(&format!("new value: {}\n", value));
                    i2 = n + 1;
                    //log(&format!("new i2: {}\n", i2));
                }
                None => {
                    value = 0.0;
                    break;
                }
            }
        }
    }

    value
}

