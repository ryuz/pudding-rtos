#![allow(dead_code)]

//use core::ptr;
use core::marker::PhantomData;
use core::ptr::NonNull;
use num::Integer;

pub trait TimeoutObject<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    fn difftim(&self) -> RELTIM;
    fn set_difftim(&mut self, difftim: RELTIM);
    fn next(&self) -> Option<NonNull<OBJ>>;
    fn set_next(&mut self, obj: Option<NonNull<OBJ>>);
    fn prev(&self) -> Option<NonNull<OBJ>>;
    fn set_prev(&mut self, obj: Option<NonNull<OBJ>>);
    fn timeout(&mut self);
    fn queue_dropped(&mut self);
}

pub struct TimeoutQueue<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    head: Option<NonNull<OBJ>>,
    _marker: PhantomData<RELTIM>,
}

impl<OBJ, RELTIM> TimeoutQueue<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    pub const fn new() -> Self {
        TimeoutQueue::<OBJ, RELTIM> {
            head: None,
            _marker: PhantomData,
        }
    }

    pub(crate) fn add(&mut self, obj: &mut OBJ, tmout: RELTIM) {
        let mut tmout = tmout;

        // ポインタ化
        let mut ptr = unsafe { NonNull::new_unchecked(obj as *mut OBJ) };

        match self.head {
            None => {
                // 最初の１つをキューに登録
                obj.set_next(Some(ptr));
                obj.set_prev(Some(ptr));
                self.head = Some(ptr);

                // タイムアウト時刻を設定
                obj.set_difftim(tmout);
            }
            Some(head) => {
                // 挿入場所を検索
                let mut next = head;
                let mut prev = unsafe { next.as_mut().prev().unwrap_unchecked() };
                while {
                    let tmout_next = unsafe { next.as_ref() }.difftim();

                    // 時間比較
                    if tmout < tmout_next {
                        // 先頭なら
                        if next == head {
                            self.head = Some(ptr); // 先頭ポインタ更新
                        }

                        // 時間の差分を設定
                        unsafe { next.as_mut() }.set_difftim(tmout_next - tmout);
                        unsafe { ptr.as_mut() }.set_difftim(tmout);

                        // リストに挿入
                        unsafe { ptr.as_mut() }.set_next(Some(next));
                        unsafe { ptr.as_mut() }.set_prev(Some(prev));
                        unsafe { prev.as_mut() }.set_next(Some(ptr));
                        unsafe { next.as_mut() }.set_prev(Some(ptr));
                        return;
                    }

                    tmout = tmout - tmout_next; // 差分を減算

                    prev = next;
                    next = unsafe { next.as_mut().next().unwrap_unchecked() }; // 次のオブジェクトへ進む
                    Some(next) != self.head // リストを一周するまでループ
                } {}

                // 残った差分を設定
                unsafe { ptr.as_mut() }.set_difftim(tmout);

                // 末尾に追加
                unsafe { ptr.as_mut() }.set_next(Some(next));
                unsafe { ptr.as_mut() }.set_prev(Some(prev));
                unsafe { prev.as_mut() }.set_next(Some(ptr));
                unsafe { next.as_mut() }.set_prev(Some(ptr));
            }
        }
    }

    // タイムアウト行列からオブジェクトを取り除く
    pub(crate) fn remove(&mut self, obj: &mut OBJ) {
        // ポインタ化
        let mut ptr = unsafe { NonNull::new_unchecked(obj as *mut OBJ) };

        let prev = obj.prev();

        // タイムアウトキューに未接続なら無視
        if prev.is_none() {
            return;
        }

        if prev == Some(ptr) {
            // キューの最後の１つなら
            self.head = None; // タイムアウトキューを空にする
        } else {
            let mut next = unsafe { ptr.as_mut().next().unwrap_unchecked() };
            let mut prev = unsafe { ptr.as_mut().prev().unwrap_unchecked() };

            // 末尾でなければ
            if Some(next) != self.head {
                // 時間差分を清算
                unsafe { next.as_mut() }.set_difftim(
                    unsafe { next.as_ref() }.difftim() + unsafe { ptr.as_ref() }.difftim(),
                );
            }

            // 先頭なら
            if Some(ptr) == self.head {
                self.head = Some(next); // 先頭位置更新
            }

            // キューから外す
            unsafe { prev.as_mut() }.set_next(Some(next));
            unsafe { next.as_mut() }.set_prev(Some(prev));
        }

        // 未接続に設定
        unsafe { ptr.as_mut() }.set_prev(None);
    }

    // タイムアウトにタイムティック供給
    pub(crate) fn sig_tim(&mut self, tictim: RELTIM) {
        match self.head {
            None => return,
            Some(mut ptr) => {
                let mut tictim = tictim;
                // タイムアウトキューの処理
                loop {
                    let diftim = unsafe { ptr.as_ref() }.difftim();

                    // タイムアウトに達しないなら
                    if diftim > tictim {
                        unsafe { ptr.as_mut() }.set_difftim(diftim - tictim); // タイムアウト時間を減算
                        break;
                    }

                    tictim = tictim - diftim; // タイムティックを減算

                    // キューから外す
                    let mut next = unsafe { ptr.as_ref().next().unwrap_unchecked() };
                    let mut prev = unsafe { ptr.as_ref().prev().unwrap_unchecked() };
                    if next == ptr {
                        // 最後の１つなら
                        // キューを空にする
                        unsafe { ptr.as_mut() }.set_prev(None);
                        self.head = None;

                        unsafe { ptr.as_mut() }.timeout(); // タイムアウトを知らせる
                        return;
                    }

                    // キューから取り外す
                    unsafe { prev.as_mut() }.set_next(Some(next));
                    unsafe { next.as_mut() }.set_prev(Some(prev));
                    unsafe { ptr.as_mut() }.set_prev(None);

                    // タイムアウトを知らせる
                    unsafe { ptr.as_mut() }.timeout(); // タイムアウトを知らせる

                    // 次に進む
                    ptr = next;
                }

                self.head = Some(ptr); // 先頭を更新
            }
        }
    }
}

