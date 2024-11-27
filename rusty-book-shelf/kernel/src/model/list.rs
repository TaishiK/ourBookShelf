#[derive(Debug)]
pub struct PaginatedList<T> {
    pub total: i64,
    pub limit: i64,//取得上限件数
    pub offset: i64,//取得開始位置
    pub items: Vec<T>,
}
impl<T> PaginatedList<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.items//itemsフィールドにVec<T>型として格納されている値を取り出す
    }
}