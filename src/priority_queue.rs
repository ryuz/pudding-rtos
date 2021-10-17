#![allow(dead_code)]

use core::marker::PhantomData;
use core::ptr::NonNull;
use num::Integer;

pub trait PriorityObject<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    fn priority(&self) -> PRI;
    fn queue(&self) -> Option<NonNull<PriorityQueue<OBJ, PRI>>>;
    fn set_queue(&mut self, que: Option<NonNull<PriorityQueue<OBJ, PRI>>>);
    fn next(&self) -> Option<NonNull<OBJ>>;
    fn set_next(&mut self, next: Option<NonNull<OBJ>>);
    fn queue_dropped(&mut self);
}

pub struct PriorityQueue<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    tail: Option<NonNull<OBJ>>,
    _marker: PhantomData<PRI>,
}


impl<OBJ, PRI> PriorityQueue<OBJ, PRI>
where
    OBJ: PriorityObject<OBJ, PRI>,
    PRI: Integer,
{
    pub const fn new() -> Self {
        PriorityQueue::<OBJ, PRI> {
            tail: None,
            _marker: PhantomData,
        }
    }

    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue(), None);
        obj.set_queue(Some(unsafe{ NonNull::new_unchecked(self as *mut Self) }));

        // ポインタ化
        let ptr = unsafe{NonNull::new_unchecked(obj as *mut OBJ)};

        match self.tail {
            None => {
                // キューにタスクが無ければ先頭に設定
                obj.set_next(Some(ptr));
                self.tail = Some(ptr);
            },
            Some(tail) => {
                // キューが空でないなら挿入位置を探索
                // タスク優先度を取得
                let pri = obj.priority();
                
                // 先頭から探索
                let mut prev = tail;
                let mut next = unsafe { prev.as_mut().next().unwrap_unchecked() };
                loop {
                    // 優先度取り出し
                    let next_pri = unsafe { next.as_ref().priority() };

                    if next_pri > pri {
                        break;
                    }

                    // 次を探す
                    prev = next;
                    next = unsafe { prev.as_mut().next().unwrap_unchecked() };

                    // 末尾なら抜ける
                    if prev == tail {
                        self.tail = Some(ptr);
                        break;
                    }
                }

                // 挿入
                unsafe { prev.as_mut().set_next(Some(ptr)); }
                obj.set_next(Some(next));
            }
        }
    }
    

    /// FIFO順で追加
    pub fn push_back(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue(), None);
        obj.set_queue(Some(unsafe{ NonNull::new_unchecked(self as *mut Self) }));

        // ポインタ化
        let ptr = unsafe{NonNull::new_unchecked(obj as *mut OBJ)};

        match self.tail {
            None => {
                // キューにタスクが無ければ先頭に設定
                obj.set_next(Some(ptr));
            },
            Some(mut tail_ptr) => {
                // キューが空でないなら末尾に追加
                obj.set_next(unsafe{tail_ptr.as_mut().next()});
                unsafe{ tail_ptr.as_mut().set_next(Some(ptr)); }
            }
        }
        self.tail = Some(ptr);
    }

    /// 先頭を参照
    pub fn front(&mut self) -> Option<&mut OBJ> {
        match self.tail {
            None => { None },
            Some(mut tail_ptr) => {
                Some(unsafe{tail_ptr.as_mut().next().unwrap_unchecked().as_mut()})
            }
        }
    }

    /// 先頭を取り出し
    pub fn pop_front<'a, 'b>(&'a mut self) -> Option<&'b mut OBJ> {
        match self.tail {
            None => {None},
            Some(mut tail) => {
                let obj_tail = unsafe { tail.as_mut() };
                let obj_head = unsafe { obj_tail.next().unwrap_unchecked().as_mut() };
                if self.tail == obj_tail.next() {
                    self.tail = None;
                } else {
                    obj_tail.set_next(obj_head.next());
                }
                obj_head.set_queue(None);
                Some(obj_head)
            }
        }
    }


    // 接続位置で時間が変わるので注意
    // 先頭しか外さない or タスク数を制約するなどで時間保証可能
    // 双方向リストする手はあるので、大量タスクを扱うケースが出たら考える
    pub fn remove(&mut self, obj: &mut OBJ) {
        debug_assert_eq!(obj.queue().unwrap().as_ptr(), self as *mut Self);

        // ポインタ化
        let ptr = unsafe{NonNull::new_unchecked(obj as *mut OBJ)};

        // 接続位置を探索
        let next = unsafe{obj.next().unwrap_unchecked()};
        if next == ptr {
            /* last one */
            self.tail = None;
        } else {
            let mut prev = unsafe{self.tail.unwrap_unchecked()};
            while unsafe{prev.as_mut().next().unwrap_unchecked()} != ptr {
                prev = unsafe{prev.as_mut().next().unwrap_unchecked()};
            }
            unsafe{prev.as_mut().set_next(obj.next())};
            if unsafe{self.tail.unwrap_unchecked()} == ptr {
                self.tail = Some(prev);
            }
        }

        // 取り外し
        obj.set_next(None);
        obj.set_queue(None);
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
        next: Option<NonNull<TestObject>>,
        que: Option<NonNull<PriorityQueue<TestObject, i32>>>,
    }

    impl TestObject {
        const fn new(id: i32) -> Self {
            TestObject {
                id: id,
                next: None,
                que: None,
            }
        }
    }

    impl PriorityObject<TestObject, i32> for TestObject {
        fn next(&self) -> Option<NonNull<TestObject>> {
            self.next
        }
        fn set_next(&mut self, next: Option<NonNull<TestObject>>) {
            self.next = next;
        }
        fn priority(&self) -> i32 {
            self.id
        }
        fn queue(&self) -> Option<NonNull<PriorityQueue<TestObject, i32>>> {
            self.que
        }

        fn set_queue(&mut self, que: Option<NonNull<PriorityQueue<TestObject, i32>>>) {
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
            assert_eq!(que.tail.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
            que.remove(&mut obj0);
            assert_eq!(que.tail.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
            que.remove(&mut obj1);
            assert_eq!(que.tail, None);

            let t0 = que.pop_front();
            assert_eq!(t0.is_some(), false);
        }

        {
            // 削除パターン2
            que.push_back(&mut obj0);
            que.push_back(&mut obj1);
            assert_eq!(que.tail.unwrap().as_ptr(), &mut obj1 as *mut TestObject);
            que.remove(&mut obj1);
            assert_eq!(que.tail.unwrap().as_ptr(), &mut obj0 as *mut TestObject);
            que.remove(&mut obj0);
            assert_eq!(que.tail, None);

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
