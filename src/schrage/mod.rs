pub mod heap_binary;
pub mod heap_tree;
pub mod std_heaps;
pub mod std_vecs;

pub mod task {
    #[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct Task {
        pub r: u32, // ready time
        pub p: u32, // working time
        pub q: u32, // cooldown time
    }

    impl Task {
        #[allow(unused)]
        pub fn new(r: u32, p: u32, q: u32) -> Task {
            Task { r, p, q }
        }
    }

    impl From<RInvariant> for Task {
        fn from(value: RInvariant) -> Self {
            value.0
        }
    }

    impl From<QInvariant> for Task {
        fn from(value: QInvariant) -> Self {
            value.0
        }
    }

    use std::fmt::Display;
    impl Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.r, self.p, self.q)
        }
    }

    use std::cmp::Ordering;

    #[derive(Eq, Debug)]
    pub struct QInvariant(pub Task);

    impl From<Task> for RInvariant {
        fn from(task: Task) -> Self {
            RInvariant(task)
        }
    }
    impl From<&Task> for RInvariant {
        fn from(task: &Task) -> Self {
            RInvariant(*task)
        }
    }
    impl PartialEq for RInvariant {
        fn eq(&self, other: &Self) -> bool {
            self.0.r == other.0.r
        }
    }
    impl PartialOrd for RInvariant {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.0.r.cmp(&self.0.r))
        }
    }
    // since RInvariant has to be in descending order
    //the order of comparison is (other) <=> (self) and not the other way around
    impl Ord for RInvariant {
        fn cmp(&self, other: &Self) -> Ordering {
            other.0.r.cmp(&self.0.r)
        }
    }

    #[derive(Eq, Debug)]
    pub struct RInvariant(pub Task);

    impl From<Task> for QInvariant {
        fn from(task: Task) -> Self {
            QInvariant(task)
        }
    }
    impl From<&Task> for QInvariant {
        fn from(task: &Task) -> Self {
            QInvariant(*task)
        }
    }
    impl PartialEq for QInvariant {
        fn eq(&self, other: &Self) -> bool {
            self.0.q == other.0.q
        }
    }
    impl PartialOrd for QInvariant {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.0.q.cmp(&other.0.q))
        }
    }
    impl Ord for QInvariant {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.q.cmp(&other.0.q)
        }
    }

    #[macro_export]
    #[allow(unused)]
    macro_rules! tasks {
        () => {
            vec![
                Task::new(30, 3, 8),
                Task::new(20, 4, 21),
                Task::new(10, 5, 7),
                Task::new(11, 7, 24),
                Task::new(30, 2, 0),
                Task::new(13, 6, 26),
                Task::new(0, 6, 17),
            ]
        };
    }
    #[macro_export]
    #[allow(unused)]
    macro_rules! correct_order {
        () => {
            vec![
                Task::new(0, 6, 17),
                Task::new(10, 5, 7),
                Task::new(13, 6, 26),
                Task::new(11, 7, 24),
                Task::new(20, 4, 21),
                Task::new(30, 3, 8),
                Task::new(30, 2, 0),
            ]
        };
    }
}

// pub trait PeekDown {
//     type Item;

//     fn peek_down(&mut self) -> Option<&Self::Item>;
// }

// use std::collections::BinaryHeap;

// impl<T: Ord> PeekDown for BinaryHeap<T> {
//     type Item = T;

//     fn peek_down(&mut self) -> Option<&Self::Item> {
//         let data = self.as_slice();

//         None
//     }
// }
