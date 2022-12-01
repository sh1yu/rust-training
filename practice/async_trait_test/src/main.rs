use async_trait::async_trait;
use core::future::Future;
use tokio;

/// 异步函数，trait和泛型下的生命周期问题
// https://rustcc.cn/article?id=6ab02bf2-5130-4774-b294-5a4b73126e2d

/// 包含异步函数的范型:App，该范型继承了Sync
#[async_trait]
trait App: Sync {
    async fn call(&self, value: &String) -> String;
}

/// 对于形如`Send + Sync + Fn(&String) -> Fut`类型的所有函数实现App trait
/// 实际上，满足这种类型的函数就是异步函数`async fn some_func(value: &String) -> String`
// #[async_trait]
// impl<T, Fut> App for T
// where
//     T: Send + Sync + Fn(&String) -> Fut,
//     Fut: Send + Sync + Future<Output = String>,
// {
//     async fn call(&self, value: &String) -> String {
//         // 直接调用自己
//         self(value).await
//     }
// }

/// 上面的实现不一定可用，如果使用wrapper包一层的话，使用下面的实现
/// 为App实现wrapper trait
#[async_trait]
impl<T> App for T
where
    T: Wrapper,
{
    async fn call(&self, value: &String) -> String {
        self.wrapped_call(value).await
    }
}

// 直接这样实现一个异步范型似乎有问题，报错one type is more general than the other：
// static dyn_tester: &dyn App = &tester;

/// 实现一个wrapper trait
trait Wrapper: Send + Sync {
    // wrapper的Res类型就是App中需要返回的future
    type Res: Future<Output = String> + Send + Sync;
    // wrapped_call的表现就如同app的call一样，不同的是这里表现上看不是async的（虽然返回了Future）
    fn wrapped_call(&self, s: &String) -> Self::Res;
}

/// 对于形如`Send + Sync + Fn(&String) -> Fut`类型的所有函数实现 Wrapper trait
/// 实际上，满足这种类型的函数就是异步函数`async fn some_func(value: &String) -> String`
impl<F, Fut> Wrapper for F
where
    F: Send + Sync + Fn(&String) -> Fut,
    Fut: Send + Sync + Future<Output = String>,
{
    type Res = Fut;
    fn wrapped_call(&self, s: &String) -> Self::Res {
        self(s)
    }
}

///////////////////////
///////////////////////

#[tokio::main]
async fn main() {
    // 直接这样实现一个异步范型似乎有问题，报错one type is more general than the other
    // 如果直接使用 let dyn_tester = &tester; 会在调用call时报错`fn_traits` is unstable
    // let dyn_tester: &dyn App = &tester;
    // dyn_tester.call(&String::from("233")).await;

    println!("{}", tester(&String::from("233")).await);
}

/// 满足`Send + Sync + Fn(&String) -> Fut`类型的函数
async fn tester(string: &String) -> String {
    string.into()
}
