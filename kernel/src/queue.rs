#![allow(dead_code)]


use core::ptr;
//use core::marker::PhantomData;

pub trait QueueObject<T> {
    fn get_next(&self) -> *mut T;
    fn set_next(&mut self, next: *mut T);
    fn get_priority(&self) -> i32;
    fn queue_removed(&mut self);
}

//pub struct Queue<'a, T: QueueObject<T>> {
pub struct Queue<T: QueueObject<T>> {
    tail: *mut T,
//  marker: PhantomData<&'a Queue<'a, T>>,
}

/*
macro_rules! queue_default {
    () => {
        Queue {
            tail: core::ptr::null_mut(),
            marker: PhantomData,
        }
    }
}
*/

//impl<'a, T> Queue<'a, T> where
impl<T> Queue<T> where
    T: QueueObject<T>
{
    pub const fn new() -> Self {
//        queue_default!()
        Queue::<T> { tail: ptr::null_mut() }
    }

    
    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, obj: &mut T) {
        // タスクに所属キューを設定
//      task.queue = self as *mut TaskQueue;
        
        // 生ポインタ化
        let ptr: *mut T = obj as *mut T;
        
        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            obj.set_next(ptr);
            self.tail = ptr;
        } else {
            // キューが空でないなら挿入位置を探索
            // タスク優先度を取得
            let pri = obj.get_priority();
            
            // 先頭から探索
            let mut prev = self.tail;
            let mut next = unsafe{&*prev}.get_next();
            loop {
                // 優先度取り出し
                let next_pri = unsafe{&*next}.get_priority();

                if next_pri > pri {
                    break;
                }
                
                // 次を探す
                prev = next;
                next = unsafe{&*prev}.get_next();
                
                // 末尾なら抜ける
                if prev == self.tail {
                    self.tail = ptr;
                    break;
                }
            }

            // 挿入
            unsafe{&mut *prev}.set_next(ptr);
            obj.set_next(next);
        }
    }

    
    /// FIFO順で追加
    pub fn push_back(&mut self, obj: &mut T) {
        // 生ポインタ化
        let ptr = obj as *mut T;
        
        // タスクに所属キューを設定
//      task.queue = self as *mut TaskQueue;

        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            obj.set_next(ptr);
        }
        else {
            // キューが空でないなら末尾に追加
            let tail_obj = unsafe { &mut *self.tail };
            obj.set_next(tail_obj.get_next());
            tail_obj.set_next(ptr);
        }
        self.tail = ptr;
    }

    /// 先頭を参照
    pub fn front(&mut self) -> Option<&mut T> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let obj = unsafe { &mut *self.tail };
            Some(unsafe { &mut *obj.get_next() })
        }
    }

    /// 先頭を取り出し
    pub fn pop_front<'a, 'b>(&'a mut self) -> Option<&'b mut T> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let obj_tail = unsafe { &mut *self.tail };
            let obj_head = unsafe { &mut *obj_tail.get_next() };
            if self.tail == obj_tail.get_next() {
                self.tail = ptr::null_mut();
            } else {
                obj_tail.set_next(obj_head.get_next());
            }
            Some(obj_head)
        }
    }

    // 接続位置で時間が変わるので注意
    // 先頭しか外さない or タスク数を制約するなどで時間保証可能
    // 双方向リストする手はあるので、大量タスクを扱うケースが出たら考える
    pub fn remove(&mut self, obj: &mut T) {
        // 生ポインタ化
        let ptr = obj as *mut T;

        // 接続位置を探索
        if obj.get_next() == ptr {
            /* last one */
            self.tail = ptr::null_mut();
        } else {
            let mut prev_ptr = self.tail;
            let mut prev_obj = unsafe { &mut *prev_ptr };
            while prev_obj.get_next() != ptr {
                prev_ptr = prev_obj.get_next();
                prev_obj = unsafe { &mut *prev_ptr };
            }
            prev_obj.set_next(obj.get_next());
            if self.tail == ptr {
                self.tail = prev_ptr;
            }
        }

        // 取り外し
        obj.set_next(ptr::null_mut());
    }
}


impl<T> Drop for Queue<T> where
    T: QueueObject<T>
{
    fn drop(&mut self) {
        // 残っているオブジェクトがあれば削除されたことを知らせる
        while let Some(obj) = self.pop_front() {
            obj.queue_removed();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        id : i32,
        next : *mut TestObject,
    }

    impl TestObject {
        const fn new(id: i32) -> Self {
            TestObject{id:id, next: ptr::null_mut() }
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
        fn queue_removed(&mut self) {}
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
    fn test_queue_static () {
        static mut QUE: Queue::<TestObject> = Queue::<TestObject>::new();
        static mut OBJ0:TestObject = TestObject { id: 0, next: ptr::null_mut() };
        static mut OBJ1:TestObject = TestObject { id: 1, next: ptr::null_mut() };
        static mut OBJ2:TestObject = TestObject { id: 2, next: ptr::null_mut() };
        
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



