#![allow(dead_code)]

use core::ptr;

use num::Integer;
use core::marker::PhantomData;

pub trait TimeoutObject<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    fn difftim(&self) -> RELTIM;
    fn set_difftim(&mut self, difftim: RELTIM);
    fn next(&self) -> *mut OBJ;
    fn set_next(&mut self, obj: *mut OBJ);
    fn prev(&self) -> *mut OBJ;
    fn set_prev(&mut self, obj: *mut OBJ);
    fn timeout(&mut self);
    fn queue_dropped(&mut self);
}

pub struct TimeoutQueue<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    head: *mut OBJ,
    _marker: PhantomData<RELTIM>,
}

impl<OBJ, RELTIM> TimeoutQueue<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + Copy,
{
    pub const fn new() -> Self {
        TimeoutQueue::<OBJ, RELTIM> {
            head: ptr::null_mut(),
            _marker: PhantomData,
        }
    }


    pub (crate) fn add(&mut self, obj: &mut OBJ, tmout: RELTIM) {
        unsafe {
            let mut tmout = tmout;
            let ptr = obj as *mut OBJ;
            
            if self.head == ptr::null_mut() {
                // 最初の１つをキューに登録
                obj.set_next(ptr);
                obj.set_prev( ptr);
                self.head = ptr;
                
                // タイムアウト時刻を設定
                obj.set_difftim(tmout);
            }
            else {
                // 挿入場所を検索
                let mut next = self.head;
                let mut prev = (*next).prev();
                while {
                    let tmout_next = (*next).difftim();
                    
                    // 時間比較
                    if tmout < tmout_next {
                        // 先頭なら
                        if next == self.head {
                            self.head = ptr;	// 先頭ポインタ更新
                        }
                        
                        // 時間の差分を設定
                        (*next).set_difftim(tmout_next - tmout);
                        (*ptr).set_difftim(tmout);
                        
                        // リストに挿入
                        (*ptr).set_next(next);
                        (*ptr).set_prev(prev);
                        (*prev).set_next(ptr);
                        (*next).set_prev(ptr);
                        return;
                    }
                    
                    tmout = tmout - tmout_next;		// 差分を減算
                    
                    prev = next;
                    next = (*next).next();		// 次のオブジェクトへ進む
                    next != self.head  // リストを一周するまでループ
                }{}

                // 残った差分を設定
                (*ptr).set_difftim(tmout);
                
                // 末尾に追加
                (*ptr).set_next(next);
                (*ptr).set_prev(prev);
                (*prev).set_next(ptr);
                (*next).set_prev(ptr);
            }
        }
    }


    // タイムアウト行列からオブジェクトを取り除く
    pub (crate) fn remove(&mut self, obj: &mut OBJ) {
        unsafe {
            let ptr = obj as *mut OBJ;
            let prev = obj.prev();

            // タイムアウトキューに未接続なら無視
            if prev == ptr::null_mut() {
                return;
            }

            // キューの最後の１つなら 
            if prev == ptr {
                self.head = ptr::null_mut();    // タイムアウトキューを空にする
            }
            else
            {
                let next = (*ptr).next();
                let prev = (*ptr).prev();

                // 末尾でなければ
                if next != self.head {
                    // 時間差分を清算
                    (*next).set_difftim((*next).difftim() + (*ptr).difftim());
                }
                
                // 先頭なら
                if ptr == self.head {
                    self.head = next;	// 先頭位置更新
                }
                
                // キューから外す
                (*prev).set_next(next);
                (*next).set_prev(prev);
            }

            // 未接続に設定
            (*ptr).set_prev(ptr::null_mut());
        }
    }


