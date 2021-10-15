#![allow(dead_code)]

use core::ptr;
use num::Integer;
use core::marker::PhantomData;

pub trait PriorityObject<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    fn priority(&self) -> PRI;
    fn queue(&self) -> *mut PriorityQueue<OBJ, PRI>;
    fn set_queue(&mut self, que: *mut PriorityQueue<OBJ, PRI>);
    fn next(&self) -> *mut OBJ;
    fn set_next(&mut self, next: *mut OBJ);
    fn queue_dropped(&mut self);
}

pub struct PriorityQueue<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    tail: *mut OBJ,
    _marker: PhantomData<PRI>,
}

impl<OBJ, PRI> PriorityQueue<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    pub const fn new() -> Self {
        PriorityQueue::<OBJ, PRI> {
            tail: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue(), ptr::null_mut());
        obj.set_queue(self as *mut Self);

        // 生ポインタ化
        let ptr: *mut OBJ = obj as *mut OBJ;

        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            obj.set_next(ptr);
            self.tail = ptr;
        } else {
            // キューが空でないなら挿入位置を探索
            // タスク優先度を取得
            let pri = obj.priority();

            // 先頭から探索
            let mut prev = self.tail;
            let mut next = unsafe { &*prev }.next();
            loop {
                // 優先度取り出し
                let next_pri = unsafe { &*next }.priority();

                if next_pri > pri {
                    break;
                }

                // 次を探す
                prev = next;
                next = unsafe { &*prev }.next();

                // 末尾なら抜ける
                if prev == self.tail {
                    self.tail = ptr;
                    break;
                }
            }

            // 挿入
            unsafe { &mut *prev }.set_next(ptr);
            obj.set_next(next);
        }
    }

    /// FIFO順で追加
    pub fn push_back(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue(), ptr::null_mut());
        obj.set_queue(self as *mut Self);

        // 生ポインタ化
        let ptr = obj as *mut OBJ;

        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            obj.set_next(ptr);
        } else {
            // キューが空でないなら末尾に追加
            let tail_obj = unsafe { &mut *self.tail };
            obj.set_next(tail_obj.next());
            tail_obj.set_next(ptr);
        }
        self.tail = ptr;
    }

    /// 先頭を参照
    pub fn front(&mut self) -> Option<&mut OBJ> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let obj = unsafe { &mut *self.tail };
            Some(unsafe { &mut *obj.next() })
        }
    }

    /// 先頭を取り出し
    pub fn pop_front<'a, 'b>(&'a mut self) -> Option<&'b mut OBJ> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let obj_tail = unsafe { &mut *self.tail };
            let obj_head = unsafe { &mut *obj_tail.next() };
            if self.tail == obj_tail.next() {
                self.tail = ptr::null_mut();
            } else {
                obj_tail.set_next(obj_head.next());
            }
            obj_head.set_queue(ptr::null_mut());
            Some(obj_head)
        }
    }

    // 接続位置で時間が変わるので注意
    // 先頭しか外さない or タスク数を制約するなどで時間保証可能
    // 双方向リストする手はあるので、大量タスクを扱うケースが出たら考える
    pub fn remove(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue(), self as *mut Self);

        // 生ポインタ化
        let ptr = obj as *mut OBJ;

        // 接続位置を探索
        if obj.next() == ptr {
            /* last one */
            self.tail = ptr::null_mut();
        } else {
            let mut prev_ptr = self.tail;
            let mut prev_obj = unsafe { &mut *prev_ptr };
            while prev_obj.next() != ptr {
                prev_ptr = prev_obj.next();
                prev_obj = unsafe { &mut *prev_ptr };
            }
            prev_obj.set_next(obj.next());
            if self.tail == ptr {
                self.tail = prev_ptr;
            }
        }

        // 取り外し
        obj.set_next(ptr::null_mut());
        obj.set_queue(ptr::null_mut());
    }
}

impl<OBJ, PRI> Drop for PriorityQueue<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
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
        que: *mut PriorityQueue<TestObject, i32>,
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

    impl PriorityObject<TestObject, i32> for TestObject {
        fn next(&self) -> *mut TestObject {
            self.next
        }
        fn set_next(&mut self, next: *mut TestObject) {
            self.next = next;
        }
        fn priority(&self) -> i32 {
            self.id
        }
        fn queue(&self) -> *mut PriorityQueue<TestObject, i32> {
            self.que
        }

        fn set_queue(&mut self, que: *mut PriorityQueue<TestObject, i32>) {
            self.que = que;
        }

        fn queue_dropped(&mut self) {}
    }

    #[test]
    fn test_queue() {
        let mut que = PriorityQueue::<TestObject, i32>::new();
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
            assert_eq!(que.front().unwrap().priority(), 0);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().priority(), 0);
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().priority(), 0);

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
            assert_eq!(que.front().unwrap().priority(), 2);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().priority(), 1);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().priority(), 0);

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
            assert_eq!(que.front().unwrap().priority(), 1);
            que.insert_priority_order(&mut obj2);
            assert_eq!(que.front().unwrap().priority(), 1);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().priority(), 0);

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
            assert_eq!(que.front().unwrap().priority(), 2);
            que.insert_priority_order(&mut obj0);
            assert_eq!(que.front().unwrap().priority(), 0);
            que.insert_priority_order(&mut obj1);
            assert_eq!(que.front().unwrap().priority(), 0);

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
        static mut QUE: PriorityQueue<TestObject, i32> = PriorityQueue::<TestObject, i32>::new();
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
