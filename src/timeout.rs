#![allow(dead_code)]

use core::ptr;

use num::Integer;
use core::marker::PhantomData;

pub trait TimeoutObject<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer,
{
//    fn queue(&self) -> *mut Timeout<OBJ, RELTIM>;
//    fn set_queue(&mut self, tmout: *mut Timeout<OBJ, RELTIM>);
    fn difftim(&self) -> RELTIM;
    fn set_difftim(&self, difftim: RELTIM);
    fn next(&self) -> *mut OBJ;
    fn set_next(&mut self, obj: *mut OBJ);
    fn prev(&self) -> *mut OBJ;
    fn set_prev(&mut self, obj: *mut OBJ);
    fn timeout(&mut self);
    fn timeout_dropped(&mut self);
}

pub struct Timeout<OBJ, RELTIM>
{
    head: *mut OBJ,
    _marker: PhantomData<RELTIM>,
}

impl<OBJ, RELTIM> Timeout<OBJ, RELTIM>
where
    OBJ: TimeoutObject<OBJ, RELTIM>,
    RELTIM: Integer + From<i32> + Copy,
{
    pub const fn new() -> Self {
        Timeout::<OBJ, RELTIM> {
            head: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    pub fn hoge(&self) -> RELTIM {
        RELTIM::from(3)
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
                let mut next = (*self.head).next();
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



/*


impl<T> Drop for Queue<T>
where
    T: QueueObject<T>,
{
    fn drop(&mut self) {
        // 残っているオブジェクトがあれば削除されたことを知らせる
        while let Some(obj) = self.pop_front() {
            obj.queue_dropped();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        id: i32,
        next: *mut TestObject,
        que: *mut Queue<TestObject>,
    }

    impl TestObject {
        const fn new(id: i32) -> Self {
            TestObject {
                id: id,
                next: ptr::null_mut(),
                que: ptr::null_mut(),
            }
        }
    }

    impl QueueObject<TestObject> for TestObject {
        fn get_next(&self) -> *mut TestObject {
            self.next
        }
        fn set_next(&mut self, next: *mut TestObject) {
            self.next = next;
        }
        fn get_priority(&self) -> i32 {
            self.id
        }
        fn get_queue(&self) -> *mut Queue<TestObject> {
            self.que
        }

        fn set_queue(&mut self, que: *mut Queue<TestObject>) {
            self.que = que;
        }

        fn queue_dropped(&mut self) {}
    }

    #[test]
    fn test_queue() {
        let mut que = Queue::<TestObject>::new();
        let mut obj0 = TestObject::new(0);
        let mut obj1 = TestObject::new(1);
        let mut obj2 = TestObject::new(2);

        {
            // 単純追加＆取り出し
            que.push_back(&mut obj0);
            que.push_back(&mut obj1);
            que.push_back(&mut obj2);
            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            let t3 = que.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }

        {
            // 削除パターン1
            que.push_back(&mut obj0);
            que.push_back(&mut obj1);
            assert_eq!(que.tail, &mut obj1 as *mut TestObject);
            que.remove(&mut obj0);
            assert_eq!(que.tail, &mut obj1 as *mut TestObject);
            que.remove(&mut obj1);
            assert_eq!(que.tail, ptr::null_mut());

            let t0 = que.pop_front();
            assert_eq!(t0.is_some(), false);
        }

        {
            // 削除パターン2
            que.push_back(&mut obj0);
            que.push_back(&mut obj1);
            assert_eq!(que.tail, &mut obj1 as *mut TestObject);
            que.remove(&mut obj1);
            assert_eq!(que.tail, &mut obj0 as *mut TestObject);
            que.remove(&mut obj0);
            assert_eq!(que.tail, ptr::null_mut());

            let t0 = que.pop_front();
            assert_eq!(t0.is_some(), false);
        }

        {
            // 優先度順パターン1
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().get_priority(), 0);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().get_priority(), 0);
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().get_priority(), 0);

            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            let t3 = que.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }

        {
            // 優先度順パターン2
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().get_priority(), 2);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().get_priority(), 1);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().get_priority(), 0);

            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            let t3 = que.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }
        {
            // 優先度順パターン3
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().get_priority(), 1);
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().get_priority(), 1);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().get_priority(), 0);

            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            let t3 = que.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }
        {
            // 優先度順パターン4
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().get_priority(), 2);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().get_priority(), 0);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().get_priority(), 0);

            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            let t3 = que.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }
    }

    #[test]
    fn test_queue_static() {
        static mut QUE: Queue<TestObject> = Queue::<TestObject>::new();
        static mut OBJ0: TestObject = TestObject::new(0);
        static mut OBJ1: TestObject = TestObject::new(1);
        static mut OBJ2: TestObject = TestObject::new(2);

        unsafe {
            // 単純追加＆取り出し
            QUE.push_back(&mut OBJ0);
            QUE.push_back(&mut OBJ1);
            QUE.push_back(&mut OBJ2);
            let t0 = QUE.pop_front();
            let t1 = QUE.pop_front();
            let t2 = QUE.pop_front();
            let t3 = QUE.pop_front();
            assert_eq!(t0.unwrap().id, 0);
            assert_eq!(t1.unwrap().id, 1);
            assert_eq!(t2.unwrap().id, 2);
            assert_eq!(t3.is_some(), false);
        }
    }
}
*/