    // タイムアウトにタイムティック供給
    pub (crate) fn sig_tim(&mut self, tictim: RELTIM) {
        unsafe {
            // 先頭タスク取得
            let mut tictim = tictim;
            let mut ptr = self.head;

            // タイムアウトキューが空ならリターン
            if ptr == ptr::null_mut() {
                return;
            }

            // タイムアウトキューの処理
            loop {
                let diftim = (*ptr).difftim();
                
                // タイムアウトに達しないなら
                if diftim > tictim {
                    (*ptr).set_difftim(diftim - tictim);    // タイムアウト時間を減算
                    break;
                }
                
                tictim = tictim - diftim;   // タイムティックを減算
                
                
                // キューから外す
                let next = (*ptr).next();
                let prev = (*ptr).prev();
                if next == ptr {	// 最後の１つなら
                    // キューを空にする
                    (*ptr).set_prev(ptr::null_mut());
                    (*ptr).timeout();   // タイムアウトを知らせる
                    ptr = ptr::null_mut();
                    break;
                }
                
                // キューから取り外す
                (*prev).set_next(next);
                (*next).set_prev(prev);
                (*ptr).set_prev(ptr::null_mut());

                // タイムアウトを知らせる
                (*ptr).timeout();
                
                // 次に進む
                ptr = next;
            }
            
            // 先頭を更新
            self.head = ptr;
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
            while self.head != ptr::null_mut() {
                let ptr = self.head;
                self.remove(&mut *ptr);
                (*ptr).queue_dropped();
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    static mut TEST_TIME:i32 = 0;

    struct TestObject {
        id: i32,
        time: i32,
        difftim: i32,
        next: *mut TestObject,
        prev: *mut TestObject,
    }

    impl TestObject {
        const fn new(id: i32) -> Self {
            TestObject {
                id: id,
                time: 0,
                difftim: 0,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }
        }
    }

    impl TimeoutObject<TestObject, i32> for TestObject {
        fn difftim(&self) -> i32{
            self.difftim
        }
        fn set_difftim(&mut self, difftim: i32) {
            self.difftim = difftim;
        }
    
        fn next(&self) -> *mut TestObject {
            self.next
        }
        fn set_next(&mut self, next: *mut TestObject) {
            self.next = next;
        }

        fn prev(&self) -> *mut TestObject {
            self.prev
        }        
        fn set_prev(&mut self, prev: *mut TestObject) {
            self.prev = prev;
        }

        fn timeout(&mut self) {
            self.time = unsafe{TEST_TIME};
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
        unsafe{ TEST_TIME = 0; };
        que.add(&mut obj0, 3);
        assert_eq!(que.head, &mut obj0 as *mut TestObject);
        que.add(&mut obj1, 1);
        assert_eq!(que.head, &mut obj1 as *mut TestObject);
        assert_eq!(obj1.next, &mut obj0 as *mut TestObject);
        assert_eq!(obj1.prev, &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next, &mut obj1 as *mut TestObject);
        assert_eq!(obj0.prev, &mut obj1 as *mut TestObject);

        que.add(&mut obj2, 4);
        assert_eq!(que.head, &mut obj1 as *mut TestObject);
        assert_eq!(obj1.next, &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next, &mut obj2 as *mut TestObject);
        assert_eq!(obj2.next, &mut obj1 as *mut TestObject);
        assert_eq!(obj1.prev, &mut obj2 as *mut TestObject);
        assert_eq!(obj2.prev, &mut obj0 as *mut TestObject);
        assert_eq!(obj0.prev, &mut obj1 as *mut TestObject);
        
        unsafe{ TEST_TIME += 1; };
        que.sig_tim(1);
        assert_eq!(obj0.time, 0);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);
        assert_eq!(que.head, &mut obj0 as *mut TestObject);
        assert_eq!(obj0.next, &mut obj2 as *mut TestObject);
        assert_eq!(obj2.next, &mut obj0 as *mut TestObject);
        assert_eq!(obj0.prev, &mut obj2 as *mut TestObject);
        assert_eq!(obj2.prev, &mut obj0 as *mut TestObject);
        

        unsafe{ TEST_TIME += 1; };
        que.sig_tim(1);
        assert_eq!(obj0.time, 0);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);

        unsafe{ TEST_TIME += 1; };
        que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 0);

        unsafe{ TEST_TIME += 1; }; que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 4);

        unsafe{ TEST_TIME += 1; }; que.sig_tim(1);
        assert_eq!(obj0.time, 3);
        assert_eq!(obj1.time, 1);
        assert_eq!(obj2.time, 4);
        assert_eq!(que.head, ptr::null_mut());
        assert_eq!(obj0.prev, ptr::null_mut());
        assert_eq!(obj1.prev, ptr::null_mut());
        assert_eq!(obj2.prev, ptr::null_mut());
    }

    #[test]
    fn test_queue_002() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 一括時間経過
        unsafe{ TEST_TIME = 0; };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);
        
        unsafe{ TEST_TIME += 4; };
        que.sig_tim(4);
        assert_eq!(obj0.time, 4);
        assert_eq!(obj1.time, 4);
        assert_eq!(obj2.time, 4);
        assert_eq!(que.head, ptr::null_mut());
        assert_eq!(obj0.prev, ptr::null_mut());
        assert_eq!(obj1.prev, ptr::null_mut());
        assert_eq!(obj2.prev, ptr::null_mut());
    }

    #[test]
    fn test_queue_003() {
        let mut que = TimeoutQueue::<TestObject, i32>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        // 先頭削除
        unsafe{ TEST_TIME = 0; };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj1);

        for _ in 0..5 {
            unsafe{ TEST_TIME += 1; };
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
        unsafe{ TEST_TIME = 0; };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj0);

        for _ in 0..5 {
            unsafe{ TEST_TIME += 1; };
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
        unsafe{ TEST_TIME = 0; };
        que.add(&mut obj0, 3);
        que.add(&mut obj1, 1);
        que.add(&mut obj2, 4);

        que.remove(&mut obj2);

        for _ in 0..5 {
            unsafe{ TEST_TIME += 1; };
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
