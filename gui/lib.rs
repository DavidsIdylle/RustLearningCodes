/* pub trait Draw {
    fn draw(&self);
}
pub struct Screen<T: Draw> {  //同质集合（homogeneous collection），适合使用trait和泛型约束
    pub components: Vec<T>,   //单个Screen实例持有的Vec可以同时包含Box<Button>和Box<TextField>
}
impl<T> Screen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self){
        //实际绘制一个按钮的代码
    }
} */

//状态模式：将可能拥有的行为封装至不同状态中，状态对象的行为及相互转换的时机掌控，拥有状态的值对此是一无所知的
/* pub struct Post {
    state: Option<Box<dyn State>>,             //state采用Option<T>，使得state的值能够从Post中转移出
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft { })),  //保证了任何创建出的Post实例都会从草稿状态开始：Post的state字段是私有的
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {   //不能通过pub直接暴露content字段，必须控制访问content字段的具体行为
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)  //as_ref得到Option<Box<dyn State>>(引用)
                                                     //最终调用了实现了State trait的content方法
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {   //Option<T>的take方法取出Some值而留下None
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;   //该方法只能被包裹着当前类型的Box实例调用，
                                                            //它会在调用过程中获取Box<T>的所有权并使旧的状态失效
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {      //默认的trait实现：文章处于草稿状态时返回空的字符串切片
        ""
    }
}
//草稿状态
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})                          //消耗当前状态并返回一个新状态PendingReview
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
//审批状态
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {  //仅返回自己：文章发起审批请求不会改变文章的当前状态
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})                              //PendingReview在调用approve时会返回Published结构体的新实例
    }
}
//已发布状态
struct Published {}
impl State for Published {
    //处于Published状态下的文章不会被下列操作改变状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //处于Published状态下则会覆盖默认trait实现，返回post.content的值
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
} */
//状态模式的缺点： 1、状态间相互耦合；2、代码逻辑存在重复

//将状态和行为编码成类型，则状态转移可实现为不同类型之间的转换
pub struct Post { //已发布文章
    content: String,
}

pub struct DraftPost { //草稿
    content: String,
}
impl Post {
    pub fn new() -> DraftPost {  //关联函数new()返回DraftPost实例
        DraftPost {
            content: String::new(), //保证所有文章都是从草稿状态开始
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {  //获取了self的所有权，因此会消耗DraftPost实例，转换成PendingReviewPost
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post {  //获取了self的所有权，因此会消耗PendingReviewPost实例，转换成已发布的Post
        Post {
            content: self.content,
        }
    }
}