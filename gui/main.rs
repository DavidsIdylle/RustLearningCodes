/* use gui::Draw;
struct SelectBox {
    width: u32, 
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        //实际绘制一个选择框的代码
    }
}
use gui:: {Screen, Button};
fn main() {
    //创建Screen实例
    let screen = Screen {
        components: vec![
            //使用Box<T>来生成SelectBox和Button的trait对象
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
} */

// Rust在使用trait约束时执行单态化；使用trait对象时实现动态派发
//trait的对象安全规则： 1、方法的返回类型不是Self； 2、方法中不包含任何泛型参数
/*
pub trait Clone {
    fn clone(&self) -> Self;  //不符合对象安全的trait，因此不能将Clone trait用于trait对象
}
*/
use blog::Post;
fn main() {
    let mut post = Post::new();

    //post.add_text("I ate a salad for lunch today");  //草稿状态自由添加文字，
    //assert_eq!("", post.content());                  //但在文章发布前无法获取文章内容

    //post.request_review();                           //可以发出审批文章的请求，
    //assert_eq!("", post.content());                  //但处于等待阶段的content方法仍然返回空字符串

    //post.approve();                                  //已发布状态则可返回完整的文章内容

    let post = post.request_review();
    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());
    //用户与库交互所涉及的数据类型只有Post类型
    //该类型采用状态模式：草稿、等待审批、已发布
    //状态变化的行为会在调用Post实例的对应方法时发生，但用户不需要直接对此过程进行管理
}