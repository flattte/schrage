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

