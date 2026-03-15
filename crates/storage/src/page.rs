pub enum BPlusTreePage <K,V> {
    Internal(BPlusTreeInternalPage <K,V>),
    Leaf(BPlusTreeLeafPage <K,V>),
}

pub enum BPlusTreePageType {
    LeafPage,
    InternalPage,
}


#[derive(Debug, Clone)]
pub struct InternalKV<K> {
    pub key: K,
    pub page_id: PageId, // this page id links to the downward node
}

#[derive(Debug, Clone)]
pub struct BPlusTreeInternalPageHeader {
    pub page_type: BPlusTreePageType,
    pub current_size: u32,
    pub max_size: u32,       
    pub next_page_id: PageId, // this page id links to the right sibling horizontally
}

#[derive(Debug, Clone)]
pub struct BPlusTreeInternalPage<K> {
    pub header: BPlusTreeInternalPageHeader,
    
    // In Rust, using a flat vector is common, but remember:
    // This is conceptually an array where Index 0 has an empty Key, 
    // and just holds the leftmost Child PageId.
    pub elements: Vec<InternalKV<K>>,
    
}

//(Key -> Full Row Data)
#[derive(Debug, Clone)]
pub struct LeafKV<K, V> {
    pub key: K,
    pub row_data: V, // complete DB struct
}
// max_size of leaf page will be considerably smaller since we are storing actual row data here. 
#[derive(Debug, Clone)]
pub struct BPlusTreeLeafPageHeader {
    pub page_type: BPlusTreePageType,
    pub current_size: u32,
    pub max_size: u32,
    pub next_page_id: PageId, // pointer to right sibling(horizontally)
}

#[derive(Debug, Clone)]
pub struct BPlusTreeLeafPage<K, V> {
    pub header: BPlusTreeLeafPageHeader,
    pub elements: Vec<LeafKV<K, V>>,
}