impl<OBJ, RELTIM> Drop for TimeoutQueue<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    fn drop(&mut self) {
        unsafe {
            // 残っているオブジェクトがあれば削除されたことを知らせる
            while !self.head.is_none() {
                let mut ptr = self.head.unwrap_unchecked();
                self.remove(ptr.as_mut());
                ptr.as_mut().queue_dropped();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut TEST_TIME: i32 = 0;

    struct TestObject {
        id: i32,
        time: i32,
        difftim: i32,
        next: Option<NonNull<TestObject>>,
        prev: Option<NonNull<TestObject>>,
    }

    impl TestObject {
        const fn new(id: i32) -> Self {
            TestObject {
                id: id,
                time: 0,
                difftim: 0,
                next: None,
                prev: None,
            }
        }
    }

    impl TimeoutObject<TestObject, i32> for TestObject {
        fn difftim(&self) -> i32 {
            self.difftim
        }
        fn set_difftim(&mut self, difftim: i32) {
            self.difftim = difftim;
        }

        fn next(&self) -> Option<NonNull<TestObject>> {
            self.next
        }
        fn set_next(&mut self, next: Option<NonNull<TestObject>>) {
            self.next = next;
        }

        fn prev(&self) -> Option<NonNull<TestObject>> {
            self.prev
        }
        fn set_prev(&mut self, prev: Option<NonNull<TestObject>>) {
            self.prev = prev;
        }

        fn timeout(&mut self) {
            self.time = unsafe { TEST_TIME };
        }

        fn queue_dropped(&mut self) {}
    }

    #[test]
    fn test_queue_001() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 単純追加＆時間経過
        unsafe {
            TEST_TIME = 0;
        };
        que.add(&mut obj0, 3);
        assert_eq!(que.head.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        que.add(&mut obj1, 1);
        assert_eq!(que.head.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
        assert_eq!(obj1.next.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj1.prev.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
        assert_eq!(obj0.prev.unwrap().as_ptr(), &mut obj1 as *mut TestObject);

        que.add(&mut obj2, 4);
        assert_eq!(que.head.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
        assert_eq!(obj1.next.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next.unwrap().as_ptr(), &mut obj2 as *mut TestObject);
        assert_eq!(obj2.next.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
        assert_eq!(obj1.prev.unwrap().as_ptr(), &mut obj2 as *mut TestObject);
        assert_eq!(obj2.prev.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj0.prev.unwrap().as_ptr(), &mut obj1 as *mut TestObject);

        unsafe {
            TEST_TIME += 1;
        };
        que.sig_tim(1);
        assert_eq!(obj0.time, 0);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);
        assert_eq!(que.head.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next.unwrap().as_ptr(), &mut obj2 as *mut TestObject);
        assert_eq!(obj2.next.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
        assert_eq!(obj0.prev.unwrap().as_ptr(), &mut obj2 as *mut TestObject);
        assert_eq!(obj2.prev.unwrap().as_ptr(), &mut obj0 as *mut TestObject);

        unsafe {
            TEST_TIME += 1;
        };
        que.sig_tim(1);
        assert_eq!(obj0.time, 0);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);

        unsafe {
            TEST_TIME += 1;
        };
        que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);

        unsafe {
            TEST_TIME += 1;
        };
        que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 4);

        unsafe {
            TEST_TIME += 1;
        };
        que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 4);
        assert_eq!(que.head, None);
        assert_eq!(obj0.prev, None);
        assert_eq!(obj1.prev, None);
        assert_eq!(obj2.prev, None);
    }

    #[test]
    fn test_queue_002() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 一括時間経過
        unsafe {
            TEST_TIME = 0;
        };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        unsafe {
            TEST_TIME += 4;
        };
        que.sig_tim(4);
        assert_eq!(obj0.time, 4);
        assert_eq!(obj1.time, 4);
        assert_eq!(obj2.time, 4);
        assert_eq!(que.head, None);
        assert_eq!(obj0.prev, None);
        assert_eq!(obj1.prev, None);
        assert_eq!(obj2.prev, None);
    }

    #[test]
    fn test_queue_003() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 先頭削除
        unsafe {
            TEST_TIME = 0;
        };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj1);

