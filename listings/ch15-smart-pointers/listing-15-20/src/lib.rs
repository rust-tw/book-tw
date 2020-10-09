pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("錯誤：你超過使用上限了！");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("緊急警告：你已經使用 90% 的配額了！");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("警告：你已經使用 75% 的配額了！");
        }
    }
}
