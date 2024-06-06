pub mod paths;

#[macro_export]
macro_rules! maybe {
    ($block:block) => {
        (|| $block)()
    };
    (async $block:block) => {
        (|| async $block)()
    };
    (async move $block:block) => {
        (|| async move $block)()
    };
}