        for _ in 0..5 {
            unsafe {
                TEST_TIME += 1;
            };
            que.sig_tim(1);
        }
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 0);
        assert_eq!(obj2.time, 4);
    }

    #[test]
    fn test_queue_004() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 中間削除
        unsafe {
            TEST_TIME = 0;
        };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj0);

        for _ in 0..5 {
            unsafe {
                TEST_TIME += 1;
            };
            que.sig_tim(1);
        }
        assert_eq!(obj0.time, 0);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 4);
    }

    #[test]
    fn test_queue_005() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 末尾削除
        unsafe {
            TEST_TIME = 0;
        };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj2);

        for _ in 0..5 {
            unsafe {
                TEST_TIME += 1;
            };
            que.sig_tim(1);
        }
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);
    }

    #[test]
    fn test_queue_static() {
        static mut QUE: TimeoutQueue<TestObject, i32> = TimeoutQueue::<TestObject, i32>::new();
        static mut OBJ0: TestObject = TestObject::new(0);
        static mut OBJ1: TestObject = TestObject::new(1);
        static mut OBJ2: TestObject = TestObject::new(2);

        unsafe {
            // 単純追加＆時間経過し
            TEST_TIME = 0;
            QUE.add(&mut OBJ0, 3);
            QUE.add(&mut OBJ1, 1);
            QUE.add(&mut OBJ2, 4);

            for _ in 0..5 {
                TEST_TIME += 1;
                QUE.sig_tim(1);
            }
            assert_eq!(OBJ0.time, 3);
            assert_eq!(OBJ1.time, 1);
            assert_eq!(OBJ2.time, 4);
        }
    }
}
